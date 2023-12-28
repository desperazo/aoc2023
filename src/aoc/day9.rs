use super::utility;

pub fn solve() -> i64 {
    predict(|acc, e| acc + e.last().unwrap())
}

pub fn solve_2() -> i64 {
    predict(|acc, e| e.first().unwrap() - acc)
}

fn predict<F>(pred: F) -> i64
where
    F: Fn(i64, &[i64]) -> i64,
{
    let input = parse();
    let mut res = vec![];
    for v in input.iter() {
        let mut temp = vec![v.clone()];
        let mut diff = v.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i64>>();
        temp.push(diff.clone());
        while diff.iter().any(|x| *x != 0) {
            diff = diff.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i64>>();
            temp.push(diff.clone());
        }
        temp.reverse();
        let add = temp.iter().fold(0, |acc, e| pred(acc, e));
        res.push(add);
    }
    res.iter().sum()
}

fn parse() -> Vec<Vec<i64>> {
    let raw = utility::read("./src/input/day9.txt");
    let input = raw
        .iter()
        .map(|x| {
            x.replace(['\r', '\n'], "")
                .split(' ')
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    input
}
