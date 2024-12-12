use fxhash::FxHashMap as HashMap;

use aoc_runner_derive::aoc;

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
    let r = if num == 0 {
        amount_of_stones(1, blinks_left - 1, cach)
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
