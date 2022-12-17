use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn top_k<T>(xs: &[T], k: usize) -> Vec<&T>
where
    T: Ord,
{
    let mut heap = BinaryHeap::from_iter(xs.iter().take(k).map(Reverse));
    for x in xs.iter().skip(k) {
        heap.push(Reverse(x));
        if heap.len() > k {
            heap.pop();
        }
    }
    heap.into_iter().map(|Reverse(x)| x).collect()
}

fn read_input() -> Vec<u32> {
    let mut elves = Vec::new();
    let mut elf = 0;

    for line in io::stdin().lines() {
        let line = line.expect("input error");
        if line.is_empty() {
            elves.push(elf);
            elf = 0;
        } else {
            elf += line
                .parse::<u32>()
                .expect("invalid input, expected an integer");
        }
    }

    if elf > 0 {
        elves.push(elf);
    }

    elves
}

fn part_one(elves: &[u32]) -> u32 {
    *elves.iter().max().expect("empty input")
}

fn part_two(elves: &[u32]) -> u32 {
    top_k(elves, 3).iter().copied().sum()
}

fn main() {
    let elves = read_input();
    println!("{}", part_one(&elves));
    println!("{}", part_two(&elves));
}
