use std::arch::x86_64::*;

use aoc_runner_derive::aoc;

#[aoc(day25, part1)]
pub fn part1(s: &str) -> u64 {
    let s = s.as_bytes();
    unsafe { part1_inner(s) }
}

const DS: usize = 7 * 6 + 1;

#[inline(always)]
unsafe fn part1_inner(s: &[u8]) -> u64 {
    let mut sum = 0;

    let mut keys = heapless::Vec::<u64, 512>::new();
    let mut holes = heapless::Vec::<u64, 512>::new();

    let mut i = 0;

    std::hint::assert_unchecked(s.len() > 0);
    while i < s.len() {
        let is_key = *s.get_unchecked(i) == b'.';

        let d = (s
            .as_ptr()
            .offset(i as isize + 6)
            .cast::<u64>()
            .read_unaligned()
            & 0x0101010101)
            + (s.as_ptr()
                .offset(i as isize + 6 + 6)
                .cast::<u64>()
                .read_unaligned()
                & 0x0101010101)
            + (s.as_ptr()
                .offset(i as isize + 6 + 12)
                .cast::<u64>()
                .read_unaligned()
                & 0x0101010101)
            + (s.as_ptr()
                .offset(i as isize + 6 + 18)
                .cast::<u64>()
                .read_unaligned()
                & 0x0101010101)
            + (s.as_ptr()
                .offset(i as isize + 6 + 24)
                .cast::<u64>()
                .read_unaligned()
                & 0x0101010101);

        let other = if is_key { &holes } else { &keys };
        let mut j = other.len();
        while j >= 4 {
            j -= 4;
            let o = other
                .as_ptr()
                .offset(j as isize)
                .cast::<__m256i>()
                .read_unaligned();
            let s = _mm256_add_epi64(o, _mm256_set1_epi64x(d as i64));
            let s = _mm256_and_si256(s, _mm256_set1_epi8(0x80u8 as i8));
            let s = _mm256_cmpeq_epi64(s, _mm256_set1_epi64x(0));
            let s = _mm256_movemask_epi8(s) as u32;

            sum += s.count_ones() as u64 / 8;
        }
        if j > 0 {
            let o = other.as_ptr().cast::<__m256i>().read_unaligned();
            let s = _mm256_add_epi64(o, _mm256_set1_epi64x(d as i64));
            let s = _mm256_and_si256(s, _mm256_set1_epi8(0x80u8 as i8));
            let s = _mm256_cmpeq_epi64(s, _mm256_set1_epi64x(0));
            let s = _mm256_movemask_epi8(s) as u32;

            let s = s & (0xFF_FF_FF >> ((3 - j) * 8));
            sum += s.count_ones() as u64 / 8;
        }

        let d = d + 0x7A7A7A7A7A;
        if is_key {
            keys.push_unchecked(d);
        } else {
            holes.push_unchecked(d);
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
