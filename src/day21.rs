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

use std::arch::x86_64::*;

// 029An980An179An4 56An379A

// 029An980An179An4 56An379A

// 029A n980 An17 9An4 56An 379A
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner(s: &str, lut: &[u64; 1000]) -> u64 {
    // 029An980An179An456An379A
    let v: __m256i = s.as_ptr().cast::<__m256i>().read_unaligned();
    let v = _mm256_subs_epu8(v, _mm256_set1_epi8(b'0' as i8));

    // 029A n980 0000 0000 An17 9An4 56An 379A
    let v = _mm256_permutevar8x32_epi32(v, _mm256_setr_epi32(0, 1, 6, 6, 2, 3, 4, 5));

    // _029_980_________ _179_456____ _379
    let idx = _mm256_setr_epi8(
        15, 0, 1, 2, 15, 5, 6, 7, //
        15, 15, 15, 15, 15, 15, 15, 15, //
        15, 2, 3, 4, 15, 7, 8, 9, //
        15, 15, 15, 15, 15, 12, 13, 14,
    );
    let v = _mm256_shuffle_epi8(v, idx);

    // 0, 29, 9, 80, 0, 0, 0, 0, 1, 79, 4, 56, 0, 0, 7, 90
    let mul: __m256i = _mm256_setr_epi8(
        0, 1, 10, 1, 0, 1, 10, 1, //
        0, 0, 0, 0, 0, 0, 0, 0, //
        0, 1, 10, 1, 0, 1, 10, 1, //
        0, 0, 0, 0, 0, 1, 10, 1, //
    );
    let v = _mm256_maddubs_epi16(v, mul);

    // 29, 980, 0, 0, 179, 456, 0, 379
    let mul: __m256i = _mm256_setr_epi16(
        100, 1, 100, 1, //
        0, 0, 0, 0, //
        100, 1, 100, 1, //
        0, 0, 100, 1,
    );
    let v = _mm256_madd_epi16(v, mul);
    lut[_mm256_extract_epi32::<0>(v) as u32 as usize]
        + lut[_mm256_extract_epi32::<1>(v) as u32 as usize]
        + lut[_mm256_extract_epi32::<4>(v) as u32 as usize]
        + lut[_mm256_extract_epi32::<5>(v) as u32 as usize]
        + lut[_mm256_extract_epi32::<7>(v) as u32 as usize]
    // let com_l = _mm256_i32gather_epi32::<8>(lut.as_ptr().cast(), v);
    // let com_h = _mm256_i32gather_epi32::<8>(lut.as_ptr().cast::<i32>().offset(1), v);

    // let h1 = _mm256_unpacklo_epi32(com_l, com_h);
    // let h2 = _mm256_unpackhi_epi32(com_l, com_h);

    // let sum = 0;
    // let v = _mm256_add_epi64(h1, h2);
    // let vs = _mm256_shuffle_epi32::<{ (1 << 6) | (0 << 4) | (3 << 2) | 2 }>(v);
    // let v = _mm256_add_epi64(v, vs);

    // _mm256_extract_epi64::<0>(v) as u64 + _mm256_extract_epi64::<2>(v) as u64
    // let mut p = [0u32; 256 / 32];
    // p.as_mut_ptr().cast::<__m256i>().write(v);
    // println!("{:?}", &p);

    // panic!()

    // let v = _mm256_shuffle_epi8(v, )

    // let s = s.as_bytes();
    // let mut sum = 0;
    // unsafe {
    //     let mut i = 0;
    //     for _ in 0..5 {
    //         let idx = (*s.get_unchecked(i + 0) as usize) * 100
    //             + (*s.get_unchecked(i + 1) as usize) * 10
    //             + (*s.get_unchecked(i + 2) as usize) * 1
    //             - (b'0' as usize * 111);

    //         sum += lut[idx];
    //         i += 5;
    //     }
    // }

    // sum
}

#[aoc(day21, part1)]
pub fn part1(s: &str) -> u64 {
    unsafe { inner(s, &LUT_P1) }
}

#[aoc(day21, part2)]
pub fn part2(s: &str) -> u64 {
    unsafe { inner(s, &LUT_P2) }
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
