use std::collections::HashMap;

use super::utility;

pub fn solve() -> i32 {
    let raw = utility::read("./src/input/day7.txt");
    let mut cards = vec![];
    for v in raw {
        cards.push(Card::new(&v));
    }
    cards.sort_unstable();
    cards
        .iter()
        .enumerate()
        .map(|(i, c)| (i as i32 + 1) * c.bits)
        .sum()
}

#[derive(PartialEq, Eq)]
struct Card {
    values: Vec<char>,
    kind: CombindKind,
    bits: i32,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut ord = (self.kind as i32).cmp(&(other.kind as i32));
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }

        let mut index = 0;
        while ord == std::cmp::Ordering::Equal && index < 5 {
            ord = self.value_of(index).cmp(&other.value_of(index));
            index += 1;
        }
        ord
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {
    pub fn new(data: &str) -> Self {
        let v = data.split(' ').collect::<Vec<&str>>();
        let values = v[0].chars().collect::<Vec<char>>();
        let bits = v[1].parse::<i32>().unwrap();
        let kind: CombindKind = CombindKind::new(&values);
        Self { values, kind, bits }
    }

    pub fn value_of(&self, i: usize) -> i32 {
        match self.values[i] {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => panic!("value of out range"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum CombindKind {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl CombindKind {
    fn new(values: &[char]) -> Self {
        let mut maps = HashMap::new();
        values.iter().for_each(|x| {
            maps.entry(x).and_modify(|e| *e += 1).or_insert(1);
        });
        match maps.len() {
            1 => CombindKind::FiveOfKind,
            2 if maps.values().any(|&x| x == 1) => CombindKind::FourOfKind,
            2 => CombindKind::FullHouse,
            3 if maps.values().any(|&x| x == 3) => CombindKind::ThreeOfKind,
            3 => CombindKind::TwoPair,
            4 => CombindKind::OnePair,
            _ => Self::HighCard,
        }
    }
}
