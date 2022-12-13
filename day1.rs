use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut best_elf: i32 = 0;
    let mut current_elf: i32 = 0;
    if let Ok(lines) = read_lines("./inputs/day1.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if "".eq(&ip) {
                    if current_elf > best_elf {
                        best_elf = current_elf;
                    }
                    current_elf = 0;
                } else {
                    current_elf += ip.parse::<i32>().unwrap();
                }
            }
        }
        println!("Best elf carries {}", best_elf);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}