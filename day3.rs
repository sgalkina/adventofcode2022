use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};


fn day3(lines: std::io::Lines<io::BufReader<File>>) {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let values: Vec<i32> = (1..53).collect();
    let scores: HashMap<char, i32> = alphabet.into_iter().zip(values.into_iter()).collect();

    let mut result: i32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let len = ip.len();
            let first: HashSet<char> = ip[0..len/2].chars().collect();
            let second: HashSet<char> = ip[len/2..].chars().collect();
            result += scores[&first.intersection(&second).collect::<Vec<&char>>()[0]];
        }
    }
    println!("Result: {}", result);
}


fn main() {
    if let Ok(lines) = read_lines("./inputs/day3.txt") {
        day3(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}