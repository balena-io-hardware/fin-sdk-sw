extern crate clap;

mod command;

use command::{get_command, Command};

use fin_lib::{get_revision, get_eeprom, set_eeprom};

fn main() {
    match get_command() {
        Command::Revision => println!("{}", get_revision()),
        Command::Eeprom(data) => {
            if let Some(ref eeprom) = data {
                set_eeprom(eeprom);
            } else {
                println!("{}", get_eeprom())
            }
        }
    }
}
