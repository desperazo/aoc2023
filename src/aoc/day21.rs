const SIZE: usize = 131;
type Area = [[FloorKind; SIZE]; SIZE];

pub fn solve() -> usize {
    let (start, mut map) = parse();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    for _i in 0..64 {
        let mut walker = vec![];
        while let Some(pos) = queue.pop_front() {
            map[pos.y][pos.x] = FloorKind::Plot;
            for c in candidate(&pos, &map) {
                map[c.y][c.x] = FloorKind::Occupied;
                walker.push(c);
            }
        }
        queue.extend(walker);
    }
    map.iter()
        .map(|v| v.iter().filter(|f| **f == FloorKind::Occupied).count())
        .sum()
}

pub fn solve_2() -> usize {
    let (start, map) = parse();
    let no_of_area = 26_501_365 / SIZE - 1;
    let edge: usize = SIZE - 1;
    let middle = SIZE / 2;
    let odd = walk(start, SIZE * 2, map.clone());
    let even = walk(start, SIZE * 2 + 1, map.clone());

    let full_distance = SIZE - 2;
    let corner_t = walk(Coord { x: middle, y: edge }, full_distance, map.clone());
    let corner_r = walk(Coord { x: 0, y: middle }, full_distance, map.clone());
    let corner_d = walk(Coord { x: middle, y: 0 }, full_distance, map.clone());
    let corner_l = walk(Coord { x: edge, y: middle }, full_distance, map.clone());

    let half_distance = SIZE / 2 - 2;
    let small_tr = walk(Coord { x: 0, y: edge }, half_distance, map.clone());
    let small_tl = walk(Coord { x: edge, y: edge }, half_distance, map.clone());
    let small_br = walk(Coord { x: 0, y: 0 }, half_distance, map.clone());
    let small_bl = walk(Coord { x: edge, y: 0 }, half_distance, map.clone());

    let quater_distance = SIZE * 3 / 2 - 2;
    let large_tr = walk(Coord { x: 0, y: edge }, quater_distance, map.clone());
    let large_tl = walk(Coord { x: edge, y: edge }, quater_distance, map.clone());
    let large_br = walk(Coord { x: 0, y: 0 }, quater_distance, map.clone());
    let large_bl = walk(Coord { x: edge, y: 0 }, quater_distance, map.clone());

    let no_of_odd = (no_of_area / 2 * 2 + 1).pow(2);
    let no_of_even = ((no_of_area + 1) / 2 * 2).pow(2);
    odd * no_of_odd
        + even * no_of_even
        + corner_t
        + corner_r
        + corner_d
        + corner_l
        + (no_of_area + 1) * (small_tr + small_tl + small_br + small_bl)
        + no_of_area * (large_tr + large_tl + large_br + large_bl)
}

fn walk(start: Coord, distance: usize, mut map: Area) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((pos, dis)) = queue.pop_front() {
        if dis > distance {
            continue;
        }
        map[pos.y][pos.x] = FloorKind::Plot;
        for c in candidate(&pos, &map) {
            map[c.y][c.x] = FloorKind::Occupied;
            queue.push_back((c, dis + 1));
        }
    }
    map.iter()
        .map(|v| v.iter().filter(|f| **f == FloorKind::Occupied).count())
        .sum()
}

fn candidate(pos: &Coord, map: &Area) -> Vec<Coord> {
    let mut new_pos = Vec::new();
    if pos.x > 0 && map[pos.y][pos.x - 1] == FloorKind::Plot {
        new_pos.push(Coord {
            x: pos.x - 1,
            y: pos.y,
        });
    }
    if pos.x < SIZE - 1 && map[pos.y][pos.x + 1] == FloorKind::Plot {
        new_pos.push(Coord {
            x: pos.x + 1,
            y: pos.y,
        });
    }
    if pos.y > 0 && map[pos.y - 1][pos.x] == FloorKind::Plot {
        new_pos.push(Coord {
            x: pos.x,
            y: pos.y - 1,
        });
    }
    if pos.y < SIZE - 1 && map[pos.y + 1][pos.x] == FloorKind::Plot {
        new_pos.push(Coord {
            x: pos.x,
            y: pos.y + 1,
        });
    }
    new_pos
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum FloorKind {
    Rock,
    Plot,
    Occupied,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

use std::{collections::VecDeque, fmt::Debug};

use super::utility;
fn parse() -> (Coord, Area) {
    let mut map = [[FloorKind::Plot; SIZE]; SIZE];
    let mut start = Coord { x: 0, y: 0 };
    for (y, line) in utility::read("./src/input/day21.txt").iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => map[y][x] = FloorKind::Rock,
                'S' => start = Coord { x, y },
                _ => (),
            }
        }
    }
    (start, map)
}
