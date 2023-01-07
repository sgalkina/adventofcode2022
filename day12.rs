use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::collections::VecDeque;


#[derive(Debug, Clone)]
struct Node {
    name: String,
    value: i32,
    parent: i32,
    children: Vec<i32>,
}


fn day12(lines: std::io::Lines<io::BufReader<File>>) {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let values: Vec<i32> = (1..27).collect();
    let scores: HashMap<char, i32> = alphabet.into_iter().zip(values.into_iter()).collect();
    for line in lines {
        if let Ok(ip) = line {
        }
    }
}


fn main() {
    if let Ok(lines) = read_lines("./inputs/day12_test.txt") {
        day12(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}