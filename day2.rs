use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};


fn day2(lines: std::io::Lines<io::BufReader<File>>) {

    let column2 = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let victories = HashSet::from(["A Y", "B Z", "C X"]);
    let losses = HashSet::from(["A Z", "B X", "C Y"]);
    let draws = HashSet::from(["A X", "B Y", "C Z"]);

    let mut result: i32 = 0;

    for line in lines {
        if let Ok(ip) = line {
            let len = ip.len();
            let score = column2.get(&ip[len-1..]).unwrap();
            if victories.contains(&ip.as_str()) {
                result += score + 6;
            }
            if losses.contains(&ip.as_str()) {
                result += score;
            }
            if draws.contains(&ip.as_str()) {
                result += score + 3;
            }
        }
    }
    println!("Result: {}", result);
}


fn day2_part2(lines: std::io::Lines<io::BufReader<File>>) {

    let column2 = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6),
    ]);

    let victories = HashMap::from([
        ("A", 2),
        ("B", 3),
        ("C", 1),
    ]);
    let losses = HashMap::from([
        ("A", 3),
        ("B", 1),
        ("C", 2),
    ]);
    let draws = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3),
    ]);

    let mut result: i32 = 0;

    for line in lines {
        if let Ok(ip) = line {
            let len = ip.len();
            let last = &ip[len-1..];
            let first = &ip[0..1];
            let score_last = column2.get(last).unwrap();

            if last.eq("Z") {
                result += victories.get(first).unwrap() + score_last;
            }
            if last.eq("Y") {
                result += draws.get(first).unwrap() + score_last;
            }
            if last.eq("X") {
                result += losses.get(first).unwrap() + score_last;
            }
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