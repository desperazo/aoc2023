use std::collections::{HashMap, VecDeque};

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
