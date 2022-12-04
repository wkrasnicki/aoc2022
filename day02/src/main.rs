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
}

impl FromStr for Options {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Options::Rock),
            "B" => Ok(Options::Paper),
            "C" => Ok(Options::Scissors),
            _ => Err(()),
        }
    }
}

enum Strategy {
    Win,
    Draw,
    Lose,
}

impl FromStr for Strategy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Strategy::Lose),
            "Y" => Ok(Strategy::Draw),
            "Z" => Ok(Strategy::Win),
            _ => Err(()),
        }
    }
}

impl Strategy {
    fn duel(&self, other: &Options) -> u32 {
        match self {
            Strategy::Win => {
                6 + (match other {
                    Options::Rock => Options::Paper,
                    Options::Paper => Options::Scissors,
                    Options::Scissors => Options::Rock,
                })
                .value()
            }
            Strategy::Draw => 3 + other.value(),
            Self::Lose => (match other {
                Options::Rock => Options::Scissors,
                Options::Paper => Options::Rock,
                Options::Scissors => Options::Paper,
            })
            .value(),
        }
    }
}

fn get_points(s: &str) -> u32 {
    let opponent: Options = s[..1].parse().unwrap();
    let strategy: Strategy = s[2..3].parse().unwrap();

    strategy.duel(&opponent)
}

fn main() {
    let file = File::open("../input.txt").expect("Could not read file");
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .map(|line| get_points(&line.unwrap()))
        .sum::<u32>();

    println!("{}", result);
}
