#[macro_use]
extern crate nix;

extern crate libc;

mod ethtool;
mod i2c;

pub use ethtool::{get_eeprom, get_revision, get_uid, set_eeprom};
