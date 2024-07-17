const BROADCASTER: &str = "broadcaster";

pub fn solve() -> usize {
    let mut modules = parse();
    let mut queue = VecDeque::new();
    queue.push_back(("".to_string(), BROADCASTER.to_string(), false));
    let (mut low_count, mut high_count) = (0, 0);
    for _i in 0..1000 {
        while let Some((sender, receiver, pulse)) = queue.pop_front() {
            if pulse {
                high_count += 1;
            } else {
                low_count += 1;
            }
            if modules.get_mut(&receiver).is_none() {
                continue;
            }
            let m = modules.get_mut(&receiver).unwrap();
            if let Some(new_pulse) = m.process(sender, pulse) {
                for con in m.connections.iter() {
                    queue.push_back((receiver.clone(), con.to_string(), new_pulse));
                }
            }
        }
        queue.push_back(("".to_string(), BROADCASTER.to_string(), false));
    }
    low_count * high_count
}

pub fn solve_2() -> u128 {
    let mut modules = parse();
    let mut queue = VecDeque::new();
    queue.push_back(("".to_string(), BROADCASTER.to_string(), false));
    let mut df_input = HashMap::new();
    'outer: for i in 1..10000 {
        while let Some((sender, receiver, pulse)) = queue.pop_front() {
            match modules.get_mut(&receiver) {
                Some(m) => {
                    if let Some(new_pulse) = m.process(sender, pulse) {
                        for con in m.connections.iter() {
                            queue.push_back((receiver.clone(), con.to_string(), new_pulse));
                        }
                    }
                    if pulse && receiver == "df" {
                        m.input.iter().filter(|(_n, v)| **v).for_each(|(n, _v)| {
                            df_input.entry(n.clone()).or_insert(i);
                        });
                        if df_input.len() == m.input.len() {
                            break 'outer;
                        }
                    }
                }
                None => (),
            }
        }
        queue.push_back(("".to_string(), BROADCASTER.to_string(), false));
    }
    df_input
        .values()
        .map(|v| *v)
        .reduce(|acc, e| lcm(acc, e))
        .unwrap()
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModuleKind {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Module {
    on: bool,
    kind: ModuleKind,
    connections: Vec<String>,
    input: HashMap<String, bool>,
}

impl Module {
    fn process(&mut self, input_name: String, pulse: bool) -> Option<bool> {
        match self.kind {
            ModuleKind::FlipFlop => {
                if pulse {
                    None
                } else {
                    self.on = !self.on;
                    Some(self.on)
                }
            }
            ModuleKind::Conjunction => {
                self.input
                    .entry(input_name)
                    .and_modify(|e| *e = pulse)
                    .or_insert(pulse);
                Some(!self.input.values().all(|x| *x))
            }
            ModuleKind::Broadcaster => Some(pulse),
        }
    }
}

use std::collections::{HashMap, VecDeque};

use super::utility;
fn parse() -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    for tmp in utility::read("./src/input/day20.txt") {
        let line = tmp.strip_suffix('\r').unwrap_or(&tmp);
        if line.is_empty() {
            break;
        }
        let data = line.split("->").collect::<Vec<_>>();
        let sign = &data[0][0..1];
        let name = data[0][1..].trim().to_string();
        let connections = data[1]
            .trim()
            .split(',')
            .map(|v| v.trim().to_string())
            .collect::<Vec<_>>();
        match sign {
            "%" => modules.insert(
                name,
                Module {
                    on: false,
                    kind: ModuleKind::FlipFlop,
                    connections,
                    input: HashMap::new(),
                },
            ),
            "&" => modules.insert(
                name,
                Module {
                    on: false,
                    kind: ModuleKind::Conjunction,
                    connections,
                    input: HashMap::new(),
                },
            ),
            _ => modules.insert(
                BROADCASTER.to_string(),
                Module {
                    on: false,
                    kind: ModuleKind::Broadcaster,
                    connections,
                    input: HashMap::new(),
                },
            ),
        };
    }
    let mut input: HashMap<String, Vec<String>> = HashMap::new();
    for (name, m) in modules.iter() {
        for c in m.connections.iter() {
            if let Some(cn) = modules.get(c) {
                if cn.kind == ModuleKind::Conjunction {
                    input
                        .entry(c.clone())
                        .and_modify(|e| e.push(name.clone()))
                        .or_insert(vec![name.clone()]);
                }
            }
        }
    }
    for (k, v) in input.iter() {
        for x in v.iter() {
            modules
                .get_mut(k)
                .unwrap()
                .input
                .entry(x.to_string())
                .or_insert(false);
        }
    }
    modules
}
