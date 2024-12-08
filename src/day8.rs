use aoc_runner_derive::aoc;
use tinyvec::ArrayVec;

use crate::memchr_inv::OneInv;

use std::simd::prelude::*;

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

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_inner(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut masts: [ArrayVec<[i32; 4]>; FREQ_RANGE] =
        [ArrayVec::from_array_empty([0; 4]); FREQ_RANGE];

    // let mut numbers = [0; 5];

    for i in unsafe { OneInv::new_unchecked(b'.').iter(s) } {
        if s[i] == b'\n' {
            continue;
        }
        let f = s[i] - b'0';
        let i = i as i32;

        masts[f as usize].push(i);
    }

    let mut antinodes = [false; (SIZE * SIZE) as usize];
    let mut total_antinotedes = 0;

    let mut set_node = |x, y| {
        total_antinotedes += !antinodes[(y * SIZE + x) as usize] as u64;
        antinodes[(y * SIZE + x) as usize] = true;
    };

    for mast in masts {
        match masts.as_slice() {
            [] => {}
            [mast_i, new_i] => {
                let mast_x = mast_i % SIZE1;
                let mast_y = mast_i / SIZE1;
                let new_x = new_i % SIZE1;
                let new_y = new_i / SIZE1;

                let node_x = mast_x + mast_x - new_x;
                let node_y = mast_y - new_y + mast_y;
                if node_x >= 0 && node_x < SIZE && node_y >= 0 && node_y < SIZE {
                    set_node(node_x, node_y);
                }

                let node_x = new_x - mast_x + new_x;
                let node_y = new_y + new_y - mast_y;
                if node_x >= 0 && node_x < SIZE && node_y >= 0 && node_y < SIZE {
                    set_node(node_x, node_y);
                }
            }
            masts => {
                let mast_is = i32x4::load_or_default(masts);
                let mast_xs = mast_is % i32x4::splat(SIZE1);
                let mast_ys = mast_is / i32x4::splat(SIZE1);

                let new_xs = i32x4::splat(new_x);
                let new_ys = i32x4::splat(new_y);

                let node1_xs = mast_xs + mast_xs - new_xs;
                let node1_ys = mast_ys + mast_ys - new_ys;

                let node2_xs = new_xs + new_xs - mast_xs;
                let node2_ys = new_ys + new_ys - mast_ys;

                for i in 0..masts.len() {
                    let node_x = node1_xs.as_array()[i];
                    let node_y = node1_ys.as_array()[i];
                    if node_x >= 0 && node_x < SIZE && node_y >= 0 && node_y < SIZE {
                        set_node(node_x, node_y);
                    }

                    let node_x = node2_xs.as_array()[i];
                    let node_y = node2_ys.as_array()[i];
                    if node_x >= 0 && node_x < SIZE && node_y >= 0 && node_y < SIZE {
                        set_node(node_x, node_y);
                    }
                }
            }
        }
    }

    // for i in 0..5 {
    //     println!("{i}: {}", numbers[i]);
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

    let mut masts: [ArrayVec<[i32; 4]>; FREQ_RANGE] =
        [ArrayVec::from_array_empty([0; 4]); FREQ_RANGE];

    let mut antinodes = [false; (SIZE * SIZE) as usize];
    let mut total_antinotedes = 0;
    let mut set_node = |x, y| {
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

            let o_diff_x = mast_x - new_x;
            let o_diff_y = (new_y - mast_y).abs() as i32;

            for k in 0.. {
                let diff_x = o_diff_x * k;
                let diff_y = o_diff_y * k;

                let mut new_node = false;

                let node_x = mast_x + diff_x;
                if node_x >= 0 && node_x < SIZE && mast_y >= diff_y {
                    let node_y = mast_y - diff_y;

                    new_node = true;
                    set_node(node_x, node_y);
                }

                let node_x = new_x - diff_x;
                if node_x >= 0 && node_x < SIZE && new_y + diff_y < SIZE {
                    let node_y = new_y + diff_y;

                    new_node = true;
                    set_node(node_x, node_y);
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
