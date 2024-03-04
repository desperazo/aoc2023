use super::utility;

pub fn solve() -> usize {
    let line = utility::read_string("./src/input/day15.txt");
    let list = line.split(',').collect::<Vec<_>>();
    list.iter()
        .map(|v| hash_value(v.chars().collect()))
        .sum::<usize>()
}

pub fn solve_2() -> usize {
    let mut slots: Vec<Vec<Box>> = vec![];
    for _ in 0..256 {
        slots.push(Vec::new());
    }
    let boxes = parse();
    for b in boxes.iter() {
        match b.ops {
            Ops::Dash => {
                if let Some(pos) = slots[b.position].iter().position(|lb| lb.label == b.label) {
                    slots[b.position].remove(pos);
                }
            }
            Ops::Equals(_) => {
                if let Some(pos) = slots[b.position].iter().position(|lb| lb.label == b.label) {
                    slots[b.position][pos].ops = b.ops;
                } else {
                    slots[b.position].push(b.clone());
                }
            }
        }
    }
    slots
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(|(j, v2)| {
                    (i + 1)
                        * (j + 1)
                        * match v2.ops {
                            Ops::Dash => panic!("invalid operation"),
                            Ops::Equals(box_value) => box_value,
                        }
                })
                .sum::<usize>()
        })
        .sum()
}

fn hash_value(chars: Vec<char>) -> usize {
    chars
        .iter()
        .fold(0, |acc, &c| ((acc + c as usize) * 17) % 256)
}

fn parse() -> Vec<Box> {
    utility::read_string("./src/input/day15.txt")
        .split(',')
        .map(Box::new)
        .collect()
}

struct Box {
    label: String,
    ops: Ops,
    position: usize,
}

impl Box {
    fn clone(&self) -> Self {
        Box {
            label: self.label.clone(),
            ops: self.ops,
            position: self.position,
        }
    }
    fn new(text: &str) -> Self {
        let chs = text.chars();
        if chs.last().unwrap() == '-' {
            let label = &text[..text.len() - 1];
            Box {
                label: label.to_string(),
                ops: Ops::Dash,
                position: hash_value(label.chars().collect()),
            }
        } else {
            let pos = text.chars().position(|c| c == '=').unwrap();
            let label = text[..pos].to_string();
            Box {
                label: label.to_string(),
                ops: Ops::Equals(text[pos + 1..].parse::<usize>().unwrap()),
                position: hash_value(label.chars().collect()),
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Ops {
    Dash,
    Equals(usize),
}
