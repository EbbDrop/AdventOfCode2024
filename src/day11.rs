use fxhash::{FxBuildHasher, FxHashMap as HashMap};

use aoc_runner_derive::aoc;

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

#[aoc(day11, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner(s, 25, 4000)
    }
}

#[aoc(day11, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner(s, 75, 230000)
    }
}

fn amount_of_stones(num: u64, blinks_left: u64, cach: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks_left == 1 {
        let r = match num {
            0..=9 => 1,
            10..=99 => 2,
            100..=999 => 1,
            1000..=9999 => 2,
            10000..=99999 => 1,
            100000..=999999 => 2,
            1000000..=9999999 => 1,
            10000000..=99999999 => 2,
            100000000..=999999999 => 1,
            1000000000..=9999999999 => 2,
            10000000000..=99999999999 => 1,
            100000000000..=999999999999 => 2,
            1000000000000..=9999999999999 => 1,
            10000000000000..=99999999999999 => 2,
            100000000000000..=999999999999999 => 1,
            1000000000000000..=9999999999999999 => 2,
            10000000000000000..=99999999999999999 => 1,
            100000000000000000..=999999999999999999 => 2,
            1000000000000000000..=9999999999999999999 => 1,
            10000000000000000000..=u64::MAX => 2,
        };
        return r;
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

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[inline(always)]
fn inner(s: &str, num_blinks: u64, cach_cap: usize) -> u64 {
    let s = s.as_bytes();

    let mut sum = 0;

    let mut num = 0;

    let mut cach = HashMap::with_capacity_and_hasher(cach_cap, FxBuildHasher::default());

    for c in s {
        if c.is_ascii_digit() {
            num *= 10;
            num += (c - b'0') as u64;
        } else {
            sum += amount_of_stones(num, num_blinks, &mut cach);
            num = 0;
        }
    }

    // let mut sums = [0; 1000];
    // for (num, _) in cach.keys() {
    //     sums.get_mut(*num as usize).map(|v| *v += 1);
    // }
    // for (i, s) in sums.iter().enumerate() {
    //     println!("{i} -> {s}");
    // }

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
