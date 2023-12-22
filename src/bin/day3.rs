use std::collections::HashMap;

fn get_adjacent_indices(index: (usize, usize), rows: i32, cols: i32) -> Vec<(usize, usize)> {
    let index = (index.0 as i32, index.1 as i32);
    let dirs: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut indices = Vec::new();
    for (i, j) in dirs {
        let new_row = index.0 + i;
        let new_col = index.1 + j;
        if (0..rows).contains(&new_row) && (0..cols).contains(&new_col) {
            indices.push((new_row as usize, new_col as usize));
        }
    }
    indices
}

fn part1(input: &str) -> u32 {
    let mut res = 0;

    let bytes = input.as_bytes();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    for i in 0..rows {
        let mut num = None;
        let mut adjacent = false;

        for (j, byte) in bytes
            .iter()
            .take((i + 1) * (cols + 1))
            .skip(i * (cols + 1))
            .enumerate()
        {
            let c = *byte as char;
            if let Some(d) = c.to_digit(10) {
                if let Some(n) = num {
                    num = Some(10 * n + d);
                } else {
                    num = Some(d);
                }
                let adjacent_indices = get_adjacent_indices((i, j), rows as i32, cols as i32);
                adjacent = adjacent
                    || adjacent_indices.iter().fold(false, |a, (x, y)| {
                        let byte = bytes[x * (cols + 1) + y] as char;
                        a || (byte != '.' && !byte.is_numeric())
                    });
            } else {
                if let Some(n) = num {
                    if adjacent {
                        res += n;
                    }
                }
                num = None;
                adjacent = false;
            }
        }
    }

    res
}

fn part2(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut gear_ratios = HashMap::new();

    for i in 0..rows {
        let mut num = None;
        let mut gear_index = None;

        for (j, byte) in bytes
            .iter()
            .take((i + 1) * (cols + 1))
            .skip(i * (cols + 1))
            .enumerate()
        {
            let c = *byte as char;
            if let Some(d) = c.to_digit(10) {
                if let Some(n) = num {
                    num = Some(10 * n + d);
                } else {
                    num = Some(d);
                }
                let adjacent_indices = get_adjacent_indices((i, j), rows as i32, cols as i32);
                if gear_index.is_none() {
                    gear_index = adjacent_indices.iter().fold(None, |a, (x, y)| {
                        if a.is_some() {
                            return a;
                        }
                        let byte = bytes[x * (cols + 1) + y] as char;
                        if byte == '*' {
                            Some((*x, *y))
                        } else {
                            None
                        }
                    })
                }
            } else {
                if let Some(n) = num {
                    if let Some((row, col)) = gear_index {
                        gear_ratios
                            .entry((row, col))
                            .and_modify(|(c, v)| {
                                *c += 1;
                                *v *= n;
                            })
                            .or_insert((1, n));
                    }
                }
                num = None;
                gear_index = None;
            }
        }
    }

    gear_ratios
        .values()
        .fold(0, |a, (c, v)| if *c == 2 { a + v } else { a })
}

fn main() {
    let input = include_str!("input/3.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
