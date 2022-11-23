use std::fs;

fn main() {
    println!("Opening file: input.txt");

    let contents = fs::read_to_string("./input.txt").expect("Could not find input.txt file");

    let mut level = 0;
    let mut pos = 0;

    for (i, letter) in contents.chars().enumerate() {
        if letter == '(' {
            level += 1;
        } else if letter == ')' {
            level -= 1;

            if pos == 0 && level == -1 {
                pos = i + 1;
            }
        }
    }

    println!("Level: {level}");
    println!("Pos: {pos}");
}
