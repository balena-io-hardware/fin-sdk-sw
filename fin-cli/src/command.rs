use clap::{App, AppSettings, Arg, SubCommand};

pub enum Command {
    Revision,
    Eeprom(Option<String>),
    Uid,
}

pub fn get_command() -> Command {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("revision").about("Prints balenaFin hardware revision"))
        .subcommand(
            SubCommand::with_name("eeprom")
                .about("Prints balenaFin raw EEPROM data")
                .arg(
                    Arg::with_name("set")
                        .long("set")
                        .help("Sets balenaFin raw EEPROM data")
                        .takes_value(true)
                        .hidden(true),
                ),
        )
        .subcommand(SubCommand::with_name("uid").about("Prints board's unique ID"))
        .get_matches();

    match matches.subcommand() {
        ("revision", _) => Command::Revision,
        ("eeprom", Some(eeprom_matches)) => {
            Command::Eeprom(eeprom_matches.value_of("set").map(String::from))
        }
        ("uid", _) => Command::Uid,
        _ => unreachable!(),
    }
}
