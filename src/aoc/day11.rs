use super::utility;

pub fn solve() -> usize {
    let mut data = parse();
    let mut count = 0;
    while count < data.len() {
        if data[count].iter().all(|v| v.kind == Kind::Space) {
            data.insert(
                count,
                vec![
                    Coord {
                        x: 0,
                        y: 0,
                        kind: Kind::Space
                    };
                    data[count].len()
                ],
            );
            count += 1;
        }
        count += 1;
    }
    count = 0;
    while count < data[0].len() {
        if data.iter().all(|r| r[count].kind == Kind::Space) {
            data.iter_mut().for_each(|r| {
                r.insert(
                    count,
                    Coord {
                        x: 0,
                        y: 0,
                        kind: Kind::Space,
                    },
                )
            });
            count += 1;
        }
        count += 1;
    }
    let mut total_distance = 0;
    let galaxy: Vec<(usize, usize)> = data
        .iter()
        .enumerate()
        .flat_map(|(y, v)| {
            v.iter()
                .enumerate()
                .filter_map(|(x, k)| match k.kind {
                    Kind::Space => None,
                    Kind::Galaxy => Some((x, y)),
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    for i in 0..galaxy.len() {
        for j in i + 1..galaxy.len() {
            let man_dist = galaxy[i].0.abs_diff(galaxy[j].0) + galaxy[i].1.abs_diff(galaxy[j].1);
            total_distance += man_dist;
        }
    }
    total_distance
}

pub fn solve_2() -> usize {
    let mut data = parse();
    let mut count = 0;
    let expand_size = 1_000_000;
    while count < data.len() {
        if data[count].iter().all(|v| v.kind == Kind::Space) {
            data.iter_mut()
                .enumerate()
                .filter(|(y, _)| *y > count)
                .for_each(|(_, r)| {
                    r.iter_mut().for_each(|v| {
                        v.y += expand_size - 1;
                    })
                });
        }
        count += 1;
    }
    count = 0;
    while count < data[0].len() {
        if data.iter().all(|r| r[count].kind == Kind::Space) {
            data.iter_mut().for_each(|r| {
                r.iter_mut()
                    .enumerate()
                    .filter(|(c, _)| *c > count)
                    .for_each(|(_, v)| {
                        v.x += expand_size - 1;
                    })
            });
        }
        count += 1;
    }
    let mut total_distance = 0;
    let galaxy: Vec<Coord> = data
        .iter()
        .enumerate()
        .flat_map(|(y, v)| {
            v.iter()
                .enumerate()
                .filter_map(|(x, k)| match k.kind {
                    Kind::Space => None,
                    Kind::Galaxy => Some(*k),
                })
                .collect::<Vec<Coord>>()
        })
        .collect::<Vec<Coord>>();

    for i in 0..galaxy.len() {
        for j in i + 1..galaxy.len() {
            let man_dist = galaxy[i].x.abs_diff(galaxy[j].x) + galaxy[i].y.abs_diff(galaxy[j].y);
            total_distance += man_dist;
        }
    }
    total_distance
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Kind {
    Space,
    Galaxy,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
    kind: Kind,
}

fn parse() -> Vec<Vec<Coord>> {
    let raw = utility::read("./src/input/day11.txt");
    let data = raw
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Coord {
                        x,
                        y,
                        kind: Kind::Space,
                    },
                    '#' => Coord {
                        x,
                        y,
                        kind: Kind::Galaxy,
                    },
                    _ => panic!("invalid char {}", c),
                })
                .collect::<Vec<Coord>>()
        })
        .collect::<Vec<Vec<Coord>>>();
    data
}
