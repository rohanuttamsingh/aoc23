use std::collections::HashSet;

fn part1(input: &str) -> u32 {
    let numbers_strs: Vec<Vec<&str>> = input
        .lines()
        .map(|l| {
            l.split(": ").collect::<Vec<&str>>()[1]
                .split(" | ")
                .collect()
        })
        .collect();
    let numbers: Vec<(Vec<u32>, HashSet<u32>)> = numbers_strs
        .iter()
        .map(|n| {
            (
                n[0].split(|c: char| c.is_whitespace())
                    .filter(|&s| !s.is_empty())
                    .map(|x| x.parse().unwrap())
                    .collect(),
                n[1].split(|c: char| c.is_whitespace())
                    .filter(|&s| !s.is_empty())
                    .map(|x| x.parse().unwrap())
                    .collect(),
            )
        })
        .collect();
    numbers
        .iter()
        .map(|(v, s)| {
            let count = v.iter().fold(0, |a, b| a + (s.contains(b) as u32));
            if count == 0 {
                0
            } else {
                1 << (count - 1)
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let numbers_strs: Vec<Vec<&str>> = input
        .lines()
        .map(|l| {
            l.split(": ").collect::<Vec<&str>>()[1]
                .split(" | ")
                .collect()
        })
        .collect();
    let mut cards: Vec<(u32, Vec<u32>, HashSet<u32>)> = numbers_strs
        .iter()
        .map(|n| {
            (
                1,
                n[0].split(|c: char| c.is_whitespace())
                    .filter(|&s| !s.is_empty())
                    .map(|x| x.parse().unwrap())
                    .collect(),
                n[1].split(|c: char| c.is_whitespace())
                    .filter(|&s| !s.is_empty())
                    .map(|x| x.parse().unwrap())
                    .collect(),
            )
        })
        .collect();
    let len = cards.len();
    for i in 0..len {
        let (card_count, mine, winning) = &cards[i];
        let card_count = *card_count;
        let winning_count = mine
            .iter()
            .fold(0, |a, b| a + (winning.contains(b) as usize));
        for (winning_card_count, _, _) in cards.iter_mut().take(i + winning_count + 1).skip(i + 1) {
            *winning_card_count += card_count;
        }
    }
    cards.iter().map(|(card_count, _, _)| card_count).sum()
}

fn main() {
    let input = include_str!("input/4.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
