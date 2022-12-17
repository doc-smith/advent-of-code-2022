use std::io;

fn read_stacks() -> Vec<Vec<u8>> {
    let mut stack_map = Vec::new();
    for line in io::stdin().lines() {
        let line = line.expect("input error");
        if line.is_empty() {
            break;
        }
        stack_map.push(line);
    }
    if stack_map.is_empty() {
        panic!("no input");
    }

    let n = (stack_map[0].len() + 1) / 4;
    if n > 10 {
        panic!("too many stacks");
    }

    let mut stacks = vec![Vec::new(); n];
    for line in stack_map.iter().rev().skip(1) {
        let crates = line.as_bytes().iter().skip(1).step_by(4);
        for (i, c) in crates.enumerate() {
            if !c.is_ascii_whitespace() {
                stacks[i].push(*c);
            }
        }
    }

    stacks
}

struct Command {
    from: usize,
    to: usize,
    cnt: usize,
}

fn decode_command(line: &str) -> Command {
    let tokens: Vec<_> = line.split_whitespace().collect();
    match tokens.as_slice() {
        ["move", cnt, "from", from, "to", to] => {
            Command {
                from: from.parse().unwrap(),
                to: to.parse().unwrap(),
                cnt: cnt.parse().unwrap(),
            }
        }
        _ => panic!("invalid input, cannot parse command: {}", line),
    }
}

struct Input {
    stacks: Vec<Vec<u8>>,
    commands: Vec<Command>,
}

fn read_input() -> Input {
    let stacks = read_stacks();
    let commands = io::stdin().lines()
        .map(|line| {
            let line = line.expect("input error");
            decode_command(&line)
        })
        .collect();
    Input { stacks, commands }
}

fn manipulate<F>(input: &Input, mut crane: F) -> String
where
    F: FnMut(Vec<u8>, &mut Vec<u8>)
{
    let mut stacks = input.stacks.clone();

    for cmd in &input.commands {
        let from = &mut stacks[cmd.from - 1];
        crane(
            from.drain(from.len() - cmd.cnt..).collect(),
            &mut stacks[cmd.to - 1],
        );
    }

    stacks.iter().map(
        |stack| *stack.last().unwrap() as char
    ).collect()
}

fn part_one(input: &Input) -> String {
    manipulate(
        input,
        |pile, to| {
            for &c in pile.iter().rev() {
                to.push(c);
            }
        })
}

fn part_two(input: &Input) -> String {
    manipulate(
        input,
        |pile, to| to.extend(pile))
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
