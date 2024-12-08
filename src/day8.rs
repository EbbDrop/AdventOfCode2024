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
    unsafe { part1_inner(s) }
}

unsafe fn part1_inner(s: &str) -> u64 {
    #[cfg(not(test))]
    const SIZE: i16 = 50;
    #[cfg(test)]
    const SIZE: i16 = 12;

    const SIZE1: i16 = SIZE + 1;
    let s = s.as_bytes();

    let mut masts: [ArrayVec<[i16; 3]>; FREQ_RANGE] =
        [ArrayVec::from_array_empty([0; 3]); FREQ_RANGE];

    let mut antinodes = [false; (SIZE * SIZE) as usize];
    let mut total_antinotedes = 0;

    // let mut numbers = [0; 5];

    for i in unsafe { OneInv::new_unchecked(b'.').iter(s) } {
        if s[i] == b'\n' {
            continue;
        }
        let f = s[i] - b'0';
        let i = i as i16;

        let new_x = i % SIZE1;
        let new_y = i / SIZE1;

        // numbers[masts[f as usize].len()] += 1;

        for mast_i in &masts[f as usize] {
            let mast_x = mast_i % SIZE1;
            let mast_y = mast_i / SIZE1;

            let diff_x = mast_x - new_x;
            let diff_y = new_y - mast_y;

            let node_x = mast_x + diff_x;
            if node_x >= 0 && node_x < SIZE && mast_y >= diff_y {
                let node_y = mast_y - diff_y;

                total_antinotedes +=
                    !antinodes.get_unchecked((node_y * SIZE + node_x) as usize) as u64;
                *antinodes.get_unchecked_mut((node_y * SIZE + node_x) as usize) = true;
            }

            let node_x = new_x - diff_x;
            if node_x >= 0 && node_x < SIZE && new_y + diff_y < SIZE {
                let node_y = new_y + diff_y;

                total_antinotedes +=
                    !antinodes.get_unchecked((node_y * SIZE + node_x) as usize) as u64;
                *antinodes.get_unchecked_mut((node_y * SIZE + node_x) as usize) = true;
            }
        }

        masts[f as usize].try_push(i);
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

                let node_x = mast_x + diff_x;
                if node_x >= 0 && node_x < SIZE && mast_y >= diff_y {
                    let node_y = mast_y - diff_y;
                    set_node(node_x, node_y);
                } else {
                    break;
                }
            }

            for k in 0.. {
                let diff_x = o_diff_x * k;
                let diff_y = o_diff_y * k;

                let node_x = new_x - diff_x;
                if node_x >= 0 && node_x < SIZE && new_y + diff_y < SIZE {
                    let node_y = new_y + diff_y;
                    set_node(node_x, node_y);
                } else {
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
