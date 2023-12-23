use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq)]
struct Cards {
    cards: Vec<char>,
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let order = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        for (&a, &b) in self.cards.iter().zip(other.cards.iter()) {
            if a != b {
                let a_position = order.iter().position(|&c| c == a);
                let b_position = order.iter().position(|&c| c == b);
                if a_position < b_position {
                    return Some(Ordering::Less);
                } else {
                    return Some(Ordering::Greater);
                }
            }
        }
        Some(Ordering::Equal)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Hand {
    High(Cards),
    Pair(Cards),
    TwoPair(Cards),
    Three(Cards),
    FullHouse(Cards),
    Four(Cards),
    Five(Cards),
}

impl Hand {
    fn new(s: &str) -> Self {
        let cards = Cards {
            cards: s.chars().collect(),
        };
        let mut counts = HashMap::new();
        for card in cards.cards.iter() {
            counts
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        match counts.values().max().unwrap() {
            5 => Self::Five(cards),
            4 => Self::Four(cards),
            3 => {
                if counts.len() == 2 {
                    Self::FullHouse(cards)
                } else {
                    Self::Three(cards)
                }
            }
            2 => {
                if counts.len() == 3 {
                    Self::TwoPair(cards)
                } else {
                    Self::Pair(cards)
                }
            }
            1 => Self::High(cards),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct JokersCards {
    cards: Vec<char>,
}

impl PartialOrd for JokersCards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let order = [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];
        for (&a, &b) in self.cards.iter().zip(other.cards.iter()) {
            if a != b {
                let a_position = order.iter().position(|&c| c == a);
                let b_position = order.iter().position(|&c| c == b);
                if a_position < b_position {
                    return Some(Ordering::Less);
                } else {
                    return Some(Ordering::Greater);
                }
            }
        }
        Some(Ordering::Equal)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum JokersHand {
    High(JokersCards),
    Pair(JokersCards),
    TwoPair(JokersCards),
    Three(JokersCards),
    FullHouse(JokersCards),
    Four(JokersCards),
    Five(JokersCards),
}

impl JokersHand {
    fn new(s: &str) -> Self {
        let cards = JokersCards {
            cards: s.chars().collect(),
        };
        let mut counts = HashMap::new();
        for card in cards.cards.iter() {
            if *card != 'J' {
                counts
                    .entry(card)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
        let num_jokers = cards.cards.iter().filter(|&c| *c == 'J').count();
        if num_jokers == 5 {
            counts.insert(&'J', 5);
        } else {
            *counts.values_mut().max().unwrap() += num_jokers;
        }
        match counts.values().max().unwrap() {
            5 => Self::Five(cards),
            4 => Self::Four(cards),
            3 => {
                if counts.len() == 2 {
                    Self::FullHouse(cards)
                } else {
                    Self::Three(cards)
                }
            }
            2 => {
                if counts.len() == 3 {
                    Self::TwoPair(cards)
                } else {
                    Self::Pair(cards)
                }
            }
            1 => Self::High(cards),
            _ => unreachable!(),
        }
    }
}

fn part1(input: &str) -> u32 {
    let raw_hands_and_bids: Vec<(&str, u32)> = input
        .lines()
        .map(|l| {
            let s: Vec<&str> = l.split(' ').collect();
            let hand = s[0];
            let bid = s[1].parse().unwrap();
            (hand, bid)
        })
        .collect();
    let mut hands_and_bids: Vec<(Hand, u32)> = raw_hands_and_bids
        .iter()
        .map(|(raw_hand, bid)| (Hand::new(raw_hand), *bid))
        .collect();
    hands_and_bids.sort_by(|(hand1, _), (hand2, _)| hand1.partial_cmp(hand2).unwrap());
    hands_and_bids
        .iter()
        .enumerate()
        .map(|(i, &(_, bid))| ((i + 1) as u32) * bid)
        .sum()
}

fn part2(input: &str) -> u32 {
    let raw_hands_and_bids: Vec<(&str, u32)> = input
        .lines()
        .map(|l| {
            let s: Vec<&str> = l.split(' ').collect();
            let hand = s[0];
            let bid = s[1].parse().unwrap();
            (hand, bid)
        })
        .collect();
    let mut hands_and_bids: Vec<(JokersHand, u32)> = raw_hands_and_bids
        .iter()
        .map(|(raw_hand, bid)| (JokersHand::new(raw_hand), *bid))
        .collect();
    hands_and_bids.sort_by(|(hand1, _), (hand2, _)| hand1.partial_cmp(hand2).unwrap());
    hands_and_bids
        .iter()
        .enumerate()
        .map(|(i, &(_, bid))| ((i + 1) as u32) * bid)
        .sum()
}

fn main() {
    let input = include_str!("input/7.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
