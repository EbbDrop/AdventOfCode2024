use std::num::NonZero;

use aoc_runner_derive::aoc;

fn search(target: u64, v: &[NonZero<u64>]) -> bool {
    match v {
        [] => {
            return target == 0;
        }
        [rest @ .., last] => {
            let last = last.get();
            if target % last == 0 {
                if search(target / last, rest) {
                    return true;
                }
            }
            if last > target {
                return false;
            }

            return search(target - last, rest);
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(s: &str) -> u64 {
    unsafe { part1_inner(s) }
}

unsafe fn part1_inner(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut sum = 0;

    let mut i = 0;
    let mut v = [NonZero::new(1).unwrap(); 15];
    let mut v_len = 0;

    while i < s.len() {
        let mut target: u64 = 0;
        while *s.get_unchecked(i) != b':' {
            target *= 10;
            target += (*s.get_unchecked(i) - b'0') as u64;
            i += 1;
        }
        i += 2;

        let mut num = 0;
        while *s.get_unchecked(i) != b'\n' {
            num *= 10;
            num += (*s.get_unchecked(i) - b'0') as u64;
            i += 1;
            if !s.get_unchecked(i).is_ascii_digit() {
                *v.get_unchecked_mut(v_len) = NonZero::new_unchecked(num);
                v_len += 1;
                num = 0;
                i += 1;
                if *s.get_unchecked(i - 1) == b'\n' {
                    break;
                }
            }
        }

        if search(target, &v.get_unchecked(..v_len)) {
            sum += target;
        }
        v_len = 0;
    }

    sum
}

fn search_part2(target: u64, v: &[NonZero<u64>]) -> bool {
    match v {
        [] => {
            return target == 0;
        }
        [rest @ .., last] => {
            let last = last.get();
            if target % last == 0 {
                if search_part2(target / last, rest) {
                    return true;
                }
            }
            if last > target {
                return false;
            }

            let size = unsafe { NonZero::new_unchecked(10u64.pow(last.ilog10() + 1)) };
            if (target - last) % size == 0 {
                if search_part2((target - last) / size, rest) {
                    return true;
                }
            }

            return search_part2(target - last, rest);
        }
    }
}

#[aoc(day7, part2)]
pub fn part2(s: &str) -> u64 {
    unsafe { part2_inner(s) }
}

unsafe fn part2_inner(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut sum = 0;

    let mut i = 0;
    let mut v = [NonZero::new(1).unwrap(); 15];
    let mut v_len = 0;

    while i < s.len() {
        let mut target: u64 = 0;
        while *s.get_unchecked(i) != b':' {
            target *= 10;
            target += (*s.get_unchecked(i) - b'0') as u64;
            i += 1;
        }
        i += 2;

        let mut num = 0;
        while *s.get_unchecked(i) != b'\n' {
            num *= 10;
            num += (*s.get_unchecked(i) - b'0') as u64;
            i += 1;
            if !s.get_unchecked(i).is_ascii_digit() {
                *v.get_unchecked_mut(v_len) = NonZero::new_unchecked(num);
                v_len += 1;
                num = 0;
                i += 1;
                if *s.get_unchecked(i - 1) == b'\n' {
                    break;
                }
            }
        }

        if search_part2(target, &v.get_unchecked(..v_len)) {
            sum += target;
        }
        v_len = 0;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 3749);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 11387);
    }
}
