use std::fs::{File, OpenOptions};
use std::io::Read;
use std::os::unix::io::AsRawFd;

const PCA9633_I2C_ADDRESS: i32 = 0x62;

const I2C_SLAVE_FORCE: u16 = 0x0706;

ioctl_write_int_bad!(set_i2c_slave_address, I2C_SLAVE_FORCE);

pub fn probe_i2c_pca9633() -> Option<()> {
    let mut file = open_dev_i2c_3()?;
    unsafe { set_i2c_slave_address(file.as_raw_fd(), PCA9633_I2C_ADDRESS) }.ok()?;
    let mut buffer = [0; 1];
    file.read(&mut buffer).ok()?;
    Some(())
}

fn open_dev_i2c_3() -> Option<File> {
    OpenOptions::new().read(true).open("/dev/i2c-3").ok()
}

