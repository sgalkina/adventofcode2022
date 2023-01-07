use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn day2(lines: std::io::Lines<io::BufReader<File>>) {
    let mut result: i32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let len = ip.len();
            let score = match &ip[len - 1 ..] {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("bad letter: {}", ip)
            };
            result += match ip.as_str() {
                "A Y" | "B Z" | "C X" => score + 6,
                "A Z" | "B X" | "C Y" => score,
                "A X" | "B Y" | "C Z" => score + 3,
                _ => panic!("bad line: {}", ip),
            }
        }
    }
    println!("Result: {}", result);
}


fn day2_part2(lines: std::io::Lines<io::BufReader<File>>) {
    let mut result: i32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let len = ip.len();
            let last = &ip[len-1..];
            let first = &ip[0..1];
            let score_last = match last {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!("bad letter: {}", ip)
            };
            result += if last == "Z" {
                match first {
                    "A" => 2,
                    "B" => 3,
                    "C" => 1,
                    _ => panic!("bad line: {}", first),
                }
            } else if last == "Y" {
                match first {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    _ => panic!("bad line: {}", first),
                }
            } else if last == "X" {
                match first {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => panic!("bad line: {}", first),
                }
            } else {
                0
            } + score_last;
        }
    }
    println!("Result: {}", result);
}


fn main() {
    if let Ok(lines) = read_lines("./inputs/day2.txt") {
        day2_part2(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}