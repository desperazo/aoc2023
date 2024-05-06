use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

use super::utility;

pub fn solve() -> u32 {
    let grid = parse();
    let mut queue = BinaryHeap::from_iter([
        Player {
            x: 1,
            y: 0,
            heat_lost: grid[0][1],
            dir: Direction::Right,
            consucive_count: 1,
            history: HashSet::from_iter([(1, 0)]),
        },
        Player {
            x: 0,
            y: 1,
            heat_lost: grid[1][0],
            dir: Direction::Down,
            consucive_count: 1,
            history: HashSet::from_iter([(0, 1)]),
        },
    ]);
    let mut visit = HashSet::new();
    while let Some(player) = queue.pop() {
        for can in player.walk(&grid).iter() {
            if can.x == grid[0].len() - 1 && can.y == grid.len() - 1 {
                println!("score {}", can.heat_lost);
                return can.heat_lost;
            }
            if !visit.insert((can.x, can.y, can.dir, can.consucive_count)) {
                continue;
            }
            queue.push(can.clone());
        }
    }
    panic!("path is not found")
}

fn parse() -> Vec<Vec<u32>> {
    utility::read("./src/input/day17.txt")
        .iter()
        .map(|x| {
            x.trim_end()
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn debug_print(player: &Player, grid: &Vec<Vec<u32>>) {
    println!("score {}", player.heat_lost);
    let mut tmp = grid.clone();
    player
        .history
        .iter()
        .for_each(|(x, y)| tmp[*y][*x] = u32::MAX);
    for l in tmp.iter() {
        for v in l.iter() {
            if *v == u32::MAX {
                print!(".");
            } else {
                print!("{}", v);
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Player {
    x: usize,
    y: usize,
    heat_lost: u32,
    dir: Direction,
    consucive_count: usize,
    history: HashSet<(usize, usize)>,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_lost.cmp(&self.heat_lost)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Player {
    fn walk(self, grid: &[Vec<u32>]) -> Vec<Self> {
        [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ]
        .iter()
        .filter(|&&d| {
            !(self.dir == Direction::Right && d == Direction::Left
                || self.dir == Direction::Up && d == Direction::Down
                || self.dir == Direction::Left && d == Direction::Right
                || self.dir == Direction::Down && d == Direction::Up)
        })
        .filter(|&&d| {
            !(self.x == 0 && d == Direction::Left
                || self.y == 0 && d == Direction::Up
                || self.x + 1 == grid[0].len() && d == Direction::Right
                || self.y + 1 == grid.len() && d == Direction::Down)
        })
        .filter_map(|&d| {
            let (x, y) = d.offset();
            let new_x = (self.x as isize + x) as usize;
            let new_y = (self.y as isize + y) as usize;
            let mut history = HashSet::from_iter(self.history.clone());
            if !history.insert((new_x, new_y)) {
                return None;
            }
            Some(Player {
                x: new_x,
                y: new_y,
                heat_lost: grid[new_y][new_x] + self.heat_lost,
                dir: d,
                consucive_count: if self.dir == d {
                    self.consucive_count + 1
                } else {
                    1
                },
                history,
            })
        })
        .filter(|p| p.consucive_count <= 3)
        .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}
