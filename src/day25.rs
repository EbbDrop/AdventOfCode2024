use std::{arch::x86_64::*, mem::MaybeUninit};

use aoc_runner_derive::aoc;

#[aoc(day25, part1)]
pub fn part1(s: &str) -> u64 {
    let s = s.as_bytes();
    unsafe { part1_inner(s) }
}

const DS: usize = 7 * 6 + 1;

const ALL_BITS: i8 = 0b111;
const KEPT_BITS: i8 = 0b011;

#[inline(always)]
unsafe fn part1_inner(s: &[u8]) -> u64 {
    let mut sum = 0;

    static mut KEYS: [__m256i; 512] = unsafe { std::mem::transmute([0u8; 512 * 32]) };
    static mut HOLES: [__m256i; 512] = unsafe { std::mem::transmute([0u8; 512 * 32]) };

    let keys = &mut *(&raw mut KEYS);
    let holes = &mut *(&raw mut HOLES);

    let mut keys_i = 0;
    let mut holes_i = 0;

    let mut i = 0;

    std::hint::assert_unchecked(s.len() > 0);
    while i < s.len() {
        let is_key = *s.get_unchecked(i) == b'.';

        let d = s
            .as_ptr()
            .offset(i as isize + 6)
            .cast::<__m256i>()
            .read_unaligned();
        let d = _mm256_and_si256(
            d,
            _mm256_setr_epi8(
                ALL_BITS, ALL_BITS, ALL_BITS, ALL_BITS, ALL_BITS, 0, //
                ALL_BITS, ALL_BITS, ALL_BITS, ALL_BITS, ALL_BITS, 0, //
                ALL_BITS, ALL_BITS, ALL_BITS, ALL_BITS, ALL_BITS, 0, //
                ALL_BITS, ALL_BITS, ALL_BITS, ALL_BITS, ALL_BITS, 0, //
                ALL_BITS, ALL_BITS, ALL_BITS, ALL_BITS, ALL_BITS, 0, 0, 0,
            ),
        );

        if is_key {
            for i in 0..holes_i {
                let o = *holes.get_unchecked(i);
                let collisions = _mm256_cmpeq_epi8(d, o);
                let collisions = _mm256_movemask_epi8(collisions);
                sum += (collisions == 0) as u64;
            }
            let d = _mm256_and_si256(d, _mm256_set1_epi8(KEPT_BITS));
            let d = _mm256_or_si256(
                d,
                _mm256_setr_epi8(
                    0, 0, 0, 0, 0, -1, //
                    0, 0, 0, 0, 0, -1, //
                    0, 0, 0, 0, 0, -1, //
                    0, 0, 0, 0, 0, -1, //
                    0, 0, 0, 0, 0, -1, -1, -1,
                ),
            );
            *keys.get_unchecked_mut(keys_i) = d;
            keys_i += 1;
        } else {
            for i in 0..keys_i {
                let o = *keys.get_unchecked(i);
                let collisions = _mm256_cmpeq_epi8(d, o);
                let collisions = _mm256_movemask_epi8(collisions);
                sum += (collisions == 0) as u64;
            }
            let d = _mm256_and_si256(d, _mm256_set1_epi8(KEPT_BITS));
            let d = _mm256_or_si256(
                d,
                _mm256_setr_epi8(
                    0, 0, 0, 0, 0, -1, //
                    0, 0, 0, 0, 0, -1, //
                    0, 0, 0, 0, 0, -1, //
                    0, 0, 0, 0, 0, -1, //
                    0, 0, 0, 0, 0, -1, -1, -1,
                ),
            );
            *holes.get_unchecked_mut(holes_i) = d;
            holes_i += 1;
        }

        i += DS;
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }
}
