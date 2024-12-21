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

    fn select_crosline(&self, i: usize) -> u8 {
        (match self {
            Dir::N => i / SIZE1,
            Dir::E => i % SIZE1,
            Dir::S => i / SIZE1,
            Dir::W => i % SIZE1,
        }) as u8
    }

    fn select_inline(&self, i: usize) -> u8 {
        (match self {
            Dir::N => i % SIZE1,
            Dir::E => i / SIZE1,
            Dir::S => i % SIZE1,
            Dir::W => i / SIZE1,
        }) as u8
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

#[derive(Debug)]
struct Line {
    start_ns: u16,
    line_start: u8,
    lien_offset: u8,
}

const QUAD_SIZE: usize = 8;
const QUADS_SIZE: usize = SIZE.div_ceil(QUAD_SIZE);
const QUADS_NEEDED: usize = 20usize.div_ceil(QUAD_SIZE);

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part2(s: &[u8]) -> u64 {
    let start = memchr::memchr(b'S', s).unwrap();

    let mut lines = [const { heapless::Vec::<Line, 1024>::new() }; 4];
    let mut quads =
        [const { [const { heapless::Vec::<usize, 8>::new() }; QUADS_SIZE * QUADS_SIZE] }; 4];

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

    lines[d as usize]
        .push(Line {
            start_ns: 0,
            line_start: d.select_crosline(i),
            lien_offset: d.select_inline(i),
        })
        .unwrap();

    let mut sum = 0;

    let mut prev_qx = usize::MAX;
    let mut prev_qy = usize::MAX;
    while s[i] != b'E' {
        let qx = (i % SIZE1) / QUAD_SIZE;
        let qy = (i / SIZE1) / QUAD_SIZE;

        for qx in
            qx.saturating_sub(QUADS_NEEDED)..qx.saturating_add(QUADS_NEEDED + 1).min(QUADS_SIZE)
        {
            for qy in
                qy.saturating_sub(QUADS_NEEDED)..qy.saturating_add(QUADS_NEEDED + 1).min(QUADS_SIZE)
            {
                for line_i in &quads[0][qy * QUADS_SIZE + qx] {
                    sum += 1;
                }
                for line_i in &quads[1][qy * QUADS_SIZE + qx] {
                    sum += 1;
                }
                for line_i in &quads[2][qy * QUADS_SIZE + qx] {
                    sum += 1;
                }
                for line_i in &quads[3][qy * QUADS_SIZE + qx] {
                    sum += 1;
                }
            }
        }

        if prev_qy != qy || prev_qx != qx {
            quads[d as usize][qy * QUADS_SIZE + qx]
                .push(lines[d as usize].len() - 1)
                .unwrap();
        }
        prev_qy = qy;
        prev_qx = qx;

        let next = d.step(i);
        if s[next] == b'#' {
            for side in d.sides() {
                let side_i = side.step(i);
                if s[side_i] != b'#' {
                    d = side;
                }
            }

            lines[d as usize]
                .push(Line {
                    start_ns: ns,
                    line_start: d.select_crosline(i),
                    lien_offset: d.select_inline(i),
                })
                .unwrap();
            quads[d as usize][qy * QUADS_SIZE + qx]
                .push(lines[d as usize].len() - 1)
                .unwrap();

            i = d.step(i);
        } else {
            i = next;
        }
        ns += 1;
    }

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
