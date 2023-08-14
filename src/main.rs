use clap::{Arg, ArgAction, Command};
use config::Config;
#[allow(unused_imports)]
use std::{fs, process::exit};

mod config;

fn main() {
    let matches = Command::new("rbcx")
        .version("0.1.0")
        .author("James Nash <jrnash.dev@gmail.com>")
        .arg(Arg::new("name").required(true))
        .arg(Arg::new("genre").short('g').long("genre"))
        .arg(
            Arg::new("compilation")
                .short('c')
                .long("compilation")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("manual")
                .short('m')
                .long("manual")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let config = match Config::new(matches) {
        Ok(config) => config,
        Err(e) => {
            print!("{}", e);
            exit(1);
        }
    };

    println!("name: {}", config.filename());
    println!("artist: {}", config.artist());
    println!("album: {}", config.album());
    println!("genre: {:?}", config.genre());
    println!("compilation: {}", config.is_compilation());
    println!("manual: {}", config.is_manual());
}
