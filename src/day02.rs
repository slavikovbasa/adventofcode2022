use core::panic;

pub const URL: &str = "https://adventofcode.com/2022/day/2/input";

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn from_str(s: &str) -> Self {
        match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("bad outcome char")
        }
    }

    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

const BEAT_PAIRS: [(Move, Move); 3] = [
    (Move::Rock, Move::Scissors),
    (Move::Paper, Move::Rock),
    (Move::Scissors, Move::Paper),
];

impl Move {

    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("bad move char")
        }
    }

    fn from_outcome(prev: Self, outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Draw => prev,
            Outcome::Win => prev.get_stronger(),
            Outcome::Lose => prev.get_weaker(),
        }
    }

    fn get_stronger(&self) -> Self {
        BEAT_PAIRS.iter().find_map(
            |(winner, loser)| if self == loser { Some(*winner) } else { None }
        ).unwrap()
    }


    fn get_weaker(&self) -> Self {
        BEAT_PAIRS.iter().find_map(
            |(winner, loser)| if self == winner { Some(*loser) } else { None }
        ).unwrap()
    }

    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn outcome(&self, other: Self) -> Outcome {
        if self == &other {
            return Outcome::Draw
        }

        if self.get_weaker() == other { Outcome::Win } else { Outcome::Lose }
    }
}



pub fn solve1(text: &str) -> u32 {
    text.lines().map(
        |x| {
            let mut moves = x.split_whitespace();
            let (left, right) = (moves.next().unwrap(), moves.next().unwrap());
            let opponent_move = Move::from_str(left);
            let my_move = Move::from_str(right);
            let outcome = my_move.outcome(opponent_move);
            my_move.score() + outcome.score()
        }
    ).sum()
}

pub fn solve2(text: &str) -> u32 {
    text.lines().map(
        |x| {
            let mut moves = x.split_whitespace();
            let (left, right) = (moves.next().unwrap(), moves.next().unwrap());
            let opponent_move = Move::from_str(left);
            let outcome = Outcome::from_str(right);
            let my_move = Move::from_outcome(opponent_move, &outcome);
            my_move.score() + outcome.score()
        }
    ).sum()
}
