#[derive(PartialEq)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl Sign {
    fn beats(&self) -> Sign {
        match self {
            Sign::Rock => Sign::Scissors,
            Sign::Paper => Sign::Rock,
            Sign::Scissors => Sign::Paper,
        }
    }

    fn wins(&self, other: &Sign) -> bool {
        self.beats() == *other
    }

    fn loses_to(&self) -> Sign {
        self.beats().beats()
    }

    fn cost(&self) -> u32 {
        match self {
            Sign::Rock => 1,
            Sign::Paper => 2,
            Sign::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn cost(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

struct Round {
    opponent: Sign,
    you: Sign,
}

fn decode_sign(c: char) -> Sign {
    match c {
        'A' | 'X' => Sign::Rock,
        'B' | 'Y' => Sign::Paper,
        'C' | 'Z' => Sign::Scissors,
        _ => panic!("invalid sign {}", c),
    }
}

fn read_rounds() -> Vec<Round> {
    let mut rounds = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line
            .expect("Failed to read line");
        match line.trim().as_bytes() {
            [ opponent, b' ', you ] => {
                let round = Round {
                    opponent: decode_sign(*opponent as char),
                    you: decode_sign(*you as char),
                };
                rounds.push(round);
            },
            _ => panic!("invalid input format on line: {}", line),
        }
    }
    rounds
}

fn score_round(you: &Sign, opponent: &Sign) -> u32 {
    let outcome = if you.wins(opponent) {
        Outcome::Win
    } else if opponent.wins(you) {
        Outcome::Loss
    } else {
        Outcome::Draw
    };
    outcome.cost() + you.cost()
}

fn part_one(rounds: &[Round]) -> u32 {
    rounds.iter().map(|round| {
        score_round(&round.you, &round.opponent)
    }).sum()
}

fn part_two(rounds: &[Round]) -> u32 {
    rounds.iter().map(|round| {
        let you = match round.opponent {
            Sign::Rock => round.opponent.loses_to(),
            Sign::Paper => Sign::Paper,
            Sign::Scissors => round.opponent.beats(),
        };
        score_round(&you, &round.opponent)
    }).sum()
}

fn main() {
    let rounds = read_rounds();
    println!("{}", part_one(&rounds));
    println!("{}", part_two(&rounds));
}
