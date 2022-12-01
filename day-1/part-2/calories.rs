use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut current_calories: i32 = 0;
    let mut top_list: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(meal) = line {
                if meal.is_empty() {
                    top_list.push(current_calories);
                    current_calories = 0;
                } else {
                    current_calories += meal.parse::<i32>().unwrap();
                }
            }
        }
    }

    let total_calories = calculate_top_three(top_list);
    println!("{}", total_calories);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_top_three(mut top_list: Vec<i32>) -> i32 {
    top_list.sort();
    top_list.reverse();
    return &top_list[0] + &top_list[1] + &top_list[2];
}