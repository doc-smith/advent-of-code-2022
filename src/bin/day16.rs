use regex::Regex;
use std::io;

struct Input {
    graph: Vec<Vec<u32>>,
    flow: Vec<u32>,
}

struct Valve {
    name: String,
    flow: u32,
    neighbors: Vec<String>,
}

fn parse_valve(line: &str) -> Valve {
    let re =
        Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(?:, [A-Z]{2})*)")
            .unwrap();
    let caps = re.captures(&line).expect("cannot parse input");
    let get_match = |i| caps.get(i).unwrap().as_str();

    let name = get_match(1).to_string();
    let flow: u32 = get_match(2).parse().expect("cannot parse flow rate");
    let neighbors: Vec<_> = get_match(3).split(", ").map(|s| s.to_string()).collect();

    Valve {
        name,
        flow,
        neighbors,
    }
}

fn read_valves() -> Vec<Valve> {
    let mut valves: Vec<_> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.expect("cannot read input");
            parse_valve(&line)
        })
        .collect();

    let start_pos = valves
        .iter()
        .position(|v| v.name == "AA")
        .expect("there is no start valve (AA)");
    valves.swap(0, start_pos);

    valves
}

fn floyd(mut distance: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let n = distance.len();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                distance[j][k] = distance[j][k].min(distance[j][i].saturating_add(distance[i][k]));
            }
        }
    }
    distance
}

fn construct_distance_matrix(valves: &[Valve]) -> Vec<Vec<u32>> {
    let n = valves.len();
    let mut distance = vec![vec![u32::MAX; n]; n];
    for (i, v) in valves.iter().enumerate() {
        distance[i][i] = 0;
        for u in &v.neighbors {
            let j = valves.iter().position(|v| v.name == *u).unwrap();
            distance[i][j] = 1;
            distance[j][i] = 1;
        }
    }
    distance
}

fn read_input() -> Input {
    let valves = read_valves();
    let distance = floyd(construct_distance_matrix(&valves));

    let working_valves: Vec<_> = valves
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v.name == "AA" || v.flow > 0 {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let n = working_valves.len();
    let mut compressed = Vec::with_capacity(n);
    for &i in &working_valves {
        let mut row = Vec::with_capacity(n);
        for &j in &working_valves {
            row.push(distance[i][j]);
        }
        compressed.push(row);
    }

    let flow: Vec<_> = working_valves.iter().map(|&i| valves[i].flow).collect();

    Input {
        graph: compressed,
        flow,
    }
}

fn walk(
    dist: &[Vec<u32>],
    flow: &[u32],
    v: usize,
    time_remaining: u32,
    visited: u16,
    visited_max_released: &mut [u32],
    released: u32,
) {
    visited_max_released[visited as usize] = visited_max_released[visited as usize].max(released);
    for (i, &f) in flow.iter().enumerate() {
        let cost = dist[v][i] + 1;
        if visited & (1 << i) == 0 && time_remaining >= cost && f > 0 {
            walk(
                dist,
                flow,
                i,
                time_remaining - cost,
                visited | (1 << i),
                visited_max_released,
                released + (time_remaining - cost) * f,
            );
        }
    }
}

fn compute_visited_max_released(dist: &[Vec<u32>], flow: &[u32], time: u32) -> Vec<u32> {
    let mut visited_max_released = vec![0; (u16::MAX as usize) + 1];
    walk(dist, flow, 0, time, 0, &mut visited_max_released, 0);
    visited_max_released
}

fn part_one(inp: &Input) -> u32 {
    let visited_max_released = compute_visited_max_released(&inp.graph, &inp.flow, 30);
    visited_max_released.iter().max().cloned().unwrap()
}

fn part_two(inp: &Input) -> u32 {
    let visited_max_released = compute_visited_max_released(&inp.graph, &inp.flow, 26);
    visited_max_released.iter().enumerate().flat_map(|(i, &you)| {
        visited_max_released.iter().enumerate().filter_map(move |(j, &elephant)| {
            if i & j == 0 {
                Some(you + elephant)
            } else {
                None
            }
        })
    }).max().unwrap()
}

fn main() {
    let input = read_input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
