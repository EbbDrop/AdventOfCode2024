use std::mem::transmute;

use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone)]
struct Pos {
    x: i16,
    y: i16,
}

const fn keypad_pos(i: usize) -> Pos {
    match i {
        0 => Pos { x: 1, y: 3 },
        1 => Pos { x: 0, y: 2 },
        2 => Pos { x: 1, y: 2 },
        3 => Pos { x: 2, y: 2 },
        4 => Pos { x: 0, y: 1 },
        5 => Pos { x: 1, y: 1 },
        6 => Pos { x: 2, y: 1 },
        7 => Pos { x: 0, y: 0 },
        8 => Pos { x: 1, y: 0 },
        9 => Pos { x: 2, y: 0 },
        10 => Pos { x: 2, y: 3 },
        _ => panic!(),
    }
}

const fn movepad_pos(i: char) -> Pos {
    match i {
        '^' => Pos { x: 1, y: 0 },
        'A' => Pos { x: 2, y: 0 },
        '<' => Pos { x: 0, y: 1 },
        'v' => Pos { x: 1, y: 1 },
        '>' => Pos { x: 2, y: 1 },
        _ => panic!(),
    }
}

const fn press_mp2(from_pos: Pos, to_pos: Pos) -> u64 {
    // let (mut to_move, dir) = if to_pos.y > from_pos.y {
    //     (to_pos.y - from_pos.y, 'v')
    // } else {
    //     (from_pos.y - to_pos.y, '^')
    // };
    // while to_move != 0 {
    //     print!("{}", dir);
    //     to_move -= 1;
    // }
    // let (mut to_move, dir) = if to_pos.x > from_pos.x {
    //     (to_pos.x - from_pos.x, '>')
    // } else {
    //     (from_pos.x - to_pos.x, '<')
    // };
    // while to_move != 0 {
    //     print!("{}", dir);
    //     to_move -= 1;
    // }
    // print!("A");
    (from_pos.x.abs_diff(to_pos.x) + from_pos.y.abs_diff(to_pos.y) + 1) as u64
}

