pub fn solve() -> usize {
    let bucket = new_bucket();
    bucket.disintegrate_count()
}

pub fn solve_2() -> usize {
    let bucket = new_bucket();
    bucket.chain_count()
}

fn new_bucket() -> Bucket {
    let mut input = parse();
    input.sort_by(|a, b| a.pos1.z.cmp(&b.pos1.z));
    let mut bucket = Bucket {
        bricks: Vec::new(),
        supported_stat: HashMap::new(),
    };
    for b in input.iter_mut() {
        bucket.falling_down(b.clone());
    }
    bucket
}

struct Bucket {
    bricks: Vec<Brick>,
    supported_stat: HashMap<usize, usize>,
}

impl Bucket {
    fn chain_count(mut self) -> usize {
        self.bricks.sort_by(|a, b| a.id.cmp(&b.id));
        let mut queue = VecDeque::new();
        for b in self.bricks.iter() {
            if b.supporting.is_empty() {
                continue;
            }
            if b.supporting
                .iter()
                .any(|s| self.bricks.get(*s - 1).unwrap().supported_count == 1)
            {
                queue.push_back(b.id);
            }
        }
        let mut count = 0;
        while let Some(q) = queue.pop_front() {
            let mut supported_count = HashMap::new();
            let mut chain = VecDeque::new();
            chain.push_front(q);
            while let Some(f) = chain.pop_front() {
                let cur = self.bricks.get(f - 1).unwrap();
                for chd in cur.supporting.iter() {
                    let ent = supported_count
                        .entry(chd)
                        .and_modify(|e| *e -= 1)
                        .or_insert(self.bricks.get(*chd - 1).unwrap().supported_count - 1);
                    if ent == &0 {
                        chain.push_back(*chd);
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn disintegrate_count(self) -> usize {
        let mut count = 0;
        for b in self.bricks.iter().rev() {
            if b.supporting.is_empty() {
                count += 1;
            } else {
                if b.supporting
                    .iter()
                    .all(|v| *self.supported_stat.get(v).unwrap_or(&0) > 1)
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn falling_down(&mut self, mut brick: Brick) {
        loop {
            let mut stop = false;
            for i in (0..self.bricks.len()).rev() {
                let b = self.bricks.get_mut(i).unwrap();
                if brick.collis(b) {
                    brick.supported_count += 1;
                    b.supporting.push(brick.id);
                    self.supported_stat
                        .entry(brick.id)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                    stop = true;
                }
            }
            if stop || !brick.fall() {
                break;
            }
        }
        self.bricks.push(brick);
    }
}

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    pos1: Coord,
    pos2: Coord,
    supported_count: u32,
    supporting: Vec<usize>,
}

impl Default for Brick {
    fn default() -> Self {
        Brick {
            id: 0,
            pos1: Coord { x: 0, y: 0, z: 0 },
            pos2: Coord { x: 0, y: 0, z: 0 },
            supported_count: 0,
            supporting: Vec::new(),
        }
    }
}

impl Brick {
    fn new(id: usize, pos1: Coord, pos2: Coord) -> Self {
        Brick {
            id,
            pos1,
            pos2,
            supported_count: 0,
            supporting: Vec::new(),
        }
    }
}

impl PartialEq for Brick {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Brick {
    fn fall(&mut self) -> bool {
        if self.pos1.z == 1 {
            return false;
        }
        self.pos1.z -= 1;
        self.pos2.z -= 1;
        true
    }

    fn collis(&self, other: &Brick) -> bool {
        if self.pos1.z != other.pos2.z + 1 {
            return false;
        }
        let collapse_x = self.pos1.x <= other.pos2.x && self.pos2.x >= other.pos1.x;
        let collapse_y = self.pos1.y <= other.pos2.y && self.pos2.y >= other.pos1.y;
        collapse_x & collapse_y
    }
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

use std::{
    collections::{HashMap, VecDeque},
    env::var,
};

use super::utility;
fn parse() -> Vec<Brick> {
    utility::read("./src/input/day22.txt")
        .iter()
        .enumerate()
        .filter_map(|(i, f)| {
            let ch = f
                .strip_suffix("\r")
                .unwrap_or("")
                .split('~')
                .collect::<Vec<_>>();
            if ch.len() != 2 {
                return None;
            }
            let left = ch[0].split(',').collect::<Vec<_>>();
            let right = ch[1].split(',').collect::<Vec<_>>();
            let brick = Brick {
                id: i + 1,
                pos1: Coord {
                    x: left[0].to_string().parse().unwrap(),
                    y: left[1].to_string().parse().unwrap(),
                    z: left[2].to_string().parse().unwrap(),
                },
                pos2: Coord {
                    x: right[0].to_string().parse().unwrap(),
                    y: right[1].to_string().parse().unwrap(),
                    z: right[2].to_string().parse().unwrap(),
                },
                ..Default::default()
            };
            Some(brick)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brick_coliss() {
        let a = Brick {
            pos1: Coord { x: 1, y: 0, z: 1 },
            pos2: Coord { x: 1, y: 2, z: 1 },
            supported_count: 0,
            ..Default::default()
        };
        let b = Brick {
            pos1: Coord { x: 0, y: 0, z: 2 },
            pos2: Coord { x: 2, y: 0, z: 2 },
            supported_count: 0,
            ..Default::default()
        };
        assert!(b.collis(&a));
    }
}
