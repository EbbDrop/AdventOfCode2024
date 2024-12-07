use std::{hint::unreachable_unchecked, mem::MaybeUninit, num::NonZero};

use aoc_runner_derive::aoc;

macro_rules! search_fn {
    ($name:ident => $name_next:ident) => {
        #[inline(always)]
        unsafe fn $name(target: u64, v: &[NonZero<u64>]) -> bool {
            match v {
                [] => unsafe { unreachable_unchecked() },
                [rest @ .., last] => {
                    let last = last.get();
                    if last > target {
                        return false;
                    }

                    if target % last == 0 {
                        if $name_next(target / last, rest) {
                            return true;
                        }
                    }

                    return $name_next(target - last, rest);
                }
            }
        }
    };
}

search_fn!(search_12 => search_11);
search_fn!(search_11 => search_10);
search_fn!(search_10 => search_9);
search_fn!(search_9 => search_8);
search_fn!(search_8 => search_7);
search_fn!(search_7 => search_6);
search_fn!(search_6 => search_5);
search_fn!(search_5 => search_4);
search_fn!(search_4 => search_3);
search_fn!(search_3 => search_2);
search_fn!(search_2 => search_1);

#[inline(always)]
unsafe fn search_1(target: u64, v: &[NonZero<u64>]) -> bool {
    match v {
        [last] => {
            return target == last.get();
        }
        _ => unsafe { unreachable_unchecked() },
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
    let mut v = [MaybeUninit::uninit(); 15];
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
        loop {
            if !s.get_unchecked(i).is_ascii_digit() {
                v.get_unchecked_mut(v_len)
                    .write(NonZero::new_unchecked(num));
                v_len += 1;
                num = 0;
                i += 1;
                if *s.get_unchecked(i - 1) == b'\n' {
                    break;
                }
            }
            num *= 10;
            num += (*s.get_unchecked(i) - b'0') as u64;
            i += 1;
        }

        let init = &*(v.get_unchecked(..v_len) as *const [MaybeUninit<NonZero<u64>>]
            as *const [NonZero<u64>]);
        if match init.len() {
            1 => search_1(target, init),
            2 => search_2(target, init),
            3 => search_3(target, init),
            4 => search_4(target, init),
            5 => search_5(target, init),
            6 => search_6(target, init),
            7 => search_7(target, init),
            8 => search_8(target, init),
            9 => search_9(target, init),
            10 => search_10(target, init),
            11 => search_11(target, init),
            12 => search_12(target, init),
            _ => unreachable_unchecked(),
        } {
            sum += target;
        }
        v_len = 0;
    }

    sum
}

fn search_part2(target: u64, v: &[NonZero<u64>]) -> bool {
    match v {
        [] => unsafe { unreachable_unchecked() },
        [rest @ .., last] => {
            let last = last.get();
            if rest.is_empty() {
                return target == last;
            }
            if last > target {
                return false;
            }

            if target % last == 0 {
                if search_part2(target / last, rest) {
                    return true;
                }
            }

            let size = if last >= 100 {
                1000
            } else if last >= 10 {
                100
            } else {
                10
            };
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
    let mut v = [MaybeUninit::uninit(); 15];
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
        loop {
            if !s.get_unchecked(i).is_ascii_digit() {
                v.get_unchecked_mut(v_len)
                    .write(NonZero::new_unchecked(num));
                v_len += 1;
                num = 0;
                i += 1;
                if *s.get_unchecked(i - 1) == b'\n' {
                    break;
                }
            }
            num *= 10;
            num += (*s.get_unchecked(i) - b'0') as u64;
            i += 1;
        }

        let init = &*(v.get_unchecked(..v_len) as *const [MaybeUninit<NonZero<u64>>]
            as *const [NonZero<u64>]);
        if search_part2(target, init) {
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
