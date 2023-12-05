use super::utility;

pub fn solve() -> i32 {
    let data = utility::read("./src/input/day1.txt");
    data.iter().fold(0, |acc, line| {
        let f: Vec<_> = line.chars().filter(|x| x.is_numeric()).collect();
        let mut v = String::new();
        v.push(*f.first().unwrap());
        v.push(*f.last().unwrap());
        acc + v.parse::<i32>().unwrap()
    })
}
