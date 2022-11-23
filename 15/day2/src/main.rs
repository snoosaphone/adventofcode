use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    println!("Opening file: input.txt");

    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut dimensions: [usize; 3] = [0, 0, 0];

    let mut totalsqft = 0;
    let mut totalribbon = 0;

    for line in reader.lines() {
        let string = line.unwrap();
        let split = string.split("x");

        for (i, item) in split.enumerate() {
            dimensions[i] = item.to_string().parse::<usize>().unwrap();
        }

        dimensions.sort();

        let sqft = get_square_footage(&dimensions);
        let slack = dimensions[0] * dimensions[1];

        totalsqft += sqft + slack;

        let ribbon = 2 * dimensions[0] + 2 * dimensions[1];
        let bowsize = dimensions[0] * dimensions[1] * dimensions[2];

        totalribbon += ribbon + bowsize;
    }

    println!("Total squarefeet: {totalsqft}");
    println!("Total ribbon: {totalribbon}");
}

fn get_square_footage(dimensions: &[usize; 3]) -> usize {
    let mut sum = 0;

    for i in 0..3 {
        sum += 2 * dimensions[i] * dimensions[(i + 1) % 3]
    }

    return sum;
}
