extern crate clap;

use clap::{Arg, App};
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use lmc;

fn main() {
    let matches = App::new("Little man computer")
        .version("1.0")
        .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Indicates that the input is a file path")
                .takes_value(true))
        .get_matches();
    
    // If --file is present, read the values from a file, otherwise, use stdin
    let reader: Box<dyn BufRead> = match matches.value_of("file") {
        Some(file_path) => {
            let path = std::path::PathBuf::from(file_path);
            let f = File::open(path).expect("File not found");
            Box::new(BufReader::new(f))
        },
        None => Box::new(BufReader::new(io::stdin()))
    };

    // Fill mailbox
    let mut mailbox: [i32; 100] = [0; 100];
    for line in reader.lines().enumerate() {
        let value = line.1.unwrap();
        mailbox[line.0] = value.parse().expect(&format!("Invalid value on line {}", line.0 + 1));

        if line.0 == 99 {
            break;
        }
    }

    // Execute program
    lmc::compute(mailbox);
}

