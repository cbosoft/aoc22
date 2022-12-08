use std::fs::{File, read};
use std::io::{BufReader, Result, Lines, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Clone)]
enum Move{
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn get_winner(&self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
    fn get_loser(&self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
    fn get_draw(&self) -> Self {
        (*self).clone()
    }
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            'A' => Move::Rock,
            'X' => Move::Rock,

            'B' => Move::Paper,
            'Y' => Move::Paper,

            'C' => Move::Scissors,
            'Z' => Move::Scissors,

            _ => panic!("unexpected char: {c}")
        }
    }
}

struct Round(Move, Move);

impl From<String> for Round {
    fn from(s: String) -> Self {
        let chars: Vec<char> = s.chars().collect();
        assert_eq!(chars.len(), 3usize);
        let a = Move::from(chars[0]);
        let b = Move::from(chars[2]);
        Round(a, b)
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

impl Round {
    fn amend_misunderstanding(self) -> Self {
        let play = match &self {
            Round(op, Move::Rock) => op.get_loser(),
            Round(op, Move::Paper) => op.get_draw(),
            Round(op, Move::Scissors) => op.get_winner()
        };
        Round(self.0, play)
    }

    fn score(self) -> u16 {
        let kind_score: u16 = match &self.1 {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };
        let outcome_score: u16 = match self {
            // Win
            Round(Move::Rock, Move::Paper) => 6,
            Round(Move::Paper, Move::Scissors) => 6,
            Round(Move::Scissors, Move::Rock) => 6,

            // Lose
            Round(Move::Rock, Move::Scissors) => 0,
            Round(Move::Paper, Move::Rock) => 0,
            Round(Move::Scissors, Move::Paper) => 0,

            // Draw
            _ => 3
        };
        kind_score + outcome_score
    }
}

fn parse_lines_get_score(lines: Lines<BufReader<File>>) -> u16
{
    let mut score = 0u16;

    for line in lines {
        if let Ok(line) = line {
            score += Round::from(line)
                .amend_misunderstanding()
                .score();
        }
    }

    score
}

fn main() {
    let lines = read_lines("strategy.txt").expect("error reading file");
    let score = parse_lines_get_score(lines);
    println!("Score: {score}");
}
