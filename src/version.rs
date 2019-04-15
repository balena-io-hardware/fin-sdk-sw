use std::fs::{File, OpenOptions};
use std::io;
use std::io::Read;
use std::os::unix::io::AsRawFd;

const PCA9633_I2C_ADDRESS: u16 = 0x62;

const I2C_SLAVE_FORCE: u16 = 0x0706;

ioctl_write_int_bad!(set_i2c_slave_address, I2C_SLAVE_FORCE);

pub fn print_version() {
    let mut version = "1.0";

    if probe_i2c_pca9633() {
        version = "1.1";
    }

    println!("{}", version);
}

fn probe_i2c_pca9633() -> bool {
    if let Ok(mut file) = open_dev_i2c_3() {
        match unsafe { set_i2c_slave_address(file.as_raw_fd(), PCA9633_I2C_ADDRESS as i32) } {
            Ok(_) => {
                let mut buffer = [0; 1];
                if let Ok(_) = file.read(&mut buffer) {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    } else {
        false
    }
}

fn open_dev_i2c_3() -> Result<File, io::Error> {
    OpenOptions::new().read(true).open("/dev/i2c-3")
}
