use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn parse_line(instr: &String, values: &mut Vec<i32>) {
    let vals: Vec<&str> = instr.split_whitespace().collect();
    match vals[0] {
        "noop" => {
            values.push(*values.last().unwrap()); 
        },
        "addx" => {
            values.push(*values.last().unwrap()); 
            values.push(*values.last().unwrap() + vals[1].parse::<i32>().unwrap()); 
        },
        _ => println!("Input error, do nothing"),
    }
}


fn draw_crt(crt: &Vec<bool>) {
    let string: String = crt.iter().map(|v| if *v {"#"} else {"."}).collect::<String>();
    for i in 0..6 {
        println!("{}", &string[i*40..(i*40 + 40)]);
    }
}


fn day10(lines: std::io::Lines<io::BufReader<File>>) {
    let mut values: Vec<i32> = Vec::new();
    let mut crt: Vec<bool> = vec![false; 240];
    values.push(1); // step 0
    for line in lines {
        if let Ok(ip) = line {
            parse_line(&ip, &mut values);
        }
    }
    values.push(*values.last().unwrap()); 
    values.push(*values.last().unwrap()); 
    println!("{:?}", values.len());
    let mut result: i32 = 0;
    for i in [20, 60, 100, 140, 180, 220] {
        println!("{:?}", (i, (i as i32)*values[i-1]));
        result += (i as i32)*values[i-1];
    }
    println!("Result {}", result);
    for i in 0..240 {
        let k = (i as i32) % 40;
        if i == 0 {
            crt[i] = (k == values[i]) | ((k + 1) == values[i]);
        } else {
            crt[i] = (k-1..k+2).contains(&values[i]);
        }
    }
    draw_crt(&crt);
}


fn main() {
    if let Ok(lines) = read_lines("./inputs/day10.txt") {
        day10(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}