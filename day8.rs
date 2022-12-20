use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn fill_visibility(ar: & Vec<Vec<i32>>, vis: &mut Vec<Vec<i32>>, ids: &Vec<(usize, usize)>) {
    let mut prev: i32 = -1;
    for (x, y) in ids.iter() {
        while ar[*x][*y] > prev {
            vis[*x][*y] = 1;
            prev = ar[*x][*y];
        }
    }
}

fn iteration_order(size: usize) -> Vec<Vec<(usize, usize)>> {
    let mut result: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut left_right: Vec<(usize, usize)> = Vec::new();
    let mut right_left: Vec<(usize, usize)> = Vec::new();
    let mut up_down: Vec<(usize, usize)> = Vec::new();
    let mut down_up: Vec<(usize, usize)> = Vec::new();
    for i in 0..size {
        for j in 0..size {
            left_right.push((i, j));
            right_left.push((i, size - j - 1));
            up_down.push((j, i));
            down_up.push((size - j - 1, i));
        }
        result.push(left_right.clone());
        result.push(right_left.clone());
        result.push(up_down.clone());
        result.push(down_up.clone());
        left_right = Vec::new();
        right_left = Vec::new();
        up_down = Vec::new();
        down_up = Vec::new();
    }
    result
}


fn len_sorted(w: &Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    let size = w.len();
    if size <= 1 {
        return res;
    }
    let mut prev: i32 = w[0];
    for i in 1..size {
        if (w[i] < w[0]) {
            res += 1;
            prev = w[i];
        } else {
            res += 1;
            break;
        }
    }
    res
}


fn fill_scentic_score(ar: & Vec<Vec<i32>>, score: &mut Vec<Vec<i32>>) {
    let mut k: usize = 0;
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut up: i32 = 0;
    let mut down: i32 = 0;
    let size: usize = ar.len();
    for i in 0..size {
        for j in 0..size {
            right = len_sorted(&ar[i][j..size].to_vec());
            down = len_sorted(&ar[i..size].iter().map(|v| v[j]).collect::<Vec<i32>>());
            left = len_sorted(&ar[i][0..j+1].iter().copied().rev().collect());
            up = len_sorted(&ar[0..i+1].iter().map(|v| v[j]).rev().collect::<Vec<i32>>());
            score[i][j] = left*down*right*up;
        }
    }
}


fn day8(lines: std::io::Lines<io::BufReader<File>>) {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            let nums = ip.chars().map(|s| s.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
            matrix.push(nums);
        }
    }
    let size: usize = matrix.len();
    let mut vis: Vec<Vec<i32>> = vec![vec![0; size]; size];
    let order = iteration_order(size);
    for v in order.iter() {
        fill_visibility(&matrix, & mut vis, &v);
    }
    println!("{:?}", vis.iter().map(|x| -> i32 { x.iter().sum() }).sum::<i32>());
    let mut score: Vec<Vec<i32>> = vec![vec![0; size]; size];
    fill_scentic_score(&matrix, & mut score);
    println!("{:?}", score.iter().map(|x| -> i32 { *x.iter().max().unwrap() }).max().unwrap());
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