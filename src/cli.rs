use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short = 'v', action = clap::ArgAction::Count)]
    pub debug: u8,
    /// The source directory of video files
    pub source_dir: PathBuf,
    /// The destination directory of video files
    pub dest_dir: PathBuf,
    /// If true, do not make any changes
    #[arg(short, long, default_value_t = false)]
    pub dry_run: bool,
}

pub fn parse_args() -> Cli {
    let args = Cli::parse();
    args
}
