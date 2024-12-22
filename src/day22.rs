use core::str;

use aoc_runner_derive::aoc;
use bitvec::array::BitArray;

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

const SEQUENCES: usize = 18 << 15 | 18 << 10 | 18 << 5 | 18;

#[aoc(day22, part2)]
pub fn part2(s: &str) -> i16 {
    let s = s.as_bytes();

    let mut sequences = [0; SEQUENCES];
    let mut done = BitArray::<[usize; SEQUENCES.div_ceil(usize::BITS as usize)]>::default();

    let mut current_best = 0;

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

            let mut diffs = 0;
            let mut prev = sn % 10;
            for _ in 0..3 {
                sn = ((sn as u64 * 64) % MAX as u64) as u32 ^ sn;
                sn = (sn / 32) ^ sn;
                sn = ((sn as u64 * 2048) % MAX as u64) as u32 ^ sn;
                let price = sn % 10;
                let diff = price + 9 - prev;
                diffs = (diffs << 5) | diff;

                prev = price;
            }

            for _ in 4..2000 {
                sn = ((sn as u64 * 64) % MAX as u64) as u32 ^ sn;
                sn = (sn / 32) ^ sn;
                sn = ((sn as u64 * 2048) % MAX as u64) as u32 ^ sn;
                let price = sn % 10;
                let diff = price + 9 - prev;
                diffs = ((diffs << 5) | diff) & 0xFFFFF;

                if !done[diffs as usize] {
                    sequences[diffs as usize] += price as i16;
                    if current_best < sequences[diffs as usize] {
                        current_best = sequences[diffs as usize];
                    }
                    done.set(diffs as usize, true);
                }

                prev = price;
            }
            done.fill(false);
        }
        // println!(
        //     "{},{},{},{}",
        //     (((current_best_i >> 15) & 0x1F) as i16) - 9,
        //     (((current_best_i >> 10) & 0x1F) as i16) - 9,
        //     (((current_best_i >> 05) & 0x1F) as i16) - 9,
        //     (((current_best_i >> 00) & 0x1F) as i16) - 9,
        // );
        // println!("{}\n", sequences[current_best_i as usize]);

        // println!(
        //     "{}",
        //     sequences[((-2 + 9) << 15 | (1 + 9) << 10 | (-1 + 9) << 05 | (3 + 9) << 00) as usize]
        // );
    }

    current_best
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
