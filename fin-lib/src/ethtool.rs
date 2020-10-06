use std::fs::{read_dir, File};
use std::os::unix::io::{AsRawFd, FromRawFd};

use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType};

use crate::i2c::probe_i2c_pca9633;

const SYSFS_ETH_NET: &str = "/sys/devices/platform/soc/3f980000.usb/usb1/1-1/1-1.1/1-1.1:1.0/net";

const SIOCETHTOOL: u16 = 0x8946;

const FIN_EEPROM_MAGIC: u32 = 0x9500;
const FIN_EEPROM_OFFSET: u32 = 0x27;
const FIN_EEPROM_LEN: usize = 21;

const ETHTOOL_GEEPROM: u32 = 0x0000_000b;
const ETHTOOL_SEEPROM: u32 = 0x0000_000c;

type IfName = [u8; libc::IFNAMSIZ];

type EEPROMData = [u8; FIN_EEPROM_LEN];

ioctl_readwrite_bad!(ioctl_ethtool, SIOCETHTOOL, ifreq);

#[derive(Clone, Debug)]
struct FinEEPROM {
    schema: u8,
    revision: String,
    serial: String,
    week: String,
    year: String,
    lot: String,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct ifmap {
    mem_start: libc::c_ulong,
    mem_end: libc::c_ulong,
    base_addr: libc::c_ushort,
    irq: libc::c_uchar,
    dma: libc::c_uchar,
    port: libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
union ifr_ifru {
    ifr_addr: libc::sockaddr,
    ifr_dstaddr: libc::sockaddr,
    ifr_broadaddr: libc::sockaddr,
    ifr_netmask: libc::sockaddr,
    ifr_hwaddr: libc::sockaddr,
    ifr_flags: libc::c_short,
    ifr_ifindex: libc::c_int,
    ifr_metric: libc::c_int,
    ifr_mtu: libc::c_int,
    ifr_map: ifmap,
    ifr_slave: IfName,
    ifr_newname: IfName,
    ifr_data: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    ifr_name: IfName,
    ifr_ifru: ifr_ifru,
}

impl ifreq {
    fn from_name(name: &str) -> Option<ifreq> {
        let mut req: ifreq = unsafe { ::std::mem::zeroed() };
        req.set_name(name)?;
        Some(req)
    }

    fn set_name(&mut self, name: &str) -> Option<()> {
        let name_c = ::std::ffi::CString::new(name).unwrap();
        let name_slice = name_c.as_bytes_with_nul();
        if name_slice.len() > libc::IFNAMSIZ {
            return None;
        }
        self.ifr_name[..name_slice.len()].clone_from_slice(name_slice);
        Some(())
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct fin_ethtool_eeprom {
    cmd: u32,
    magic: u32,
    offset: u32,
    len: u32,
    data: EEPROMData,
}

impl Default for fin_ethtool_eeprom {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub fn get_revision() -> String {
    if let Some(eeprom_revision) = get_eeprom_revision() {
        eeprom_revision
    } else if probe_i2c_pca9633().is_some() {
        "10".to_string()
    } else {
        "09".to_string()
    }
}

fn get_eeprom_revision() -> Option<String> {
    let data = get_eeprom_data()?;
    let parsed = parse_eeprom_data(&data)?;
    Some(parsed.revision)
}

pub fn get_uid() -> Option<String> {
    let data = get_eeprom_data()?;
    let parsed = parse_eeprom_data(&data)?;
    Some(format!("{}{}{}", parsed.serial, parsed.week, parsed.year))
}

pub fn set_eeprom(eeprom: &str) -> Option<()> {
    let data = eeprom_data_from_str(eeprom)?;
    set_eeprom_data(data)
}

pub fn get_eeprom() -> Option<String> {
    let data = get_eeprom_data()?;
    eeprom_data_to_string(&data)
}

fn get_eeprom_data() -> Option<EEPROMData> {
    let ctl_fd = create_control_socket()?;
    let interface = get_builtin_eth_interface()?;
    read_eeprom_data(&ctl_fd, &interface)
}

fn set_eeprom_data(data: EEPROMData) -> Option<()> {
    let ctl_fd = create_control_socket()?;
    let interface = get_builtin_eth_interface()?;
    write_eeprom_data(&ctl_fd, &interface, data)
}

fn eeprom_data_from_str(eeprom: &str) -> Option<EEPROMData> {
    let data_slice = eeprom.as_bytes();
    if data_slice.len() != FIN_EEPROM_LEN {
        return None;
    }
    let mut data: EEPROMData = unsafe { ::std::mem::zeroed() };
    data.clone_from_slice(data_slice);
    parse_eeprom_data(&data)?;
    Some(data)
}

fn eeprom_data_to_string(data: &EEPROMData) -> Option<String> {
    String::from_utf8(data.to_vec()).ok()
}

fn get_builtin_eth_interface() -> Option<String> {
    let dir_iter = read_dir(SYSFS_ETH_NET).ok()?;
    let entry = dir_iter.last()?;
    let dir = entry.ok()?;
    dir.file_name().into_string().ok()
}

fn create_control_socket() -> Option<File> {
    if let Ok(raw_fd) = socket(
        AddressFamily::Inet,
        SockType::Datagram,
        SockFlag::empty(),
        None,
    ) {
        Some(unsafe { File::from_raw_fd(raw_fd) })
    } else {
        None
    }
}

fn read_eeprom_data<F: AsRawFd>(ctl_fd: &F, ifname: &str) -> Option<EEPROMData> {
    let mut req = ifreq::from_name(ifname)?;
    let mut ereq = fin_ethtool_eeprom::default();

    ereq.cmd = ETHTOOL_GEEPROM;
    ereq.offset = FIN_EEPROM_OFFSET;
    ereq.len = std::mem::size_of::<EEPROMData>() as u32;
    req.ifr_ifru.ifr_data = &mut ereq as *mut _ as *mut _;

    if unsafe { ioctl_ethtool(ctl_fd.as_raw_fd(), &mut req) }.is_err() {
        return None;
    }

    Some(ereq.data)
}

fn write_eeprom_data<F: AsRawFd>(ctl_fd: &F, ifname: &str, data: EEPROMData) -> Option<()> {
    let mut req = ifreq::from_name(ifname)?;
    let mut ereq = fin_ethtool_eeprom::default();

    ereq.cmd = ETHTOOL_SEEPROM;
    ereq.magic = FIN_EEPROM_MAGIC;
    ereq.offset = FIN_EEPROM_OFFSET;
    ereq.len = std::mem::size_of::<EEPROMData>() as u32;
    ereq.data = data;
    req.ifr_ifru.ifr_data = &mut ereq as *mut _ as *mut _;

    if unsafe { ioctl_ethtool(ctl_fd.as_raw_fd(), &mut req) }.is_err() {
        return None;
    }

    Some(())
}

fn parse_eeprom_data(data: &EEPROMData) -> Option<FinEEPROM> {
    let data = String::from_utf8(data.to_vec()).ok()?;

    let schema: u8 = data[0..1].parse().ok()?;
    if schema != 1 {
        return None;
    }

    let revision_u16: u16 = data[1..3].parse().ok()?;
    if revision_u16 < 10 {
        return None;
    }
    let revision: String = data[1..3].to_string();

    let serial: String = data[3..8].to_string();

    let week_u8: u8 = data[8..10].parse().ok()?;
    if week_u8 < 1 || week_u8 > 52 {
        return None;
    }
    let week: String = data[8..10].to_string();

    let year_u8: u8 = data[10..12].parse().ok()?;
    if year_u8 < 17 {
        return None;
    }
    let year: String = data[10..12].to_string();

    let lot: String = data[12..21].to_string();
    if lot.chars().nth(4)? != '-' {
        return None;
    }

    Some(FinEEPROM {
        schema,
        revision,
        serial,
        week,
        year,
        lot,
    })
}
