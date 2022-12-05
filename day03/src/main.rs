use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const GROUP_SIZE: usize = 3;

fn get_priority(c: char) -> u8 {
    if c.is_ascii_lowercase() {
        (c as u8) - b'a' + 1
    } else {
        (c as u8) - b'A' + 27
    }
}

fn get_bitmask(str: &str) -> u64 {
    let mut bitmask = 0u64;

    for p in str.chars().map(get_priority) {
        bitmask |= 1u64 << p;
    }

    bitmask
}

fn find_common_item(group: &[String]) -> u32 {
    group
        .iter()
        .map(|s| get_bitmask(s))
        .reduce(|acc, cur| acc & cur)
        .unwrap_or(0)
        .trailing_zeros()
}

fn main() {
    let file = File::open("./input.txt").expect("Could not read file");
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .chunks(GROUP_SIZE)
        .map(find_common_item)
        .sum::<u32>();

    println!("{:?}", result);
}
