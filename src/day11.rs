use aoc_runner_derive::aoc;
use bytemuck::{cast_ref, Pod, Zeroable};

#[repr(C, align(8))]
#[derive(Clone, Copy)]
struct AlignSlice([u8; 80_000_000]);

unsafe impl Zeroable for AlignSlice {}
unsafe impl Pod for AlignSlice {}

static BIG_LUT25: &AlignSlice =
    &AlignSlice(*include_bytes!(concat!("../big_lut25.bin")));
static BIG_LUT75: &AlignSlice =
    &AlignSlice(*include_bytes!(concat!("../big_lut75.bin")));

#[aoc(day11, part1)]
pub fn part1(s: &str) -> u64 {
    let big_lut: &[u64; 10_000_000] = cast_ref(BIG_LUT25);

    let s = s.as_bytes();

    let mut sum = 0;
    let mut num = 0;

    for c in s {
        if c.is_ascii_digit() {
            num *= 10;
            num += (c - b'0') as usize;
        } else {
            sum += unsafe { big_lut.get_unchecked(num) };
            num = 0;
        }
    }

    sum
}

#[aoc(day11, part2)]
pub fn part2(s: &str) -> u64 {
    let big_lut: &[u64; 10_000_000] = cast_ref(BIG_LUT75);

    let s = s.as_bytes();

    let mut sum = 0;
    let mut num = 0;

    for c in s {
        if c.is_ascii_digit() {
            num *= 10;
            num += (c - b'0') as usize;
        } else {
            sum += unsafe { big_lut.get_unchecked(num) };
            num = 0;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "125 17\n";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 55312);
    }
}
