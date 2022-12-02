use std::{env, fs};

#[derive(Copy, Clone, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Loss,
    Tie,
    Win,
}

#[derive(Copy, Clone, Debug)]
struct Round(Move, Outcome);

trait Score {
    fn score(&self) -> i64;
}

impl Move {
    fn from_str(raw: &str) -> Move {
        match raw {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Score for Move {
    fn score(&self) -> i64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Outcome {
    fn from_str(raw: &str) -> Outcome {
        match raw {
            "X" => Outcome::Loss,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            _ => unreachable!(),
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> i64 {
        match self {
            Outcome::Loss => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }
}

impl Round {
    fn player_move_from_outcome(&self) -> Move {
        match self.1 {
            Outcome::Loss => match self.0 {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Outcome::Tie => self.0,
            Outcome::Win => match self.0 {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
        }
    }

    // Parse space-separated pairs. e.g. "A X"
    fn from_str_pair(pair: &str) -> Round {
        let values = pair.split(' ').collect::<Vec<&str>>();
        let (raw_opponent, raw_player) = (values[0], values[1]);

        Round(Move::from_str(raw_opponent), Outcome::from_str(raw_player))
    }
}

impl Score for Round {
    fn score(&self) -> i64 {
        self.player_move_from_outcome().score() + self.1.score()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("You must supply the path to the input file");
    }

    let path = &args[1];
    let round_scores = fs::read(path)
        .map(|bytes| String::from_utf8(bytes).unwrap())
        .map(|contents| {
            contents
                .split('\n')
                .collect::<Vec<&str>>()
                .iter()
                .filter(|round| round.trim() != "")
                .map(|round| Round::from_str_pair(round).score())
                .collect::<Vec<i64>>()
        });

    match round_scores {
        Ok(scores) => println!(
            "The total score for the player is {}",
            scores.iter().sum::<i64>()
        ),
        Err(err) => panic!("There was an error gathering the scores: {:?}", err),
    }
}
