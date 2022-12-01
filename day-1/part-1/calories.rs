use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut most_calories: i32 = 0;
    let mut current_calories: i32 = 0;

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(meal) = line {
                if meal.is_empty() {
                    if current_calories > most_calories {
                        most_calories = current_calories;
                    }

                    current_calories = 0;
                } else {
                    current_calories += meal.parse::<i32>().unwrap();
                }
            }
        }
    }

    println!("{}", most_calories);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
