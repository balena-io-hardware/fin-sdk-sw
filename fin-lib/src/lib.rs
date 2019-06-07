#[macro_use]
extern crate nix;

extern crate libc;

mod ethtool;
mod i2c;
mod version;

pub use version::get_fin_version;
