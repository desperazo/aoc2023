use std::collections::HashMap;

use super::utility;

pub fn solve() -> i32 {
    let raw = utility::read("./src/input/day8.txt");
    let nav = raw[0].chars().collect::<Vec<char>>();
    let mut maps = HashMap::new();
    for v in raw.iter().skip(1) {
        let line = v.replace(['=', '(', ')', ','], "").to_string();
        let tmp = line.split(' ').collect::<Vec<&str>>();
        maps.insert(tmp[0].to_string(), (tmp[2].to_string(), tmp[3].to_string()));
    }
    let mut cur = "AAA".to_string();
    let mut ind = 0;
    let mut steps = 0;
    loop {
        let cmd = nav[ind];
        let (left, right) = &maps[&cur];
        cur = if cmd == 'L' {
            left.to_string()
        } else {
            right.to_string()
        };
        steps += 1;
        if cur == "ZZZ" {
            return steps;
        }
        ind += 1;
        ind %= nav.len();
    }
}

pub fn solve_2() -> u128 {
    let raw = utility::read("./src/input/day8.txt");
    let nav = raw[0].chars().collect::<Vec<char>>();
    let mut maps = HashMap::new();
    let mut starters = vec![];
    for v in raw.iter().skip(1) {
        let line = v.replace(['=', '(', ')', ',', '\r', '\n'], "").to_string();
        let tmp = line.split(' ').collect::<Vec<&str>>();
        maps.insert(tmp[0].to_string(), (tmp[2].to_string(), tmp[3].to_string()));
        if tmp[0].chars().nth(2).unwrap() == 'A' {
            starters.push(tmp[0].to_string());
        }
    }
    let mut ind = 0;
    let mut steps = 0;
    let mut data = vec![0; starters.len()];
    while data.iter().any(|x| *x == 0) {
        steps += 1;
        starters = starters
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let cmd = nav[ind];
                let (left, right) = &maps[v];
                let next = if cmd == 'L' {
                    left.to_string()
                } else {
                    right.to_string()
                };
                if next.ends_with('Z') && data[i] == 0 {
                    data[i] = steps;
                }
                next
            })
            .collect();
        ind += 1;
        ind %= nav.len();
    }

    let ans = data.iter().fold(1, |acc, x| lcm(acc, *x));
    ans
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}
