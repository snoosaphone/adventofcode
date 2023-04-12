use core::ops::Range;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    println!("Opening file input.txt");

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    part1(reader);
}

fn part1<T: BufRead>(reader: T) {
    let mut num_fully_contained = 0;
    let mut num_overlapping = 0;

    for line in reader.lines() {
        let string = line.unwrap();

        let assignments = get_section_ranges(&string);

        let elf1_range = assignments.0;
        let elf2_range = assignments.1;

        if range_fully_contained(elf1_range.clone(), elf2_range.clone()) {
            num_fully_contained += 1;
        }

        if range_overlapping(elf1_range, elf2_range) {
            num_overlapping += 1;
        }
    }

    println!("Found the following number of fully contained ranges: {num_fully_contained}");
    println!("Found the following number of overlapping ranges: {num_overlapping}");
}

fn range_fully_contained(range1: Range<u32>, range2: Range<u32>) -> bool {
    if (range1.start <= range2.start && range1.end >= range2.end) || (range2.start <= range1.start && range2.end >= range1.end) {
        return true;
    }

    return false;
}

fn range_overlapping(range1: Range<u32>, range2: Range<u32>) -> bool {
    if (range1.start <= range2.end) && (range2.start <= range1.end) {
        return true;
    }
    return false;
}

fn get_section_ranges(string: &str) -> (Range<u32>, Range<u32>) {
    let assignments: Vec<&str> = string.split(",").collect();
    let tuple1: Vec<&str> = assignments[0].split("-").collect();
    let tuple2: Vec<&str> = assignments[1].split("-").collect();

    let range1 = (tuple1[0].parse::<u32>().unwrap(), tuple1[1].parse::<u32>().unwrap());
    let range2 = (tuple2[0].parse::<u32>().unwrap(), tuple2[1].parse::<u32>().unwrap());

    return (range1.0..range1.1, range2.0..range2.1);
}
