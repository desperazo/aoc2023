use super::utility;

pub fn solve() -> usize {
    let raw = utility::read("./src/input/day6.txt");
    let times = parse_line(&raw[0]);
    let distances = parse_line(&raw[1]);
    let res = times
        .iter()
        .enumerate()
        .map(|(i, &t)| {
            (1..t)
                .filter_map(|x| match (t - x) * x > distances[i] {
                    true => Some(1),
                    false => None,
                })
                .count()
        })
        .reduce(|acc, s| acc * s)
        .unwrap();
    res
}

fn parse_line(data: &str) -> Vec<i32> {
    data.split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .filter_map(|x| match x.is_empty() {
            true => None,
            false => Some(x.parse::<i32>().unwrap()),
        })
        .collect()
}
