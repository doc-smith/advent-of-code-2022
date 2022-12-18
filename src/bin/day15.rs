use regex::Regex;
use std::io;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: &Self) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }
}

#[derive(Clone)]
struct Interval {
    start: i32,
    end: i32,
}

struct Sensor {
    location: Point,
    nearest_beacon: Point,
}

impl Sensor {
    fn min_range(&self) -> u32 {
        self.location.manhattan_distance(&self.nearest_beacon)
    }
}

fn parse_sensor_output(s: &str) -> Sensor {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let caps = re.captures(s).expect("cannot parse sensor output");

    let extract = |i| {
        let captured = caps.get(i).unwrap().as_str();
        captured.parse::<i32>().expect("cannot parse sensor output")
    };
    Sensor {
        location: Point {
            x: extract(1),
            y: extract(2),
        },
        nearest_beacon: Point {
            x: extract(3),
            y: extract(4),
        },
    }
}

fn read_input() -> Vec<Sensor> {
    io::stdin()
        .lines()
        .map(|line| line.expect("cannot read input"))
        .map(|line| parse_sensor_output(&line))
        .collect()
}

fn merge_intervals(intervals: &[Interval]) -> Vec<Interval> {
    let mut intervals = intervals.to_vec();
    intervals.sort_by_key(|i| i.start);

    let mut merged = vec![intervals[0].clone()];
    for int in intervals.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if int.start <= last.end {
            last.end = last.end.max(int.end);
        } else {
            merged.push(int.clone());
        }
    }
    merged
}

fn part_one(sensors: &[Sensor]) -> u32 {
    const ROW: i32 = 2000000;

    let mut intervals: Vec<_> = sensors
        .iter()
        .filter_map(|s| {
            let min_range = s.min_range();
            let y_distance = (s.location.y - ROW).abs() as u32;
            if y_distance > min_range {
                None
            } else {
                let x_range = (min_range - y_distance) as i32;
                Some(Interval {
                    start: s.location.x - x_range,
                    end: s.location.x + x_range,
                })
            }
        })
        .collect();

    let merged = merge_intervals(&intervals);
    merged.iter().map(|i| (i.end - i.start) as u32 + 1).sum()
}

fn main() {
    let sensors = read_input();
    println!("{}", part_one(&sensors));
}
