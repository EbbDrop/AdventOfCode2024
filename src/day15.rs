use std::mem::MaybeUninit;

use aoc_runner_derive::aoc;
// use memchr::{memchr2, memrchr2};
use tinyvec::ArrayVec;

#[cfg(test)]
const SIZE: usize = 10;
#[cfg(not(test))]
const SIZE: usize = 50;

const SIZE1: usize = SIZE + 1;

#[aoc(day15, part1)]
pub fn part1(s: &str) -> u64 {
    unsafe { inner_part1(s.as_bytes()) }
}

unsafe fn memchr2(needle1: u8, needle2: u8, haystack: &[u8]) -> usize {
    let mut i = 0;
    loop {
        if *haystack.get_unchecked(i) == needle1 || *haystack.get_unchecked(i) == needle2 {
            return i;
        }
        i += 1;
    }
}

unsafe fn memrchr2(needle1: u8, needle2: u8, haystack: &[u8]) -> usize {
    let mut i = haystack.len() - 1;
    loop {
        if *haystack.get_unchecked(i) == needle1 || *haystack.get_unchecked(i) == needle2 {
            return i;
        }
        i -= 1;
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_part1(s: &[u8]) -> u64 {
    let robot_i = memchr::memchr(b'@', s).unwrap();
    let mut robot = (robot_i / SIZE1) * SIZE + robot_i % SIZE1;

    let mut field = [MaybeUninit::uninit(); SIZE * SIZE];

    let mut sum = 0;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let c = *s.get_unchecked(y * SIZE1 + x);
            if c == b'O' {
                sum += 100 * y + x;
            }
            field.get_unchecked_mut(y * SIZE + x).write(c);
        }
    }
    field.get_unchecked_mut(robot).write(b'.');
    let mut field = std::mem::transmute::<_, [u8; SIZE * SIZE]>(field);

    let mut i = SIZE * SIZE1 + 1;
    while i < s.len() {
        let c = *s.get_unchecked(i);
        if c == b'\n' {
            i += 1;
            continue;
        }
        let mut moves_left = 1;
        while *s.get_unchecked(i + 1) == c {
            i += 1;
            moves_left += 1;
        }

        if c == b'<' {
            let mut p = memrchr2(b'.', b'#', &field[..robot]);
            while moves_left > 0 {
                if *field.get_unchecked(p) == b'.' {
                    if *field.get_unchecked(robot - 1) == b'O' {
                        sum -= robot - 1 - p;
                        *field.get_unchecked_mut(p) = b'O';
                        *field.get_unchecked_mut(robot - 1) = b'.';
                    }
                    robot -= 1;
                    moves_left -= 1;
                    p -= 1;
                } else if *field.get_unchecked(p) == b'O' {
                    p = memrchr2(b'.', b'#', &field[..p]);
                } else {
                    moves_left = 0;
                }
            }
        } else if c == b'v' {
            let mut p = robot + SIZE;

            while moves_left > 0 {
                if *field.get_unchecked(p) == b'.' {
                    if *field.get_unchecked(robot + SIZE) == b'O' {
                        sum += 100 * (p - robot - SIZE) / SIZE;
                        *field.get_unchecked_mut(p) = b'O';
                        *field.get_unchecked_mut(robot + SIZE) = b'.';
                    }
                    robot += SIZE;
                    moves_left -= 1;
                    p += SIZE;
                } else if *field.get_unchecked(p) == b'O' {
                    while *field.get_unchecked(p) == b'O' {
                        p += SIZE;
                    }
                } else {
                    moves_left = 0;
                }
            }
        } else if c == b'^' {
            let mut p = robot - SIZE;
            while moves_left > 0 {
                if *field.get_unchecked(p) == b'.' {
                    if *field.get_unchecked(robot - SIZE) == b'O' {
                        sum -= 100 * (robot - SIZE - p) / SIZE;
                        *field.get_unchecked_mut(p) = b'O';
                        *field.get_unchecked_mut(robot - SIZE) = b'.';
                    }
                    robot -= SIZE;
                    moves_left -= 1;
                    p -= SIZE;
                } else if *field.get_unchecked(p) == b'O' {
                    while *field.get_unchecked(p) == b'O' {
                        p -= SIZE;
                    }
                } else {
                    moves_left = 0;
                }
            }
        } else {
            let mut p = memchr2(b'.', b'#', &field[robot + 1..]) + robot + 1;
            while moves_left > 0 {
                if *field.get_unchecked(p) == b'.' {
                    if *field.get_unchecked(robot + 1) == b'O' {
                        sum += p - robot - 1;
                        *field.get_unchecked_mut(p) = b'O';
                        *field.get_unchecked_mut(robot + 1) = b'.';
                    }
                    robot += 1;
                    moves_left -= 1;
                    p += 1;
                } else if *field.get_unchecked(p) == b'O' {
                    p = memchr2(b'.', b'#', &field[p + 1..]) + p + 1;
                } else {
                    moves_left = 0;
                }
            }
        }

        // let mut new_field = field.clone();
        // *new_field.get_unchecked_mut(robot) = b'@';

        // println!("Move: {}", c as char,);
        // for y in 0..SIZE {
        //     println!(
        //         "{}",
        //         String::from_utf8_lossy(&new_field[y * SIZE..(y + 1) * SIZE]),
        //     );
        // }
        // println!("Sum: {}", sum);

        i += 1;
    }

    // let mut sum = 0;
    // for y in 0..SIZE {
    //     for x in 0..SIZE {
    //         if field[y * SIZE1 + x] == b'O' {
    //             sum += 100 * y + x;
    //         }
    //     }
    // }

    sum as u64
}

