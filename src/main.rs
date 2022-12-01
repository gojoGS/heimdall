mod args;
mod color;
mod config;
mod file;
mod log;
mod parse;

use crate::args::CliArgs;
use crate::color::colorize_log_line;
use crate::config::AppConfig;
use clap::Parser;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::sync::mpsc::channel;

use crate::file::LogFileDescriptor;
use crate::parse::{LineParser, RegexParser};

fn main() {
    let args: CliArgs = CliArgs::parse();

    println!("{:?}", args);

    startup(&args);
}

fn startup(args: &CliArgs) -> Result<(), io::Error> {
    let config: AppConfig;

    if args.config.is_none() {
        println!("creating config file");
        config::create_config_if_not_exists().expect("creating config failed");
        config = config::load_config(config::get_config_path()).expect("loading config failed");
    } else {
        let path = &args.path;
        config = config::load_config(path).expect("loading config failed");
    }

    run(args, &config)
}

fn run(args: &CliArgs, config: &AppConfig) -> Result<(), io::Error> {
    println!("{:?}", config);

    let path = args.path.as_os_str().to_str().unwrap();
    let mut file = File::open(path)?;
    let result = file::read_contents(LogFileDescriptor::File(&file))?;

    let mut content = result.content;
    let mut pos = result.position;

    let parse_result = RegexParser::parse(content.as_str());
    parse_result
        .lines
        .into_iter()
        .map(colorize_log_line)
        .for_each(|line| println!("{}", line));

    let (sender, receiver) = channel();

    let mut watcher = recommended_watcher(sender).unwrap();

    watcher
        .watch(Path::new(path), RecursiveMode::NonRecursive)
        .expect("Couldnt match");

    loop {
        match receiver.recv() {
            Ok(_) => {
                file.seek(SeekFrom::Start(pos)).unwrap();

                let re_read_result = file::read_contents_from(LogFileDescriptor::File(&file), pos)?;

                content = re_read_result.content;
                pos = re_read_result.position;

                print!("{}", content);
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
