use clap::{Parser};

#[derive(Parser, Debug)]
pub struct Cli {
    // Turn debugging information on
    #[arg(short = 'v', action = clap::ArgAction::Count, help = "Turn debugging information on")]
    pub debug: u8
}

pub fn parse_args() -> Cli {
    let args = Cli::parse();
    args
}