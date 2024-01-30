use std::{collections::HashMap, usize};

use super::utility;

pub fn solve() -> usize {
    combind_count(1)
}

pub fn solve_2() -> usize {
    combind_count(5)
}

fn combind_count(repeat: usize) -> usize {
    let data = parse(repeat);
    data.iter()
        .map(|(txt, g)| {
            let mut chs = txt.chars().collect::<Vec<_>>();
            chs.push('.');
            let mut mem = HashMap::new();
            fit(&chs, 0, 0, g, &mut mem)
        })
        .sum()
}

fn fit(
    txt: &[char],
    mut offset: usize,
    grp_count: usize,
    grp: &[usize],
    mem: &mut HashMap<(usize, usize), usize>,
) -> usize {
    while offset < txt.len() && txt[offset] == '.' {
        offset += 1;
    }
    if grp.iter().sum::<usize>() + grp.len() > txt.len() - offset {
        return 0;
    }
    let end = txt.len() - 1;
    let mut t = 0;
    for i in offset..end {
        let right = i + grp[0];
        if right >= txt.len() || txt[i] == '.' || i > 0 && txt[i - 1] == '#' || txt[right] == '#' {
            continue;
        }
        if txt.iter().skip(i).take(grp[0]).any(|v| *v == '.') {
            continue;
        }
        if txt.iter().skip(offset).take(i - offset).any(|v| *v == '#') {
            break;
        }
        if grp.len() == 1 && txt.iter().skip(i + grp[0]).any(|v| *v == '#') {
            continue;
        }

        if grp.len() > 1 {
            if let Some(stored) = mem.get(&(grp_count, i)) {
                t += *stored;
            } else {
                let c = fit(txt, right + 1, grp_count + 1, &grp[1..], mem);
                mem.entry((grp_count, i)).or_insert(c);
                t += c;
            }
        } else {
            t += 1;
        }
    }
    t
}

fn parse(size: usize) -> Vec<(String, Vec<usize>)> {
    let raw = utility::read("./src/input/day12.txt");
    raw.iter()
        .map(|s| {
            let t = s.split(' ').collect::<Vec<&str>>();
            let txt = vec![t[0].to_string(); size];
            let nums = t[1]
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (txt.join("?"), nums.repeat(size))
        })
        .collect()
}
