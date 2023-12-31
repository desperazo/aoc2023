use std::{
    collections::VecDeque,
    ops::{Add, AddAssign},
};

use super::utility;

pub fn solve() -> u64 {
    let (x, y, mut data) = parse();
    let mut queue = VecDeque::from([(x, y)]);
    let mut max = 0;
    while let Some((x, y)) = queue.pop_front() {
        let mut curr = data[y][x];
        if let Some((nx, ny)) = curr.neighbor(&mut data) {
            if data[ny][nx].block == Block::ST {
                max = max.max(data[ny][nx].distance / 2);
            }
            queue.push_front((nx, ny));
        }
    }
    max
}

fn print(data: &[Vec<Node>]) {
    for l in data.iter() {
        for v in l.iter() {
            if v.distance > 0 {
                print!("{}", v.distance);
            } else {
                print!(
                    "{}",
                    match v.block {
                        Block::VE => "|",
                        Block::HE => "-",
                        Block::NE => "L",
                        Block::NW => "J",
                        Block::SE => "F",
                        Block::SW => "7",
                        Block::GR => ".",
                        Block::ST => "S",
                    }
                );
            }
        }
        println!();
    }
    println!();
    println!();
}

#[derive(Clone, Copy)]
struct Node {
    block: Block,
    distance: u64,
    x: usize,
    y: usize,
    edges: [bool; 4],
}

impl Node {
    fn new(x: usize, y: usize, block: Block) -> Self {
        Node {
            block,
            distance: 0,
            edges: match block {
                Block::VE => [true, false, true, false],
                Block::HE => [false, true, false, true],
                Block::NE => [true, true, false, false],
                Block::NW => [true, false, false, true],
                Block::SE => [false, true, true, false],
                Block::SW => [false, false, true, true],
                Block::GR => [false, false, false, false],
                Block::ST => [true, true, true, true],
            },
            x,
            y,
        }
    }

    fn neighbor(&mut self, maps: &mut [Vec<Node>]) -> Option<(usize, usize)> {
        for (i, (xa, ya)) in [(0, -1), (1, 0), (0, 1), (-1, 0)].into_iter().enumerate() {
            if !self.edges[i]
                || self.y == 0 && ya == -1
                || self.y == maps.len() - 1 && ya == 1
                || self.x == 0 && xa == -1
                || self.x == maps[0].len() - 1 && xa == 1
            {
                continue;
            }
            let x = self.x.wrapping_add_signed(xa);
            let y = self.y.wrapping_add_signed(ya);
            let nb_edges = (i + 2) % 4;
            if !maps[y][x].edges[nb_edges] {
                continue;
            }
            self.edges[i] = false;
            maps[y][x].edges[nb_edges] = false;
            maps[y][x].distance = self.distance + 1;
            return Some((x, y));
        }
        None
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Block {
    VE,
    HE,
    NE,
    NW,
    SE,
    SW,
    GR,
    ST,
}

fn parse() -> (usize, usize, Vec<Vec<Node>>) {
    let raw = utility::read("./src/input/day10.txt");
    let (mut sx, mut sy) = (0, 0);
    let input = raw
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.replace(['\r', '\n'], "")
                .chars()
                .enumerate()
                .map(|(x, s)| {
                    if s == 'S' {
                        sx = x;
                        sy = y;
                    }
                    match s {
                        '|' => Node::new(x, y, Block::VE),
                        '-' => Node::new(x, y, Block::HE),
                        'L' => Node::new(x, y, Block::NE),
                        'J' => Node::new(x, y, Block::NW),
                        '7' => Node::new(x, y, Block::SW),
                        'F' => Node::new(x, y, Block::SE),
                        '.' => Node::new(x, y, Block::GR),
                        'S' => Node::new(x, y, Block::ST),
                        _ => panic!("`{}` invalid charactor", s),
                    }
                })
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();
    (sx, sy, input)
}
