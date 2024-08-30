use std::collections::{HashMap, HashSet, VecDeque};
const SIZE: usize = 141;
type Map = [[Tile; SIZE]; SIZE];
type Pos = (usize, usize);

pub fn solve() -> usize {
    longest_path(true)
}

pub fn solve_2() -> usize {
    longest_path(false)
}

fn longest_path(follow_slop: bool) -> usize {
    let goal = (SIZE - 2, SIZE - 1);
    let map = parse();
    let mut queue = VecDeque::new();
    queue.push_back(Walker {
        pos: (1, 0),
        seen: HashSet::from([(1, 0)]),
        steps: 0,
    });
    let graph = build_graph(&map, follow_slop);
    let mut max_steps = 0;
    while let Some(w) = queue.pop_front() {
        if w.pos == goal {
            max_steps = max_steps.max(w.steps);
            continue;
        }
        if let Some(v) = graph.get(&w.pos) {
            for (n, d) in v.iter() {
                let mut wn = w.clone();
                if !wn.seen.insert(*n) {
                    continue;
                }
                wn.pos = *n;
                wn.steps += d;
                queue.push_front(wn);
            }
        }
    }
    max_steps
}

fn build_graph(map: &Map, follow_slop: bool) -> HashMap<Pos, Vec<(Pos, usize)>> {
    let mut nodes = vec![];
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let start = Walker {
        pos: (1, 0),
        seen: HashSet::from([(1, 0)]),
        steps: 0,
    };
    nodes.push(start.clone());
    queue.push_front(start);
    while let Some(mut walker) = queue.pop_front() {
        if !seen.insert(walker.pos) {
            continue;
        }
        let next = walker.next(&map, follow_slop);
        let size = next.len();
        if size > 1 || walker.pos == (SIZE - 2, SIZE - 1) {
            nodes.push(walker);
        }
        for n in next.iter() {
            queue.push_front(n.clone());
        }
    }
    let mut graph: HashMap<Pos, Vec<(Pos, usize)>> = HashMap::new();
    for w in nodes.iter_mut() {
        w.steps = 0;
        w.seen.clear();
        w.seen.insert(w.pos);
        for mut n in w.next(&map, follow_slop) {
            loop {
                let test = n.next(&map, follow_slop);
                if test.len() == 1 {
                    n = test[0].clone();
                    continue;
                }
                break;
            }
            graph
                .entry(w.pos)
                .and_modify(|e| e.push((n.pos, n.steps)))
                .or_insert(vec![(n.pos, n.steps)]);
        }
    }
    graph
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Walker {
    pos: Pos,
    seen: HashSet<Pos>,
    steps: usize,
}

impl Walker {
    fn next(&mut self, map: &Map, follow_slop: bool) -> Vec<Self> {
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
                Tile::Slope(d) if d == nd && follow_slop => next.push(Self {
                    pos: (nx, ny),
                    seen: self.seen.clone(),
                    steps: self.steps + 1,
                }),
                Tile::Slope(_) if follow_slop => continue,
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
