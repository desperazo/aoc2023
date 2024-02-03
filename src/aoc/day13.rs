use super::utility;

pub fn solve() -> usize {
    let mirrors = parse();
    let mut score = 0;
    for m in mirrors {
        if let Some(v) = hor_score(&m) {
            score += v * 100;
        } else if let Some(v) = ver_score(&m) {
            score += v;
        }
    }
    score
}

fn ver_score(data: &[Vec<Mirror>]) -> Option<usize> {
    let mut start = 0;
    while start < data[0].len() - 1 {
        let mut exp = 0;
        while start.checked_sub(exp).is_some()
            && start + exp + 1 < data[0].len()
            && data.iter().all(|v| v[start - exp] == v[start + exp + 1])
        {
            exp += 1;
        }
        if exp > 0 && (exp == start + 1 || start + exp + 1 == data[0].len()) {
            return Some(start + 1);
        }
        start += 1;
    }
    None
}

fn hor_score(data: &[Vec<Mirror>]) -> Option<usize> {
    let mut start = 0;
    while start < data.len() - 1 {
        let mut exp = 0;
        while start.checked_sub(exp).is_some()
            && start + exp + 1 < data.len()
            && data[start - exp] == data[start + exp + 1]
        {
            exp += 1;
        }
        if exp > 0 && (exp == start + 1 || start + exp + 1 == data.len()) {
            return Some(start + 1);
        }
        start += 1;
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mirror {
    Rock,
    Ash,
}

fn parse() -> Vec<Vec<Vec<Mirror>>> {
    let raw = utility::read_string("./src/input/day13.txt");
    let mut data = vec![];
    let mut item = vec![];
    let lines = raw.split('\n').collect::<Vec<_>>();
    for line in lines.iter() {
        if line.is_empty() {
            data.push(item.clone());
            item.clear();
            continue;
        }
        item.push(
            line.chars()
                .map(|v| if v == '#' { Mirror::Rock } else { Mirror::Ash })
                .collect(),
        );
    }
    data.push(item.clone());
    data
}
