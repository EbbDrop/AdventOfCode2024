use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day2, part1)]
fn part1(s: &str) -> u64 {
    let mut good_lines = 0;
    for line in s.lines() {
        let ok = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .tuple_windows()
            .map(|(a, b)| match a - b {
                -3..=-1 => -1,
                1..=3 => 1,
                _ => 0,
            })
            .reduce(|acc, e| {
                if acc == 0 {
                    0
                } else if acc == e {
                    acc
                } else {
                    0
                }
            })
            .unwrap_or_default();

        if ok != 0 {
            good_lines += 1;
        }
    }

    good_lines
}

#[aoc(day2, part2)]
fn part2(s: &str) -> u64 {
    let mut good_lines = 0;

    for line in s.lines() {
        let line: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let ok = line
            .iter()
            .tuple_windows()
            .map(|(a, b)| match a - b {
                -3..=-1 => -1,
                1..=3 => 1,
                _ => 0,
            })
            .reduce(|acc, e| {
                if acc == 0 {
                    0
                } else if acc == e {
                    acc
                } else {
                    0
                }
            })
            .unwrap_or_default()
            != 0;
        if ok {
            good_lines += 1;
            continue;
        }

        for i in 0..line.len() {
            let mut del_line = line.clone();
            del_line.remove(i);

            let ok = del_line
                .iter()
                .tuple_windows()
                .map(|(a, b)| match a - b {
                    -3..=-1 => -1,
                    1..=3 => 1,
                    _ => 0,
                })
                .reduce(|acc, e| {
                    if acc == 0 {
                        0
                    } else if acc == e {
                        acc
                    } else {
                        0
                    }
                })
                .unwrap_or_default()
                != 0;
            if ok {
                good_lines += 1;
                break;
            }
        }
    }

    good_lines
}
