use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*; 
use clap::{App, Arg};//.Bring the clap::App and clap::Arg objects into local scope
use regex::Regex; //1. Bringing regex into local scope.

fn main() {
    version_one();
    // version_two_clap();
    version_three();
}

fn version_one() {
    let re = Regex::new("picture").unwrap(); //2. unwrap() "unwraps" a Result, crashing if an error occurs.

    let quote = "Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            //3. Using match
            Some(_) => println!("{}", line), //4. Some(T) is the positive case of an Option. In this case means that re.find() has been successful. _ matches all values.
            None => (), //5. None is the negative case of an Option. () can be thought of as a null placeholder value here.
        }
    }
}

#[allow(dead_code)]
fn version_two_clap() {
    let args = App::new("grep_lite") //.Incrementally build up a command argument parser. Each argument takes an Arg. In our case we only need one.
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let pattern = args.value_of("pattern").unwrap(); //.Extract the pattern argument.
    let re = Regex::new(pattern).unwrap();
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";
    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn version_three() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let input = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) { //.line is a String but re.find() takes &str as an argument
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

