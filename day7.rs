use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Dir {
    pub name: String,
    pub files: HashMap<String, Fil>,
    pub dirs: HashMap<String, usize>,
    pub parent: Option<usize>,
}

#[derive(Debug, Clone)]
struct Fil {
    pub name: String,
    pub size: i32,
}


impl Dir {
    pub fn from(name: &String, parent: Option<usize>) -> Self {
        Dir {
            name: name.clone(),
            files: HashMap::new(),
            dirs: HashMap::new(),
            parent: parent,
        }
    }

    pub fn add_dir(& mut self, name: &String, idx: usize) {
        self.dirs.insert(name.clone(), idx);
    }

    pub fn add_file(& mut self, child: Fil) {
        self.files.insert(
            child.name.clone(),
            child,
        );
    }

    pub fn files_sizes(& self) -> i32 {
        let mut result: i32 = 0;
        for (_, value) in self.files.clone().into_iter() {
            result += value.size; 
        }
        result
    }
}


fn parse_file(line: & String) -> Fil {
    let vals: Vec<&str> = line.split_whitespace().collect();
    let size: i32 = vals[0].parse().unwrap();
    Fil { name: vals[1].to_string(), size: size }
}

#[derive(Debug)]
enum Action {
    GoToChild, 
    GoToParent,
    AddFile,
    AddDir,
    SkipLs,
}


fn parse_input(s: &String) -> (Action, String) {
    if s.eq("$ cd ..") {
        (Action::GoToParent, String::new())
    } else if s.eq("$ cd /") {
        (Action::SkipLs, String::new()) // hack for the first line
    } else if s.starts_with("$ cd") {
        (Action::GoToChild, s.replace("$ cd ", ""))
    } else if s.starts_with(char::is_numeric) {
        (Action::AddFile, s.clone())
    } else if s.starts_with("dir") {
        (Action::AddDir, s.replace("dir ", ""))
    } else {
        (Action::SkipLs, String::new())
    }
}


fn calculate_sizes(dirs: &Vec<Dir>, idx: usize, results: &mut Vec<i32>) {
    let mut result: i32 = dirs[idx].files_sizes();
    for (_, c) in dirs[idx].dirs.clone().into_iter() {
        calculate_sizes(dirs, c, results);
        result += results[c];
    }
    results[idx] = result;
}


fn day7(lines: std::io::Lines<io::BufReader<File>>) {
    let mut dirs: Vec<Dir> = Vec::new();
    let mut files_sizes: Vec<i32> = Vec::new();
    dirs.push(Dir::from(&"/".to_string(), None));
    let mut cur: usize = 0;
    for line in lines {
        if let Ok(ip) = line {
            let (action, s) = parse_input(&ip);
            match action {
                Action::AddFile => {
                    let fil = parse_file(&ip);
                    dirs[cur].add_file(fil.clone());
                    files_sizes.push(fil.size.clone());
                },
                Action::AddDir => {
                    let idx = dirs.len();
                    dirs.push(Dir::from(&s, Some(cur)));
                    dirs[cur].add_dir(&s, idx);
                },
                Action::SkipLs => continue,
                Action::GoToChild => {
                    cur = *dirs[cur].dirs.get(&s).unwrap();
                },
                Action::GoToParent => {
                    cur = dirs[cur].parent.unwrap();
                },
            };
        }
    }
    let mut results: Vec<i32> = vec![0; dirs.len()];
    calculate_sizes(&dirs, 0, &mut results);
    println!("{:?}", results);

    // Find the answers to the puzzle
    println!("Part 1: {}", results.iter().filter(|x| *x < &100000).sum::<i32>());
    
    let space_remaining: i32 = 70_000_000 - files_sizes.iter().sum::<i32>();
    let space_needed = 30_000_000 - space_remaining;

    println!("Part 2: {}", results.iter().filter(|i| (*i >= &space_needed)).min().unwrap());
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day7.txt") {
        day7(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}