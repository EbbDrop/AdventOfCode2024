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

    for masts in masts {
        for i in 0..masts.len() {
            let new_x = masts[i] % SIZE1;
            let new_y = masts[i] / SIZE1;

            for other_i in 0..masts.len() {
                if other_i == i {
                    continue;
                }
                let mast_x = masts[other_i] % SIZE1;
                let mast_y = masts[other_i] / SIZE1;

                let node_x = mast_x + mast_x - new_x;
                let node_y = mast_y + mast_y - new_y;

                if node_x >= 0 && node_x < SIZE && node_y >= 0 && node_y < SIZE {
                    set_node(node_x, node_y);
                }
            }
        }
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

    for masts in masts {
        for i in 0..masts.len() {
            let new_x = masts[i] % SIZE1;
            let new_y = masts[i] / SIZE1;

            for other_i in 0..masts.len() {
                if other_i == i {
                    continue;
                }
                let mast_x = masts[other_i] % SIZE1;
                let mast_y = masts[other_i] / SIZE1;

                for k in 0.. {
                    let node_x = mast_x + (mast_x - new_x) * k;
                    let node_y = mast_y + (mast_y - new_y) * k;

                    if node_x >= 0 && node_x < SIZE && node_y >= 0 && node_y < SIZE {
                        set_node(node_x, node_y);
                    } else {
                        break;
                    }
                }
            }
        }
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
