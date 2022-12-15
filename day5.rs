use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn parse_stacks(vec: &mut Vec<String>) -> Vec<Vec<String>> {
    let columns: Vec<i32> = vec.pop().expect("pop error")
                .trim().split_whitespace()
                .map(|s| s.parse().expect("parse error"))
                .collect();
    println!("columns: {}", columns.len());
    let mut result = Vec::<Vec<String>>::new();
    for i in 0..columns.len() {
        result.push(Vec::<String>::new());
    }
    for v in vec {
        for i in 0..columns.len() {
            let c = v.chars().nth(4*i + 1).unwrap().to_string();
            if !" ".eq(&c) {
                result[i].push(c);
            }
        }
    }
    // Print the contents of the vector
    for sub_vec in &result {
        print!("{:?}\n", sub_vec);
    }
    result
}


fn day5(lines: std::io::Lines<io::BufReader<File>>) {
    let mut result: i32 = 0;
    let mut strings = Vec::<String>::new();
    let mut stacks = Vec::<Vec<String>>::new();
    let mut to_collect: bool = true;
    for line in lines {
        if let Ok(ip) = line {
            if to_collect & !"".eq(&ip) {
                strings.push(ip.clone());
            }
            if to_collect & "".eq(&ip) {
                to_collect = false;
                stacks = parse_stacks(&mut strings);
            }
            // if !to_collect {

            // }
        }
    }
    println!("Result: {}", result);
}


fn day5_part2(lines: std::io::Lines<io::BufReader<File>>) {
}


fn main() {
    if let Ok(lines) = read_lines("./inputs/day5.txt") {
        day5(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}