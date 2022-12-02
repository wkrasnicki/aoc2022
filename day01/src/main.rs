use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./input.txt").expect("Could not read file");
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .split(|line| line.trim().is_empty())
        .map(|items| {
            items
                .iter()
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap_or(0);

    println!("{}", result);
}
