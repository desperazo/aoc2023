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

pub fn solve_2() -> i32 {
    let data = utility::read("./src/input/day1.txt");
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    data.iter().fold(0, |acc, line| {
        let mut min = usize::MAX;
        let mut max = usize::MAX;
        for (i, v) in line.chars().enumerate() {
            match v.is_numeric() {
                true => {
                    if min == usize::MAX {
                        min = v.to_digit(10).unwrap() as usize;
                    }
                    max = v.to_digit(10).unwrap() as usize;
                }
                false => {
                    let mut num = String::new();
                    for c in line.chars().skip(i).take(5) {
                        num.push(c);
                        if num.len() < 3 {
                            continue;
                        }
                        if let Some(p) = nums.iter().position(|x| **x == num) {
                            if min == usize::MAX {
                                min = p + 1;
                            }
                            max = p + 1;
                            break;
                        }
                    }
                }
            }
        }
        let mut v = String::new();
        v.push_str(&min.to_string());
        v.push_str(&max.to_string());
        acc + v.parse::<i32>().unwrap()
    })
}
