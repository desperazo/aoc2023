use super::utility;

pub fn solve() -> usize {
    let data = parse();
    let ans = data.iter().enumerate().fold(0, |acc, (i, v)| {
        match v
            .iter()
            .all(|c| c.red <= 12 && c.green <= 13 && c.blue <= 14)
        {
            true => acc + i + 1,
            false => acc,
        }
    });
    ans
}

pub fn solve_2() -> i32 {
    let data = parse();
    let ans = data.iter().fold(0, |acc, v| {
        let (mut blue, mut green, mut red) = (1, 1, 1);
        for c in v.iter() {
            blue = blue.max(c.blue);
            green = green.max(c.green);
            red = red.max(c.red);
        }
        acc + blue * green * red
    });
    ans
}

fn parse() -> Vec<Vec<Cube>> {
    let data = utility::read("./src/input/day2.txt");

    data.iter()
        .map(|v| {
            let tmp = v.split(':').collect::<Vec<_>>();
            tmp[1]
                .split(';')
                .map(|d| {
                    let mut c = Cube {
                        blue: 0,
                        green: 0,
                        red: 0,
                    };
                    for dv in d.split(',') {
                        let dv2 = dv.trim().split(' ').collect::<Vec<&str>>();
                        match dv2[1] {
                            "blue" => c.blue = dv2[0].parse().unwrap(),
                            "green" => c.green = dv2[0].parse().unwrap(),
                            "red" => c.red = dv2[0].parse().unwrap(),
                            _ => (),
                        }
                    }
                    c
                })
                .collect::<Vec<Cube>>()
        })
        .collect()
}

struct Cube {
    blue: i32,
    green: i32,
    red: i32,
}
