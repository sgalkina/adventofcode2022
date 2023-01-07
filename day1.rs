use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elves = Vec::<i32>::new();
    let mut current_elf: i32 = 0;
    if let Ok(lines) = read_lines("./inputs/day1.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    elves.push(current_elf);
                    current_elf = 0;
                } else {
                    current_elf += ip.parse::<i32>().unwrap();
                }
            }
        }
        elves.sort();
        println!("Three best elves carry {}", elves.iter().rev().take(3).sum::<i32>());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}