use std::cmp::Ordering;

use aoc_runner_derive::aoc;
use tinyvec::ArrayVec;

fn to_index(a: u8, b: u8) -> u8 {
    a.wrapping_mul(10)
        .wrapping_add(b)
        .wrapping_sub(b'0'.wrapping_mul(10).wrapping_add(b'0'))
        .wrapping_sub(10)
}

#[derive(Clone, Debug)]
struct Graph {
    connections: [u128; 90],
}

impl Graph {
    fn new() -> Self {
        Self {
            connections: const {
                let mut indexes = [0; 90];
                let mut i = 0;
                while i < 90 {
                    indexes[i] |= 1 << i;
                    i += 1;
                }
                indexes
            },
        }
    }

    fn add_relation(&mut self, r: &[u8]) {
        let to = to_index(r[0], r[1]) as usize;
        let from = to_index(r[3], r[4]) as usize;

        self.connections[from] |= 1 << to;
    }
}

#[derive(Clone, Debug)]
struct Marks {
    marks: u128,
}

impl Marks {
    fn new() -> Self {
        Self { marks: 0 }
    }

    fn reset(&mut self) {
        self.marks = 0;
    }

    // Returns true if that index was already marked, marks all predecesors
    fn mark(&mut self, graph: &Graph, id: u8) -> bool {
        if (self.marks & 1 << id) != 0 {
            return true;
        }

        self.marks |= graph.connections[id as usize];
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
    let mut v = ArrayVec::<[u8; 23]>::new();
    while i + 2 < s.len() {
        let num = to_index(s[i], s[i + 1]);
        v.push(num);
        if marks.mark(&graph, num) {
            if let Some(new_i) = memchr::memchr(b'\n', &s[i..]) {
                i += new_i + 1;
                marks.reset();
                v.clear();
                continue;
            } else {
                break;
            }
        }
        if s[i + 2] == b'\n' {
            sum += v[v.len() / 2] as u32 + 10;
            marks.reset();
            v.clear();
        }
        i += 3;
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
    let mut v = ArrayVec::<[u8; 23]>::new();
    let mut good = true;

    while i + 2 < s.len() {
        let num = to_index(s[i], s[i + 1]);
        v.push(num);
        if good && marks.mark(&graph, num) {
            good = false;
        }
        if s[i + 2] == b'\n' {
            if !good {
                let k = v.len() / 2;
                sum += *v
                    .select_nth_unstable_by(k, |a, b| {
                        if (graph.connections[*a as usize] & 1 << *b) != 0 {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    })
                    .1 as u32
                    + 10;
            }
            good = true;
            marks.reset();
            v.clear();
        }
        i += 3;
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
97,13,75,29,47
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 143);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 123);
    }
}
