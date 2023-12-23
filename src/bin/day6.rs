fn parse_line(line: &str) -> Vec<u64> {
    line.split(": ").collect::<Vec<&str>>()[1]
        .split(|c: char| c.is_whitespace())
        .filter(|s| !s.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_line_no_spacing(line: &str) -> u64 {
    line.split(": ").collect::<Vec<&str>>()[1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn min_to_beat(time: u64, distance: u64) -> u64 {
    // x * (time - x) >= distance
    for x in 1..time {
        if x * (time - x) > distance {
            return x;
        }
    }
    0
}

fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    let times = parse_line(lines[0]);
    let distances = parse_line(lines[1]);
    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| {
            let min = min_to_beat(time, distance);
            time - 2 * min + 1
        })
        .product()
}

fn part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    let time = parse_line_no_spacing(lines[0]);
    let distance = parse_line_no_spacing(lines[1]);
    let min = min_to_beat(time, distance);
    time - 2 * min + 1
}

fn main() {
    let input = include_str!("input/6.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
