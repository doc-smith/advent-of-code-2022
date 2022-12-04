use std::str::FromStr;

struct Range {
    start: u32,
    end: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRangeError;

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once('-').ok_or(ParseRangeError)?;
        if let (Ok(l), Ok(r)) = (l.parse(), r.parse()) {
            Ok(Range { start: l, end: r })
        } else {
            Err(ParseRangeError)
        }
    }
}

impl Range {
    fn contains(&self, rhs: &Self) -> bool {
        self.start <= rhs.start && self.end >= rhs.end
    }

    fn overlaps(&self, rhs: &Self) -> bool {
        self.start <= rhs.end && self.end >= rhs.start
    }
}

fn read_input() -> Vec<(Range, Range)> {
    let mut pairs = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.expect("failed to read line");
        if let Some((l, r)) = line.split_once(',') {
            pairs.push((
                l.parse().unwrap_or_else(|_| panic!("failed to parse range: {}", l)),
                r.parse().unwrap_or_else(|_| panic!("failed to parse range: {}", r)),
            ));
        } else {
            panic!("invalid input line, cannot be split: {}", line);
        }
    }

    pairs
}

fn part_one(input: &[(Range, Range)]) -> usize {
    input.iter()
        .filter(|(l, r)| l.contains(r) || r.contains(l))
        .count()
}

fn part_two(input: &[(Range, Range)]) -> usize {
    input.iter()
        .filter(|(l, r)| l.overlaps(r))
        .count()
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}