#[macro_use]
extern crate nix;

extern crate libc;

mod ethtool;
mod i2c;

pub use ethtool::{get_revision, get_eeprom, set_eeprom, get_uid};
