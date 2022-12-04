use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

enum Options {
    Rock,
    Paper,
    Scissors,
}

impl Options {
    fn value(&self) -> u32 {
        match self {
            Options::Rock => 1,
            Options::Paper => 2,
            Options::Scissors => 3,
        }
    }

    fn duel(&self, other: &Options) -> u32 {
        match self {
            Options::Rock => match other {
                Options::Scissors => 6,
                Options::Rock => 3,
                _ => 0,
            },
            Options::Paper => match other {
                Options::Rock => 6,
                Options::Paper => 3,
                _ => 0,
            },
            Options::Scissors => match other {
                Options::Paper => 6,
                Options::Scissors => 3,
                _ => 0,
            },
        }
    }
}

impl FromStr for Options {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Options::Rock),
            "B" => Ok(Options::Paper),
            "C" => Ok(Options::Scissors),
            "X" => Ok(Options::Rock),
            "Y" => Ok(Options::Paper),
            "Z" => Ok(Options::Scissors),
            _ => Err(()),
        }
    }
}

fn get_points(s: &str) -> u32 {
    let options: Vec<Options> = s
        .split(' ')
        .map(|option| option.parse::<Options>().unwrap())
        .collect();

    options[1].duel(&options[0]) + (options[1].value())
}

fn main() {
    let file = File::open("./input.txt").expect("Could not read file");
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .map(|line| get_points(&line.unwrap()))
        .sum::<u32>();

    println!("{}", result);
}
