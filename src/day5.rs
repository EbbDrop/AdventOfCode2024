use std::cmp::Ordering;

use aoc_runner_derive::aoc;

fn to_index(a: u8, b: u8) -> u8 {
    a.wrapping_mul(10)
        .wrapping_add(b)
        .wrapping_sub(b'0'.wrapping_mul(10).wrapping_add(b'0'))
        .wrapping_sub(9)
}

#[derive(Clone, Debug)]
struct Graph {
    connections: [Vec<u8>; 91],
}

impl Graph {
    fn new() -> Self {
        Self {
            connections: [const { Vec::new() }; 91],
        }
    }

    fn add_relation(&mut self, r: &[u8]) {
        let to = to_index(r[0], r[1]);
        let from = to_index(r[3], r[4]);
        self.connections[from as usize].push(to);
    }
}

#[derive(Clone, Debug)]
struct Marks {
    marks: [bool; 91],
}

impl Marks {
    fn new() -> Self {
        Self { marks: [false; 91] }
    }

    fn reset(&mut self) {
        self.marks.fill(false);
    }

    // Returns true if that index was already marked, marks all predecesors
    fn mark(&mut self, graph: &Graph, id: u8) -> bool {
        if self.marks[id as usize] {
            return true;
        }

        self.marks[id as usize] = true;
        for con in &graph.connections[id as usize] {
            self.marks[*con as usize] = true;
        }
        return false;
    }
}

#[aoc(day5, part1)]
pub fn part1(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut graph = Graph::new();

    let mut i = 0;
    loop {
        if s[i + 2] != b'|' {
            break;
        }
        graph.add_relation(&s[i..i + 5]);
        i += 6;
    }
    // Skip the newline
    i += 1;

    let mut sum = 0;
    let mut marks = Marks::new();

    'line: for line in s[i..].split_inclusive(|b| *b == b'\n') {
        marks.reset();
        let mut v = Vec::with_capacity(23);
        for num in line.chunks_exact(3) {
            let num = to_index(num[0], num[1]);
            v.push(num);

            if marks.mark(&graph, num) {
                continue 'line;
            }
        }

        sum += v[v.len() / 2] as u32 + 9;
    }

    sum
}

#[aoc(day5, part2)]
pub fn part2(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut graph = Graph::new();

    let mut i = 0;
    loop {
        if s[i + 2] != b'|' {
            break;
        }
        graph.add_relation(&s[i..i + 5]);
        i += 6;
    }
    // Skip the newline
    i += 1;

    let mut sum = 0;
    let mut marks = Marks::new();

    for line in s[i..].split_inclusive(|b| *b == b'\n') {
        marks.reset();
        let mut v = Vec::with_capacity(23);

        let mut good = true;

        for num in line.chunks(3) {
            let num = to_index(num[0], num[1]);
            v.push(num);

            if good && marks.mark(&graph, num) {
                good = false;
            }
        }
        if !good {
            let k = v.len() / 2;
            sum += *v
                .select_nth_unstable_by(k, |a, b| {
                    if a == b {
                        Ordering::Equal
                    } else if graph.connections[*a as usize].contains(b) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                })
                .1 as u32
                + 9;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 143);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 123);
    }
}
