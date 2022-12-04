use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use alphabet::*;

fn main() {
    let mut sum_of_priorities :i32 = 0;
    alphabet!(LATIN = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

    if let Ok(lines) = read_lines("./src/input") {
        for line in lines {
            if let Ok(items) = line {
                let item_len = items.len();
                let (first_rucksack, second_rucksack) = items.split_at(item_len / 2);
                let duplicate = find_duplicate(first_rucksack, second_rucksack);
                sum_of_priorities += calculate_priority(duplicate.unwrap(), LATIN);
            }
        }
        println!("{:?}", sum_of_priorities);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_priority(character: char, letters: [char; 52]) -> i32 {
    let position = letters.iter().position(|&x| x == character).unwrap() as i32;
    return position + 1;
}

fn find_duplicate(a: &str,b: &str) -> Option<char> {
    let characters = a.chars();

    for char in characters {
        if b.contains(char) {
            return Some(char);
        }
    }

    None
}

