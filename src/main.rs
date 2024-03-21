use crate::config::{Config, ConfigBuilder};

use audiotags::Tag;
use clap::{Arg, ArgAction, Command};
use console::Style;
use dialoguer::Input;
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

    let config: Config = if !matches.get_flag("manual") {
        // Consume matches here as they are no longer needed if our Config constructs successfully
        Config::new(matches)?
    } else {
        // If manual input is specified we use a builder pattern to create a track config
        let mut builder = ConfigBuilder::new(matches);
        let style = Style::new().cyan().bold();

        let album: String = Input::new()
            .with_prompt(format!("{}", style.apply_to("\nAlbum Name")))
            .interact_text()
            .unwrap();
        builder.add_album_name(album.trim());

        let artist: String = Input::new()
            .with_prompt(format!("{}", style.apply_to("Album Artist")))
            .interact_text()
            .unwrap();
        builder.add_artist(artist.trim());

        let genre: String = Input::new()
            .with_prompt(format!("{} (optional)", style.apply_to("Genre")))
            .allow_empty(true)
            .with_initial_text(builder.genre().unwrap_or("".to_string()))
            .interact_text()
            .unwrap();

        if !genre.trim().is_empty() {
            builder.add_genre(Some(genre.trim()));
        } else {
            builder.add_genre(None::<String>);
        }

        builder.build()
    };
    println!("{:?}", config);
    todo!();
    let archive: Vec<u8> = fs::read(config.filepath())?;
    let target_path = PathBuf::from(config.album());
    if let Err(e) = zip_extract::extract(Cursor::new(archive), &target_path, true) {
        return Err(Error::new(ErrorKind::Other, e));
    };
    let file_dir = fs::read_dir(config.album())?;
    for entry in file_dir {
        let f_name = entry?.file_name();
        if !is_valid(f_name.to_str().unwrap()) {
            continue;
        }
        if let (Some(genre), s) = (config.genre(), f_name.to_str().unwrap()) {
            add_genre_info(s, genre)?
        }
        rename_item(f_name.to_str().unwrap(), &config)?;
    }
    Ok(())
}

fn rename_item(entry: &str, config: &Config) -> Result<()> {
    let mut rmv_str = "".to_string();
    rmv_str = rmv_str + config.artist() + " - " + config.album() + " - ";

    let mut par_str = ".\\".to_string();
    par_str = par_str + config.album() + "\\" + entry;
    let entry = &par_str[..];

    fs::rename(entry, entry.replace(&rmv_str[..], ""))?;
    Ok(())
}

fn add_genre_info(entry: &str, genre: &str) -> Result<()> {
    let mut tag = Tag::new().read_from_path(entry).unwrap();
    tag.set_genre(genre);
    tag.write_to_path(entry)
        .expect("Failed to save genre information");
    Ok(())
}

fn is_valid(file_name: &str) -> bool {
    file_name.ends_with(".mp3") || file_name.ends_with(".wav") || file_name.ends_with(".flac")
}