const fn press_mp1(from_pos: Pos, to_pos: Pos) -> u64 {
    if from_pos.x == to_pos.x && from_pos.y == to_pos.y {
        return press_mp2(movepad_pos('A'), movepad_pos('A'));
    }

    let mut sum_y_first = 0;
    if !(from_pos.x == 0 && to_pos.y == 0) {
        let mut move_pad_pos = movepad_pos('A');
        let (mut to_move, dir) = if to_pos.y > from_pos.y {
            (to_pos.y - from_pos.y, 'v')
        } else {
            (from_pos.y - to_pos.y, '^')
        };
        while to_move != 0 {
            let next_pos = movepad_pos(dir);
            sum_y_first += press_mp2(move_pad_pos, next_pos);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        let (mut to_move, dir) = if to_pos.x > from_pos.x {
            (to_pos.x - from_pos.x, '>')
        } else {
            (from_pos.x - to_pos.x, '<')
        };
        while to_move != 0 {
            let next_pos = movepad_pos(dir);
            sum_y_first += press_mp2(move_pad_pos, next_pos);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        sum_y_first += press_mp2(move_pad_pos, movepad_pos('A'));
    } else {
        sum_y_first = u64::MAX;
    }

    let mut sum_x_first = 0;
    if !(from_pos.y == 0 && to_pos.x == 0) {
        let mut move_pad_pos = movepad_pos('A');
        let (mut to_move, dir) = if to_pos.x > from_pos.x {
            (to_pos.x - from_pos.x, '>')
        } else {
            (from_pos.x - to_pos.x, '<')
        };
        while to_move != 0 {
            let next_pos = movepad_pos(dir);
            sum_x_first += press_mp2(move_pad_pos, next_pos);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        let (mut to_move, dir) = if to_pos.y > from_pos.y {
            (to_pos.y - from_pos.y, 'v')
        } else {
            (from_pos.y - to_pos.y, '^')
        };
        while to_move != 0 {
            let next_pos = movepad_pos(dir);
            sum_x_first += press_mp2(move_pad_pos, next_pos);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        sum_x_first += press_mp2(move_pad_pos, movepad_pos('A'));
    } else {
        sum_x_first = u64::MAX;
    }

    if sum_x_first < sum_y_first {
        sum_x_first
    } else {
        sum_y_first
    }
}

const fn press_number(from_pos: Pos, to_pos: Pos) -> u64 {
    if from_pos.x == to_pos.x && from_pos.y == to_pos.y {
        return press_mp1(movepad_pos('A'), movepad_pos('A'));
    }

    let mut sum_y_first = 0;
    if !(from_pos.x == 0 && to_pos.y == 3) {
        let mut move_pad_pos = movepad_pos('A');
        let (mut to_move, dir) = if to_pos.y > from_pos.y {
            (to_pos.y - from_pos.y, 'v')
        } else {
            (from_pos.y - to_pos.y, '^')
        };
        while to_move != 0 {
            let next_pos = movepad_pos(dir);
            sum_y_first += press_mp1(move_pad_pos, next_pos);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        let (mut to_move, dir) = if to_pos.x > from_pos.x {
            (to_pos.x - from_pos.x, '>')
        } else {
            (from_pos.x - to_pos.x, '<')
        };
        while to_move != 0 {
            let next_pos = movepad_pos(dir);
            sum_y_first += press_mp1(move_pad_pos, next_pos);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        sum_y_first += press_mp1(move_pad_pos, movepad_pos('A'));
    } else {
        sum_y_first = u64::MAX;
    }

    let mut sum_x_first = 0;
    if !(from_pos.y == 3 && to_pos.x == 0) {
        let mut move_pad_pos = movepad_pos('A');
        let (mut to_move, dir) = if to_pos.x > from_pos.x {
            (to_pos.x - from_pos.x, '>')
        } else {
            (from_pos.x - to_pos.x, '<')
        };
        while to_move != 0 {
            let next_pos = movepad_pos(dir);
            sum_x_first += press_mp1(move_pad_pos, next_pos);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        let (mut to_move, dir) = if to_pos.y > from_pos.y {
            (to_pos.y - from_pos.y, 'v')
        } else {
            (from_pos.y - to_pos.y, '^')
        };
        while to_move != 0 {
            let next_pos = movepad_pos(dir);
            sum_x_first += press_mp1(move_pad_pos, next_pos);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        sum_x_first += press_mp1(move_pad_pos, movepad_pos('A'));
    } else {
        sum_x_first = u64::MAX;
    }

    if sum_x_first < sum_y_first {
        sum_x_first
    } else {
        sum_y_first
    }
}

const fn part1_inner(a: usize, b: usize, c: usize) -> u64 {
    press_number(keypad_pos(10), keypad_pos(a))
        + press_number(keypad_pos(a), keypad_pos(b))
        + press_number(keypad_pos(b), keypad_pos(c))
        + press_number(keypad_pos(c), keypad_pos(10))
}

static LUT_P1: [u64; 10usize.pow(3)] = const {
    let mut lut = [0u64; 10usize.pow(3)];

    let mut a = 0;
    while a < 10 {
        let mut b = 0;
        while b < 10 {
            let mut c = 0;
            while c < 10 {
                let code = a as u64 * 100 + b as u64 * 10 + c as u64;
                lut[a * 100 + b * 10 + c] = part1_inner(a, b, c) * code;
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }

    lut
};

#[aoc(day21, part1)]
pub fn part1(s: &str) -> u64 {
    let s = s.as_bytes();
    let mut sum = 0;
    unsafe {
        let mut i = 0;
        for _ in 0..5 {
            let idx = (*s.get_unchecked(i + 0) as usize) * 100
                + (*s.get_unchecked(i + 1) as usize) * 10
                + (*s.get_unchecked(i + 2) as usize) * 1
                - (b'0' as usize * 111);

            sum += LUT_P1[idx];
            i += 5;
        }
    }

    sum
}

static LUT_P2: [u64; 10usize.pow(3)] =
    unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day21.bin"))) };

#[aoc(day21, part2)]
pub fn part2(s: &str) -> u64 {
    let s = s.as_bytes();
    let mut sum = 0;
    unsafe {
        let mut i = 0;
        for _ in 0..5 {
            let idx = (*s.get_unchecked(i + 0) as usize) * 100
                + (*s.get_unchecked(i + 1) as usize) * 10
                + (*s.get_unchecked(i + 2) as usize) * 1
                - (b'0' as usize * 111);

            sum += LUT_P2[idx];
            i += 5;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_inner() {
        assert_eq!(part1_inner(0, 2, 9), 68);
        assert_eq!(part1_inner(9, 8, 0), 60);
        assert_eq!(part1_inner(1, 7, 9), 68);
        assert_eq!(part1_inner(4, 5, 6), 64);
        assert_eq!(part1_inner(3, 7, 9), 64);
    }

    #[test]
    fn example_part1() {
        let s = r"029A
980A
179A
456A
379A
";

        assert_eq!(part1(s), 126384);
    }
}
