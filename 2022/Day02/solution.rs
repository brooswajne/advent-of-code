use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_opponent_pick(pick: char) -> Option<Self> {
        match pick {
            'A' => Some(Shape::Rock),
            'B' => Some(Shape::Paper),
            'C' => Some(Shape::Scissors),
            _ => None,
        }
    }

    fn from_my_pick(pick: char) -> Option<Self> {
        match pick {
            'X' => Some(Shape::Rock),
            'Y' => Some(Shape::Paper),
            'Z' => Some(Shape::Scissors),
            _ => None,
        }
    }

    fn fight_against(&self, other: &Shape) -> Outcome {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Loss,
                Shape::Scissors => Outcome::Win,
            },
            Shape::Paper => match other {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Loss,
            },
            Shape::Scissors => match other {
                Shape::Rock => Outcome::Loss,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Draw,
            },
        }
    }

    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

fn main() {
    let input_path = std::env::args()
        .nth(1)
        .expect("input path must be provided as a command line argument");
    let input_path = std::env::current_dir()
        .expect("unable to get current working directory")
        .join(&input_path);
    let input = File::open(input_path).expect("unable to open input file");
    let lines = BufReader::new(input).lines();

    let mut score = 0;
    for line in lines {
        let line = line.expect("unable to read input line");
        let mut chars = line.chars();
        let them = chars
            .next()
            .and_then(Shape::from_opponent_pick)
            .expect("unable to parse opponent pick");
        let me = chars
            .nth(1)
            .and_then(Shape::from_my_pick)
            .expect("unable to parse my pick");
        let outcome = me.fight_against(&them);
        score += me.score() + outcome.score();
    }

    println!("Final score: {score}")
}
