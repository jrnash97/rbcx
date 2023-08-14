use clap::ArgMatches;
use std::process::exit;

pub struct Config {
    filepath: String,
    artist: String,
    album: String,
    genre: Option<String>,
    compilation: bool,
    manual: bool,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Config, String> {
        let filepath = matches.get_one::<String>("name").unwrap().to_owned();
        if let Err(error) = Config::check_is_archive(&filepath) {
            println!("{}", error);
            exit(1);
        };
        let (artist, album) = Config::artist_album(&filepath)?;
        let genre = matches
            .get_one::<String>("genre")
            .map(|genre| genre.to_owned());
        let compilation = matches.get_flag("compilation");
        let manual = matches.get_flag("manual");

        let r = Config {
            filepath,
            artist,
            album,
            genre,
            compilation,
            manual,
        };

        Ok(r)
    }

    fn check_is_archive(filename: &String) -> Result<(), &str> {
        let length = filename.len();
        if &filename[(length - 4)..] != ".zip" {
            return Err("File must be of type .zip");
        }
        Ok(())
    }

    // Getters to access configuration details
    pub fn filename(&self) -> &String {
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

    pub fn is_manual(&self) -> bool {
        self.manual
    }

    fn artist_album(filepath: &String) -> Result<(String, String), &str> {
        let filename = filepath.split('\\').last().unwrap_or(filepath);
        let split = filename
            .find(" - ")
            .ok_or("Cannot find valid album or artist name. Consider using manual input -m")?;
        let artist = String::from(&filename[..split]);
        let album = String::from(&filename[split + 3..filename.len() - 4]);
        Ok((artist, album))
    }
}
