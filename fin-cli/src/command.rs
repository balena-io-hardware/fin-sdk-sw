use clap::{App, AppSettings, SubCommand};

pub enum Command {
    Version,
}

pub fn get_command() -> Command {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("version").about("Prints balenaFin hardware version"))
        .get_matches();

    match matches.subcommand() {
        ("version", _) => Command::Version,
        _ => unreachable!(),
    }
}
