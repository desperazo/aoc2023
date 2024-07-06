use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

use super::utility;

pub fn solve() -> u32 {
    let grid = parse();
    find_best_path(
        [
            Player {
                x: 1,
                y: 0,
                heat_lost: grid[0][1],
                dir: Direction::Right,
                consucive_count: 1,
            },
            Player {
                x: 0,
                y: 1,
                heat_lost: grid[1][0],
                dir: Direction::Down,
                consucive_count: 1,
            },
        ]
        .to_vec(),
        &grid,
        &|p| p.walk(&grid),
    )
}

pub fn solve_2() -> u32 {
    let grid = parse();
    find_best_path(
        [Player {
            x: 1,
            y: 0,
            heat_lost: grid[0][1],
            dir: Direction::Right,
            consucive_count: 1,
        }]
        .to_vec(),
        &grid,
        &|p| p.ultra_walk(&grid),
    )
}

fn find_best_path(
    players: Vec<Player>,
    grid: &[Vec<u32>],
    func: &dyn Fn(Player) -> Vec<Player>,
) -> u32 {
    let mut queue = BinaryHeap::from_iter(players);
    let mut visit = HashSet::new();
    while let Some(player) = queue.pop() {
        for can in func(player).iter() {
            if can.x == grid[0].len() - 1 && can.y == grid.len() - 1 {
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Player {
    x: usize,
    y: usize,
    heat_lost: u32,
    dir: Direction,
    consucive_count: usize,
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
    fn ultra_walk(self, grid: &[Vec<u32>]) -> Vec<Self> {
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
            self.dir == d && self.consucive_count <= 9 || self.dir != d && self.consucive_count >= 4
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

            if self.dir != d {
                if d == Direction::Right && new_x + 3 >= grid[0].len() {
                    return None;
                }
                if d == Direction::Left && new_x < 3 {
                    return None;
                }
                if d == Direction::Down && new_y + 3 >= grid.len() {
                    return None;
                }
                if d == Direction::Up && new_y < 3 {
                    return None;
                }
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
            })
        })
        .collect()
    }

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
        .map(|&d| {
            let (x, y) = d.offset();
            let new_x = (self.x as isize + x) as usize;
            let new_y = (self.y as isize + y) as usize;
            Player {
                x: new_x,
                y: new_y,
                heat_lost: grid[new_y][new_x] + self.heat_lost,
                dir: d,
                consucive_count: if self.dir == d {
                    self.consucive_count + 1
                } else {
                    1
                },
            }
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
