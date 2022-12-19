use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Tree {
    height: i32,
    is_visible: Option<bool>,
}


fn day8(lines: std::io::Lines<io::BufReader<File>>) {
    for line in lines {
        if let Ok(ip) = line {
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day8.txt") {
        day8(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}