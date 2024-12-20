use std::hint::assert_unchecked;

use aoc_runner_derive::aoc;

const ID_LUT: [usize; 127] = const {
    let mut lut = [0; 127];

    lut[b'b' as usize] = 0;
    lut[b'g' as usize] = 1;
    lut[b'r' as usize] = 2;
    lut[b'u' as usize] = 3;
    lut[b'w' as usize] = 4;

    lut
};

// Transforms b, g, r, u ,w to 0, 1, 4, 3, 5 NOT

// Transforms b, g, r, u ,w to 0, 1, 2, 3, 4
#[inline(always)]
fn to_idx(i: u8) -> usize {
    unsafe {
        let r = *ID_LUT.get_unchecked(i as usize);
        assert_unchecked(r < 5);
        r
    }
}

#[derive(Debug, Copy, Clone)]
enum NfaTrans {
    None,
    Start,
    Next(usize),
    Both(usize),
}

impl NfaTrans {
    fn add_start(&mut self) {
        match *self {
            NfaTrans::None => *self = NfaTrans::Start,
            NfaTrans::Next(n) => *self = NfaTrans::Both(n),
            _ => {}
        }
    }

    fn add_or_foolow(&mut self, new_nfa_node: impl FnOnce() -> usize) -> usize {
        match *self {
            NfaTrans::None => {
                let new = new_nfa_node();
                *self = NfaTrans::Next(new);
                new
            }
            NfaTrans::Start => {
                let new = new_nfa_node();
                *self = NfaTrans::Both(new);
                new
            }
            NfaTrans::Next(n) => n,
            NfaTrans::Both(n) => n,
        }
    }
}

const NFA_SIZE: usize = 1024;

#[aoc(day19, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part1(s.as_bytes())
    }
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part1(s: &[u8]) -> u64 {
    let mut nfa = heapless::Vec::<[NfaTrans; 5], NFA_SIZE>::new();
    nfa.push([NfaTrans::None; 5]).unwrap();

    let mut i = 0;
    let mut nfa_node = 0;
    loop {
        let color = to_idx(s[i]);
        let next = s[i + 1];

        if next == b',' || next == b'\n' {
            nfa[nfa_node][color].add_start();

            nfa_node = 0;
            i += 3;
            if next == b'\n' {
                break;
            }
        } else {
            let mut nfa_trans = nfa[nfa_node][color];
            let next_nfa_node = nfa_trans.add_or_foolow(|| {
                let new_nfa_node = nfa.len();
                nfa.push([NfaTrans::None; 5]).unwrap();
                new_nfa_node
            });
            nfa[nfa_node][color] = nfa_trans;

            nfa_node = next_nfa_node;
            i += 1;
        }
    }

    let mut sum = 0;

    let mut states1 = &mut (true, heapless::Vec::<usize, NFA_SIZE>::new());
    let mut states2 = &mut (false, heapless::Vec::<usize, NFA_SIZE>::new());

    while i < s.len() {
        if s[i] == b'\n' {
            if states1.0 {
                sum += 1;
            }
            states1.1.clear();
            states1.0 = true;
            i += 1;
            continue;
        }
        let color = to_idx(s[i]);

        states2.1.clear();
        states2.0 = false;

        if states1.0 {
            let next = nfa[0][color];
            match next {
                NfaTrans::None => {}
                NfaTrans::Start => {
                    states2.0 = true;
                }
                NfaTrans::Next(n) => {
                    states2.1.push(n).unwrap();
                }
                NfaTrans::Both(n) => {
                    states2.0 = true;
                    states2.1.push(n).unwrap();
                }
            }
        }
        for s in states1.1.iter() {
            let next = nfa[*s][color];
            match next {
                NfaTrans::None => {}
                NfaTrans::Start => {
                    states2.0 = true;
                }
                NfaTrans::Next(n) => {
                    states2.1.push(n).unwrap();
                }
                NfaTrans::Both(n) => {
                    states2.0 = true;
                    states2.1.push(n).unwrap();
                }
            }
        }
        std::mem::swap(&mut states2, &mut states1);

        if states1.0 == false && states1.1.is_empty() {
            while i < s.len() && s[i] != b'\n' {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    sum
}

#[aoc(day19, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part2(s.as_bytes())
    }
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part2(s: &[u8]) -> u64 {
    let mut nfa = heapless::Vec::<[NfaTrans; 5], NFA_SIZE>::new();
    nfa.push([NfaTrans::None; 5]).unwrap();

    let mut i = 0;
    let mut nfa_node = 0;
    loop {
        let color = to_idx(s[i]);
        let next = s[i + 1];

        if next == b',' || next == b'\n' {
            nfa[nfa_node][color].add_start();

            nfa_node = 0;
            i += 3;
            if next == b'\n' {
                break;
            }
        } else {
            let mut nfa_trans = nfa[nfa_node][color];
            let next_nfa_node = nfa_trans.add_or_foolow(|| {
                let new_nfa_node = nfa.len();
                nfa.push([NfaTrans::None; 5]).unwrap();
                new_nfa_node
            });
            nfa[nfa_node][color] = nfa_trans;

            nfa_node = next_nfa_node;
            i += 1;
        }
    }

    let mut sum = 0;

    let mut states1 = &mut (1, heapless::Vec::<(usize, u64), NFA_SIZE>::new());
    let mut states2 = &mut (0, heapless::Vec::<(usize, u64), NFA_SIZE>::new());

    while i < s.len() {
        if s[i] == b'\n' {
            sum += states1.0;
            states1.1.clear();
            states1.0 = 1;
            i += 1;
            continue;
        }
        let color = to_idx(s[i]);

        states2.1.clear();
        states2.0 = 0;

        if states1.0 > 0 {
            let next = nfa[0][color];
            match next {
                NfaTrans::None => {}
                NfaTrans::Start => {
                    states2.0 += states1.0;
                }
                NfaTrans::Next(n) => {
                    states2.1.push((n, states1.0)).unwrap();
                }
                NfaTrans::Both(n) => {
                    states2.0 += states1.0;
                    states2.1.push((n, states1.0)).unwrap();
                }
            }
        }
        for (s, amount) in states1.1.iter() {
            let next = nfa[*s][color];
            match next {
                NfaTrans::None => {}
                NfaTrans::Start => {
                    states2.0 += *amount;
                }
                NfaTrans::Next(n) => {
                    states2.1.push((n, *amount)).unwrap();
                }
                NfaTrans::Both(n) => {
                    states2.0 += *amount;
                    states2.1.push((n, *amount)).unwrap();
                }
            }
        }

        std::mem::swap(&mut states2, &mut states1);
        if states1.0 == 0 && states1.1.is_empty() {
            while i < s.len() && s[i] != b'\n' {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 6);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 16);
    }
}
