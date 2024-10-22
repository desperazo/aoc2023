use itertools::Itertools;

pub fn solve() -> usize {
    let input = utility::read_string("./src/input/day25.txt");
    let graph = parse(&input);

    graph
        .keys()
        .tuple_combinations()
        .find_map(|(from, to)| {
            let mut copy = graph.clone();

            // delete 3 routes starting at "from" and ending in "to"
            for _ in 0..3 {
                delete_route(&mut copy, from, to)
            }

            // if "from" and "to" were in 2 halves, the connecting edges were deleted and "to" will not be reachable starting at "from"
            // if this is the case, count how many nodes were reachable, this is one half. If not, move on to an other "from"-"to" pair
            let half1 = reachable_nodes(&copy, from, to)?;
            let half2 = copy.len() - half1;

            Some(half1 * half2)
        })
        .unwrap()
}

fn delete_route<'a>(graph: &mut HashMap<&'a str, HashSet<&'a str>>, from: &'a str, to: &'a str) {
    let mut q = VecDeque::from([from]);
    let mut seen = HashSet::from([from]);
    let mut prev_map = HashMap::new();

    'outer: while let Some(node) = q.pop_front() {
        for &neighbour in graph.get(node).unwrap() {
            if seen.insert(neighbour) {
                q.push_back(neighbour);
                prev_map.insert(neighbour, node);
                if neighbour == to {
                    break 'outer;
                }
            }
        }
    }

    // delete every edge on the path "from"-"to"
    // if "from" and "to" were in the 2 halves, one of the connecting edges is guaranteed to be deleted this way
    let mut curr = to;
    while curr != from {
        let prev = prev_map.get(curr).unwrap();
        graph.entry(curr).or_default().remove(prev);
        graph.entry(prev).or_default().remove(curr);
        curr = prev;
    }
}

fn reachable_nodes(graph: &HashMap<&str, HashSet<&str>>, from: &str, to: &str) -> Option<usize> {
    let mut q = VecDeque::from([from]);
    let mut seen = HashSet::from([from]);

    while let Some(node) = q.pop_front() {
        for neighbour in graph.get(node).unwrap() {
            if *neighbour == to {
                // the graph was not cut in 2
                return None;
            }
            if seen.insert(neighbour) {
                q.push_back(neighbour);
            }
        }
    }

    Some(seen.len())
}

use std::collections::{HashMap, HashSet, VecDeque};

use super::utility;
fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .fold(HashMap::new(), |mut acc, (from, rhs)| {
            for to in rhs.split_whitespace() {
                acc.entry(from).or_default().insert(to);
                acc.entry(to).or_default().insert(from);
            }
            acc
        })
}
