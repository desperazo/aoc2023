use super::utility;

pub fn solve() -> usize {
    let line = utility::read_string("./src/input/day15.txt");
    let list = line.split(',').collect::<Vec<_>>();
    list.iter()
        .map(|v| v.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256))
        .sum::<usize>()
}
