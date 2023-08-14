use clap::{Arg, ArgAction, Command};
use config::Config;
use std::ffi::OsString;
use std::fs;
use std::io::*;
use std::path::PathBuf;

mod config;

fn main() -> Result<()> {
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

    // Consume matches here as they are no longer needed if our Config constructs successfully
    let config = Config::new(matches)?;

    println!("name: {}", config.filepath());
    println!("artist: {}", config.artist());
    println!("album: {}", config.album());
    println!("genre: {:?}", config.genre());
    println!("compilation: {}", config.is_compilation());
    println!("manual: {}", config.is_manual());

    let archive: Vec<u8> = fs::read(config.filepath())?;
    let target_path = PathBuf::from(config.album());
    if let Err(e) = zip_extract::extract(Cursor::new(archive), &target_path, true) {
        return Err(Error::new(ErrorKind::Other, e));
    };

    let file_dir = fs::read_dir(config.album())?;
    let file_iter = file_dir.enumerate();
    for (_i, file) in file_iter {
        rename_item(&file?.file_name(), &config)?;
    }

    Ok(())
}

#[allow(unused_variables)]
fn rename_item(entry: &OsString, config: &Config) -> Result<()> {
    todo!();
}
