fn part1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let first_digit = line
            .chars()
            .find(|c| c.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit = line
            .chars()
            .rev()
            .find(|c| c.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap();
        sum += 10 * first_digit + last_digit;
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let first_digit_idx = line
            .chars()
            .map(|c| c.is_numeric())
            .collect::<Vec<bool>>()
            .iter()
            .position(|b| *b);
        let last_digit_idx = line
            .chars()
            .map(|c| c.is_numeric())
            .collect::<Vec<bool>>()
            .iter()
            .rposition(|b| *b);

        let mut first_word_digits_idx = None;
        let mut last_word_digits_idx = None;
        let mut first_word_idx = None;
        let mut last_word_idx = None;
        for (i, digit) in digits.iter().enumerate() {
            if let Some(idx) = line.find(digit) {
                match first_word_idx {
                    Some(existing) if idx < existing => {
                        first_word_digits_idx = Some(i);
                        first_word_idx = Some(idx);
                    }
                    None => {
                        first_word_digits_idx = Some(i);
                        first_word_idx = Some(idx);
                    }
                    _ => {}
                }
            }
            if let Some(idx) = line.rfind(*digit) {
                match last_word_idx {
                    Some(existing) if idx > existing => {
                        last_word_digits_idx = Some(i);
                        last_word_idx = Some(idx);
                    }
                    None => {
                        last_word_digits_idx = Some(i);
                        last_word_idx = Some(idx);
                    }
                    _ => {}
                }
            }
        }

        let first_digit = match (first_digit_idx, first_word_idx) {
            (Some(a), Some(b)) => {
                if a < b {
                    line.chars()
                        .collect::<Vec<char>>()
                        .get(a)
                        .unwrap()
                        .to_digit(10)
                        .unwrap()
                } else {
                    (first_word_digits_idx.unwrap() + 1) as u32
                }
            }
            (Some(a), None) => line
                .chars()
                .collect::<Vec<char>>()
                .get(a)
                .unwrap()
                .to_digit(10)
                .unwrap(),
            (None, Some(_)) => (first_word_digits_idx.unwrap() + 1) as u32,
            _ => 0,
        };
        let last_digit = match (last_digit_idx, last_word_idx) {
            (Some(a), Some(b)) => {
                if a > b {
                    line.chars()
                        .collect::<Vec<char>>()
                        .get(a)
                        .unwrap()
                        .to_digit(10)
                        .unwrap()
                } else {
                    (last_word_digits_idx.unwrap() + 1) as u32
                }
            }
            (Some(a), None) => line
                .chars()
                .collect::<Vec<char>>()
                .get(a)
                .unwrap()
                .to_digit(10)
                .unwrap(),
            (None, Some(_)) => (last_word_digits_idx.unwrap() + 1) as u32,
            _ => 0,
        };

        sum += 10 * first_digit + last_digit;
    }
    sum
}

fn main() {
    let input = include_str!("input/1.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
