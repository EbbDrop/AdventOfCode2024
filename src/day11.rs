use fxhash::FxHashMap as HashMap;

use aoc_runner_derive::aoc;

const LUT_SIZE: u64 = 2u64.pow(11);

const LUT: [u64; LUT_SIZE as usize] = const {
    let mut lut = [0; LUT_SIZE as usize];

    let mut i = 0u64;
    while i < LUT_SIZE {
        let r = if i == 0 {
            1
        } else if i.ilog10() % 2 == 1 {
            let i_digits = i.ilog10() + 1;
            let tens = 10u64.pow(i_digits / 2);

            (i / tens) << 32 | (i % tens) | 1 << 63
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
        inner(s, 25)
    }
}

#[aoc(day11, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner(s, 75)
    }
}

fn amount_of_stones(num: u64, blinks_left: u64, cach: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks_left == 0 {
        return 1;
    }
    if let Some(r) = cach.get(&(num, blinks_left)) {
        return *r;
    }
    let r = if num < LUT_SIZE {
        let r = LUT[num as usize];
        if r & 1 << 63 != 0 {
            // println!(
            //     "{num} -> {} and {}",
            //     r & (2u64.pow(32) - 1),
            //     (r >> 32) & (2u64.pow(31) - 1)
            // );
            amount_of_stones(r & (2u64.pow(32) - 1), blinks_left - 1, cach)
                + amount_of_stones((r >> 32) & (2u64.pow(31) - 1), blinks_left - 1, cach)
        } else {
            amount_of_stones(r, blinks_left - 1, cach)
        }
    } else if num.ilog10() % 2 == 1 {
        let num_digits = num.ilog10() + 1;
        let tens = 10u64.pow(num_digits / 2);

        amount_of_stones(num / tens, blinks_left - 1, cach)
            + amount_of_stones(num % tens, blinks_left - 1, cach)
    } else {
        amount_of_stones(num * 2024, blinks_left - 1, cach)
    };
    cach.insert((num, blinks_left), r);
    r
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[inline(always)]
fn inner(s: &str, num_blinks: u64) -> u64 {
    let s = s.as_bytes();

    let mut sum = 0;

    let mut num = 0;

    let mut cach = HashMap::default();

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
