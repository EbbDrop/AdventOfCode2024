use aoc_runner_derive::aoc;

#[cfg(test)]
const SIZE: usize = 15;
#[cfg(not(test))]
const SIZE: usize = 141;

const SIZE1: usize = SIZE + 1;

#[cfg(test)]
const MIN_CHEAT: u16 = 50;
#[cfg(not(test))]
const MIN_CHEAT: u16 = 100;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Dir {
    fn sides(&self) -> [Dir; 2] {
        match self {
            Dir::N => [Dir::E, Dir::W],
            Dir::E => [Dir::N, Dir::S],
            Dir::S => [Dir::E, Dir::W],
            Dir::W => [Dir::N, Dir::S],
        }
    }

    fn step(&self, i: usize) -> usize {
        match self {
            Dir::N => i.wrapping_sub(SIZE1),
            Dir::E => i.wrapping_add(1),
            Dir::S => i.wrapping_add(SIZE1),
            Dir::W => i.wrapping_sub(1),
        }
    }

    fn step2(&self, i: usize) -> usize {
        match self {
            Dir::N => i.wrapping_sub(SIZE1 * 2).min(SIZE * SIZE1 - 1),
            Dir::E => i.wrapping_add(2),
            Dir::S => i.wrapping_add(SIZE1 * 2).min(SIZE * SIZE1 - 1),
            Dir::W => i.wrapping_sub(2),
        }
    }
}

#[aoc(day20, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part1(s.as_bytes())
    }
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part1(s: &[u8]) -> u64 {
    let start = memchr::memchr(b'S', s).unwrap();

    let mut ns_map = [0u16; SIZE1 * SIZE];

    let mut ns = 1;

    let mut i = start;
    let mut d = if s[i - 1] == b'.' {
        Dir::W
    } else if s[i + 1] == b'.' {
        Dir::E
    } else if s[i - SIZE1] == b'.' {
        Dir::N
    } else {
        Dir::S
    };

    let mut sum = 0;
    while s[i] != b'E' {
        ns_map[i] = ns;

        let next = d.step(i);
        if s[next] == b'#' {
            for side in d.sides() {
                let side_i = side.step(i);
                if s[side_i] != b'#' {
                    d = side;
                }
            }
        } else {
            i = next;
            ns += 1;
        }

        for side in d.sides() {
            let side_i = side.step2(i);
            if ns_map[side_i] != 0 {
                let diff = ns - ns_map[side_i];
                if diff >= MIN_CHEAT + 2 {
                    sum += 1;
                }
            }
        }
    }
    let forward_i = d.step2(i);
    if ns_map[forward_i] != 0 {
        let diff = ns - ns_map[forward_i];
        if diff >= MIN_CHEAT + 2 {
            sum += 1;
        }
    }
    sum
}

#[aoc(day20, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part2(s.as_bytes())
    }
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part2(s: &[u8]) -> u64 {
    let start = memchr::memchr(b'S', s).unwrap();

    let mut ns_map = [0u16; SIZE1 * SIZE];

    let mut ns = 1;

    let mut d = if s[start - 1] == b'.' {
        Dir::W
    } else if s[start + 1] == b'.' {
        Dir::E
    } else if s[start - SIZE1] == b'.' {
        Dir::N
    } else {
        Dir::S
    };

    let mut cy = start as i32 / SIZE1 as i32;
    let mut cx = start as i32 % SIZE1 as i32;

    let idx = |x, y| (y * SIZE1 as i32 + x) as usize;

    let mut sum = 0;
    while s[idx(cx, cy)] != b'E' {
        ns_map[idx(cx, cy)] = ns;

        let next = d.step(idx(cx, cy));
        if s[next] == b'#' {
            for side in d.sides() {
                let side_i = side.step(idx(cx, cy));
                if s[side_i] != b'#' {
                    d = side;
                }
            }
        }
        match d {
            Dir::N => cy -= 1,
            Dir::E => cx += 1,
            Dir::S => cy += 1,
            Dir::W => cx -= 1,
        }
        ns += 1;

        for y in (cy - 20).max(1)..(cy + 21).min(SIZE as i32 - 1) {
            for x in
                (-20 + (y - cy).abs() + cx).max(1)..(21 - (y - cy).abs() + cx).min(SIZE as i32 - 1)
            {
                let dist = (y - cy).abs() + (x - cx).abs();
                if dist <= 1 {
                    continue;
                }

                let side_i = idx(x, y);
                if ns_map[side_i] != 0 {
                    let diff = ns - ns_map[side_i];
                    if diff >= MIN_CHEAT + dist as u16 {
                        sum += 1;
                    }
                }
            }
        }
    }

    // for y in 0..SIZE {
    //     for x in 0..SIZE.min(40) {
    //         if ns_map[y * SIZE1 + x] != 0 {
    //             print!("{:^4} ", ns_map[y * SIZE1 + x]);
    //         } else {
    //             print!("  .  ");
    //         }
    //     }
    //     println!("");
    // }
    // println!("");

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 1);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 285);
    }
}
