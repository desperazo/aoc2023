pub fn solve() -> i64 {
    let ins = parse();
    calculate(&ins)
}

pub fn solve_2() -> i64 {
    let ins = parse2();
    calculate(&ins)
}

fn calculate(ins: &[Instruction]) -> i64 {
    let mut world = World {
        x: 0,
        y: 0,
        edge_len: 0,
        cubes: Vec::new(),
    };
    ins.iter().for_each(|i| world.travel(i));
    world.sholelace()
}

struct World {
    x: i64,
    y: i64,
    edge_len: i64,
    cubes: Vec<Cube>,
}

impl World {
    fn sholelace(&self) -> i64 {
        let mut val = 0;
        let tmp = &self.cubes;
        for i in 0..tmp.len() - 1 {
            val += tmp[i].x * tmp[i + 1].y - tmp[i].y * tmp[i + 1].x;
        }
        val += tmp.last().unwrap().x * tmp[0].y - tmp.last().unwrap().y * tmp[0].x;
        (val.abs() + self.edge_len) / 2 + 1
    }

    fn travel(&mut self, ins: &Instruction) {
        self.edge_len += ins.dist;
        match ins.dir {
            Direction::Up => {
                self.y -= ins.dist;
            }
            Direction::Down => {
                self.y += ins.dist;
            }
            Direction::Left => {
                self.x -= ins.dist;
            }
            Direction::Right => {
                self.x += ins.dist;
            }
        }
        self.cubes.push(Cube {
            x: self.x,
            y: self.y,
        });
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    dir: Direction,
    dist: i64,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Cube {
    x: i64,
    y: i64,
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

fn parse2() -> Vec<Instruction> {
    utility::read("./src/input/day18.txt")
        .iter()
        .map(|x| {
            let p = x.char_indices().position(|(_, c)| c == '#').unwrap();
            let v = x[p + 1..p + 6].to_string();
            let v2 = x[p + 6..p + 7].to_string();
            Instruction {
                dir: match v2.as_str() {
                    "2" => Direction::Left,
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "3" => Direction::Up,
                    _ => panic!("invalid chars {}", x),
                },
                dist: i64::from_str_radix(&v, 16).unwrap(),
            }
        })
        .collect()
}
