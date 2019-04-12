extern crate clap;

mod command;

use command::{get_command, Command};

fn main() {
    match get_command() {
        Command::Version => println!("1.0"),
    }
}
