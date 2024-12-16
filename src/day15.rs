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
        let mut moves_left = 1;
        while s[i + 1] == c {
            i += 1;
            moves_left += 1;
        }

        if c == b'<' {
            let mut p = memchr::memrchr2(b'.', b'#', &field[..robot]).unwrap();
            while moves_left > 0 {
                if field[p] == b'.' {
                    field.swap(robot - 1, p);
                    robot -= 1;
                    moves_left -= 1;
                    p -= 1;
                } else if field[p] == b'O' {
                    p = memchr::memrchr2(b'.', b'#', &field[..p]).unwrap();
                } else {
                    moves_left = 0;
                }
            }
        } else if c == b'v' {
            let mut p = robot + SIZE1;

            while moves_left > 0 {
                if field[p] == b'.' {
                    field.swap(robot + SIZE1, p);
                    robot += SIZE1;
                    moves_left -= 1;
                    p += SIZE1;
                } else if field[p] == b'O' {
                    while field[p] == b'O' {
                        p += SIZE1;
                    }
                } else {
                    moves_left = 0;
                }
            }
        } else if c == b'^' {
            let mut p = robot - SIZE1;
            while moves_left > 0 {
                if field[p] == b'.' {
                    field.swap(robot - SIZE1, p);
                    robot -= SIZE1;
                    moves_left -= 1;
                    p -= SIZE1;
                } else if field[p] == b'O' {
                    while field[p] == b'O' {
                        p -= SIZE1;
                    }
                } else {
                    moves_left = 0;
                }
            }
        } else {
            let mut p = memchr::memchr2(b'.', b'#', &field[robot + 1..]).unwrap() + robot + 1;
            while moves_left > 0 {
                if field[p] == b'.' {
                    field.swap(robot + 1, p);
                    robot += 1;
                    moves_left -= 1;
                    p += 1;
                } else if field[p] == b'O' {
                    p = memchr::memchr2(b'.', b'#', &field[p + 1..]).unwrap() + p + 1;
                } else {
                    moves_left = 0;
                }
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
    let mut creates = ArrayVec::from_array_empty([0; 20]);

    let mut i = (WIDTH / 2) * (WIDTH / 2 + 1);
    while i < s.len() {
        let c = s[i];
        if c == b'\n' {
            i += 1;
            continue;
        }
        let mut moves_left = 1;
        while s[i + 1] == c {
            i += 1;
            moves_left += 1;
        }

        println!("Move {}*{}:", c as char, moves_left);

        if c == b'<' {
            while moves_left > 0 {
                let mut p = memchr::memrchr2(b'.', b'#', &field[..robot]).unwrap();
                if field[p] == b'.' {
                    let creates = p + 1..robot;
                    while field[p - 1] == b'.' && moves_left != 1 {
                        p -= 1;
                        moves_left -= 1;
                        robot -= 1;
                    }
                    moves_left -= 1;
                    robot -= 1;
                    field.copy_within(creates.start..creates.end, p);
                    field[robot..creates.end].fill(b'.');
                } else {
                    moves_left = 0;
                }
            }
        } else if c == b'v' {
            'cancel: while moves_left > 0 {
                let mut max_move = moves_left;
                stack.push(robot + WIDTH1);
                while let Some(t) = stack.pop() {
                    if field[t] == b']' {
                        if !creates.contains(&(t - 1)) {
                            creates.push(t - 1);
                            stack.push(t + WIDTH1);
                            stack.push(t + WIDTH1 - 1);
                        }
                    } else if field[t] == b'[' {
                        if !creates.contains(&(t)) {
                            creates.push(t);
                            stack.push(t + WIDTH1);
                            stack.push(t + WIDTH1 + 1);
                        }
                    } else if field[t] == b'#' {
                        stack.clear();
                        creates.clear();
                        break 'cancel;
                    } else {
                        for i in 1..max_move {
                            if field[t + WIDTH1 * i] != b'.' {
                                max_move = i;
                                break;
                            }
                        }
                    }
                }
                creates.sort_unstable_by(|a, b| b.cmp(a));

                for c in &creates {
                    let c = *c;
                    field.swap(c, c + WIDTH1 * max_move);
                    field.swap(c + 1, c + WIDTH1 * max_move + 1);
                }
                stack.clear();
                creates.clear();

                robot += WIDTH1 * max_move;
                moves_left -= max_move;
            }
        } else if c == b'^' {
            'cancel: while moves_left > 0 {
                let mut max_move = moves_left;

                stack.push(robot - WIDTH1);
                while let Some(t) = stack.pop() {
                    if field[t] == b']' {
                        if !creates.contains(&(t - 1)) {
                            creates.push(t - 1);
                            stack.push(t - WIDTH1);
                            stack.push(t - WIDTH1 - 1);
                        }
                    } else if field[t] == b'[' {
                        if !creates.contains(&t) {
                            creates.push(t);
                            stack.push(t - WIDTH1);
                            stack.push(t - WIDTH1 + 1);
                        }
                    } else if field[t] == b'#' {
                        stack.clear();
                        creates.clear();
                        break 'cancel;
                    } else {
                        for i in 1..max_move {
                            if field[t - WIDTH1 * i] != b'.' {
                                max_move = i;
                                break;
                            }
                        }
                    }
                }
                creates.sort_unstable();

                for c in &creates {
                    let c = *c;
                    field.swap(c, c - WIDTH1 * max_move);
                    field.swap(c + 1, c - WIDTH1 * max_move + 1);
                }
                stack.clear();
                creates.clear();

                robot -= WIDTH1 * max_move;
                moves_left -= max_move;
            }
        } else {
            while moves_left > 0 {
                let mut p = memchr::memchr2(b'.', b'#', &field[robot + 1..]).unwrap() + robot + 1;
                if field[p] == b'.' {
                    let creates = robot + 1..p;
                    while field[p + 1] == b'.' && moves_left != 1 {
                        p += 1;
                        moves_left -= 1;
                        robot += 1;
                    }
                    moves_left -= 1;
                    robot += 1;
                    field.copy_within(creates.start..creates.end, robot + 1);
                    field[creates.start..=robot].fill(b'.');
                } else {
                    moves_left = 0;
                }
            }
        }

        // let mut new_field = field.clone();
        // new_field[robot] = b'@';

        // println!("{}", String::from_utf8_lossy(&new_field));
        // use std::io::{stdin, stdout, Read, Write};
        // let mut stdout = stdout();
        // stdout.write(b"Press Enter to continue...").unwrap();
        // stdout.flush().unwrap();
        // stdin().read(&mut [0]).unwrap();

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
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    //     const EXAMPLE: &str = r"#######
    // #...#.#
    // #.....#
    // #..OO@#
    // #..O..#
    // #.....#
    // #######

    // <vv<<^^<<^^
    // ";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 10092);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 9021);
    }
}
