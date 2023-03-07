use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("readme.md").unwrap(); //.Creating a File requires a path argument and error handling in case the file does not exist. This program crashes if a readme.md is not present.
    let mut reader = BufReader::new(f);

    let mut line = String::new(); //. We’ll re-use a single String object over the lifetime of the program
    loop {
        //.loop loops until the program encounters return, break or panics
        let len = reader.read_line(&mut line).unwrap(); //.Reading from disk can fail and we need to explicitly handle this. In our case, errors crash the program.
        if len == 0 {
            break;
        }
        println!("{} ({} bytes long)", line, len);

        line.truncate(0); //.Shrink the String back to length 0, preventing lines from persisting into the following ones
    }
}
