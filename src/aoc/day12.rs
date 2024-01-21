use super::utility;

pub fn solve() -> i32 {
    let data = parse();
    data.iter()
        .map(|(txt, g)| {
            let arr = arrange(txt.to_string());
            match_count(arr, g)
        })
        .sum()
}

fn arrange(txt: String) -> Vec<String> {
    let mut set = vec![String::new()];
    let mut c = 0;
    let chrs = txt.chars().collect::<Vec<_>>();
    let mut is_question = chrs[0] == '?';
    while c < txt.len() {
        let prev = c;
        while c < txt.len() && (chrs[c] == '?' && is_question || !is_question && chrs[c] != '?') {
            c += 1;
        }

        if is_question {
            let mut tmp = vec![];
            for s in set.iter() {
                for n in new_strings(c - prev).iter() {
                    let mut v = s.clone();
                    v.push_str(n);
                    tmp.push(v);
                }
            }
            set = tmp;
        } else {
            let t2 = &txt[prev..c];
            set.iter_mut().for_each(|v| v.push_str(t2));
        }
        is_question = !is_question;
    }
    set
}

fn new_strings(size: usize) -> Vec<String> {
    if size == 0 {
        return vec![];
    }
    let mut res = vec![String::from("#"), String::from(".")];
    for _ in 1..size {
        let mut tmp = res.clone();
        for v in res.iter_mut() {
            v.push('#');
        }
        for v in tmp.iter_mut() {
            v.push('.');
        }
        res.extend(tmp);
    }
    res
}

fn match_count(set: Vec<String>, grp: &[usize]) -> i32 {
    let mut count = 0;
    for s in set.iter() {
        let tmp = s
            .split('.')
            .filter(|v| !v.is_empty())
            .collect::<Vec<&str>>();
        if tmp.len() != grp.len() {
            continue;
        }
        if tmp.iter().enumerate().all(|(i, v)| v.len() == grp[i]) {
            count += 1;
        }
    }
    count
}

fn parse() -> Vec<(String, Vec<usize>)> {
    let raw = utility::read("./src/input/day12.txt");
    raw.iter()
        .map(|s| {
            let t = s.split(' ').collect::<Vec<&str>>();
            (
                t[0].to_string(),
                t[1].split(',')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect()
}
