extern crate clap;

#[macro_use]
extern crate nix;

mod command;
mod version;

use command::{get_command, Command};
use version::print_version;

fn main() {
    match get_command() {
        Command::Version => print_version(),
    }
}
