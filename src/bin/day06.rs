use std::io;

fn find_unique_subsequence(s: &[u8], len: usize) -> Option<usize> {
    let mut start: usize = 0;
    let mut counts = [0; std::u8::MAX as usize + 1];

    for (end, &b) in s.iter().enumerate() {
        counts[b as usize] += 1;

        while counts[b as usize] > 1 {
            counts[s[start] as usize] -= 1;
            start += 1;
        }

        if end - start + 1 == len {
            return Some(start);
        }
    }

    None
}

fn solve(signal: &[u8], marker_len: usize) -> usize {
    find_unique_subsequence(signal, marker_len)
        .expect("the signal doesn't contain a long enough marker")
        + marker_len
}

fn part_one(signal: &[u8]) -> usize {
    solve(signal, 4)
}

fn part_two(signal: &[u8]) -> usize {
    solve(signal, 14)
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    println!("{}", part_one(input.as_bytes()));
    println!("{}", part_two(input.as_bytes()));
}

