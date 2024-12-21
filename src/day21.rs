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

static LUT_P2: [u64; 10usize.pow(3)] =
    unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day21.bin"))) };

#[aoc(day21, part1)]
pub fn part1(s: &str) -> u64 {
    static LCPI0_0: [u8; 32] = [
        3, 8, 9, 10, 3, 13, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0, 7, 10, 11, 12, 7, 15, 0, 1, 0, 0, 0, 0,
        7, 4, 5, 6,
    ];
    static LCPI0_1: [u8; 32] = [
        48, 48, 48, 48, 48, 48, 48, 48, 0, 0, 0, 0, 0, 0, 0, 0, 48, 48, 48, 48, 48, 48, 48, 48, 0,
        0, 0, 0, 48, 48, 48, 48,
    ];
    static LCPI0_2: [u8; 32] = [
        0, 1, 10, 1, 0, 1, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 10, 1, 0, 1, 10, 1, 0, 0, 0, 0, 0,
        1, 10, 1,
    ];
    static LCPI0_3: [u16; 16] = [100, 1, 100, 1, 0, 0, 0, 0, 100, 1, 100, 1, 0, 0, 100, 1];

    let r: u64;
    unsafe {
        std::arch::asm!(
            "vpermq          {ymm:y}, ymmword ptr [{s}], 99",
            "vpshufb         {ymm:y}, {ymm:y}, ymmword ptr [rip + {LCPI0_0}]",
            "vpsubusb        {ymm:y}, {ymm:y}, ymmword ptr [rip + {LCPI0_1}]",
            "vpmaddubsw      {ymm:y}, {ymm:y}, ymmword ptr [rip + {LCPI0_2}]",
            "vpmaddwd        {ymm:y}, {ymm:y}, ymmword ptr [rip + {LCPI0_3}]",
            "vmovd           {c:e}, {ymm:x}",
            "vpextrd         {a:e}, {ymm:x}, 1",
            "mov             {a:r}, qword ptr [{lut} + 8*{a:r}]",
            "add             {a:r}, qword ptr [{lut} + 8*{c:r}]",
            "vextracti128    {ymm:x}, {ymm:y}, 1",
            "vmovd           {c:e}, {ymm:x}",
            "add             {a:r}, qword ptr [{lut} + 8*{c:r}]",
            "vpextrd         {c:e}, {ymm:x}, 1",
            "add             {a:r}, qword ptr [{lut} + 8*{c:r}]",
            "vpextrd         {c:e}, {ymm:x}, 3",
            "add             {a:r}, qword ptr [{lut} + 8*{c:r}]",

            LCPI0_0 = sym LCPI0_0,
            LCPI0_1 = sym LCPI0_1,
            LCPI0_2 = sym LCPI0_2,
            LCPI0_3 = sym LCPI0_3,
            s = in(reg) s.as_ptr(),
            lut = in(reg) LUT_P1.as_ptr(),
            ymm = out(ymm_reg) _,
            c = out(reg) _,
            a = out(reg) r,
            options(nostack)
        );
    }
    r
}

#[aoc(day21, part2)]
pub fn part2(s: &str) -> u64 {
    static LCPI0_0: [u8; 32] = [
        3, 8, 9, 10, 3, 13, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0, 7, 10, 11, 12, 7, 15, 0, 1, 0, 0, 0, 0,
        7, 4, 5, 6,
    ];
    static LCPI0_1: [u8; 32] = [
        48, 48, 48, 48, 48, 48, 48, 48, 0, 0, 0, 0, 0, 0, 0, 0, 48, 48, 48, 48, 48, 48, 48, 48, 0,
        0, 0, 0, 48, 48, 48, 48,
    ];
    static LCPI0_2: [u8; 32] = [
        0, 1, 10, 1, 0, 1, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 10, 1, 0, 1, 10, 1, 0, 0, 0, 0, 0,
        1, 10, 1,
    ];
    static LCPI0_3: [u16; 16] = [100, 1, 100, 1, 0, 0, 0, 0, 100, 1, 100, 1, 0, 0, 100, 1];

    let r: u64;
    unsafe {
        std::arch::asm!(
            "vpermq          {ymm}, ymmword ptr [{s}], 99",
            "vpshufb         {ymm}, {ymm}, ymmword ptr [rip + {LCPI0_0}]",
            "vpsubusb        {ymm}, {ymm}, ymmword ptr [rip + {LCPI0_1}]",
            "vpmaddubsw      {ymm}, {ymm}, ymmword ptr [rip + {LCPI0_2}]",
            "vpmaddwd        {ymm}, {ymm}, ymmword ptr [rip + {LCPI0_3}]",
            "vmovd           {t:e}, {ymm:x}",
            "vpextrd         {r:e}, {ymm:x}, 1",
            "mov             {r:r}, qword ptr [{lut} + 8*{r:r}]",
            "add             {r:r}, qword ptr [{lut} + 8*{t:r}]",
            "vextracti128    {ymm:x}, {ymm}, 1",
            "vmovd           {t:e}, {ymm:x}",
            "add             {r:r}, qword ptr [{lut} + 8*{t:r}]",
            "vpextrd         {t:e}, {ymm:x}, 1",
            "add             {r:r}, qword ptr [{lut} + 8*{t:r}]",
            "vpextrd         {t:e}, {ymm:x}, 3",
            "add             {r:r}, qword ptr [{lut} + 8*{t:r}]",

            LCPI0_0 = sym LCPI0_0,
            LCPI0_1 = sym LCPI0_1,
            LCPI0_2 = sym LCPI0_2,
            LCPI0_3 = sym LCPI0_3,
            s = in(reg) s.as_ptr(),
            lut = in(reg) LUT_P2.as_ptr(),
            r = out(reg) r,
            ymm = out(ymm_reg) _,
            t = out(reg) _,
            options(nostack)
        );
    }
    r
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
