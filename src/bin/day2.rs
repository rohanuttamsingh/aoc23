use std::cmp::max;

#[derive(Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

fn cube_count_parser(cube_count: &str) -> u32 {
    cube_count
        .split(' ')
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .parse()
        .unwrap()
}

fn compute_max_set(line: &str) -> Set {
    let set_strs = line
        .split(": ")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split("; ")
        .collect::<Vec<&str>>();
    let mut max_set = Set::default();
    for set_str in set_strs {
        let mut set = Set::default();
        for cube_count in set_str.split(", ").collect::<Vec<&str>>() {
            if cube_count.ends_with("red") {
                set.red = cube_count_parser(cube_count);
            } else if cube_count.ends_with("blue") {
                set.blue = cube_count_parser(cube_count);
            } else {
                set.green = cube_count_parser(cube_count);
            }
        }
        max_set.red = max(max_set.red, set.red);
        max_set.green = max(max_set.green, set.green);
        max_set.blue = max(max_set.blue, set.blue);
    }
    max_set
}

fn part1(input: &str) -> u32 {
    let mut res = 0;
    for (i, line) in input.lines().enumerate() {
        let max_set = compute_max_set(line);
        if max_set.red <= 12 && max_set.green <= 13 && max_set.blue <= 14 {
            res += (i as u32) + 1;
        }
    }
    res
}

fn part2(input: &str) -> u32 {
    let mut res = 0;
    for line in input.lines() {
        let max_set = compute_max_set(line);
        let power = max_set.red * max_set.green * max_set.blue;
        res += power;
    }
    res
}

fn main() {
    let input = include_str!("input/2.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
