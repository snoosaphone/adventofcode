use aoc_lib::*;
use std::io;

#[derive(Debug)]
struct Instruction {
    how_many: usize,
    from: usize,
    to: usize
}

fn main() {
    println!("Opening file input.txt");

    let contents = get_file_contents("./input.txt");

    part1(contents);
}

fn part1(contents: Vec<String>) {
    let input: Vec<Vec<&str>> = vec![
        vec!["R", "G", "J", "B", "T", "V", "Z"],
        vec!["J", "R", "V", "L"],
        vec!["S", "Q", "F"],
        vec!["Z", "H", "N", "L", "F", "V", "Q", "G"],
        vec!["R", "Q", "T", "J", "C", "S", "M", "W"],
        vec!["S", "W", "T", "C", "H", "F"],
        vec!["D", "Z", "C", "V", "F", "N", "J"],
        vec!["L", "G", "Z", "D", "W", "R", "F", "Q"],
        vec!["J", "B", "W", "V", "P"],
    ];

    // println!("How many stacks are there?");
    // let mut input_string = String::new();
    // io::stdin().read_line(&mut input_string).unwrap();
    // let num_stacks = input_string.trim().parse::<usize>().unwrap();
    let num_stacks: usize = 9;

    println!("Going forward with {} stacks.", num_stacks);

    // let mut stacks: Vec<Vec<String>> = Vec::with_capacity(num_stacks);
    // println!("{}", stacks.len());

    // for num in 0..num_stacks {
        // println!("Enter crates for stack {}:", num + 1);
        // let mut input_string = String::new();

        // io::stdin().read_line(&mut input_string).unwrap();

        // stacks.push(Vec::new());

        // input_string.trim().chars().collect::<Vec<_>>().into_iter().for_each(|letter| {
            // stacks[num].push(letter.to_string());
        // });
    // }

    let mut stacks = input.clone();
    let mut stacks2 = stacks.clone();

    let instructions = process_instructions(contents);

    for instruction in &instructions {
        // println!("{:?}", instruction);
        for _ in 0..instruction.how_many {
            let popped = stacks[instruction.from].pop().unwrap();
            stacks[instruction.to].push(popped);
        }

        let final_length = stacks2[instruction.from].len().saturating_sub(instruction.how_many);
        let mut popped_vec = stacks2[instruction.from].split_off(final_length);
        stacks2[instruction.to].append(&mut popped_vec);
    }

    for i in 0..num_stacks {
        println!("Day1");
        println!("Stack {} ends in crate {}", i + 1, stacks[i].pop().unwrap());
    }

    for i in 0..stacks2.len() {
        println!("Day2");
        let value = stacks2[i].pop();
        match value {
            Some(x) => println!("Stack {} ends in crate {}", i + 1, x),
            None => println!("Stack {} is empty", i + 1)
        }
        // println!("Stack {} ends in crate {}", i + 1, stacks2[i][stacks2[i].len() - 1]);
    }
}

fn process_instructions(raw_instructions: Vec<String>) -> Vec<Instruction>{
    let mut instruction_list = Vec::new();

    for raw in &raw_instructions {
        let mut raw_split = Vec::new();

        for (i, item) in raw.split(' ').enumerate() {
            if (i + 1) % 2 == 0 {
                raw_split.push(item.parse::<usize>().unwrap());
            }
        }

        instruction_list.push(Instruction { how_many: raw_split[0], from: raw_split[1] - 1, to: raw_split[2] - 1 })
    }

    instruction_list
}
