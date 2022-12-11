use std::io;

enum Instruction {
    Addx(i64),
    Nop
}

fn read_input() -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in io::stdin().lines() {
        let line = line.expect("failed to read line");
        let tokens: Vec<_> = line.split(' ').collect();
        match tokens.as_slice() {
            ["noop"] => {
                instructions.push(Instruction::Nop);
            },
            ["addx", x] => {
                let arg = x.parse().unwrap();
                instructions.push(Instruction::Addx(arg));
            },
            _ => panic!("invalid instruction: {}", line),
        }
    }
    instructions
}

fn compute_x_state(instructions: &[Instruction]) -> Vec<i64> {
    let mut x = 1;
    let mut x_state = vec![x];
    for instruction in instructions {
        match instruction {
            Instruction::Addx(arg) => {
                x_state.push(x);
                x += arg;
                x_state.push(x);
            },
            Instruction::Nop => {
                x_state.push(x);
            },
        }
    }
    x_state
}

fn part_one(instructions: &[Instruction]) -> i64 {
    let x_state = compute_x_state(instructions);
    let points = vec![20, 60, 100, 140, 180, 220];
    points.iter()
        .map(|&p| x_state[p - 1] * p as i64)
        .sum()
}

fn part_two(instructions: &[Instruction]) -> String {
    const ROW_WIDTH: usize = 40;
    let mut lines = Vec::new();

    for (i, x) in compute_x_state(instructions).iter()
        .enumerate()
    {
        if i % ROW_WIDTH == 0 {
            lines.push(String::new());
        }
        let pos = (i % ROW_WIDTH) as i64;
        lines.last_mut().unwrap().push(
            if x - 1 <= pos && x + 1 >= pos { '#' } else { '.' }
        );
    }

    lines.join("\n")
}

fn main() {
    let instructions = read_input();
    println!("part one: {}", part_one(&instructions));
    println!("part two:\n{}", part_two(&instructions));
}