#[macro_use]
extern crate nix;

extern crate libc;

mod eeprom;
mod ethtool;
mod i2c;

pub use eeprom::{get_revision, get_eeprom, set_eeprom};
