use super::utility;

pub fn solve() -> usize {
    let mut data = parse();
    for y in 1..data.len() {
        for x in 0..data[0].len() {
            if data[y][x] != Tile::RoundRock {
                continue;
            }
            let mut new_y = y;
            while new_y > 0 && data[new_y - 1][x] == Tile::Space {
                data[new_y - 1][x] = Tile::RoundRock;
                data[new_y][x] = Tile::Space;
                new_y -= 1;
            }
        }
    }
    data.iter()
        .enumerate()
        .map(|(y, r)| r.iter().filter(|x| **x == Tile::RoundRock).count() * (data.len() - y))
        .sum()
}

fn debug_print(data: &[Vec<Tile>]) {
    data.iter().enumerate().for_each(|(y, r)| {
        r.iter().enumerate().for_each(|(x, c)| {
            let t = match c {
                Tile::RoundRock => 'O',
                Tile::CubeRock => '#',
                Tile::Space => '.',
            };
            print!("{t}");
        });
        println!();
    });
    println!();
    println!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    RoundRock,
    CubeRock,
    Space,
}

fn parse() -> Vec<Vec<Tile>> {
    utility::read("./src/input/day14.txt")
        .iter()
        .map(|x| {
            x.chars()
                .filter(|v| *v != '\r' && *v != '\n')
                .map(|v| match v {
                    '#' => Tile::CubeRock,
                    'O' => Tile::RoundRock,
                    '.' => Tile::Space,
                    _ => panic!("value `{v}` not match"),
                })
                .collect()
        })
        .collect()
}
