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

const HAS_START: u16 = 1 << 15;
#[derive(Debug, Copy, Clone)]
struct NfaTrans(u16);

impl NfaTrans {
    #[inline(always)]
    const fn empty() -> Self {
        NfaTrans(0)
    }

    #[inline(always)]
    fn has_start(&self) -> bool {
        self.0 & HAS_START != 0
    }

    #[inline(always)]
    fn get_next(&self) -> u16 {
        self.0 & (!HAS_START)
    }

    #[inline(always)]
    fn add_start(&mut self) {
        self.0 |= HAS_START;
    }

    #[inline(always)]
    fn add_or_foolow(&mut self, new_nfa_node: impl FnOnce() -> u16) -> u16 {
        if self.get_next() == 0 {
            let new = new_nfa_node();
            self.0 |= new;
            new
        } else {
            self.get_next()
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
    nfa.push([NfaTrans::empty(); 5]).unwrap();

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
                let new_nfa_node = nfa.len() as u16;
                nfa.push([NfaTrans::empty(); 5]).unwrap();
                new_nfa_node
            });
            nfa[nfa_node][color] = nfa_trans;

            nfa_node = next_nfa_node as usize;
            i += 1;
        }
    }

    let mut sum = 0;

    let mut states1 = &mut (true, heapless::Vec::<u16, NFA_SIZE>::new());
    let mut states2 = &mut (false, heapless::Vec::<u16, NFA_SIZE>::new());

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

            states2.0 |= next.has_start();
            if next.get_next() != 0 {
                states2.1.push(next.get_next()).unwrap();
            }
        }
        for s in states1.1.iter() {
            let next = nfa[*s as usize][color];

            states2.0 |= next.has_start();
            if next.get_next() != 0 {
                states2.1.push(next.get_next()).unwrap();
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
    nfa.push([NfaTrans::empty(); 5]).unwrap();

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
                let new_nfa_node = nfa.len() as u16;
                nfa.push([NfaTrans::empty(); 5]).unwrap();
                new_nfa_node
            });
            nfa[nfa_node][color] = nfa_trans;

            nfa_node = next_nfa_node as usize;
            i += 1;
        }
    }

    let mut sum = 0;

    let mut states1 = &mut (1u64, heapless::Vec::<(u16, u64), NFA_SIZE>::new());
    let mut states2 = &mut (0u64, heapless::Vec::<(u16, u64), NFA_SIZE>::new());

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

            if next.has_start() {
                states2.0 += states1.0;
            }
            if next.get_next() != 0 {
                states2.1.push((next.get_next(), states1.0)).unwrap();
            }
        }
        for (s, amount) in states1.1.iter() {
            let next = nfa[*s as usize][color];

            if next.has_start() {
                states2.0 += amount;
            }
            if next.get_next() != 0 {
                states2.1.push((next.get_next(), *amount)).unwrap();
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
