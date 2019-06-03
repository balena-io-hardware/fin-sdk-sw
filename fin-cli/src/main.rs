extern crate clap;

mod command;

use command::{get_command, Command};

use fin_base::get_fin_version;

fn main() {
    match get_command() {
        Command::Version => println!("{}", get_fin_version()),
    }
}
