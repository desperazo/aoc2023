use std::collections::{HashMap, HashSet};

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

pub fn solve_2() -> usize {
    let data = parse();
    let mut scores = (0..data.len())
        .map(|x| (x, 0))
        .collect::<HashMap<usize, usize>>();

    for (i, (l, w)) in data.iter().enumerate() {
        let count = l.iter().filter(|x| w.contains(x)).count();
        if count > 0 || scores[&i] > 0 {
            scores.entry(i).and_modify(|e| *e += 1);
        }
        let sc = scores[&i];
        for j in i + 1..=i + count {
            scores.entry(j).and_modify(|e| *e += &sc);
        }
    }
    scores
        .values()
        .map(|v| match *v {
            0 => 1,
            v => v,
        })
        .sum()
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
