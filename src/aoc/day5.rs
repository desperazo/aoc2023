use super::utility;

pub fn solve() -> u128 {
    let raw = utility::read("./src/input/day5.txt");
    let seeds = raw[0].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let mut maps = vec![];
    let mut tmp = vec![];
    for v in raw.iter().skip(2) {
        if v.is_empty() || !v.chars().next().unwrap().is_numeric() {
            maps.push(tmp.clone());
            tmp.clear();
            continue;
        }
        let t = v
            .split(' ')
            .map(|v| v.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        tmp.push(t);
    }
    maps.push(tmp.clone());
    let mut min_location = u128::MAX;
    for s in seeds.iter() {
        let mut sv = *s;
        for tmp in maps.iter() {
            for m in tmp.iter() {
                let begin = m[1];
                let end = m[1] + m[2];
                if sv >= begin && sv < end {
                    let d = sv - begin;
                    sv = m[0] + d;
                    break;
                }
            }
        }
        min_location = min_location.min(sv);
    }
    min_location
}
