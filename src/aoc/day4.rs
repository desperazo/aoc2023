use super::utility;

pub fn solve() -> i32 {
    let data = parse();
    let ans = data
        .iter()
        .map(|(w, l)| {
            let count = l.iter().filter(|x| w.contains(x)).count();
            if count == 0 {
                return 0;
            }
            2_i32.pow((count - 1) as u32)
        })
        .sum::<i32>();
    ans
}

fn parse() -> Vec<(Vec<i32>, Vec<i32>)> {
    let raw = utility::read("./src/input/day4.txt");
    raw.iter()
        .map(|s| {
            let d = s.split(':').collect::<Vec<&str>>()[1]
                .split('|')
                .collect::<Vec<&str>>();
            let w = d[0]
                .split(' ')
                .filter(|x| !x.trim().is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let l = d[1]
                .split(' ')
                .filter(|x| !x.trim().is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (w, l)
        })
        .collect()
}
