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

static LUT: [u32; 4] = [0, 0xFF, 0xFF_FF, 0xFF_FF_FF];

#[inline(always)]
unsafe fn part1_inner(s: &[u8]) -> u64 {
    let mut sum = 0;

    static mut KEYS: [u64; 256] = unsafe { std::mem::transmute([0u8; 256 * 8]) };
    static mut HOLES: [u64; 256] = unsafe { std::mem::transmute([0u8; 256 * 8]) };

    let keys = &mut *(&raw mut KEYS);
    let holes = &mut *(&raw mut HOLES);

    let mut keys_i = 0;
    let mut holes_i = 0;

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

        if is_key {
            std::arch::asm!(
                "test      {max_i}, {max_i}",
                "je        2f",               // Jump on empty
                "mov       {i}, {max_i}",
                "cmp       {i}, 16",
                "jb        6f",               // Jump to < 16 case

                "4:",
                "vpaddq    {vt}, {d},  ymmword ptr [{os} + 8*{i} - 32]",
                "add       {i},  -16",
                "vpand     {vt}, {vt}, {msb}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "vpaddq    {vt}, {d},  ymmword ptr [{os} + 8*{i} + 32*2]",
                "popcnt    {t},  {t}",
                "vpand     {vt}, {vt}, {msb}",
                "add       {sum},{t}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "vpaddq    {vt}, {d},  ymmword ptr [{os} + 8*{i} + 32*1]",
                "popcnt    {t},  {t}",
                "vpand     {vt}, {vt}, {msb}",
                "add       {sum},{t}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "vpaddq    {vt}, {d},  ymmword ptr [{os} + 8*{i} + 32*0]",
                "popcnt    {t},  {t}",
                "vpand     {vt}, {vt}, {msb}",
                "add       {sum},{t}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "popcnt    {t},  {t}",
                "add       {sum},{t}",
                "cmp       {i}, 16",
                "jae       4b",               // Loop
                "6:",
                "cmp       {i}, 4",
                "jb        3f",               // Is < 4
                // Is >= 4 and < 16

                "5:",
                "vpaddq    {vt}, {d},  ymmword ptr [{os} + 8*{i} - 32]",
                "add       {i}, -4",
                "vpand     {vt}, {vt}, {msb}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "popcnt    {t},  {t}",
                "add       {sum},{t}",
                "cmp       {i}, 4",
                "jae       5b",               // Loop
                "3:",
                "test      {i}, {i}",
                "je        2f",               // Is zero

                // Is > 0 and < 4
                "vpaddq    {vt}, {d},  ymmword ptr [{os}]",
                "vpand     {vt}, {vt}, {msb}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "and       {t:e}, dword ptr [{lut} + 4*{i}]",
                "popcnt    {t},  {t}",
                "add       {sum},{t}",
                "2:",
                d = in(ymm_reg) _mm256_set1_epi64x(d as i64),
                msb = in(ymm_reg) _mm256_set1_epi8(0x80u8 as i8),
                zero = in(ymm_reg) _mm256_set1_epi64x(0),
                lut = in(reg) LUT.as_ptr(),
                os = in(reg) holes,
                max_i = in(reg) holes_i,
                sum = inout(reg) sum,
                i = out(reg) _,
                t = out(reg) _,
                vt = out(ymm_reg) _,
                options(nostack),
            );
            *keys.get_unchecked_mut(keys_i) = d + 0x7A7A7A7A7A;
            keys_i += 1;
        } else {
            std::arch::asm!(

                "test      {max_i}, {max_i}",
                "je        2f",               // Jump on empty
                "mov       {i}, {max_i}",
                "cmp       {i}, 16",
                "jb        6f",               // Jump to < 16 case

                "4:",
                "vpaddq    {vt}, {d},  ymmword ptr [{os} + 8*{i} - 32]",
                "add       {i},  -16",
                "vpand     {vt}, {vt}, {msb}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "vpaddq    {vt}, {d},  ymmword ptr [{os} + 8*{i} + 32*2]",
                "popcnt    {t},  {t}",
                "vpand     {vt}, {vt}, {msb}",
                "add       {sum},{t}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "vpaddq    {vt}, {d},  ymmword ptr [{os} + 8*{i} + 32*1]",
                "popcnt    {t},  {t}",
                "vpand     {vt}, {vt}, {msb}",
                "add       {sum},{t}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "vpaddq    {vt}, {d},  ymmword ptr [{os} + 8*{i} + 32*0]",
                "popcnt    {t},  {t}",
                "vpand     {vt}, {vt}, {msb}",
                "add       {sum},{t}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "popcnt    {t},  {t}",
                "add       {sum},{t}",
                "cmp       {i}, 16",
                "jae       4b",               // Loop
                "6:",
                "cmp       {i}, 4",
                "jb        3f",               // Is < 4
                // Is >= 4 and < 16

                "5:",
                "vpaddq    {vt}, {d},  ymmword ptr [{os} + 8*{i} - 32]",
                "add       {i}, -4",
                "vpand     {vt}, {vt}, {msb}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "popcnt    {t},  {t}",
                "add       {sum},{t}",
                "cmp       {i}, 4",
                "jae       5b",               // Loop
                "3:",
                "test      {i}, {i}",
                "je        2f",               // Is zero

                // Is > 0 and < 4
                "vpaddq    {vt}, {d},  ymmword ptr [{os}]",
                "vpand     {vt}, {vt}, {msb}",
                "vpcmpeqq  {vt}, {vt}, {zero}",
                "vpmovmskb {t},  {vt}",
                "and       {t:e}, dword ptr [{lut} + 4*{i}]",
                "popcnt    {t},  {t}",
                "add       {sum},{t}",
                "2:",
                d = in(ymm_reg) _mm256_set1_epi64x(d as i64),
                msb = in(ymm_reg) _mm256_set1_epi8(0x80u8 as i8),
                zero = in(ymm_reg) _mm256_set1_epi64x(0),
                lut = in(reg) LUT.as_ptr(),
                os = in(reg) keys,
                max_i = in(reg) keys_i,
                sum = inout(reg) sum,
                i = out(reg) _,
                t = out(reg) _,
                vt = out(ymm_reg) _,
                options(nostack),
            );
            *holes.get_unchecked_mut(holes_i) = d + 0x7A7A7A7A7A;
            holes_i += 1;
        }

        i += DS;
    }

    sum / 8
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
