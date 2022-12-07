use std::collections::HashMap;
use std::io;

fn compute_dir_sizes() -> HashMap<String, u32> {
    let mut dir_sizes = HashMap::new();
    let mut cwd: Vec<String> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.expect("failed to read line");
        let tokens : Vec<_> = line.split_whitespace().collect();
        match tokens.as_slice() {
            ["$", "cd", dir] => {
                match *dir {
                    "/"  => cwd = vec![".".to_string()],
                    ".." => {
                        cwd.pop();
                    },
                    _ => {
                        cwd.push(dir.to_string());
                    }
                }
            }
            ["$", "ls"] => (),
            ["dir", _dirname] => (),
            [size, _filename] => {
                let mut dir = String::new();
                for seg in &cwd {
                    dir.push_str(&format!("/{}", seg));
                    let dir_size = dir_sizes.entry(dir.clone()).or_insert(0);
                    *dir_size += size.parse::<u32>()
                        .unwrap_or_else(|_| panic!("failed to parse size: {}", size));
                }
            }
            _ => panic!("something is wrong with this line: {}", line)
        }
    }

    dir_sizes
}

fn part_one(dir_sizes: &HashMap<String, u32>) -> u32 {
    const AT_MOST: u32 = 100_000;
    dir_sizes.iter()
        .filter_map(|(_, &size)| {
            if size < AT_MOST {
                Some(size)
            } else {
                None
            }
        })
        .sum()
}

fn part_two(dir_sizes: &HashMap<String, u32>) -> u32 {
    const DISK_SIZE: u32 = 70_000_000;
    const SIZE_REQUIRED_FOR_UPDATE: u32 = 30_000_000;

    let free = DISK_SIZE - *dir_sizes.get("/.").unwrap();

    dir_sizes.iter()
        .filter_map(|(_, &size)| {
            if size >= SIZE_REQUIRED_FOR_UPDATE - free {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap_or_else(|| panic!("no directory is large enough"))
}

fn main() {
    let dir_sizes = compute_dir_sizes();
    println!("{}", part_one(&dir_sizes));
    println!("{}", part_two(&dir_sizes));
}