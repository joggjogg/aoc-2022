use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
#[derive(PartialEq)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

struct Move {
    kind: Item,
}

fn main() {
    let mut total_score: i32 = 0;

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(round) = line {
                let mut iterator = round.split_whitespace();
                let opponent_move = iterator.next();
                let my_outcome = iterator.next();
                let my_move = decide_move(&opponent_move.unwrap(), &my_outcome.unwrap());

                total_score += calculate_score(my_move, my_outcome.unwrap())

            }
        }
        println!("{:?}", total_score);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn decide_move(opponent_move: &str, outcome: &str) -> Move  {
    if opponent_move.eq("A") && outcome.eq("X") {
        return Move { kind: (Item::Scissors) }
    }
    if opponent_move.eq("A") && outcome.eq("Y") {
        return Move { kind: (Item::Rock) }
    }
    if opponent_move.eq("A") && outcome.eq("Z") {
        return Move { kind: (Item::Paper) }
    }
    if opponent_move.eq("B") && outcome.eq("X") {
        return Move { kind: (Item::Rock) }
    }
    if opponent_move.eq("B") && outcome.eq("Y") {
        return Move { kind: (Item::Paper) }
    }
    if opponent_move.eq("B") && outcome.eq("Z") {
        return Move { kind: (Item::Scissors) }
    }
    if opponent_move.eq("C") && outcome.eq("X") {
        return Move { kind: (Item::Paper) }
    }
    if opponent_move.eq("C") && outcome.eq("Y") {
        return Move { kind: (Item::Scissors) }
    }
    if opponent_move.eq("C") && outcome.eq("Z") {
        return Move { kind: (Item::Rock) }
    } else {
        return Move { kind: (Item::Rock) }
    }
}

fn calculate_score(my_move: Move, my_outcome: &str) -> i32 {
    let mut move_score :i32 = 0;
    let mut outcome_score :i32 = 0;

    if my_move.kind == Item::Rock {
        move_score = 1;
    }
    if my_move.kind == Item::Paper {
        move_score = 2;
    }
    if my_move.kind == Item::Scissors {
        move_score = 3;
    }

    if my_outcome.eq("X") {
        outcome_score = 0;
    }
    if my_outcome.eq("Y") {
        outcome_score = 3;
    }
    if my_outcome.eq("Z") {
        outcome_score = 6;
    }

    return move_score + outcome_score;
}

