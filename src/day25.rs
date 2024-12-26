use std::arch::x86_64::*;

use aoc_runner_derive::aoc;

#[aoc(day25, part1)]
pub fn part1(s: &str) -> u64 {
    let s = s.as_bytes();
    unsafe { part1_inner(s) }
}

pub fn part2(_s: &str) -> u64 {
    // To be sure you know...
    42
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
            std::arch::asm!(
                "test      {max_i}, {max_i}",
                "je        2f",               // Jump on empty
                "cmp       {max_i}, 1",
                "je        3f",               // Jump to one case
                "mov       {i}, {max_i}",
                "shl       {i}, 5",
                "jmp       5f",

                "4:",
                "add       {i}, -32 * 4",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i} + 96]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i} + 64]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i} + 32]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i}]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "5:",
                "cmp       {i}, 96",
                "jg        4b",               // Loop
                "cmp       {i}, 32",
                "jl        2f",               // Is zero
                "je        3f",               // Is one
                // Is 2 or 3

                "4:",
                "add       {i}, -32 * 2",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i} + 32]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i}]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "cmp       {i}, 32",
                "jg        4b",               // Loop
                "jne       2f",               // Is zero

                "3:",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os}]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "2:",
                os = in(reg) holes,
                max_i = in(reg) holes_i,
                d = in(ymm_reg) d,
                i = out(reg) _,
                sum = inout(reg) sum,
                t = out(reg) _,
                vt = out(ymm_reg) _,
                options(nostack),
            );
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
            std::arch::asm!(
                "test      {max_i}, {max_i}",
                "je        2f",               // Jump on empty
                "cmp       {max_i}, 1",
                "je        3f",               // Jump to one case
                "mov       {i}, {max_i}",
                "shl       {i}, 5",
                "jmp       5f",

                "4:",
                "add       {i}, -32 * 4",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i} + 96]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i} + 64]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i} + 32]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i}]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "5:",
                "cmp       {i}, 96",
                "jg        4b",               // Loop
                "cmp       {i}, 32",
                "jl        2f",               // Is zero
                "je        3f",               // Is one
                // Is 2 or 3

                "4:",
                "add       {i}, -32 * 2",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i} + 32]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os} + {i}]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "cmp       {i}, 32",
                "jg        4b",               // Loop
                "jne       2f",               // Is zero

                "3:",
                "vpcmpeqb  {vt}, {d}, ymmword ptr [{os}]",
                "vpmovmskb {t}, {vt}",
                "cmp       {t}, 1",
                "adc       {sum}, 0",
                "2:",
                os = in(reg) keys,
                max_i = in(reg) keys_i,
                d = in(ymm_reg) d,
                i = out(reg) _,
                sum = inout(reg) sum,
                t = out(reg) _,
                vt = out(ymm_reg) _,
                options(nostack),
            );
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
