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


fn day3_part2(lines: std::io::Lines<io::BufReader<File>>) {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let values: Vec<i32> = (1..53).collect();
    let scores: HashMap<char, i32> = alphabet.into_iter().zip(values.into_iter()).collect();

    let mut result: i32 = 0;
    let mut counter: i32 = 0;
    let mut first = String::new();
    let mut second = String::new();
    let mut third = String::new();

    for line in lines {
        if let Ok(ip) = line {
            if counter == 2 {
                third = ip.clone();

                let firsts: HashSet<char> = first.chars().collect();
                let seconds: HashSet<char> = second.chars().collect();
                let thirds: HashSet<char> = third.chars().collect();
        
                let diff1: HashSet<_> = firsts.intersection(&seconds).collect();
                let diff2: HashSet<_> = thirds.intersection(&seconds).collect();
        
                let res = diff1.intersection(&diff2);
                result += scores[res.collect::<Vec<&&char>>()[0]];

                counter = 0;
            } else {
                match counter {
                    0 => first = ip.clone(),
                    1 => second = ip.clone(),
                    _ => println!("Wrong"),
                };
                counter += 1;
            }
        }
    }
    println!("Result: {}", result);
}


fn main() {
    if let Ok(lines) = read_lines("./inputs/day3.txt") {
        day3_part2(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}