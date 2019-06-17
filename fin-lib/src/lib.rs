#[macro_use]
extern crate nix;

extern crate libc;

mod ethtool;
mod i2c;
mod eeprom;

pub use eeprom::get_revision;
