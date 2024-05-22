use std::collections::{HashSet, VecDeque};

pub fn solve() -> usize {
    let mut world = World {
        min_x: i32::MAX,
        max_x: i32::MIN,
        min_y: i32::MAX,
        max_y: i32::MIN,
        x: 0,
        y: 0,
        cubes: Vec::new(),
    };
    let ins = parse();
    ins.iter().for_each(|i| world.dig(i));
    world.dig_inner();
    // world.draw();
    world.cubes.len()
}

struct World {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    x: i32,
    y: i32,
    cubes: Vec<Cube>,
}

impl World {
    fn draw(&mut self) {
        let mut test = HashSet::new();

        self.cubes.iter_mut().for_each(|c| {
            let pos = c.pos_rel(self.min_x, self.min_y);
            test.insert(pos);
        });

        for y in 0..=self.min_y.abs_diff(self.max_y) as usize {
            for x in 0..=self.min_x.abs_diff(self.max_x) as usize {
                if test.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
        println!();
    }

    fn dig_inner(&mut self) {
        let start = Cube::new(1, 1);
        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut visit: HashSet<(i32, i32)> =
            HashSet::from_iter(self.cubes.iter().map(|c| (c.x, c.y)));
        visit.insert((start.x, start.y));
        self.cubes.push(start);
        while let Some(cube) = queue.pop_front() {
            for n in cube.neighbor() {
                if !visit.insert((n.x, n.y)) {
                    continue;
                }
                self.cubes.push(n);
                queue.push_back(n);
            }
        }
    }

    fn dig(&mut self, ins: &Instruction) {
        //println!("bef ins: {:?} x:{} y:{}", ins, self.x, self.y);
        match ins.dir {
            Direction::Up => {
                self.min_y = self.min_y.min(self.y - ins.dist);
                for y in 1..=ins.dist {
                    self.cubes.push(Cube::new(self.x, self.y - y));
                }
                self.y -= ins.dist;
            }
            Direction::Down => {
                self.max_y = self.max_y.max(self.y + ins.dist);
                for y in self.y + 1..=self.y + ins.dist {
                    self.cubes.push(Cube::new(self.x, y));
                }
                self.y += ins.dist;
            }
            Direction::Left => {
                self.min_x = self.min_x.min(self.x - ins.dist);
                for x in 1..=ins.dist {
                    self.cubes.push(Cube::new(self.x - x, self.y));
                }
                self.x -= ins.dist;
            }
            Direction::Right => {
                self.max_x = self.max_x.max(self.x + ins.dist);
                for x in self.x + 1..=self.x + ins.dist {
                    self.cubes.push(Cube::new(x, self.y));
                }
                self.x += ins.dist;
            }
        }
        //println!("aft ins: {:?} x:{} y:{}", ins, self.x, self.y);
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    dir: Direction,
    dist: i32,
}

#[derive(Clone, Copy, Debug)]
struct Cube {
    x: i32,
    y: i32,
}

impl Cube {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn pos_rel(&mut self, x: i32, y: i32) -> (usize, usize) {
        ((self.x - x) as usize, (self.y - y) as usize)
    }

    fn neighbor(self) -> Vec<Cube> {
        vec![
            Cube::new(self.x - 1, self.y),
            Cube::new(self.x + 1, self.y),
            Cube::new(self.x, self.y - 1),
            Cube::new(self.x, self.y + 1),
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use super::utility;
fn parse() -> Vec<Instruction> {
    utility::read("./src/input/day18.txt")
        .iter()
        .map(|x| {
            let v = x.split(' ').collect::<Vec<&str>>();
            Instruction {
                dir: match v[0] {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    "D" => Direction::Down,
                    "U" => Direction::Up,
                    _ => panic!("invalid chars {}", v[0]),
                },
                dist: v[1].parse().unwrap(),
            }
        })
        .collect()
}
