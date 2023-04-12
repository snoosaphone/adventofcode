use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    println!("Opening file input.txt");

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    part1(reader);
}

fn part1<T: BufRead>(reader: T) {
    let stack_re = Regex::new(r"");

    for line in reader {
        
    }
}
