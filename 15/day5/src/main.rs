use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    println!("Opening file: input.txt");

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    day1(reader);

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    day2(reader);
}

fn day1<T: BufRead>(reader: T) {
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

    println!("Found this many OK words: {ok_count}");
}

fn day2<T: BufRead>(reader: T) {
    let mut num_ok = 0;

    for line in reader.lines() {
        let string = line.unwrap();

        // println!("Line: {string}");

        let mut found_doubles = false;
        let mut found_sandwich = false;

        let char_array: Vec<_> = string.chars().collect();

        for ii in 0..char_array.len() - 1 {
            let matches: Vec<_> = string.match_indices(char_array[ii]).collect();

            if matches.len() > 1 {
                for item in matches {
                    if item.0 > ii + 1 && item.0 > ii && item.0 != char_array.len() - 1 {
                        // println!("Match: {:?}", item);
                        if char_array[ii + 1] == char_array[item.0 + 1]{
                            found_doubles = true;
                        }
                    }
                }
            }

            if ii > 0 && char_array[ii - 1] == char_array[ii + 1] {
                found_sandwich = true;
            }
        }

        if !found_doubles || !found_sandwich {
            continue;
        }

        // println!("OK string: {string}");
        num_ok += 1;
    }

    println!("OK words found: {num_ok}");
}
