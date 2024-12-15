use std::collections::BTreeSet;

use aoc_runner_derive::aoc;
use tinyvec::ArrayVec;

#[cfg(test)]
const SIZE: usize = 10;
#[cfg(not(test))]
const SIZE: usize = 50;

const SIZE1: usize = SIZE + 1;

#[aoc(day15, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part1(s.as_bytes())
    }
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part1(s: &[u8]) -> u64 {
    let mut robot = memchr::memchr(b'@', s).unwrap();
    let mut field = [0; SIZE * SIZE1];
    field.copy_from_slice(&s[..SIZE * SIZE1]);
    field[robot] = b'.';

    let mut i = SIZE * SIZE1 + 1;
    while i < s.len() {
        let c = s[i];
        if c == b'\n' {
            i += 1;
            continue;
        }
        if c == b'<' {
            let p = memchr::memrchr2(b'.', b'#', &field[..robot]).unwrap();
            if field[p] == b'.' {
                field[p] = field[robot - 1];
                field[robot - 1] = b'.';
                robot -= 1;
            }
        } else if c == b'v' {
            let mut p = robot + SIZE1;
            while field[p] == b'O' {
                p += SIZE1;
            }
            if field[p] == b'.' {
                field[p] = field[robot + SIZE1];
                field[robot + SIZE1] = b'.';
                robot += SIZE1;
            }
        } else if c == b'^' {
            let mut p = robot - SIZE1;
            while field[p] == b'O' {
                p -= SIZE1;
            }
            if field[p] == b'.' {
                field[p] = field[robot - SIZE1];
                field[robot - SIZE1] = b'.';
                robot -= SIZE1;
            }
        } else {
            let p = memchr::memchr2(b'.', b'#', &field[robot + 1..]).unwrap() + robot + 1;
            if field[p] == b'.' {
                field[p] = field[robot + 1];
                field[robot + 1] = b'.';
                robot += 1;
            }
        }

        i += 1;
    }

    let mut sum = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            if field[y * SIZE1 + x] == b'O' {
                sum += 100 * y + x;
            }
        }
    }

    sum as u64
}

#[aoc(day15, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part2(s.as_bytes())
    }
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part2(s: &[u8]) -> u64 {
    const WIDTH: usize = SIZE * 2;
    const HIGHT: usize = SIZE;
    const WIDTH1: usize = WIDTH + 1;

    let mut robot = 0;
    let mut field = [0; HIGHT * WIDTH1];
    let mut j = 0;
    for i in 0..(WIDTH / 2) * (WIDTH / 2 + 1) {
        match s[i] {
            b'#' => {
                field[j + 0] = b'#';
                field[j + 1] = b'#';
            }
            b'O' => {
                field[j + 0] = b'[';
                field[j + 1] = b']';
            }
            b'@' => {
                robot = j;
                field[j + 0] = b'.';
                field[j + 1] = b'.';
            }
            b'\n' => {
                field[j] = b'\n';
                j -= 1;
            }
            _ => {
                field[j + 0] = b'.';
                field[j + 1] = b'.';
            }
        }
        j += 2;
    }

    let mut stack = ArrayVec::from_array_empty([0; 20]);
    let mut creates = BTreeSet::new();

    let mut i = (WIDTH / 2) * (WIDTH / 2 + 1);
    while i < s.len() {
        let c = s[i];
        if c == b'\n' {
            i += 1;
            continue;
        }
        if c == b'<' {
            let p = memchr::memrchr2(b'.', b'#', &field[..robot]).unwrap();
            if field[p] == b'.' {
                for i in p..=robot - 1 {
                    field[i] = field[i + 1];
                }
                robot -= 1;
            }
        } else if c == b'v' {
            'cancel: {
                stack.push(robot + WIDTH1);
                while let Some(t) = stack.pop() {
                    if field[t] == b']' {
                        creates.insert(t - 1);
                        stack.push(t + WIDTH1);
                        stack.push(t + WIDTH1 - 1);
                    } else if field[t] == b'[' {
                        creates.insert(t);
                        stack.push(t + WIDTH1);
                        stack.push(t + WIDTH1 + 1);
                    } else if field[t] == b'#' {
                        stack.clear();
                        creates.clear();
                        break 'cancel;
                    }
                }

                while let Some(c) = creates.pop_last() {
                    field.swap(c, c + WIDTH1);
                    field.swap(c + 1, c + WIDTH1 + 1);
                }

                robot += WIDTH1;
            }
        } else if c == b'^' {
            'cancel: {
                stack.push(robot - WIDTH1);
                while let Some(t) = stack.pop() {
                    if field[t] == b']' {
                        creates.insert(t - 1);
                        stack.push(t - WIDTH1);
                        stack.push(t - WIDTH1 - 1);
                    } else if field[t] == b'[' {
                        creates.insert(t);
                        stack.push(t - WIDTH1);
                        stack.push(t - WIDTH1 + 1);
                    } else if field[t] == b'#' {
                        stack.clear();
                        creates.clear();
                        break 'cancel;
                    }
                }

                while let Some(c) = creates.pop_first() {
                    field.swap(c, c - WIDTH1);
                    field.swap(c + 1, c - WIDTH1 + 1);
                }

                robot -= WIDTH1;
            }
        } else {
            let p = memchr::memchr2(b'.', b'#', &field[robot + 1..]).unwrap() + robot + 1;
            if field[p] == b'.' {
                for i in (robot + 1..=p).rev() {
                    field[i] = field[i - 1];
                }
                robot += 1;
            }
        }

        i += 1;
    }

    let mut sum = 0;
    for y in 0..HIGHT {
        for x in 0..WIDTH {
            if field[y * WIDTH1 + x] == b'[' {
                sum += 100 * y + x;
            }
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
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 10092);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 9021);
    }
}
