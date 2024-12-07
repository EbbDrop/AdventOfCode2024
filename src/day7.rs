use std::{hint::unreachable_unchecked, mem::MaybeUninit, num::NonZero};

use aoc_runner_derive::aoc;

macro_rules! search_fn {
    ($name:ident($a1:ident, $($a:ident),*) => $name_next:ident) => {
        #[inline(always)]
        unsafe fn $name(target: u64, $($a: NonZero<u64>,)* $a1: NonZero<u64>) -> bool {
            let last = $a1.get();
            if last > target {
                return false;
            }

            if target % last == 0 {
                if $name_next(target / last, $($a),*) {
                    return true;
                }
            }

            return $name_next(target - last, $($a),*);
        }
    };
}

search_fn!(search_12(a,b,c,d,e,f,g,h,i,j,k,l) => search_11);
search_fn!(search_11(a,b,c,d,e,f,g,h,i,j,k) => search_10);
search_fn!(search_10(a,b,c,d,e,f,g,h,i,j) => search_9);
search_fn!(search_9(a,b,c,d,e,f,g,h,i) => search_8);
search_fn!(search_8(a,b,c,d,e,f,g,h) => search_7);
search_fn!(search_7(a,b,c,d,e,f,g) => search_6);
search_fn!(search_6(a,b,c,d,e,f) => search_5);
search_fn!(search_5(a,b,c,d,e) => search_4);
search_fn!(search_4(a,b,c,d) => search_3);
search_fn!(search_3(a,b,c) => search_2);
search_fn!(search_2(a,b) => search_1);

#[inline(always)]
fn search_1(target: u64, last: NonZero<u64>) -> bool {
    return target == last.get();
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
        if match init {
            [a, b, c, d, e, f, g, h, i, j, k, l] => {
                search_12(target, *a, *b, *c, *d, *e, *f, *g, *h, *i, *j, *k, *l)
            }
            [a, b, c, d, e, f, g, h, i, j, k] => {
                search_11(target, *a, *b, *c, *d, *e, *f, *g, *h, *i, *j, *k)
            }
            [a, b, c, d, e, f, g, h, i, j] => {
                search_10(target, *a, *b, *c, *d, *e, *f, *g, *h, *i, *j)
            }
            [a, b, c, d, e, f, g, h, i] => search_9(target, *a, *b, *c, *d, *e, *f, *g, *h, *i),
            [a, b, c, d, e, f, g, h] => search_8(target, *a, *b, *c, *d, *e, *f, *g, *h),
            [a, b, c, d, e, f, g] => search_7(target, *a, *b, *c, *d, *e, *f, *g),
            [a, b, c, d, e, f] => search_6(target, *a, *b, *c, *d, *e, *f),
            [a, b, c, d, e] => search_5(target, *a, *b, *c, *d, *e),
            [a, b, c, d] => search_4(target, *a, *b, *c, *d),
            [a, b, c] => search_3(target, *a, *b, *c),
            [a, b] => search_2(target, *a, *b),
            [a] => search_1(target, *a),
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
