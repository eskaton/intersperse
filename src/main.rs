#[macro_use]
extern crate clap;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, AppSettings, Arg, ArgMatches};

const ARG_DELIM: &'static str = "delimiter";
const ARG_FILE: &'static str = "file";

fn parse_args(args: &Vec<String>) -> ArgMatches {
    let arg_delim = Arg::with_name(ARG_DELIM)
        .help("Delimiter ")
        .short("d")
        .long(ARG_DELIM)
        .value_name("DELIM");

    let arg_input = Arg::with_name(ARG_FILE)
        .help("Input file")
        .short("f")
        .long(ARG_FILE)
        .value_name("FILE");

    return App::new("Intersperse")
        .version(crate_version!())
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(&arg_delim)
        .arg(&arg_input)
        .get_matches_from(args);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let matches = parse_args(&args);
    let delim = if matches.is_present(ARG_DELIM) {
        matches.value_of(ARG_DELIM).unwrap()
    } else {
        ""
    };
    let file: Box<dyn BufRead> = match matches.value_of(ARG_FILE) {
        Some(f) => Box::new(BufReader::new(File::open(f).unwrap())),
        None => Box::new(BufReader::new(io::stdin()))
    };
    let mut first = true;

    for line in file.lines().into_iter() {
        if first {
            first = false;
        } else {
            print!("{}", delim);
        }

        print!("{}", line.unwrap());
    }

    println!();
}
