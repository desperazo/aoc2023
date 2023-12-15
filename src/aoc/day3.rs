use std::{collections::HashSet, ops::Add};

use super::utility;

const MOEVMENT: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

pub fn solve() -> i32 {
    let raw = utility::read("./src/input/day3.txt");
    let data: Vec<Vec<char>> = raw.iter().map(|v| v.chars().collect()).collect();
    let mut sum = 0;
    let mut num = String::new();
    for (y, v) in data.iter().enumerate() {
        let mut touch = false;
        for (x, c) in v.iter().enumerate() {
            match c {
                '0'..='9' => {
                    num.push(*c);
                    touch = touch || touch_symbol(&data, x, y);
                    if x == data[0].len() - 1 && touch {
                        sum += num.parse::<i32>().unwrap();
                    }
                }
                _ if !num.is_empty() => {
                    if touch {
                        sum += num.parse::<i32>().unwrap();
                    }
                    touch = false;
                    num.clear();
                }
                _ => (),
            }
        }
    }
    sum
}

fn touch_symbol(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let max_x = data[0].len() as i32;
    let max_y = data.len() as i32;
    for (i, j) in MOEVMENT.iter() {
        let x1 = i + x as i32;
        let y1 = j + y as i32;
        if x1 < 0 || x1 >= max_x || y1 < 0 || y1 >= max_y {
            continue;
        }
        if !data[y1 as usize][x1 as usize].is_numeric() && data[y1 as usize][x1 as usize] != '.' {
            return true;
        }
    }
    false
}

pub fn solve_2() -> i32 {
    let raw = utility::read("./src/input/day3.txt");
    let data: Vec<Vec<char>> = raw.iter().map(|v| v.chars().collect()).collect();
    let mut sum = 0;
    for (y, v) in data.iter().enumerate() {
        for (x, _) in v.iter().enumerate().filter(|(_, c)| **c == '*') {
            let gears = adj_gear(&data, x, y);
            if gears.len() != 2 {
                continue;
            }
            sum += gears.iter().fold(1, |acc, x| acc * *x);
        }
    }
    sum
}

fn adj_gear(data: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<i32> {
    let max_x = data[0].len() as i32;
    let max_y = data.len() as i32;
    let mut nums = vec![];
    let mut checked = HashSet::new();
    for (i, j) in MOEVMENT.iter() {
        let x1 = i + x as i32;
        let y1 = j + y as i32;
        if x1 < 0 || x1 >= max_x || y1 < 0 || y1 >= max_y {
            continue;
        }
        let yu = y1 as usize;
        let xu = x1 as usize;
        if !data[yu][xu].is_numeric() || !checked.insert((xu, yu)) {
            continue;
        }
        let mut num = String::new();
        let mut index = xu;
        if xu <= x {
            while let Some(c) = data[yu].get(index) {
                if !c.is_numeric() {
                    index += 1;
                    break;
                }
                if index == 0 {
                    break;
                }
                index -= 1;
            }
        }
        while let Some(c) = data[yu].get(index) {
            if !c.is_numeric() {
                break;
            }
            checked.insert((index, yu));
            num.push(*c);
            index += 1;
        }
        nums.push(num.parse::<i32>().unwrap());
        if nums.len() > 2 {
            break;
        }
    }
    nums
}
