extern crate clap;

mod command;

use command::{get_command, Command};

use fin_lib::get_revision;

fn main() {
    match get_command() {
        Command::Revision => println!("{}", get_revision()),
    }
}
