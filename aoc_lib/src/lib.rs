mod lib_tests;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn get_file_contents(filepath: &str) -> Vec<String> {
    let file_handle = File::open(filepath).unwrap();
    let reader = BufReader::new(file_handle);
    let mut contents: Vec<String> = Vec::new();

    for line in reader.lines() {
        contents.push(line.unwrap());
    }

    contents
}
