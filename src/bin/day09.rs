use std::collections::HashSet;
use std::io;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point(i32, i32);

struct Move(Point, u32);

fn read_input() -> Vec<Move> {
    let mut moves = Vec::new();
    for line in io::stdin().lines() {
        let line = line.expect("Failed to read line");
        if let Some((direction, steps)) = line.split_once(' ') {
            let direction = match direction {
                "U" => Point(-1,  0),
                "D" => Point( 1,  0),
                "L" => Point( 0, -1),
                "R" => Point( 0,  1),
                _   => panic!("Invalid direction: {}", direction),
            };
            let steps = steps.parse().unwrap();
            moves.push(Move(direction, steps));
        } else {
            panic!("invalid input: {line}");
        }
    }
    moves
}

fn part_one(moves: &[Move]) -> usize {
    let mut visited = HashSet::new();
    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);

    for Move(direction, steps) in moves {
        for _ in 0..*steps {
            head.0 += direction.0;
            head.1 += direction.1;
            let detached = (head.0 - tail.0).abs() > 1 ||
                (head.1 - tail.1).abs() > 1;
            if detached {
                tail.0 += (head.0 - tail.0).signum();
                tail.1 += (head.1 - tail.1).signum();
            }
            visited.insert(tail);
        }
    }

    visited.len()
}

fn part_two(moves: &[Move]) -> usize {
    let mut visited = HashSet::new();

    let mut rope = [Point(0, 0); 10];

    for Move(direction, steps) in moves {
        for _ in 0..*steps {
            rope[0].0 += direction.0;
            rope[0].1 += direction.1;

            for i in 1..rope.len() {
                let detached = (rope[i].0 - rope[i - 1].0).abs() > 1 ||
                    (rope[i].1 - rope[i - 1].1).abs() > 1;
                if detached {
                    rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                    rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn main() {
    let moves = read_input();
    println!("{}", part_one(&moves));
    println!("{}", part_two(&moves));
}