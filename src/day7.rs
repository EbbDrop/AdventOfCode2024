use std::{hint::unreachable_unchecked, mem::MaybeUninit, num::NonZero};

use aoc_runner_derive::aoc;

macro_rules! search_fn {
    ($name:ident; $n:expr => $name_next:ident) => {
        unsafe fn $name(target: u64, v: &[NonZero<u64>; $n]) -> bool {
            let [rest @ .., last] = v;

            let last = last.get();

            if target % last == 0 {
                if $name_next(target / last, rest) {
                    return true;
                }
            }
            if last >= target {
                return false;
            }

            return $name_next(target - last, rest);
        }
    };
}

search_fn!(search_12; 12 => search_11);
search_fn!(search_11; 11 => search_10);
search_fn!(search_10; 10 => search_9);
search_fn!(search_9; 9 => search_8);
search_fn!(search_8; 8 => search_7);
search_fn!(search_7; 7 => search_6);
search_fn!(search_6; 6 => search_5);
search_fn!(search_5; 5 => search_4);
search_fn!(search_4; 4 => search_3);
search_fn!(search_3; 3 => search_2);
search_fn!(search_2; 2 => search_1);

#[inline(always)]
unsafe fn search_1(target: u64, v: &[NonZero<u64>; 1]) -> bool {
    match v {
        [last] => {
            return target == last.get();
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
            1 => search_1(target, init.try_into().unwrap_unchecked()),
            2 => search_2(target, init.try_into().unwrap_unchecked()),
            3 => search_3(target, init.try_into().unwrap_unchecked()),
            4 => search_4(target, init.try_into().unwrap_unchecked()),
            5 => search_5(target, init.try_into().unwrap_unchecked()),
            6 => search_6(target, init.try_into().unwrap_unchecked()),
            7 => search_7(target, init.try_into().unwrap_unchecked()),
            8 => search_8(target, init.try_into().unwrap_unchecked()),
            9 => search_9(target, init.try_into().unwrap_unchecked()),
            10 => search_10(target, init.try_into().unwrap_unchecked()),
            11 => search_11(target, init.try_into().unwrap_unchecked()),
            12 => search_12(target, init.try_into().unwrap_unchecked()),
            _ => unreachable_unchecked(),
        } {
            sum += target;
        }
        v_len = 0;
    }

    sum
}

macro_rules! search_fn {
    ($name:ident; $n:expr => $name_next:ident) => {
        unsafe fn $name(target: u64, v: &[NonZero<u64>; $n]) -> bool {
            let [rest @ .., last] = v;

            let last = last.get();

            if target % last == 0 {
                if $name_next(target / last, rest) {
                    return true;
                }
            }

            if last >= target {
                return false;
            }

            let size = if last >= 100 {
                1000
            } else if last >= 10 {
                100
            } else {
                10
            };
            if (target - last) % size == 0 {
                if $name_next((target - last) / size, rest) {
                    return true;
                }
            }

            return $name_next(target - last, rest);
        }
    };
}

search_fn!(search_p2_12; 12 => search_p2_11);
search_fn!(search_p2_11; 11 => search_p2_10);
search_fn!(search_p2_10; 10 => search_p2_9);
search_fn!(search_p2_9; 9 => search_p2_8);
search_fn!(search_p2_8; 8 => search_p2_7);
search_fn!(search_p2_7; 7 => search_p2_6);
search_fn!(search_p2_6; 6 => search_p2_5);
search_fn!(search_p2_5; 5 => search_p2_4);
search_fn!(search_p2_4; 4 => search_p2_3);
search_fn!(search_p2_3; 3 => search_p2_2);
search_fn!(search_p2_2; 2 => search_p2_1);

#[inline(always)]
unsafe fn search_p2_1(target: u64, v: &[NonZero<u64>; 1]) -> bool {
    match v {
        [last] => {
            return target == last.get();
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
        if match init.len() {
            1 => search_p2_1(target, init.try_into().unwrap_unchecked()),
            2 => search_p2_2(target, init.try_into().unwrap_unchecked()),
            3 => search_p2_3(target, init.try_into().unwrap_unchecked()),
            4 => search_p2_4(target, init.try_into().unwrap_unchecked()),
            5 => search_p2_5(target, init.try_into().unwrap_unchecked()),
            6 => search_p2_6(target, init.try_into().unwrap_unchecked()),
            7 => search_p2_7(target, init.try_into().unwrap_unchecked()),
            8 => search_p2_8(target, init.try_into().unwrap_unchecked()),
            9 => search_p2_9(target, init.try_into().unwrap_unchecked()),
            10 => search_p2_10(target, init.try_into().unwrap_unchecked()),
            11 => search_p2_11(target, init.try_into().unwrap_unchecked()),
            12 => search_p2_12(target, init.try_into().unwrap_unchecked()),
            _ => unreachable_unchecked(),
        } {
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
