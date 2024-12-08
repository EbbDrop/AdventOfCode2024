use aoc_runner_derive::aoc;
use tinyvec::ArrayVec;

use crate::memchr_inv::OneInv;

#[cfg(not(test))]
const SIZE: i32 = 50;
#[cfg(test)]
const SIZE: i32 = 12;

const SIZE1: i32 = SIZE + 1;

/// Has the `SIZE` lsb set
const FIELD_SIZE: u64 = 2u64.pow(SIZE as u32) - 1;

const FREQ_RANGE: usize = (b'z' - b'0' + 1) as usize;

#[aoc(day8, part1)]
pub fn part1(s: &str) -> u32 {
    unsafe { part1_inner(s) }
}

// const SHIFT_LUT: [u64; (SIZE * SIZE * 2) as usize] = {
//     let mut lut = [0; (SIZE * SIZE * 2) as usize];

//     let mut x = 0;
//     while x < SIZE {
//         let mut diff_x = -SIZE + 1;

//         let field = 1 << x;

//         while diff_x < SIZE {
//             lut[(x * SIZE * 2 + diff_x + SIZE - 1) as usize] = if diff_x.is_positive() {
//                 field << diff_x
//             } else {
//                 field >> -diff_x
//             };

//             diff_x += 1;
//         }

//         x += 1;
//     }

//     lut
// };

unsafe fn part1_inner(s: &str) -> u32 {
    #[cfg(not(test))]
    const SIZE: i16 = 50;
    #[cfg(test)]
    const SIZE: i16 = 12;

    const SIZE1: i16 = SIZE + 1;

    let s = s.as_bytes();

    let mut masts: [ArrayVec<[(i16, i16); 3]>; FREQ_RANGE] =
        [ArrayVec::from_array_empty([(0, 0); 3]); FREQ_RANGE];

    let mut antinodes = [0u64; SIZE as usize];

    for i in unsafe { OneInv::new_unchecked(b'.').iter(s) } {
        if s[i] == b'\n' {
            continue;
        }
        let f = s[i] - b'0';
        let i = i as i16;

        let new_x = i % SIZE1;
        let new_y = i / SIZE1;

        // numbers[masts[f as usize].len()] += 1;

        for (mast_y, mast_x) in masts.get_unchecked(f as usize) {
            let diff_x = mast_x - new_x;
            let diff_y = new_y - mast_y;

            if *mast_y >= diff_y {
                let node_y = mast_y - diff_y;

                *antinodes.get_unchecked_mut(node_y as usize) |= if diff_x.is_positive() {
                    1 << mast_x << diff_x
                } else {
                    1 << mast_x >> -diff_x
                };
            }

            if new_y + diff_y < SIZE {
                let node_y = new_y + diff_y;

                *antinodes.get_unchecked_mut(node_y as usize) |= if diff_x.is_positive() {
                    1 << new_x >> diff_x
                } else {
                    1 << new_x << -diff_x
                };
            }
        }

        masts[f as usize].try_push((new_y, new_x));
    }
    // for y in 0..SIZE {
    //     for x in 0..SIZE {
    //         print!(
    //             "{}",
    //             if antinodes[y as usize] & 1 << x != 0 {
    //                 '#'
    //             } else {
    //                 s[(y * SIZE1 + x) as usize] as char
    //             }
    //         )
    //     }
    //     println!("");
    // }

    antinodes
        .iter()
        .map(|field| (field & FIELD_SIZE).count_ones())
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(s: &str) -> u32 {
    unsafe { part2_inner(s) }
}

unsafe fn part2_inner(s: &str) -> u32 {
    #[cfg(not(test))]
    const SIZE: i16 = 50;
    #[cfg(test)]
    const SIZE: i16 = 12;

    const SIZE1: i16 = SIZE + 1;

    let s = s.as_bytes();

    let mut masts: [ArrayVec<[(i16, i16); 3]>; FREQ_RANGE] =
        [ArrayVec::from_array_empty([(0, 0); 3]); FREQ_RANGE];

    let mut antinodes = [0u64; SIZE as usize];

    for i in unsafe { OneInv::new_unchecked(b'.').iter(s) } {
        if s[i] == b'\n' {
            continue;
        }
        let f = s[i] - b'0';
        let i = i as i16;

        let new_x = i % SIZE1;
        let new_y = i / SIZE1;

        // numbers[masts[f as usize].len()] += 1;

        for (mast_y, mast_x) in masts.get_unchecked(f as usize) {
            let diff_x_o = mast_x - new_x;
            let diff_y_o = new_y - mast_y;

            for k in 0.. {
                let diff_x = diff_x_o * k;
                let diff_y = diff_y_o * k;

                if *mast_y >= diff_y {
                    let node_y = mast_y - diff_y;

                    let field = if diff_x.is_positive() {
                        1 << mast_x << diff_x
                    } else {
                        1 << mast_x >> -diff_x
                    };
                    if field == 0 {
                        break;
                    } else {
                        *antinodes.get_unchecked_mut(node_y as usize) |= field;
                    }
                } else {
                    break;
                }
            }

            for k in 0.. {
                let diff_x = diff_x_o * k;
                let diff_y = diff_y_o * k;

                if new_y + diff_y < SIZE {
                    let node_y = new_y + diff_y;

                    let field = if diff_x.is_positive() {
                        1 << new_x >> diff_x
                    } else {
                        1 << new_x << -diff_x
                    };
                    if field == 0 {
                        break;
                    } else {
                        *antinodes.get_unchecked_mut(node_y as usize) |= field;
                    }
                } else {
                    break;
                }
            }
        }

        masts[f as usize].try_push((new_y, new_x));
    }
    // for y in 0..SIZE {
    //     for x in 0..SIZE {
    //         print!(
    //             "{}",
    //             if antinodes[y as usize] & 1 << x != 0 {
    //                 '#'
    //             } else {
    //                 s[(y * SIZE1 + x) as usize] as char
    //             }
    //         )
    //     }
    //     println!("");
    // }

    antinodes
        .iter()
        .map(|field| (field & FIELD_SIZE).count_ones())
        .sum()
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
