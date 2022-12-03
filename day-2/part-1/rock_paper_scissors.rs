use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut total_score :i32 = 0;
    let moves = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
           if let Ok(round) = line {
               let score = moves.get(&round as &str);

               match score {
                   Some(int) => {
                       total_score += int;
                   }
                   None => ()
               }
           }
        }
    }

    println!("{:?}", total_score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
