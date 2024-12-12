use aoc_runner_derive::aoc;
use bytemuck::{cast_ref, Pod, Zeroable};
use fxhash::FxHashMap as HashMap;

#[repr(C, align(8))]
#[derive(Clone, Copy)]
struct AlignSlice([u8; 80_000_000]);

unsafe impl Zeroable for AlignSlice {}
unsafe impl Pod for AlignSlice {}

static BIG_LUT25: &AlignSlice =
    &AlignSlice(*include_bytes!(concat!("../big_lut25.bin")));
static BIG_LUT75: &AlignSlice =
    &AlignSlice(*include_bytes!(concat!("../big_lut75.bin")));

const LUT_SIZE: u64 = 100;

const LUT: [u64; LUT_SIZE as usize] = const {
    let mut lut = [0; LUT_SIZE as usize];

    let mut i = 0u64;
    while i < LUT_SIZE {
        let r = if i == 0 {
            1
        } else if i.ilog10() % 2 == 1 {
            let i_digits = i.ilog10() + 1;
            let tens = 10u64.pow(i_digits / 2);

            (i % tens) << 32 | (i / tens)
        } else {
            i * 2024
        };
        lut[i as usize] = r;
        i += 1;
    }

    lut
};

fn amount_of_stones(num: u64, blinks_left: u64, cach: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks_left == 0 {
        return 1;
    }
    if let Some(r) = cach.get(&(num, blinks_left)) {
        return *r;
    }
    const { assert!(LUT_SIZE == 100) };
    let r = match num {
        0 => amount_of_stones(1, blinks_left - 1, cach),
        1..=9 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        10..=99 => {
            let r = LUT[num as usize];
            amount_of_stones(r & (2u64.pow(32) - 1), blinks_left - 1, cach)
                + amount_of_stones((r >> 32) & (2u64.pow(32) - 1), blinks_left - 1, cach)
        }
        100..=999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        1000..=9999 => {
            amount_of_stones(num / 100, blinks_left - 1, cach)
                + amount_of_stones(num % 100, blinks_left - 1, cach)
        }
        10000..=99999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        100000..=999999 => {
            amount_of_stones(num / 1000, blinks_left - 1, cach)
                + amount_of_stones(num % 1000, blinks_left - 1, cach)
        }
        1000000..=9999999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        10000000..=99999999 => {
            amount_of_stones(num / 10000, blinks_left - 1, cach)
                + amount_of_stones(num % 10000, blinks_left - 1, cach)
        }
        100000000..=999999999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        1000000000..=9999999999 => {
            amount_of_stones(num / 100000, blinks_left - 1, cach)
                + amount_of_stones(num % 100000, blinks_left - 1, cach)
        }
        10000000000..=99999999999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        100000000000..=999999999999 => {
            amount_of_stones(num / 1000000, blinks_left - 1, cach)
                + amount_of_stones(num % 1000000, blinks_left - 1, cach)
        }
        1000000000000..=9999999999999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        10000000000000..=99999999999999 => {
            amount_of_stones(num / 10000000, blinks_left - 1, cach)
                + amount_of_stones(num % 10000000, blinks_left - 1, cach)
        }
        100000000000000..=999999999999999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        1000000000000000..=9999999999999999 => {
            amount_of_stones(num / 100000000, blinks_left - 1, cach)
                + amount_of_stones(num % 100000000, blinks_left - 1, cach)
        }
        10000000000000000..=99999999999999999 => {
            amount_of_stones(num * 2024, blinks_left - 1, cach)
        }
        100000000000000000..=999999999999999999 => {
            amount_of_stones(num / 1000000000, blinks_left - 1, cach)
                + amount_of_stones(num % 1000000000, blinks_left - 1, cach)
        }
        1000000000000000000..=9999999999999999999 => {
            amount_of_stones(num * 2024, blinks_left - 1, cach)
        }
        10000000000000000000..=u64::MAX => {
            amount_of_stones(num / 10000000000, blinks_left - 1, cach)
                + amount_of_stones(num % 10000000000, blinks_left - 1, cach)
        }
    };
    cach.insert((num, blinks_left), r);
    r
}

#[aoc(day11, part1)]
pub fn part1(s: &str) -> u64 {
    let big_lut: &[u64; 10_000_000] = cast_ref(BIG_LUT25);

    let s = s.as_bytes();

    let mut sum = 0;

    let mut num = 0;

    let mut cach = HashMap::default();
    for c in s {
        if c.is_ascii_digit() {
            num *= 10;
            num += (c - b'0') as u64;
        } else {
            if (num as usize) < big_lut.len() {
                sum += big_lut[num as usize];
            } else {
                sum += amount_of_stones(num, 25, &mut cach);
            }
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

    let mut cach = HashMap::default();
    for c in s {
        if c.is_ascii_digit() {
            num *= 10;
            num += (c - b'0') as u64;
        } else {
            if (num as usize) < big_lut.len() {
                sum += big_lut[num as usize];
            } else {
                sum += amount_of_stones(num, 75, &mut cach);
            }
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
