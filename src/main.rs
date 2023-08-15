use clap::{Arg, ArgAction, Command};
use config::Config;
use std::fs;
use std::io::*;
use std::path::PathBuf;

mod config;
mod file_handler;

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
    for entry in file_dir {
        let f_name = entry?.file_name();
        rename_item(f_name.to_str().unwrap(), &config)?;
        if let (Some(genre), s) = (config.genre(), f_name.to_str().unwrap()) {
            add_genre_info(s, genre)?
        }
    }
    Ok(())
}

fn rename_item(entry: &str, config: &Config) -> Result<()> {
    let mut rmv_str = "".to_string();
    rmv_str = rmv_str + config.artist() + " - " + config.album() + " - ";

    let mut par_str = ".\\".to_string();
    par_str = par_str + config.album() + "\\" + entry;
    let entry = &par_str[..];

    if is_valid(entry) {
        fs::rename(entry, entry.replace(&rmv_str[..], ""))?;
    }
    Ok(())
}

fn add_genre_info(entry: &str, genre: &String) -> Result<()> {
    println!("{entry} - {genre}");
    Ok(())
}

fn is_valid(file_name: &str) -> bool {
    file_name.ends_with(".mp3") || file_name.ends_with(".wav") || file_name.ends_with(".flac")
}
