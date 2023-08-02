use clap::{Arg, ArgAction, ArgMatches, Command};
use std::{fs, process::exit};

struct Rbcx {
    filename: String,
    genre: Option<String>,
    compilation: bool,
    manual: bool,
}

impl Rbcx {
    fn new(matches: &ArgMatches) -> Rbcx {
        let r = Rbcx {
            filename: matches.get_one::<String>("name").unwrap().to_owned(),
            genre: matches
                .get_one::<String>("genre")
                .map(|genre| genre.to_owned()),
            compilation: matches.get_flag("compilation"),
            manual: matches.get_flag("manual"),
        };

        if let Err(error) = Rbcx::check_is_archive(&r.filename) {
            println!("{}", error);
            exit(1);
        };

        r
    }

    fn check_is_archive(filename: &String) -> Result<(), &str> {
        let length = filename.len();
        if &filename[(length - 4)..] != ".zip" {
            return Err("File must be of type .zip");
        }
        Ok(())
    }
}

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

    let rbcx = Rbcx::new(&matches);

    drop(matches);

    println!("name: {}", rbcx.filename);
    println!("genre: {:?}", rbcx.genre);
    println!("compilation: {}", rbcx.compilation);
    println!("manual: {}", rbcx.manual);
}
