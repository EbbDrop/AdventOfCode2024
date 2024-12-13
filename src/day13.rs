use aoc_runner_derive::aoc;
use std::arch::x86_64::*;

#[aoc(day13, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner::<0>(s)
    }
}

#[aoc(day13, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner::<10000000000000>(s)
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner<const OFFSET: i64>(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut sum = 0;

    let mut axs = [0i64; 320];
    let mut ays = [0i64; 320];
    let mut bxs = [0i64; 320];
    let mut bys = [0i64; 320];
    let mut xs = [0i64; 320];
    let mut ys = [0i64; 320];

    let mut p = 0;
    let mut i = 0;
    while p < s.len() {
        let ax = ((s[p + 12] - b'0') * 10 + s[p + 13] - b'0') as i64;
        let ay = ((s[p + 18] - b'0') * 10 + s[p + 19] - b'0') as i64;

        let bx = ((s[p + 33] - b'0') * 10 + s[p + 34] - b'0') as i64;
        let by = ((s[p + 39] - b'0') * 10 + s[p + 40] - b'0') as i64;
        p += 51;

        let mut x = 0;
        while s[p] != b',' {
            x *= 10;
            x += (s[p] - b'0') as i64;
            p += 1;
        }
        x += OFFSET;
        p += 4;

        let mut y = 0;
        while s[p] != b'\n' {
            y *= 10;
            y += (s[p] - b'0') as i64;
            p += 1;
        }
        y += OFFSET;
        p += 2;

        *axs.get_unchecked_mut(i) = ax;
        *ays.get_unchecked_mut(i) = ay;
        *bxs.get_unchecked_mut(i) = bx;
        *bys.get_unchecked_mut(i) = by;
        *xs.get_unchecked_mut(i) = x;
        *ys.get_unchecked_mut(i) = y;
        i += 1;
    }

    for i in 0..i {
        let ax = *axs.get_unchecked(i);
        let ay = *ays.get_unchecked(i);
        let bx = *bxs.get_unchecked(i);
        let by = *bys.get_unchecked(i);
        let x = *xs.get_unchecked(i);
        let y = *ys.get_unchecked(i);

        let numerator = x * by - y * bx;
        let denominator = ax * by - ay * bx;

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
