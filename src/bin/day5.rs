fn compute_single_map(
    src: u64,
    dst_range_start: u64,
    src_range_start: u64,
    range_len: u64,
) -> Option<u64> {
    if (src_range_start..src_range_start + range_len).contains(&src) {
        Some(dst_range_start + (src - src_range_start))
    } else {
        None
    }
}

fn compute_all_maps(src: u64, maps: &[Vec<u64>]) -> u64 {
    for map in maps.iter() {
        if let Some(v) = compute_single_map(src, map[0], map[1], map[2]) {
            return v;
        }
    }
    src
}

fn compute_single_reverse_map(
    dst: u64,
    dst_range_start: u64,
    src_range_start: u64,
    range_len: u64,
) -> Option<u64> {
    if (dst_range_start..dst_range_start + range_len).contains(&dst) {
        Some(src_range_start + (dst - dst_range_start))
    } else {
        None
    }
}

fn compute_all_reverse_maps(dst: u64, maps: &[Vec<u64>]) -> u64 {
    for map in maps.iter() {
        if let Some(v) = compute_single_reverse_map(dst, map[0], map[1], map[2]) {
            return v;
        }
    }
    dst
}

fn is_in_seeds(seed: u64, seeds_raw: &[u64]) -> bool {
    for c in seeds_raw.chunks(2) {
        let start = c[0];
        let length = c[1];
        if (start..start + length).contains(&seed) {
            return true;
        }
    }
    false
}

fn part1(input: &str) -> u64 {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut seeds: Vec<u64> = sections[0].split(": ").collect::<Vec<&str>>()[1]
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    for section in sections.get(1..).unwrap() {
        let mut maps_raw: Vec<&str> = section.split('\n').filter(|s| !s.is_empty()).collect();
        maps_raw.remove(0);
        let maps: Vec<Vec<u64>> = maps_raw
            .iter()
            .map(|map_raw| map_raw.split(' ').map(|x| x.parse().unwrap()).collect())
            .collect();
        seeds = seeds
            .iter()
            .map(|seed| compute_all_maps(*seed, &maps))
            .collect();
    }
    *seeds.iter().min().unwrap()
}

fn part2(input: &str) -> u64 {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let seeds_raw: Vec<u64> = sections[0].split(": ").collect::<Vec<&str>>()[1]
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut all_maps = Vec::new();
    for section in sections.get(1..).unwrap().iter().rev() {
        let mut maps_raw: Vec<&str> = section.split('\n').filter(|s| !s.is_empty()).collect();
        maps_raw.remove(0);
        let maps: Vec<Vec<u64>> = maps_raw
            .iter()
            .map(|map_raw| map_raw.split(' ').map(|x| x.parse().unwrap()).collect())
            .collect();
        all_maps.push(maps);
    }
    let mut curr = 0;
    loop {
        let mut x = curr;
        for maps in all_maps.iter() {
            x = compute_all_reverse_maps(x, maps);
        }
        if is_in_seeds(x, &seeds_raw) {
            break;
        } else {
            curr += 1;
        }
    }
    curr
}

fn main() {
    let input = include_str!("input/5.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
