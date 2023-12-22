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

pub fn solve_2() -> u64 {
    let raw = utility::read("./src/input/day6.txt");
    let times = raw[0].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let distances = raw[1].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let mut offset = distances / times;
    while (times - offset) * offset < distances {
        offset += 1;
    }
    times - offset - offset + 1
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
