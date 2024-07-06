pub fn solve() -> i32 {
    let data = parse();
    data.accepted_value()
}

pub fn solve_2() -> usize {
    let data = parse();
    let mut parts = [(1,4000), (1,4000), (1,4000), (1,4000)];
    let ans = data.range_acceptable("in", &mut parts);
    ans
}

struct Instruction {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Vec<(Cat, i32)>>,
}

impl Instruction {
    fn range_acceptable(&self, label: &str, parts: &mut [(i32, i32); 4]) -> usize {
        if label == "A" {
            return parts.iter().map(|&(l,r)| r.abs_diff(l) as usize + 1).product();
        }
        if label == "R" {
            return 0;
        }
        let f = &self.workflows[label];
        let mut total = 0;
        for r in f.rules.iter() {
            let i = r.cat.to_index();
            if r.greater {
                if r.value > parts[i].0 {
                    let mut gt = *parts;
                    gt[i].0 = r.value + 1;
                    parts[i].1 = r.value;
                    total += self.range_acceptable(&r.label, &mut gt);
                }
            } else {
                if r.value < parts[i].1 {
                    let mut lt = *parts;
                    lt[i].1 = r.value - 1;
                    parts[i].0 = r.value;
                    total += self.range_acceptable(&r.label, &mut lt);
                }
            }
        }
        total += self.range_acceptable(&f.last_label, parts);
        total
    }

    fn accepted_value(&self) -> i32 {
        self.parts
            .iter()
            .filter_map(|p| {
                if self.acceptable("in", p) {
                    Some(p.iter().map(|p| p.1).sum::<i32>())
                } else {
                    None
                }
            })
            .sum()
    }

    fn acceptable(&self, label: &str, part: &[(Cat, i32)]) -> bool {
        match label {
            "A" => true,
            "R" => false,
            _ => {
                let f = &self.workflows[label];
                for r in f.rules.iter() {
                    if part.iter().any(|(cat, val)| {
                        *cat == r.cat
                            && (*val > r.value && r.greater || *val < r.value && !r.greater)
                    }) {
                        return self.acceptable(&r.label, part);
                    }
                }
                self.acceptable(&f.last_label, part)
            }
        }
    }
}

#[derive(Debug)]
struct Rule {
    cat: Cat,
    greater: bool,
    value: i32,
    label: String,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    last_label: String,
}

impl Workflow {
    fn new() -> Self {
        Workflow {
            rules: vec![],
            last_label: "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cat {
    x,
    m,
    a,
    s,
}

impl Cat {
    fn to_index(self) -> usize {
        match self {
            Cat::x => 0,
            Cat::m => 1,
            Cat::a => 2,
            Cat::s => 3,
        }
    }
}

impl FromStr for Cat {
    type Err = ();
    fn from_str(value: &str) -> Result<Cat, Self::Err> {
        match value {
            "x" => Ok(Cat::x),
            "m" => Ok(Cat::m),
            "a" => Ok(Cat::a),
            "s" => Ok(Cat::s),
            _ => panic!("value {} out of range!!", value),
        }
    }
}

use std::{
    collections::{btree_map::Range, HashMap},
    ops::RangeInclusive,
    str::FromStr,
};

use super::utility;
fn parse() -> Instruction {
    let mut instruct = Instruction {
        workflows: HashMap::new(),
        parts: vec![],
    };
    let mut is_command = true;
    for tmp in utility::read("./src/input/day19.txt") {
        let line = tmp.strip_suffix('\r').unwrap_or(&tmp);
        if line.is_empty() {
            is_command = !is_command;
            continue;
        }
        if is_command {
            let open_bracket = line.chars().position(|c| c == '{').unwrap();
            let label = line[0..open_bracket].to_string();
            let mut wf = Workflow::new();
            for v in line[open_bracket + 1..].split(',') {
                if let Some(colon) = v.chars().position(|c| c == ':') {
                    let rule = Rule {
                        cat: Cat::from_str(&v[0..1]).unwrap(),
                        greater: &v[1..2] == ">",
                        value: i32::from_str(&v[2..colon]).unwrap(),
                        label: v[colon + 1..].to_string(),
                    };
                    wf.rules.push(rule);
                } else {
                    wf.last_label = v[..v.len() - 1].to_string();
                }
            }
            instruct.workflows.insert(label, wf);
        } else {
            let mut p = Vec::new();
            for v in line[1..line.len() - 1].split(',') {
                let d = v.split('=').collect::<Vec<_>>();
                p.push((Cat::from_str(d[0]).unwrap(), i32::from_str(d[1]).unwrap()))
            }
            instruct.parts.push(p);
        }
    }
    instruct
}
