use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    println!("Opening file input.txt");

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    part1(reader);

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    part2(reader);
}

fn part1<T: BufRead>(reader: T) {
    let mut sum = 0;

    for line in reader.lines() {
        let string = line.unwrap();

        let first_compartment = &string[0..string.len() / 2];
        let second_compartment = &string[string.len() / 2..];

        let mut dupe = ' ';

        for character in first_compartment.chars() {
            if second_compartment.contains(character) {
                dupe = character;
            }
        }

        let value = get_priority_value(dupe);

        sum += value;
    }

    println!("Total priorities: {sum}");
}

fn part2<T: BufRead>(reader: T) {
    let mut sum = 0;

    let mut line_buffer: Vec<String> = Vec::new();

    for line in reader.lines() {
        let string = line.unwrap();
        line_buffer.push(string);
    }

    let num_groups = line_buffer.len() / 3;

    for ii in 0..num_groups {
        let offset = ii * 3;

        for character in line_buffer[offset].chars() {
            if line_buffer[offset + 1].contains(character) && line_buffer[offset + 2].contains(character) {
                let value = get_priority_value(character);
                sum += value;
                break;
            }
        }
    }

    println!("Total priorities: {sum}");
}

fn get_priority_value(character: char) -> i32 {
    let mut value = character as i32 - 96;
    if value < 0 {
        value += 58;
    }
    return value
}
