use std::{collections::HashMap, fs::File, io::Write, mem::transmute, path::PathBuf};

const NUM_PADS: u32 = 25;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

fn press_mp(
    from_pos: Pos,
    to_pos: Pos,
    padds_left: u32,
    cache: &mut HashMap<(Pos, Pos, u32), u64>,
) -> u64 {
    if padds_left == 1 {
        return (from_pos.x.abs_diff(to_pos.x) + from_pos.y.abs_diff(to_pos.y) + 1) as u64;
    }
    if from_pos.x == to_pos.x && from_pos.y == to_pos.y {
        return 1;
    }
    if let Some(c) = cache.get(&(from_pos, to_pos, padds_left)) {
        return *c;
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
            sum_y_first += press_mp(move_pad_pos, next_pos, padds_left - 1, cache);
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
            sum_y_first += press_mp(move_pad_pos, next_pos, padds_left - 1, cache);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        sum_y_first += press_mp(move_pad_pos, movepad_pos('A'), padds_left - 1, cache);
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
            sum_x_first += press_mp(move_pad_pos, next_pos, padds_left - 1, cache);
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
            sum_x_first += press_mp(move_pad_pos, next_pos, padds_left - 1, cache);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        sum_x_first += press_mp(move_pad_pos, movepad_pos('A'), padds_left - 1, cache);
    } else {
        sum_x_first = u64::MAX;
    }

    let r = if sum_x_first < sum_y_first {
        sum_x_first
    } else {
        sum_y_first
    };

    cache.insert((from_pos, to_pos, padds_left), r);
    r
}

fn press_number2(from_pos: Pos, to_pos: Pos, cache: &mut HashMap<(Pos, Pos, u32), u64>) -> u64 {
    if from_pos.x == to_pos.x && from_pos.y == to_pos.y {
        return 1;
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
            sum_y_first += press_mp(move_pad_pos, next_pos, NUM_PADS, cache);
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
            sum_y_first += press_mp(move_pad_pos, next_pos, NUM_PADS, cache);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        sum_y_first += press_mp(move_pad_pos, movepad_pos('A'), NUM_PADS, cache);
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
            sum_x_first += press_mp(move_pad_pos, next_pos, NUM_PADS, cache);
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
            sum_x_first += press_mp(move_pad_pos, next_pos, NUM_PADS, cache);
            move_pad_pos = next_pos;
            to_move -= 1;
        }
        sum_x_first += press_mp(move_pad_pos, movepad_pos('A'), NUM_PADS, cache);
    } else {
        sum_x_first = u64::MAX;
    }

    if sum_x_first < sum_y_first {
        sum_x_first
    } else {
        sum_y_first
    }
}

#[allow(unused)]
fn part2_inner(a: usize, b: usize, c: usize, cache: &mut HashMap<(Pos, Pos, u32), u64>) -> u64 {
    press_number2(keypad_pos(10), keypad_pos(a), cache)
        + press_number2(keypad_pos(a), keypad_pos(b), cache)
        + press_number2(keypad_pos(b), keypad_pos(c), cache)
        + press_number2(keypad_pos(c), keypad_pos(10), cache)
}

#[allow(unused)]
fn write_day21() {
    let mut lut = [0u64; 10usize.pow(3)];

    let mut cache = HashMap::new();
    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10 {
                let code = a as u64 * 100 + b as u64 * 10 + c as u64;
                lut[a * 100 + b * 10 + c] = part2_inner(a, b, c, &mut cache) * code;
            }
        }
    }

    let mut path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    path.push("day21.bin");
    let mut file = File::create(path).unwrap();

    let lut: [u8; 8 * 10usize.pow(3)] = unsafe { transmute(lut) };

    file.write_all(&lut).unwrap();
}

// #[allow(unused)]
// fn write_day22() {
//     const MAX: u32 = 16777216;

//     let mut lut = Vec::with_capacity(MAX as usize);

//     for i in 0..MAX {
//         let mut sn = i;
//         for _ in 0..2000 {
//             sn = ((sn as u64 * 64) % MAX as u64) as u32 ^ sn;
//             sn = (sn / 32) ^ sn;
//             sn = ((sn as u64 * 2048) % MAX as u64) as u32 ^ sn;
//         }

//         lut.push(sn);
//     }

//     let lut: Box<[u32; MAX as usize]> = lut.into_boxed_slice().try_into().unwrap();
//     let mut lut = std::mem::ManuallyDrop::new(lut);
//     let lut: Box<[u8; 4 * MAX as usize]> = unsafe { Box::from_raw(lut.as_mut_ptr().cast()) };

//     let mut path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
//     path.push("day22.bin");
//     let mut file = File::create(&path).unwrap();

//     file.write_all(lut.as_slice()).unwrap();
//     println!("{}", path.display());
// }

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    write_day21();
    // write_day22();
}
