use clap::ArgMatches;
use std::io::*;

#[derive(Debug)]
pub struct Config {
    filepath: String,
    artist: String,
    album: String,
    genre: Option<String>,
    compilation: bool,
}

#[derive(Debug)]
pub struct ConfigBuilder {
    filepath: String,
    artist: Option<String>,
    album: Option<String>,
    genre: Option<String>,
    compilation: bool,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Self> {
        let filepath = matches.get_one::<String>("name").unwrap().to_owned();
        check_is_archive(&filepath)?;
        let (artist, album) = split_artist_and_album(&filepath)?;
        let genre = matches
            .get_one::<String>("genre")
            .map(|genre| genre.to_owned());
        let compilation = matches.get_flag("compilation");

        let r = Config {
            filepath,
            artist,
            album,
            genre,
            compilation,
        };

        Ok(r)
    }

    // Getters to access configuration details
    pub fn filepath(&self) -> &String {
        &self.filepath
    }

    pub fn artist(&self) -> &String {
        &self.artist
    }

    pub fn album(&self) -> &String {
        &self.album
    }

    pub fn genre(&self) -> Option<&String> {
        self.genre.as_ref()
    }

    pub fn is_compilation(&self) -> bool {
        self.compilation
    }
}

impl ConfigBuilder {
    pub fn new(matches: ArgMatches) -> Self {
        ConfigBuilder {
            filepath: matches.get_one::<String>("name").unwrap().to_owned(),
            artist: None,
            album: None,
            genre: matches
                .get_one::<String>("genre")
                .map(|genre| genre.to_owned()),
            compilation: matches.get_flag("compilation"),
        }
    }

    pub fn build(&self) -> Config {
        Config {
            filepath: self.filepath.clone(),
            album: self.album.clone().unwrap(),
            artist: self.artist.clone().unwrap(),
            genre: self.genre.clone(),
            compilation: self.compilation,
        }
    }

    pub fn add_album_name(&mut self, album: impl Into<String>) {
        self.album = Some(album.into());
    }

    pub fn add_artist(&mut self, artist: impl Into<String>) {
        self.artist = Some(artist.into());
    }

    pub fn add_genre(&mut self, genre: Option<String>) {
        self.genre = genre;
    }
}

// helper functions

fn check_is_archive(filepath: &String) -> Result<()> {
    let length = filepath.len();
    if &filepath[(length - 4)..] != ".zip" {
        Result::Err(Error::new(
            std::io::ErrorKind::Unsupported,
            "File must be of type .zip",
        ))?;
    }
    Ok(())
}

fn split_artist_and_album(filepath: &String) -> Result<(String, String)> {
    let filename = filepath.split('\\').last().unwrap_or(filepath);
    let split = filename.find(" - ").ok_or(Error::new(
        std::io::ErrorKind::InvalidInput,
        "Cannot find valid album orartist name. Consider using manual input -m",
    ))?;
    let artist = String::from(&filename[..split]);
    let album = String::from(&filename[split + 3..filename.len() - 4]);
    Ok((artist, album))
}
