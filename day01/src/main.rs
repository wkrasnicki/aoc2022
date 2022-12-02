use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

fn main() {
    let file = File::open("./input.txt").expect("Could not read file");
    let reader = BufReader::new(file);

    let n = 3;

    let mut elves = reader
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
        .collect::<BinaryHeap<i32>>();

    println!("{}", elves.peek().unwrap_or(&0));

    let top_elves = iter::repeat(()).take(n).map(|_| elves.pop().unwrap_or(0));

    println!("{}", top_elves.sum::<i32>());
}
