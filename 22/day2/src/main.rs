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
    let mut score = 0;

    for line in reader.lines() {
        let string = line.unwrap();

        let game_vec: Vec<_> = string.split_whitespace().into_iter().collect();

        let game: ( u32, u32 ) = (
            game_vec[0].to_string().chars().nth(0).unwrap() as u32 - 64, 
            game_vec[1].to_string().chars().nth(0).unwrap() as u32 - 64 - 23
        );

        let num = (game.0 + 1) % 4;

        let num = match num {
            0 => 1,
            _ => num,
        };

        if game.0 == game.1 {
            // Draw
            let value = 3 + game.1;
            score += value;
        } else if num == game.1 {
            // Win
            let value = 6 + game.1;
            score += value;
        } else {
            // Loss
            let value = game.1;
            score += value;
        }
    }

    println!("Final Score: {score}");
}

fn part2<T: BufRead>(reader: T) {
    let mut score = 0;

    for line in reader.lines() {
        let string = line.unwrap();

        let game_vec: Vec<_> = string.split_whitespace().into_iter().collect();

        let game: ( u32, u32 ) = (
            game_vec[0].to_string().chars().nth(0).unwrap() as u32 - 64, 
            game_vec[1].to_string().chars().nth(0).unwrap() as u32 - 64 - 23
        );

        let win_num = (game.0 + 1) % 4;

        let win_num = match win_num {
            0 => 1,
            _ => win_num,
        };

        let lose_num = game.0 - 1;

        let lose_num = match lose_num {
            0 => 3,
            _ => lose_num,
        };

        let play_score = match game.1 {
            // Loss
            1 => lose_num,
            // Draw
            2 => game.0 + 3,
            // Win
            3 => win_num + 6,
            _ => 0
        };

        score += play_score;
    }

    println!("Final Score: {score}");
}
