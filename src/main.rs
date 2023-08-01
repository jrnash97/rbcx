use clap::{Arg, ArgAction, ArgMatches, Command};
use std::fs;
// use std::path::PathBuf;

struct Rbcx {
    matches: ArgMatches,
}

impl Rbcx {
    fn new() -> Rbcx {
        Rbcx {
            matches: Command::new("rbcx")
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
                .get_matches(),
        }
    }
}

fn main() {
    let rbcx = Rbcx::new();
    let (filename, genre, is_compilation, is_manual) = get_args(&rbcx.matches);

    println!("name: {}", filename);
    println!("genre: {:?}", genre);
    println!("compilation: {}", is_compilation);
    println!("manual: {}", is_manual);
}

fn get_args(matches: &ArgMatches) -> (&String, Option<&String>, bool, bool) {
    let filename = matches.get_one::<String>("name").expect("required");
    let genre = matches.get_one::<String>("genre");
    let manual = matches.get_flag("manual");
    let compilation = matches.get_flag("compilation");

    (filename, genre, compilation, manual)
}
