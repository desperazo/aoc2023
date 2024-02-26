use std::collections::HashMap;

use super::utility;

pub fn solve() -> usize {
    let mut data = parse();
    roll_north(&mut data);
    compute_load(&data)
}

fn compute_load(data: &[Vec<Tile>]) -> usize {
    data.iter()
        .enumerate()
        .map(|(y, r)| r.iter().filter(|x| **x == Tile::RoundRock).count() * (data.len() - y))
        .sum()
}

pub fn solve_2() -> usize {
    let mut data = parse();
    let max_loops = 1_000_000_000;
    let mut seen = HashMap::new();
    let mut loads = vec![];
    for count in 0..max_loops {
        roll_north(&mut data);
        roll_west(&mut data);
        roll_south(&mut data);
        roll_east(&mut data);

        if seen.get(&data).is_some() {
            break;
        }

        let val = compute_load(&data);
        loads.push(val);
        seen.entry(data.clone()).or_insert(count);
    }

    let start = seen.get(&data).unwrap();
    let cycle = seen.len() - start;
    let dist = (max_loops - seen.len() - 1) % cycle;
    loads[start + dist]
}

fn roll_north(data: &mut [Vec<Tile>]) {
    for y in 1..data.len() {
        for x in 0..data[0].len() {
            if data[y][x] != Tile::RoundRock {
                continue;
            }
            let mut pos = y;
            while pos > 0 && data[pos - 1][x] == Tile::Space {
                data[pos - 1][x] = Tile::RoundRock;
                data[pos][x] = Tile::Space;
                pos -= 1;
            }
        }
    }
}

fn roll_west(data: &mut [Vec<Tile>]) {
    for y in 0..data.len() {
        for x in 1..data[0].len() {
            if data[y][x] != Tile::RoundRock {
                continue;
            }
            let mut pos = x;
            while pos > 0 && data[y][pos - 1] == Tile::Space {
                data[y][pos - 1] = Tile::RoundRock;
                data[y][pos] = Tile::Space;
                pos -= 1;
            }
        }
    }
}

fn roll_south(data: &mut [Vec<Tile>]) {
    for y in (0..data.len() - 1).rev() {
        for x in 0..data[0].len() {
            if data[y][x] != Tile::RoundRock {
                continue;
            }
            let mut pos = y;
            while pos < data.len() - 1 && data[pos + 1][x] == Tile::Space {
                data[pos + 1][x] = Tile::RoundRock;
                data[pos][x] = Tile::Space;
                pos += 1;
            }
        }
    }
}

fn roll_east(data: &mut [Vec<Tile>]) {
    for y in 0..data.len() {
        for x in (0..data[0].len() - 1).rev() {
            if data[y][x] != Tile::RoundRock {
                continue;
            }
            let mut pos = x;
            while pos < data[0].len() - 1 && data[y][pos + 1] == Tile::Space {
                data[y][pos + 1] = Tile::RoundRock;
                data[y][pos] = Tile::Space;
                pos += 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    RoundRock,
    CubeRock,
    Space,
}

fn parse() -> Vec<Vec<Tile>> {
    utility::read("./src/input/day14.txt")
        .iter()
        .map(|x| {
            x.chars()
                .filter(|v| *v != '\r' && *v != '\n')
                .map(|v| match v {
                    '#' => Tile::CubeRock,
                    'O' => Tile::RoundRock,
                    '.' => Tile::Space,
                    _ => panic!("value `{v}` not match"),
                })
                .collect()
        })
        .collect()
}
