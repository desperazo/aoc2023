use super::utility;

pub fn solve() -> u128 {
    let raw = utility::read("./src/input/day5.txt");
    let seeds = raw[0].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let maps = parse_map(raw);
    min_location(maps, seeds)
}

pub fn solve_2() -> u128 {
    let raw = utility::read("./src/input/day5.txt");
    let ranges = raw[0].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let mut all_locs = vec![];
    let mut maps = vec![];
    let mut tmp = vec![];

    for v in raw.iter().skip(2) {
        if v.is_empty() || !v.chars().next().unwrap().is_numeric() {
            maps.push(tmp.clone());
            tmp.clear();
            continue;
        }
        let t = v
            .trim()
            .split(' ')
            .map(|v| v.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let ori_start = t[1];
        let ori_end = t[1] + t[2] - 1;
        all_locs.push((ori_end, ori_start));
        tmp.push(t);
    }
    maps.push(tmp.clone());
    all_locs.sort_unstable();

    let all_seeds = seeds_ranges(ranges);

    let mut prev_ori_end = u128::MIN;
    for &(ori_end, ori_start) in all_locs.iter() {
        let cur_ori_start = ori_start.max(prev_ori_end);
        prev_ori_end = ori_end;

        let mut seed_pos: u128 = seed_location(ori_end, &maps);

        if all_seeds
            .iter()
            .any(|&(l, r)| seed_pos >= l && seed_pos <= r)
        {
            for v in cur_ori_start..=ori_end {
                seed_pos = seed_location(v, &maps);
                if all_seeds
                    .iter()
                    .any(|&(l, r)| seed_pos >= l && seed_pos <= r)
                {
                    return v;
                }
            }
        }
    }
    panic!("cannot find seeds location");
}

fn seeds_ranges(ranges: Vec<u128>) -> Vec<(u128, u128)> {
    let mut count = 0;
    let mut all_seeds = vec![];
    while count < ranges.len() {
        let seed_start = ranges[count];
        let seed_end = seed_start + ranges[count + 1] - 1;
        all_seeds.push((seed_start, seed_end));
        count += 2;
    }
    all_seeds
}

fn seed_location(origin: u128, maps: &[Vec<Vec<u128>>]) -> u128 {
    let mut sv = origin;
    for tmp in maps.iter().rev() {
        for m in tmp.iter() {
            let begin = m[0];
            let end = m[0] + m[2];
            if sv >= begin && sv < end {
                let d = sv - begin;
                sv = m[1] + d;
                break;
            }
        }
    }
    sv
}

fn min_location(maps: Vec<Vec<Vec<u128>>>, seeds: Vec<u128>) -> u128 {
    let mut min_location = u128::MAX;
    for s in seeds.iter() {
        let mut sv: u128 = *s;
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

fn parse_map(raw: Vec<String>) -> Vec<Vec<Vec<u128>>> {
    let mut maps = vec![];
    let mut tmp = vec![];
    for v in raw.iter().skip(2) {
        if v.is_empty() || !v.chars().next().unwrap().is_numeric() {
            maps.push(tmp.clone());
            tmp.clear();
            continue;
        }
        let t = v
            .trim()
            .split(' ')
            .map(|v| v.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        tmp.push(t);
    }
    maps.push(tmp.clone());
    maps
}