#[aoc(day15, part2)]
pub fn part2(s: &str) -> u64 {
    unsafe { inner_part2(s.as_bytes()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_part2(s: &[u8]) -> u64 {
    const WIDTH: usize = SIZE * 2;
    const HIGHT: usize = SIZE;

    let mut field = [MaybeUninit::uninit(); HIGHT * WIDTH];

    let mut robot = 0;

    let mut j = 0;
    for i in 0..SIZE * SIZE1 {
        match *s.get_unchecked(i) {
            b'#' => {
                field.get_unchecked_mut(j + 0).write(b'#');
                field.get_unchecked_mut(j + 1).write(b'#');
            }
            b'O' => {
                field.get_unchecked_mut(j + 0).write(b'[');
                field.get_unchecked_mut(j + 1).write(b']');
            }
            b'@' => {
                robot = j;
                field.get_unchecked_mut(j + 0).write(b'.');
                field.get_unchecked_mut(j + 1).write(b'.');
            }
            b'\n' => {
                j -= 2;
            }
            _ => {
                field.get_unchecked_mut(j + 0).write(b'.');
                field.get_unchecked_mut(j + 1).write(b'.');
            }
        }
        j += 2;
    }

    let mut field = std::mem::transmute::<_, [u8; WIDTH * HIGHT]>(field);

    let mut stack = ArrayVec::from_array_empty([0; 20]);
    let mut creates = ArrayVec::from_array_empty([0; 20]);

    // let mut new_field = field.clone();
    // *new_field.get_unchecked_mut(robot) = b'@';
    // for y in 0..HIGHT {
    //     println!(
    //         "{}",
    //         String::from_utf8_lossy(&new_field[y * WIDTH..(y + 1) * WIDTH])
    //     );
    // }

    let mut i = SIZE * SIZE1;
    while i < s.len() {
        let c = *s.get_unchecked(i);
        if c == b'\n' {
            i += 1;
            continue;
        }
        if c == b'<' {
            let p = memrchr2(b'.', b'#', &field[..robot]);
            if *field.get_unchecked(p) == b'.' {
                for i in p..=robot - 1 {
                    *field.get_unchecked_mut(i) = *field.get_unchecked(i + 1);
                }
                robot -= 1;
            }
        } else if c == b'v' {
            'cancel: {
                stack.push(robot + WIDTH);
                while let Some(t) = stack.pop() {
                    if *field.get_unchecked(t) == b']' {
                        if !creates.contains(&(t - 1)) {
                            creates.push(t - 1);
                            stack.push(t + WIDTH);
                            stack.push(t + WIDTH - 1);
                        }
                    } else if *field.get_unchecked(t) == b'[' {
                        if !creates.contains(&(t)) {
                            creates.push(t);
                            stack.push(t + WIDTH);
                            stack.push(t + WIDTH + 1);
                        }
                    } else if *field.get_unchecked(t) == b'#' {
                        stack.clear();
                        creates.clear();
                        break 'cancel;
                    }
                }
                creates.sort_unstable_by(|a, b| b.cmp(a));

                for c in &creates {
                    let c = *c;
                    *field.get_unchecked_mut(c) = b'.';
                    *field.get_unchecked_mut(c + 1) = b'.';
                    *field.get_unchecked_mut(c + WIDTH) = b'[';
                    *field.get_unchecked_mut(c + WIDTH + 1) = b']';
                }
                stack.clear();
                creates.clear();

                robot += WIDTH;
            }
        } else if c == b'^' {
            'cancel: {
                stack.push(robot - WIDTH);
                while let Some(t) = stack.pop() {
                    if *field.get_unchecked(t) == b']' {
                        if !creates.contains(&(t - 1)) {
                            creates.push(t - 1);
                            stack.push(t - WIDTH);
                            stack.push(t - WIDTH - 1);
                        }
                    } else if *field.get_unchecked(t) == b'[' {
                        if !creates.contains(&t) {
                            creates.push(t);
                            stack.push(t - WIDTH);
                            stack.push(t - WIDTH + 1);
                        }
                    } else if *field.get_unchecked(t) == b'#' {
                        stack.clear();
                        creates.clear();
                        break 'cancel;
                    }
                }
                creates.sort_unstable();

                for c in &creates {
                    let c = *c;
                    *field.get_unchecked_mut(c) = b'.';
                    *field.get_unchecked_mut(c + 1) = b'.';
                    *field.get_unchecked_mut(c - WIDTH) = b'[';
                    *field.get_unchecked_mut(c - WIDTH + 1) = b']';
                }
                stack.clear();
                creates.clear();

                robot -= WIDTH;
            }
        } else {
            let p = memchr2(b'.', b'#', &field[robot + 1..]) + robot + 1;
            if *field.get_unchecked(p) == b'.' {
                for i in (robot + 1..=p).rev() {
                    *field.get_unchecked_mut(i) = *field.get_unchecked(i - 1);
                }
                robot += 1;
            }
        }

        // let mut new_field = field.clone();
        // *new_field.get_unchecked_mut(robot) = b'@';
        // for y in 0..HIGHT {
        //     println!(
        //         "{}",
        //         String::from_utf8_lossy(&new_field[y * WIDTH..(y + 1) * WIDTH])
        //     );
        // }
        // println!("{}\n", sum);

        // use std::io::{stdin, stdout, Read, Write};
        // let mut stdout = stdout();
        // stdout.write(b"Press Enter to continue...").unwrap();
        // stdout.flush().unwrap();
        // stdin().read(&mut [0]).unwrap();

        i += 1;
    }

    let mut sum = 0;
    for i in 0..WIDTH * HIGHT {
        if *field.get_unchecked(i) == b'[' {
            sum += i;
        }
    }

    sum as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    //     const EXAMPLE: &str = r"########
    // #..O.O.#
    // ##@.O..#
    // #...O..#
    // #.#.O..#
    // #...O..#
    // #......#
    // ########

    // <^^>>>vv<v>>v<<
    // ";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 10092);
    }

    // #[test]
    // fn example_part2() {
    //     assert_eq!(part2(EXAMPLE), 9021);
    // }
}
