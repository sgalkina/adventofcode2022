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


fn main() {
    if let Ok(lines) = read_lines("./inputs/day2.txt") {
        day2(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}