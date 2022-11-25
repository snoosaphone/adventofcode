use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    println!("Opening file: input.txt");

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let disallowed_substrings = ["ab", "cd", "pq", "xy"];
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut ok_count = 0;

    for line in reader.lines() {
        let mut num_vowels = 0;
        let mut found_disallowed_substring = false;
        let mut found_double = false;


        let string = line.unwrap();

        // println!("Current: {string}");

        // Substring check
        for substring in disallowed_substrings {
            if string.contains(substring) {
                found_disallowed_substring = true;
                break;
            }
        }

        if found_disallowed_substring {
            // println!("Found disallowed substring: {string}");
            continue;
        }
        
        // Vowel count check
        for vowel in vowels {
            let matches: Vec<&str> = string.matches(vowel).collect();

            num_vowels += matches.len();
        }

        // println!("Num vowels found: {num_vowels}");
        
        if num_vowels < 3 {
            // println!("Doesn't contain 3 or more vowels!");
            continue;
        }

        // Doubles check
        let mut last_char = ';';

        for character in string.chars() {
            if character == last_char {
                found_double = true;
            }
            last_char = character;
        }

        if !found_double {
            // println!("Didn't find doubles");
            continue;
        }

        ok_count += 1;
    }

    println!("Found this many OK works: {ok_count}");
}
