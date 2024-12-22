use core::str;

use aoc_runner_derive::aoc;

const MAX: u32 = 16777216;

// static LUT_P1: [u32; MAX as usize] =
//     unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day22.bin"))) };

#[aoc(day22, part1)]
pub fn part1(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut sum = 0;
    let mut i = 0;
    unsafe {
        while i < s.len() {
            #[cfg(not(test))]
            let mut sn = (*s.get_unchecked(i + 0) as u32) * 100000
                + (*s.get_unchecked(i + 1) as u32) * 10000
                + (*s.get_unchecked(i + 2) as u32) * 1000
                + (*s.get_unchecked(i + 3) as u32) * 100
                + (*s.get_unchecked(i + 4) as u32) * 10
                + (*s.get_unchecked(i + 5) as u32) * 1
                - (b'0' as u32 * 111_111);
            #[cfg(not(test))]
            {
                i += 6;
            }

            #[cfg(test)]
            let mut sn = 0;
            while *s.get_unchecked(i) != b'\n' {
                sn *= 10;
                sn += (s.get_unchecked(i) - b'0') as u32;
                i += 1;
            }
            i += 1;

            for _ in 0..2000 {
                sn = ((sn as u64 * 64) % MAX as u64) as u32 ^ sn;
                sn = (sn / 32) ^ sn;
                sn = ((sn as u64 * 2048) % MAX as u64) as u32 ^ sn;
            }

            sum += sn as u64;
        }
    }

    sum
}

const SEQUENCES: usize = 18 * 18 * 18 * 18;

#[aoc(day22, part2)]
pub fn part2(s: &str) -> i16 {
    let s = s.as_bytes();

    let mut sequences = [0; SEQUENCES];
    let mut done = [0u16; SEQUENCES];

    let mut i = 0;
    let mut monky = 1;
    unsafe {
        while i < s.len() {
            #[cfg(not(test))]
            let mut sn = (*s.get_unchecked(i + 0) as u32) * 100000
                + (*s.get_unchecked(i + 1) as u32) * 10000
                + (*s.get_unchecked(i + 2) as u32) * 1000
                + (*s.get_unchecked(i + 3) as u32) * 100
                + (*s.get_unchecked(i + 4) as u32) * 10
                + (*s.get_unchecked(i + 5) as u32) * 1
                - (b'0' as u32 * 111_111);
            #[cfg(not(test))]
            {
                i += 6;
            }

            #[cfg(test)]
            let mut sn = 0;
            while *s.get_unchecked(i) != b'\n' {
                sn *= 10;
                sn += (s.get_unchecked(i) - b'0') as u32;
                i += 1;
            }
            i += 1;

            let mut diffs = 0;
            let mut prev = sn % 10;
            for _ in 0..3 {
                sn = ((sn as u64 * 64) % MAX as u64) as u32 ^ sn;
                sn = (sn / 32) ^ sn;
                sn = ((sn as u64 * 2048) % MAX as u64) as u32 ^ sn;
                let price = sn % 10;
                let diff = price + 9 - prev;
                diffs = diffs * 18 + diff;

                prev = price;
            }

            for _ in 4..2000 {
                sn = ((sn as u64 * 64) % MAX as u64) as u32 ^ sn;
                sn = (sn / 32) ^ sn;
                sn = ((sn as u64 * 2048) % MAX as u64) as u32 ^ sn;
                let price = sn % 10;
                let diff = price + 9 - prev;
                diffs = (diffs * 18 + diff) % SEQUENCES as u32;

                if done[diffs as usize] != monky {
                    sequences[diffs as usize] += price as i16;

                    done[diffs as usize] = monky;
                }

                prev = price;
            }
            monky += 1;
        }

        sequences.into_iter().max().unwrap_unchecked()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let s = r"1
10
100
2024
";

        assert_eq!(part1(s), 37327623);
    }

    #[test]
    fn example_part2() {
        let s = r"1
2
3
2024
";

        assert_eq!(part2(s), 23);
    }
}
