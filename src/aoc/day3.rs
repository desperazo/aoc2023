use super::utility;

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
                    touch = touch || adj_symbol(&data, x, y);
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

fn adj_symbol(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let adj = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let max_x = data[0].len() as i32;
    let max_y = data.len() as i32;
    for (i, j) in adj.iter() {
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
