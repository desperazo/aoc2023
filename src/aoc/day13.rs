use std::ops::Sub;

use super::utility;

pub fn solve(allow_fix: bool) -> usize {
    let mirrors = parse();
    let mut score = 0;
    for m in mirrors {
        if let Some(v) = find_mirror(allow_fix, m.len(), |lower: usize, upper| {
            m[lower]
                .iter()
                .zip(m[upper].iter())
                .map(|(l, r)| if l == r { 0 } else { 1 })
                .sum::<usize>()
        }) {
            score += v * 100;
        } else if let Some(v) = find_mirror(allow_fix, m[0].len(), |lower: usize, upper| {
            m.iter()
                .map(|v: &Vec<Mirror>| if v[lower] == v[upper] { 0 } else { 1 })
                .sum::<usize>()
        }) {
            score += v;
        }
    }
    score
}

fn find_mirror<F>(allow_fix: bool, max_len: usize, differ: F) -> Option<usize>
where
    F: Fn(usize, usize) -> usize,
{
    let mut start = 0;
    while start < max_len - 1 {
        let target = (start + 1).min(max_len.sub(start + 1));
        let mut expand = 0;
        let mut fix_count = if allow_fix { 1 } else { 0 };
        while expand < target {
            let lower = start - expand;
            let upper = start + expand + 1;
            let dif = differ(lower, upper);
            if dif == 0 || dif == 1 && fix_count > 0 {
                expand += 1;
                if dif > 0 {
                    fix_count -= 1;
                }
            } else {
                break;
            }
        }
        if expand == target && fix_count == 0 {
            return Some(start + 1);
        }
        start += 1;
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mirror {
    Rock,
    Ash,
}

fn parse() -> Vec<Vec<Vec<Mirror>>> {
    let raw = utility::read_string("./src/input/day13.txt");
    let mut data = vec![];
    let mut item = vec![];
    let lines = raw.split('\n').collect::<Vec<_>>();
    for line in lines.iter() {
        if line.is_empty() {
            data.push(item.clone());
            item.clear();
            continue;
        }
        item.push(
            line.chars()
                .map(|v| if v == '#' { Mirror::Rock } else { Mirror::Ash })
                .collect(),
        );
    }
    data.push(item.clone());
    data
}
