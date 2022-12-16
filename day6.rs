use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;


fn day6(lines: std::io::Lines<io::BufReader<File>>) {
    for line in lines {
        if let Ok(ip) = line {
            let vec: Vec<char> = ip.chars().collect();
            for i in 14..vec.len() {
                let mut uniq = HashSet::new();
                if vec[i-14..i].into_iter().all(move |x| uniq.insert(x)) {
                    println!("Result {}", i);
                    break;
                }
            }
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day6.txt") {
        day6(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}