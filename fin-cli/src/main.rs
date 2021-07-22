extern crate clap;

mod command;

use command::{get_command, Command};

use fin_lib::{get_eeprom, get_revision, get_uid, set_eeprom};

fn main() {
    match get_command() {
        Command::Revision => println!("{}", get_revision()),
        Command::Eeprom(data) => {
            if let Some(ref eeprom) = data {
                if set_eeprom(eeprom).is_none() {
                    println!("Incorrect EEPROM value");
                }
            } else if let Some(eeprom) = get_eeprom() {
                println!("{}", eeprom);
            }
        }
        Command::Uid => {
            if let Some(uid) = get_uid() {
                println!("{}", uid);
            }
        }
    }
}
