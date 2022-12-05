use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_priority(c: char) -> u8 {
    if c.is_ascii_lowercase() {
        (c as u8) - b'a' + 1
    } else {
        (c as u8) - b'A' + 27
    }
}

fn find_common_item(str: &str) -> u32 {
    let first = &str[..(str.len() / 2)];
    let second = &str[(str.len() / 2)..];

    let mut bitmask = 0u64;

    first
        .chars()
        .map(get_priority)
        .for_each(|p| bitmask |= 1u64 << p);

    for p in second.chars().map(get_priority) {
        if bitmask & 1u64 << p > 0 {
            return p.into();
        }
    }

    0
}

fn main() {
    let file = File::open("./input.txt").expect("Could not read file");
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .map(|line| find_common_item(&line.unwrap()))
        .sum::<u32>();

    println!("{:?}", result);
}
