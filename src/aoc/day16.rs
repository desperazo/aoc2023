use std::collections::VecDeque;

use super::utility;

pub fn solve() -> usize {
    let mut grids = parse();
    energize(&mut grids, 0, 0, Direction::Right)
}

pub fn solve_2() -> usize {
    let grids = parse();
    let mut max_energize = 0;
    let right_most = grids[0].len() - 1;
    let bottom_most = grids.len() - 1;
    for y in 0..grids.len() {
        let mut tmp = grids.clone();
        let value = energize(&mut tmp, 0, y, Direction::Right);
        max_energize = max_energize.max(value);

        let mut tmp = grids.clone();
        let value = energize(&mut tmp, right_most, y, Direction::Left);
        max_energize = max_energize.max(value);
    }

    for x in 0..grids[0].len() {
        let mut tmp = grids.clone();
        let value = energize(&mut tmp, x, 0, Direction::Down);
        max_energize = max_energize.max(value);

        let mut tmp = grids.clone();
        let value = energize(&mut tmp, x, bottom_most, Direction::Up);
        max_energize = max_energize.max(value);
    }
    max_energize
}

fn energize(grids: &mut [Vec<Tile>], x: usize, y: usize, init_dir: Direction) -> usize {
    let mut queue = VecDeque::new();
    Player::next_directions(grids[y][x].kind, init_dir)
        .iter()
        .for_each(|d| {
            grids[y][x].dirs.push(*d);
            queue.push_back(Player { dir: *d, x, y })
        });
    while let Some(q) = queue.pop_front() {
        if let Some(new_players) = q.walk(grids) {
            queue.extend(new_players);
        }
    }
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

        let dirs = Player::next_directions(grid[new_y][new_x].kind, self.dir);
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

    fn next_directions(kind: TileKind, current_direction: Direction) -> Vec<Direction> {
        match (kind, current_direction) {
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
        }
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
