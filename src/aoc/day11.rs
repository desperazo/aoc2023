use super::utility;

pub fn solve() -> usize {
    let mut data = parse();
    let mut count = 0;
    while count < data.len() {
        if data[count].iter().all(|v| *v == Kind::Space) {
            data.insert(count, vec![Kind::Space; data[count].len()]);
            count += 1;
        }
        count += 1;
    }
    count = 0;
    while count < data[0].len() {
        if data.iter().all(|r| r[count] == Kind::Space) {
            data.iter_mut().for_each(|r| r.insert(count, Kind::Space));
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
                .filter_map(|(x, k)| match k {
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Kind {
    Space,
    Galaxy,
}

fn parse() -> Vec<Vec<Kind>> {
    let raw = utility::read("./src/input/day11.txt");
    let data = raw
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Kind::Space,
                    '#' => Kind::Galaxy,
                    _ => panic!("invalid char {}", c),
                })
                .collect::<Vec<Kind>>()
        })
        .collect::<Vec<Vec<Kind>>>();
    data
}
