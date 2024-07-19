const SIZE: usize = 131;

pub fn solve() -> usize {
    let (start, mut map) = parse();
    //debug_print(&map);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    for i in 0..64 {
        let mut walker = vec![];
        while let Some(pos) = queue.pop_front() {
            map[pos.y][pos.x] = FloorKind::Plot;
            for c in candidate(&pos, &map) {
                map[c.y][c.x] = FloorKind::Occupied;
                walker.push(c);
            }
        }
        //debug_print(&map);
        queue.extend(walker);
    }
    map.iter()
        .map(|v| v.iter().filter(|f| **f == FloorKind::Occupied).count())
        .sum()
}

fn debug_print(map: &[[FloorKind; SIZE]; SIZE]) {
    println!("--- display ----");
    for (y, r) in map.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            match c {
                FloorKind::Rock => print!("#"),
                FloorKind::Plot => print!("."),
                FloorKind::Occupied => print!("O"),
            }
        }
        println!();
    }
    println!();
    println!();
}

fn candidate(pos: &Coord, map: &[[FloorKind; SIZE]; SIZE]) -> Vec<Coord> {
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

#[derive(Clone, Copy, PartialEq, Eq)]
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

use std::collections::VecDeque;

use super::utility;
fn parse() -> (Coord, [[FloorKind; SIZE]; SIZE]) {
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
