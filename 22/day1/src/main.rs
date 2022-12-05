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
    let mut top_elf_calories = 0;
    let mut curr_elf_cals = 0;

    for line in reader.lines() {
        let calories = line.unwrap().parse::<u32>();

        match calories {
            Ok(n) => curr_elf_cals += n,
            Err(_why) => {
                if curr_elf_cals > top_elf_calories {
                    top_elf_calories = curr_elf_cals;
                }

                curr_elf_cals = 0;
            },
        }
    }

    println!("Top calories: {top_elf_calories}");
}

fn part2<T: BufRead>(reader: T) {
    let mut top_three_calories: [u32; 3] = [0; 3];
    let mut curr_elf_cals = 0;

    for line in reader.lines() {
        let calories = line.unwrap().parse::<u32>();

        match calories {
            Ok(n) => curr_elf_cals += n,
            Err(_why) => {
                for ii in 0..top_three_calories.len() {
                    if curr_elf_cals > top_three_calories[ii] {
                        let buf = top_three_calories[ii];
                        top_three_calories[ii] = curr_elf_cals;
                        curr_elf_cals = buf;
                    }
                }
                curr_elf_cals = 0;
            },
        }
    }

    println!("{:?}", top_three_calories);

    let total_calories = sum(top_three_calories);

    println!("Total calories: {total_calories}");
}

fn sum(arr: [u32; 3]) -> u32 {
    let mut sum = 0;

    for num in arr {
        sum += num;
    }

    return sum;
}
