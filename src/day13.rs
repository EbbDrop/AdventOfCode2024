use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
pub fn part1(s: &str) -> u64 {
    unsafe { inner_part1(s) }
}

#[aoc(day13, part2)]
pub fn part2(s: &str) -> u64 {
    unsafe { inner_part2(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_part1(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut sum = 0;

    let mut i = 0;
    while i < s.len() {
        let ax = (s
            .get_unchecked(i + 12)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 13))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i32;
        let ay = (s
            .get_unchecked(i + 18)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 19))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i32;

        let bx = (s
            .get_unchecked(i + 33)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 34))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i32;
        let by = (s
            .get_unchecked(i + 39)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 40))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i32;
        i += 51;

        let mut x = 0;
        while *s.get_unchecked(i) != b',' {
            x *= 10;
            x += (*s.get_unchecked(i) - b'0') as i32;
            i += 1;
        }
        i += 4;

        let mut y = 0;
        while *s.get_unchecked(i) != b'\n' {
            y *= 10;
            y += (*s.get_unchecked(i) - b'0') as i32;
            i += 1;
        }
        i += 2;

        let numerator = x * by - y * bx;
        let denominator = ax * by - ay * bx;
        std::hint::assert_unchecked(denominator != 0);
        std::hint::assert_unchecked(numerator != i32::MIN);
        std::hint::assert_unchecked(by != 0);
        std::hint::assert_unchecked(by != -1);

        if numerator % denominator != 0 {
            continue;
        }
        let a = numerator / denominator;
        if (y - a * ay) % by != 0 {
            continue;
        }
        let b = (y - a * ay) / by;

        sum += (a * 3 + b) as u64;
    }

    sum
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_part2(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut sum = 0;

    let mut i = 0;
    while i < s.len() {
        let ax = (s
            .get_unchecked(i + 12)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 13))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i64;
        let ay = (s
            .get_unchecked(i + 18)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 19))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i64;

        let bx = (s
            .get_unchecked(i + 33)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 34))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i64;
        let by = (s
            .get_unchecked(i + 39)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 40))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i64;
        i += 51;

        let mut x = 0;
        while *s.get_unchecked(i) != b',' {
            x *= 10;
            x += (*s.get_unchecked(i) - b'0') as i64;
            i += 1;
        }
        x += 10000000000000;
        i += 4;

        let mut y = 0;
        while *s.get_unchecked(i) != b'\n' {
            y *= 10;
            y += (*s.get_unchecked(i) - b'0') as i64;
            i += 1;
        }
        y += 10000000000000;
        i += 2;

        let numerator = x * by - y * bx;
        let denominator = ax * by - ay * bx;
        std::hint::assert_unchecked(denominator != 0);
        std::hint::assert_unchecked(numerator != i64::MIN);
        std::hint::assert_unchecked(by != 0);
        std::hint::assert_unchecked(by != -1);

        if numerator % denominator != 0 {
            continue;
        }
        let a = numerator / denominator;
        if (y - a * ay) % by != 0 {
            continue;
        }
        let b = (y - a * ay) / by;

        sum += (a * 3 + b) as u64;
    }

    sum
}

// #[aoc(day13, part2)]
// pub fn part2(s: &str) -> u64 {
//     #[expect(unused_unsafe)]
//     unsafe {
//         part2_inner(s)
//     }
// }

// // #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
// fn part2_inner(s: &str) -> u64 {
//     let s = s.as_bytes();
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 480);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 875318608908);
    }
}
