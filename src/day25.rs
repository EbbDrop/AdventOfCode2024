use std::arch::x86_64::*;

use aoc_runner_derive::aoc;

#[aoc(day25, part1)]
pub fn part1(s: &str) -> u64 {
    let s = s.as_bytes();
    unsafe { part1_inner(s) }
}

#[cfg(not(test))]
const SIZE: usize = 500;
#[cfg(test)]
const SIZE: usize = 5;

const DS: usize = 7 * 6 + 1;

#[inline(always)]
unsafe fn part1_inner(s: &[u8]) -> u64 {
    let mut sum = 0;

    let mut keys = heapless::Vec::<__m256i, 512>::new();
    let mut holes = heapless::Vec::<__m256i, 512>::new();

    for i in 0..SIZE {
        let i = i * DS;
        let is_key = *s.get_unchecked(i) == b'.';

        let d = s
            .as_ptr()
            .offset(i as isize + 6)
            .cast::<__m256i>()
            .read_unaligned();
        let d = _mm256_and_si256(
            d,
            _mm256_setr_epi8(
                -1, -1, -1, -1, -1, 0, //
                -1, -1, -1, -1, -1, 0, //
                -1, -1, -1, -1, -1, 0, //
                -1, -1, -1, -1, -1, 0, //
                -1, -1, -1, -1, -1, 0, 0, 0,
            ),
        );

        let other = if is_key { &holes } else { &keys };
        for o in other {
            let collisions = _mm256_cmpeq_epi8(
                _mm256_add_epi8(d, *o),
                _mm256_set1_epi8(b'#'.wrapping_add(b'#') as i8),
            );
            let collisions = _mm256_movemask_epi8(collisions);
            sum += (collisions == 0) as u64;
        }

        if is_key {
            keys.push_unchecked(d);
        } else {
            holes.push_unchecked(d);
        }
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
