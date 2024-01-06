use std::collections::VecDeque;

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

pub fn solve_2() -> u64 {
    let (x, y, mut data) = parse();
    let mut queue = VecDeque::from([(x, y)]);
    while let Some((x, y)) = queue.pop_front() {
        let mut curr = data[y][x];
        if let Some((nx, ny)) = curr.neighbor(&mut data) {
            queue.push_front((nx, ny));
        }
    }

    data[y][x].distance = u64::MAX;
    data[y][x].block = start_shape(x, y, &mut data);

    let mut count = 0;
    for y in 0..data.len() {
        let mut cur_side = SideKind::Outside;
        let mut prev_con_block = None;
        let mut prev_con_side = SideKind::Outside;
        for x in 0..data[0].len() {
            let n = data[y][x];
            if n.distance == 0 {
                data[y][x].side = cur_side;
                if cur_side == SideKind::Inside {
                    count += 1;
                }
                continue;
            }
            if n.block == Block::HE {
                continue;
            }

            cur_side = cur_side.invert();
            if n.block.is_corner() {
                prev_con_side = cur_side;
            }

            if let Some(prev) = prev_con_block {
                // F7 and LJ
                if n.block == Block::SW && prev == Block::SE
                    || n.block == Block::NW && prev == Block::NE
                {
                    cur_side = prev_con_side;
                }
                // FJ and L7
                if prev == Block::SE && n.block == Block::NW
                    || prev == Block::NE && n.block == Block::SW
                {
                    cur_side = prev_con_side.invert();
                }
            }
            prev_con_block = Some(n.block);
        }
    }
    count
}

fn start_shape(x: usize, y: usize, data: &mut [Vec<Node>]) -> Block {
    data[y][x + 1].edges = data[y][x + 1].block.edges();
    data[y][x - 1].edges = data[y][x - 1].block.edges();
    if y > 0 {
        data[y - 1][x].edges = data[y - 1][x].block.edges();
    }
    data[y + 1][x].edges = data[y + 1][x].block.edges();

    if data[y][x + 1].edges[3] && data[y + 1][x].edges[0] {
        return Block::SE;
    }
    if y > 0 && data[y][x + 1].edges[3] && data[y - 1][x].edges[2] {
        return Block::NE;
    }
    if data[y][x - 1].edges[1] && data[y + 1][x].edges[0] {
        return Block::SW;
    }
    if y > 0 && data[y][x - 1].edges[1] && data[y - 1][x].edges[2] {
        return Block::NW;
    }
    if y > 0 && data[y - 1][x].edges[2] && data[y + 1][x].edges[0] {
        return Block::VE;
    }
    if data[y][x - 1].edges[1] && data[y][x + 1].edges[3] {
        return Block::HE;
    }
    panic!("not found");
}

#[derive(Clone, Copy)]
struct Node {
    block: Block,
    distance: u64,
    x: usize,
    y: usize,
    edges: [bool; 4],
    side: SideKind,
}

impl Node {
    fn new(x: usize, y: usize, block: Block) -> Self {
        Node {
            block,
            distance: 0,
            edges: block.edges(),
            x,
            y,
            side: SideKind::Unknown,
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
enum SideKind {
    Unknown,
    Inside,
    Outside,
}

impl SideKind {
    fn invert(self) -> Self {
        match self {
            SideKind::Unknown => SideKind::Unknown,
            SideKind::Inside => SideKind::Outside,
            SideKind::Outside => SideKind::Inside,
        }
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

impl Block {
    fn is_corner(self) -> bool {
        self == Block::SW || self == Block::SE || self == Block::NE || self == Block::NW
    }

    fn edges(self) -> [bool; 4] {
        match self {
            Block::VE => [true, false, true, false],
            Block::HE => [false, true, false, true],
            Block::NE => [true, true, false, false],
            Block::NW => [true, false, false, true],
            Block::SE => [false, true, true, false],
            Block::SW => [false, false, true, true],
            Block::GR => [false, false, false, false],
            Block::ST => [true, true, true, true],
        }
    }
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
