use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::collections::VecDeque;


#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i32>,
    operation: Option<String>,
    test: Option<i32>,
    pass_true: Option<usize>,
    pass_false: Option<usize>,
    counter: usize,
}


fn parse_line(instr: &String, monkey: &mut Option<Monkey>) {
    if monkey.is_none() {
        println!("No monkey!");
        return
    }
    let vals: Vec<&str> = instr.split(": ").collect();
    match vals[0] {
        "  Starting items" => { monkey.as_mut().unwrap().items = vals[1].split(", ")
                                .map(|s| s.parse().expect("parse error"))
                                .collect(); },
        "  Operation" => { monkey.as_mut().unwrap().operation = Some(vals[1].replace("new = ", "")); },
        "  Test" => { monkey.as_mut().unwrap().test = Some(FromStr::from_str(&vals[1].replace("divisible by ", "")).unwrap()); },
        "    If true" => { monkey.as_mut().unwrap().pass_true = Some(FromStr::from_str(&vals[1].replace("throw to monkey ", "")).unwrap()); },
        "    If false" => { monkey.as_mut().unwrap().pass_false = Some(FromStr::from_str(&vals[1].replace("throw to monkey ", "")).unwrap()); },
        _ => println!("Other input, do nothing"),
    }
}


fn operation(ops: &String, old: i32) -> i32 {
    let vals: Vec<&str> = ops.split_whitespace().collect();
    let first = match vals[0] {
        "old" => old,
        _ => FromStr::from_str(&vals[0]).unwrap(),
    };
    let second = match vals[2] {
        "old" => old,
        _ => FromStr::from_str(&vals[2]).unwrap(),
    };
    match vals[1] {
        "*" => first*second,
        "+" => first+second,
        "/" => first/second,
        "-" => first-second,
        _ => 666,
    }
}


fn turn(monkeys: &mut Vec<Monkey>, monkey_id: usize) {
    println!("In {:?} making {}", monkeys, monkey_id);
    while !monkeys[monkey_id].items.is_empty() {
        let worry_old = monkeys[monkey_id].items.pop_front().unwrap();
        monkeys[monkey_id].counter += 1;
        println!("Monkey {} inspects an item with a worry level of  {}", monkey_id, worry_old);
        let worry_new = operation(monkeys[monkey_id].operation.as_ref().unwrap(), worry_old) / 3;
        if worry_new % monkeys[monkey_id].test.unwrap() == 0 {
            let true_val = monkeys[monkey_id].pass_true.unwrap();
            monkeys[true_val].items.push_back(worry_new);
            println!("Divisible! Item with worry level {} is thrown to monkey {}", worry_new, true_val);
        } else {
            let false_val = monkeys[monkey_id].pass_false.unwrap();
            monkeys[false_val].items.push_back(worry_new);
            println!("Not divisible! Item with worry level {} is thrown to monkey {}", worry_new, false_val);
        }
    }
}


fn day11(lines: std::io::Lines<io::BufReader<File>>) {
    let mut monkey: Option<Monkey> = None;
    let mut monkeys: Vec<Monkey> = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            if ip.starts_with("Monkey") {
                if !monkey.is_none() {
                    monkeys.push(
                        monkey.unwrap(),
                    );
                }
                let a = ip.replace(":", "");
                let vals: Vec<&str> = a.split_whitespace().collect();
                monkey = Some(Monkey {
                    items: VecDeque::new(),
                    operation: None, 
                    test: None, 
                    pass_true: None, 
                    pass_false: None,
                    counter: 0,
                });
            } else {
                parse_line(&ip, &mut monkey);
            }
        }
    }
    monkeys.push(
        monkey.unwrap(),
    );
    let size = monkeys.len();
    println!("{:?}", monkeys);
    for _ in 0..20 {
        for k in 0..size {
            turn(&mut monkeys, k);
        }
        println!("{:?}", monkeys);
    }
    let mut counts: Vec<usize> = (0..size).map(|k| monkeys[k].counter).collect();
    counts.sort();
    counts.reverse();
    println!("{:?}", counts[0] * counts[1]);
}


fn main() {
    if let Ok(lines) = read_lines("./inputs/day11.txt") {
        day11(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}