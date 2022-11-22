use clap::{command, arg, ArgMatches};


pub fn parse_args() -> ArgMatches {
    let matches = command!()
        .arg(arg!(-v --verbose ... "Set logging verbosity"))
        .get_matches();
    matches
}