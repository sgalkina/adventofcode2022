use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;


fn go_to((x, y): (usize, usize), instr: &String) -> Vec<(usize, usize)> {
    let vals: Vec<&str> = instr.split_whitespace().collect();
    let num: usize = vals[1].parse().unwrap();
    match vals[0] {
        "R" => (1..num+1).map(|i| (x+i, y)).collect(),
        "L" => (1..num+1).map(|i| (x-i, y)).collect(),
        "U" => (1..num+1).map(|i| (x, y+i)).collect(),
        "D" => (1..num+1).map(|i| (x, y-i)).collect(),
        _ => {
            println!("Input error, do nothing");
            Vec::new()
        },
    }
}


fn new_value_both(x_t: i32, x_diff: i32) -> usize {
    let mut x_r = x_t;
    match x_diff {
        -1 => {x_r -= 1;},
        1 => {x_r += 1;},
        -2 => {x_r -= 1;},
        2 => {x_r += 1;},
        _ => {x_r = x_r;},
    }
    x_r as usize
}

fn new_value_one(x_t: i32, x_diff: i32) -> usize {
    let mut x_r = x_t;
    match x_diff {
        -2 => {x_r -= 1;},
        2 => {x_r += 1;},
        _ => {x_r = x_r;},
    }
    x_r as usize
}


fn move_tail((x_t, y_t): (usize, usize), (x_h, y_h): (usize, usize)) -> (usize, usize) {
    let (x_diff, y_diff) = ((x_h as i32 - x_t as i32), (y_h as i32 - y_t as i32));
    if (x_diff.abs() == y_diff.abs()) & (x_diff.abs() == 1) {
        println!("{:?}", x_diff.abs());
        (x_t, y_t)
    } else if x_diff == 0 {
        (x_t, new_value_one(y_t as i32, y_diff))
    } else if y_diff == 0 {
        (new_value_one(x_t as i32, x_diff), y_t)
    } else {
        (new_value_both(x_t as i32, x_diff), new_value_both(y_t as i32, y_diff))
    }
}


fn day9(lines: std::io::Lines<io::BufReader<File>>) {
    let mut head_coords: (usize, usize) = (100, 100);
    let mut tail_coords: (usize, usize) = (100, 100);
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    points.insert(tail_coords);
    for line in lines {
        if let Ok(ip) = line {
            let steps_head = go_to(head_coords, &ip);
            head_coords = *steps_head.last().unwrap();
            for cs in steps_head {
                tail_coords = move_tail(tail_coords, cs);
                points.insert(tail_coords);
                println!("head {:?}", cs);
                println!("tail {:?}", tail_coords);
            }
        }
    }
    println!("{:?}", points.len());
}


fn day9_part2(lines: std::io::Lines<io::BufReader<File>>) {
    let mut head_coords: (usize, usize) = (100, 100);
    let mut tail_coords_vec: Vec<(usize, usize)> = vec![head_coords; 9];
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    points.insert(*tail_coords_vec.last().unwrap());
    for line in lines {
        if let Ok(ip) = line {
            let steps_head = go_to(head_coords, &ip);
            head_coords = *steps_head.last().unwrap();
            for cs in steps_head {
                for i in 0..tail_coords_vec.len() {
                    if i == 0 {
                        tail_coords_vec[i] = move_tail(tail_coords_vec[i], cs);
                    } else {
                        tail_coords_vec[i] = move_tail(tail_coords_vec[i], tail_coords_vec[i-1]);
                    }
                }
                points.insert(*tail_coords_vec.last().unwrap());
            }
        }
    }
    println!("{:?}", points.len());
}


fn main() {
    if let Ok(lines) = read_lines("./inputs/day9.txt") {
        day9_part2(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}