use clap::{arg, command, value_parser, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct CliArgs {
    /// Run the program as a simple command line program
    #[arg(long, default_value_t = true)]
    pub headless: bool,

    /// Location of the log file to be monitored
    #[arg(short, long, value_parser = value_parser!(PathBuf))]
    pub path: PathBuf,

    /// Location of the log file to be monitored
    #[arg(short, long, value_parser = value_parser!(PathBuf))]
    pub config: Option<PathBuf>,
}
