use std::collections::{HashSet, VecDeque};
const SIZE: usize = 141;
type Map = [[Tile; SIZE]; SIZE];
type Pos = (usize, usize);

pub fn solve() -> usize {
    let goal = (SIZE - 2, SIZE - 1);
    let map = parse();
    let mut queue = VecDeque::new();
    queue.push_back(Walker {
        pos: (1, 0),
        seen: HashSet::from([(1, 0)]),
        steps: 0,
    });
    let mut max_steps = 0;
    while let Some(mut walker) = queue.pop_front() {
        if walker.pos == goal {
            println!("GOAL: {}", walker.steps);
            max_steps = max_steps.max(walker.steps);
            continue;
        }
        // print_running(&map, &walker.seen);
        queue.extend(walker.next(&map));
    }
    max_steps
}

fn print_running(map: &Map, running: &HashSet<Pos>) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            if running.contains(&(x, y)) {
                print!("O");
            } else {
                match map[y][x] {
                    Tile::Path => print!("."),
                    Tile::Forest => print!("#"),
                    Tile::Slope(Direction::Up) => print!("^"),
                    Tile::Slope(Direction::Down) => print!("v"),
                    Tile::Slope(Direction::Left) => print!("<"),
                    Tile::Slope(Direction::Right) => print!(">"),
                }
            }
        }
        println!();
    }
    println!();
    println!();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Walker {
    pos: Pos,
    seen: HashSet<Pos>,
    steps: usize,
}

impl Walker {
    fn next(&mut self, map: &Map) -> Vec<Self> {
        let mut next = Vec::new();
        let (x, y) = self.pos;
        let dir = vec![0, 1, 2, 3]
            .iter()
            .filter_map(|&i| match i {
                0 if y > 0 && map[y - 1][x] != Tile::Forest => Some((x, y - 1, Direction::Up)),
                1 if y < SIZE - 1 && map[y + 1][x] != Tile::Forest => {
                    Some((x, y + 1, Direction::Down))
                }
                2 if x > 0 && map[y][x - 1] != Tile::Forest => Some((x - 1, y, Direction::Left)),
                3 if x < SIZE - 1 && map[y][x + 1] != Tile::Forest => {
                    Some((x + 1, y, Direction::Right))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        for (nx, ny, nd) in dir {
            if !self.seen.insert((nx, ny)) {
                continue;
            }
            match map[y][x] {
                Tile::Slope(d) if d == nd => next.push(Self {
                    pos: (nx, ny),
                    seen: self.seen.clone(),
                    steps: self.steps + 1,
                }),
                Tile::Slope(_) => continue,
                _ => next.push(Self {
                    pos: (nx, ny),
                    seen: self.seen.clone(),
                    steps: self.steps + 1,
                }),
            }
        }
        next
    }

    fn new(pos: Pos) -> Self {
        Self {
            pos,
            seen: HashSet::new(),
            steps: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,

    Forest,

    Slope(Direction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,

    Down,

    Left,

    Right,
}

use super::utility;
fn parse() -> Map {
    let mut map = [[Tile::Path; SIZE]; SIZE];
    let input = utility::read("./src/input/day23.txt");
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[y][x] = match c {
                '.' => Tile::Path,
                '#' => Tile::Forest,
                d => Tile::Slope(match d {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => unreachable!(),
                }),
            }
        }
    }
    map
}
