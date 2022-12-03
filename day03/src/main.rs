struct Backpack {
    items: Vec<u8>,
}

impl Backpack {
    fn items(&self) -> &[u8] {
        &self.items
    }

    fn compartments(&self) -> (&[u8], &[u8]) {
        self.items.split_at(self.items.len() / 2)
    }
}

fn read_backpacks() -> Vec<Backpack> {
    let mut backpacks = Vec::new();
    for line in std::io::stdin().lines() {
        let items = line
            .expect("failed to read line")
            .into_bytes();
        items.iter().for_each(|&item| {
            assert!(item.is_ascii_alphabetic(), "items must be ascii alphabetic");
        });
        backpacks.push(Backpack { items });
    }
    backpacks
}

fn score_item(item: u8) -> Option<u8> {
    match item {
        b'a'..=b'z' => Some(item - b'a' + 1),
        b'A'..=b'Z' => Some(item - b'A' + 27),
        _ => None
    }
}

fn find_first_common(first: &[u8], rest: &[&[u8]]) -> Option<u8> {
    first.iter().find(|&&item| {
        rest.iter().all(|other| {
            other.contains(&item)
        })
    }).copied()
}

fn part_one(backpacks: &[Backpack]) -> u32 {
    backpacks
        .iter()
        .map(|b| {
            let (xs, ys) = b.compartments();
            if let Some(x) = find_first_common(xs, &[ys]) {
                score_item(x).unwrap() as u32
            } else {
                panic!("invalid backpack");
            }
        })
        .sum()
}

fn part_two(backpacks: &[Backpack]) -> u32 {
    backpacks.chunks_exact(3)
        .map(|group|
            if let Some(x) = find_first_common(
                group[0].items(),
                &[group[1].items(), group[2].items()]
            ) {
                score_item(x).unwrap() as u32
            } else {
                panic!("invalid backpack");
            }
        )
        .sum()
}


fn main() {
    let backpacks = read_backpacks();
    println!("{}", part_one(&backpacks));
    println!("{}", part_two(&backpacks));
}