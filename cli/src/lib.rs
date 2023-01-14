use clap::{Parser, ArgMatches};

#[derive(Parser, Debug)]
pub struct Cli {
    // Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8
}

pub fn parse_args() -> ArgMatches {
    let args = Cli::parse();
    let app = Args::into
}rg