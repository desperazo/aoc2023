pub fn solve() -> usize {
    let input = parse();
    let mut ans = 0;
    let min_range = 200000000000000_f64;
    let max_range = 400000000000000_f64;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if let Some(v) = intercept(input[i], input[j]) {
                if v.x < min_range || v.x > max_range || v.y < min_range || v.y > max_range {
                    continue;
                }
                if input[i].move_forward(v) && input[j].move_forward(v) {
                    ans += 1;
                }
            }
        }
    }
    ans
}

pub fn solve_2() -> usize {
    let input = parse();
    0
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Hailstone {
    position: Position,
    velocity: Velocity,
}

impl Hailstone {
    fn move_forward(self, pos: Position) -> bool {
        let dx = pos.x - self.position.x;
        let dy = pos.y - self.position.y;
        let fw_x = dx > 0.0 && self.velocity.x > 0.0 || dx < 0.0 && self.velocity.x < 0.0;
        let fw_y = dy > 0.0 && self.velocity.y > 0.0 || dy < 0.0 && self.velocity.y < 0.0;
        fw_x && fw_y
    }

    fn predict_z(self, time: f64, delta_vz: f64) -> f64 {
        self.position.z + time * (self.velocity.z + delta_vz)
    }
}

fn intercept(s1: Hailstone, s2: Hailstone) -> Option<Position> {
    let m1 = s1.velocity.y / s1.velocity.x;
    let c1 = s1.position.y - m1 * s1.position.x;

    let m2 = s2.velocity.y / s2.velocity.x;
    let c2 = s2.position.y - m2 * s2.position.x;

    if m1 == m2 {
        return None;
    }

    let x = (c2 - c1) / (m1 - m2);
    let y = m1 * x + c1;

    Some(Position { x, y, z: 0.0 })
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Velocity {
    x: f64,
    y: f64,
    z: f64,
}

use super::utility;
fn parse() -> Vec<Hailstone> {
    let mut data = vec![];
    let input = utility::read("./src/input/day24.txt");
    for (y, line) in input.iter().enumerate() {
        let d = line.split('@').collect::<Vec<_>>();
        let p = d[0].split(',').collect::<Vec<_>>();
        let v = d[1].split(',').collect::<Vec<_>>();
        data.push(Hailstone {
            position: Position {
                x: p[0].trim().parse().unwrap(),
                y: p[1].trim().parse().unwrap(),
                z: p[2].trim().parse().unwrap(),
            },
            velocity: Velocity {
                x: v[0].trim().parse().unwrap(),
                y: v[1].trim().parse().unwrap(),
                z: v[2].trim().parse().unwrap(),
            },
        })
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_intercept() {
        let s1 = Hailstone {
            position: Position {
                x: 19.0,
                y: 13.0,
                z: 30.0,
            },
            velocity: Velocity {
                x: -2.0,
                y: 1.0,
                z: -2.0,
            },
        };
        let s2 = Hailstone {
            position: Position {
                x: 18.0,
                y: 19.0,
                z: 22.0,
            },
            velocity: Velocity {
                x: -1.0,
                y: -1.0,
                z: -2.0,
            },
        };
        let actual = intercept(s1, s2);
        assert!(actual.is_some());
    }
}
