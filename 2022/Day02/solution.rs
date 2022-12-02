use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Shape {
    type Error = ();

    fn try_from(c: char) -> Result<Self, ()> {
        match c {
            'A' => Ok(Shape::Rock),
            'B' => Ok(Shape::Paper),
            'C' => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

impl Shape {
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

    fn response_for(&self, outcome: &Outcome) -> Shape {
        match outcome {
            Outcome::Win => match self {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            Outcome::Draw => self.clone(),
            Outcome::Loss => match self {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
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

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(c: char) -> Result<Self, ()> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
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
        let them: Shape = chars
            .next()
            .expect("no opponent pick")
            .try_into()
            .expect("unable to parse opponent pick");
        let outcome: Outcome = chars
            .nth(1)
            .expect("no desired outcome")
            .try_into()
            .expect("unable to parse desired outcome");
        let me = them.response_for(&outcome);
        score += me.score() + outcome.score();
    }

    println!("Final score: {score}")
}
