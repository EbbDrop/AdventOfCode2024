use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(s: &str) -> u64 {
    let mut a: Vec<i64> = Vec::new();
    let mut b: Vec<i64> = Vec::new();
    for line in s.lines() {
        let mut parts = line.split_whitespace();

        a.push(parts.next().unwrap().parse().unwrap());
        b.push(parts.next().unwrap().parse().unwrap());
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut sum = 0;
    for (a, b) in a.iter().zip(&b) {
        sum += a.abs_diff(*b);
    }

    sum
}

#[aoc(day1, part2)]
pub fn part2(s: &str) -> u64 {
    let mut a: Vec<i64> = Vec::new();
    let mut b: HashMap<i64, u64> = HashMap::new();
    for line in s.lines() {
        let mut parts = line.split_whitespace();

        a.push(parts.next().unwrap().parse().unwrap());
        let b_num: i64 = parts.next().unwrap().parse().unwrap();

        *b.entry(b_num).or_insert(0) += 1;
    }

    let mut sum = 0;

    for a in a {
        sum += a as u64 * b.get(&a).cloned().unwrap_or(0);
    }

    sum
}
