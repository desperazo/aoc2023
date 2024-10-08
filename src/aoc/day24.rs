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

pub fn solve_2() -> f64 {
    let input = parse();
    let mut data = input
        .windows(2)
        .take(4)
        .map(|h| {
            Vec::from_iter([
                h[1].velocity.y - h[0].velocity.y,
                h[0].velocity.x - h[1].velocity.x,
                h[0].position.y - h[1].position.y,
                h[1].position.x - h[0].position.x,
                ((h[0].position.y * h[0].velocity.x)
                    + (-h[0].position.x * h[0].velocity.y)
                    + (-h[1].position.y * h[1].velocity.x)
                    + (h[1].position.x * h[1].velocity.y)),
            ])
        })
        .collect::<Vec<_>>();
    let g = gaussian_elimination(&mut data);
    let (rock_x, rock_y, rock_dx, rock_dy) = (g[0], g[1], g[2], g[3]);
    let mut data2 = input
        .windows(2)
        .take(2)
        .map(|h| {
            Vec::from_iter([
                h[0].velocity.x - h[1].velocity.x,
                h[1].position.x - h[0].position.x,
                ((h[0].position.z * h[0].velocity.x)
                    - (h[0].position.x * h[0].velocity.z)
                    - (h[1].position.z * h[1].velocity.x)
                    + (h[1].position.x * h[1].velocity.z)
                    - ((h[1].velocity.z - h[0].velocity.z) * rock_x)
                    - ((h[0].position.z - h[1].position.z) * rock_dx)),
            ])
        })
        .collect::<Vec<_>>();
    let g2 = gaussian_elimination(&mut data2);
    rock_x + rock_y + g2[0]
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

fn gaussian_elimination(matrix: &mut [Vec<f64>]) -> Vec<f64> {
    let size = matrix.len();
    assert_eq!(size, matrix[0].len() - 1);

    for i in 0..size - 1 {
        for j in i..size - 1 {
            echelon(matrix, i, j);
        }
    }

    for i in (1..size).rev() {
        eliminate(matrix, i);
    }

    // Disable cargo clippy warnings about needless range loops.
    // Checking the diagonal like this is simpler than any alternative.
    #[allow(clippy::needless_range_loop)]
    for i in 0..size {
        if matrix[i][i] == 0f64 {
            println!("Infinitely many solutions");
        }
    }

    let mut result: Vec<f64> = vec![0f64; size];
    for i in 0..size {
        result[i] = (matrix[i][size] / matrix[i][i]).round();
    }
    result
}

fn echelon(matrix: &mut [Vec<f64>], i: usize, j: usize) {
    let size = matrix.len();
    if matrix[i][i] == 0f64 {
    } else {
        let factor = matrix[j + 1][i] / matrix[i][i];
        (i..size + 1).for_each(|k| {
            matrix[j + 1][k] -= factor * matrix[i][k];
        });
    }
}

fn eliminate(matrix: &mut [Vec<f64>], i: usize) {
    let size = matrix.len();
    if matrix[i][i] == 0f64 {
    } else {
        for j in (1..i + 1).rev() {
            let factor = matrix[j - 1][i] / matrix[i][i];
            for k in (0..size + 1).rev() {
                matrix[j - 1][k] -= factor * matrix[i][k];
            }
        }
    }
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
    fn gaussion_elimination_test() {
        let mut data = vec![
            vec![2.0, 1.0, -1.0, 8.0],
            vec![-3.0, -1.0, 2.0, -11.0],
            vec![-2.0, 1.0, 2.0, -3.0],
        ];
        let g = gaussian_elimination(&mut data);
        assert_eq!(g, vec![2.0, 3.0, -1.0]);
    }

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
