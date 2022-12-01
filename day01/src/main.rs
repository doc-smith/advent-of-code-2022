use std::cmp::Reverse;
use std::collections;
use std::io;

fn read_supplies() -> Vec<u64> {
    let mut supplies = Vec::new();

    let mut elf = 0;

    for line in io::stdin().lines() {
        let l = String::from(line.expect("Failed to read line").trim());

        if l.is_empty() {
            supplies.push(elf);
            elf = 0;
        } else {
            let calories: u64 = l.parse().expect("Failed to parse integer");
            elf += calories;
        }
    }

    if elf > 0 {
        supplies.push(elf);
    }

    supplies
}

fn part_one(supplies: &[u64]) -> u64 {
    *supplies.iter().max().unwrap()
}

fn top_k<T: Ord + Copy>(xs: &[T], k: usize) -> Vec<T> {
    let mut heap = collections::BinaryHeap::from_iter(
        xs.iter().take(k).map(|&x| Reverse(x)),
    );
    for &x in xs {
        heap.push(Reverse(x));
        if heap.len() > k {
            heap.pop();
        }
    }
    heap.drain().map(|Reverse(x)| x).collect()
}

fn part_two(supplies: &[u64]) -> u64 {
    top_k(&supplies, 3).iter().sum()
}

fn main() {
    let supplies = read_supplies();
    println!("{}", part_one(&supplies));
    println!("{}", part_two(&supplies));
}
