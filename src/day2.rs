use aoc_runner_derive::aoc;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

fn line_good(line: &str) -> u64 {
    let mut acc = -2;
    for diff in line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .tuple_windows()
        .map(|(a, b)| match a - b {
            -3..=-1 => -1,
            1..=3 => 1,
            _ => 0,
        })
    {
        if diff == 0 {
            return 0;
        }
        if acc == -2 {
            acc = diff
        } else if acc != diff {
            return 0;
        }
    }
    return 1;
}

fn line_good_n(n: &[i32]) -> bool {
    let mut acc = -2;
    for diff in n.iter().tuple_windows().map(|(a, b)| match a - b {
        -3..=-1 => -1,
        1..=3 => 1,
        _ => 0,
    }) {
        if diff == 0 {
            return false;
        }
        if acc == -2 {
            acc = diff
        } else if acc != diff {
            return false;
        }
    }
    return true;
}

#[aoc(day2, part1)]
pub fn part1(s: &str) -> u64 {
    s.par_lines().map(line_good).sum()
}

#[aoc(day2, part2)]
pub fn part2(s: &str) -> u64 {
    s.par_lines()
        .map(|line| {
            let line: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            if line_good_n(&line) {
                return 1;
            }

            for i in 0..line.len() {
                let mut del_line = line.clone();
                del_line.remove(i);

                if line_good_n(&del_line) {
                    return 1;
                }
            }
            return 0;
        })
        .sum()
}
