use std::collections::VecDeque;

use super::utility;

pub fn solve() -> usize {
    let mut grids = parse();
    //debug_print(&grids);
    let player = Player {
        dir: Direction::Down,
        x: 0,
        y: 0,
    };
    grids[0][0].dirs.push(Direction::Right);
    let mut queue = VecDeque::new();
    queue.push_back(player);
    while let Some(q) = queue.pop_front() {
        if let Some(new_players) = q.walk(&mut grids) {
            queue.extend(new_players);
        }
        //debug_print(&grids);
        if queue.len() % 1000 == 0 {
            println!("remaing {}", queue.len());
        }
    }
    energize_print(&grids);
    grids
        .iter()
        .map(|v| v.iter().filter(|f| !f.dirs.is_empty()).count())
        .sum()
}

#[derive(Clone, Copy, Debug)]
struct Player {
    dir: Direction,
    x: usize,
    y: usize,
}

impl Player {
    fn walk(&self, grid: &mut [Vec<Tile>]) -> Option<Vec<Self>> {
        let (x, y) = match self.dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        if self.x.checked_add_signed(x).is_none() || self.y.checked_add_signed(y).is_none() {
            return None;
        }
        // println!("{:?} {x} {y}", self);
        let new_x = (self.x as isize + x) as usize;
        let new_y = (self.y as isize + y) as usize;
        if new_x >= grid[0].len() || new_y >= grid.len() {
            return None;
        }

        let dirs = match (grid[new_y][new_x].kind, self.dir) {
            (TileKind::MirrorSlash, Direction::Up) => vec![Direction::Right],
            (TileKind::MirrorSlash, Direction::Down) => vec![Direction::Left],
            (TileKind::MirrorSlash, Direction::Left) => vec![Direction::Down],
            (TileKind::MirrorSlash, Direction::Right) => vec![Direction::Up],
            (TileKind::MirrorBackSlash, Direction::Up) => vec![Direction::Left],
            (TileKind::MirrorBackSlash, Direction::Down) => vec![Direction::Right],
            (TileKind::MirrorBackSlash, Direction::Left) => vec![Direction::Up],
            (TileKind::MirrorBackSlash, Direction::Right) => vec![Direction::Down],
            (TileKind::SpliterVertical, dir)
                if dir == Direction::Left || dir == Direction::Right =>
            {
                vec![Direction::Up, Direction::Down]
            }
            (TileKind::SplitterHorizontal, dir)
                if dir == Direction::Up || dir == Direction::Down =>
            {
                vec![Direction::Left, Direction::Right]
            }
            (_, dir) => vec![dir],
        };
        let filters = dirs
            .iter()
            .filter(|d| !grid[new_y][new_x].dirs.iter().any(|gd| d == &gd))
            .collect::<Vec<_>>();
        if filters.is_empty() {
            return None;
        }
        grid[new_y][new_x].dirs.extend(filters.clone());
        Some(
            filters
                .iter()
                .map(|d| Self {
                    dir: **d,
                    x: new_x,
                    y: new_y,
                })
                .collect(),
        )
    }
}

#[derive(Clone)]
struct Tile {
    dirs: Vec<Direction>,
    kind: TileKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileKind {
    Space,
    MirrorSlash,
    MirrorBackSlash,
    SpliterVertical,
    SplitterHorizontal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn debug_print(data: &Vec<Vec<Tile>>) {
    for line in data.iter() {
        for v in line.iter() {
            let c = match v.kind {
                TileKind::MirrorSlash => "/".to_owned(),
                TileKind::MirrorBackSlash => "\\".to_owned(),
                TileKind::SpliterVertical => "|".to_owned(),
                TileKind::SplitterHorizontal => "-".to_owned(),
                TileKind::Space => match v.dirs.len() {
                    0 => ".".to_string(),
                    1 => match v.dirs[0] {
                        Direction::Up => "^".to_owned(),
                        Direction::Down => "V".to_owned(),
                        Direction::Left => "<".to_owned(),
                        Direction::Right => ">".to_owned(),
                    },
                    cnt => cnt.to_string(),
                },
            };
            print!("{c}");
        }
        println!();
    }
    println!();
    println!();
}

fn energize_print(data: &Vec<Vec<Tile>>) {
    for line in data.iter() {
        for v in line.iter() {
            let c = match v.dirs.len() {
                0 => ".".to_string(),
                _ => "#".to_string(),
            };
            print!("{c}");
        }
        println!();
    }
    println!();
    println!();
}

fn parse() -> Vec<Vec<Tile>> {
    utility::read("./src/input/day16.txt")
        .iter()
        .map(|x| {
            x.trim_end()
                .chars()
                .map(|c| match c {
                    '.' => Tile {
                        dirs: vec![],
                        kind: TileKind::Space,
                    },
                    '/' => Tile {
                        dirs: vec![],
                        kind: TileKind::MirrorSlash,
                    },
                    '\\' => Tile {
                        dirs: vec![],
                        kind: TileKind::MirrorBackSlash,
                    },
                    '-' => Tile {
                        dirs: vec![],
                        kind: TileKind::SplitterHorizontal,
                    },
                    '|' => Tile {
                        dirs: vec![],
                        kind: TileKind::SpliterVertical,
                    },
                    _ => panic!("invalid chars {c}"),
                })
                .collect()
        })
        .collect()
}
