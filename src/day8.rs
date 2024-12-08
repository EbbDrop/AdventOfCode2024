use aoc_runner_derive::aoc;
use tinyvec::ArrayVec;

use crate::memchr_inv::OneInv;

#[cfg(not(test))]
const SIZE: i32 = 50;
#[cfg(test)]
const SIZE: i32 = 12;

const SIZE1: i32 = SIZE + 1;

const FREQ_RANGE: usize = (b'z' - b'0' + 1) as usize;

#[aoc(day8, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        part1_inner(s)
    }
}

fn part1_inner(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut masts: [ArrayVec<[i32; 5]>; FREQ_RANGE] =
        [ArrayVec::from_array_empty([0; 5]); FREQ_RANGE];

    let mut antinodes = [false; (SIZE * SIZE) as usize];
    let mut total_antinotedes = 0;

    let mut set_node = |x, y| {
        if x < 0 || y < 0 || x >= SIZE || y >= SIZE {
            return;
        }
        total_antinotedes += !antinodes[(y * SIZE + x) as usize] as u64;
        antinodes[(y * SIZE + x) as usize] = true;
    };

    for i in unsafe { OneInv::new_unchecked(b'.').iter(s) } {
        if s[i] == b'\n' {
            continue;
        }
        let f = s[i] - b'0';
        let i = i as i32;

        let new_x = i % SIZE1;
        let new_y = i / SIZE1;
        for mast_i in &masts[f as usize] {
            let mast_x = mast_i % SIZE1;
            let mast_y = mast_i / SIZE1;

            let diff_x = new_x.abs_diff(mast_x) as i32;
            let diff_y = new_y.abs_diff(mast_y) as i32;

            if new_x > mast_x {
                set_node(mast_x - diff_x, mast_y - diff_y);
                set_node(new_x + diff_x, new_y + diff_y);
            } else {
                set_node(mast_x + diff_x, mast_y - diff_y);
                set_node(new_x - diff_x, new_y + diff_y);
            }
        }

        masts[f as usize].push(i);
    }

    // for y in 0..SIZE {
    //     for x in 0..SIZE {
    //         print!(
    //             "{}",
    //             if antinodes[(y * SIZE + x) as usize] {
    //                 '#'
    //             } else {
    //                 s[(y * SIZE1 + x) as usize] as char
    //             }
    //         )
    //     }
    //     println!("");
    // }

    total_antinotedes
}

#[aoc(day8, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        part2_inner(s)
    }
}

fn part2_inner(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut masts: [ArrayVec<[i32; 5]>; FREQ_RANGE] =
        [ArrayVec::from_array_empty([0; 5]); FREQ_RANGE];

    let mut antinodes = [false; (SIZE * SIZE) as usize];
    let mut total_antinotedes = 0;
    let mut set_node = |x, y| {
        if x < 0 || y < 0 || x >= SIZE || y >= SIZE {
            return false;
        }
        total_antinotedes += !antinodes[(y * SIZE + x) as usize] as u64;
        antinodes[(y * SIZE + x) as usize] = true;
        true
    };

    for i in unsafe { OneInv::new_unchecked(b'.').iter(s) } {
        if s[i] == b'\n' {
            continue;
        }
        let f = s[i] - b'0';
        let i = i as i32;

        let new_x = i % SIZE1;
        let new_y = i / SIZE1;
        for mast_i in &masts[f as usize] {
            let mast_x = mast_i % SIZE1;
            let mast_y = mast_i / SIZE1;

            let o_diff_x = new_x.abs_diff(mast_x) as i32;
            let o_diff_y = new_y.abs_diff(mast_y) as i32;

            for k in 0.. {
                let diff_x = o_diff_x * k;
                let diff_y = o_diff_y * k;

                let mut new_node = false;

                if new_x > mast_x {
                    new_node |= set_node(mast_x - diff_x, mast_y - diff_y);
                    new_node |= set_node(new_x + diff_x, new_y + diff_y);
                } else {
                    new_node |= set_node(mast_x + diff_x, mast_y - diff_y);
                    new_node |= set_node(new_x - diff_x, new_y + diff_y);
                }

                if !new_node {
                    break;
                }
            }
        }

        masts[f as usize].push(i);
    }

    total_antinotedes
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 14);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 34);
    }
}
