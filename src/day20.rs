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
pub fn part2(s: &str) -> u32 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part2(s.as_bytes())
    }
}

#[derive(Debug)]
struct Line {
    start_ns: u16,
    line_start: u8,
    line_end: u8,
    line_offset: u8,
}

const QUAD_SIZE: usize = 4;
const QUADS_SIZE: usize = SIZE.div_ceil(QUAD_SIZE);
const QUADS_NEEDED: usize = 20usize.div_ceil(QUAD_SIZE);

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part2(s: &[u8]) -> u32 {
    let start = memchr::memchr(b'S', s).unwrap();

    let mut lines = [const { heapless::Vec::<Line, 2048>::new() }; 4];
    let mut quads = [const {
        [const { heapless::Vec::<usize, { QUAD_SIZE * 2 }>::new() }; QUADS_SIZE * QUADS_SIZE]
    }; 4];

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

    let mut ns = 0;
    let mut cur_line = Line {
        start_ns: ns,
        line_start: d.select_crosline(i),
        line_offset: d.select_inline(i),
        line_end: 0,
    };

    let mut sum = 0;

    let qx = (i % SIZE1) / QUAD_SIZE;
    let qy = (i / SIZE1) / QUAD_SIZE;

    quads[d as usize][qy * QUADS_SIZE + qx]
        .push(lines[d as usize].len())
        .unwrap();
    let mut prev_qx = qx;
    let mut prev_qy = qy;
    while s[i] != b'E' {
        let next = d.step(i);
        if s[next] == b'#' {
            cur_line.line_end = d.select_crosline(i);
            lines[d as usize].push(cur_line).unwrap();

            for side in d.sides() {
                let side_i = side.step(i);
                if s[side_i] != b'#' {
                    d = side;
                }
            }
            i = d.step(i);
            ns += 1;

            cur_line = Line {
                start_ns: ns,
                line_start: d.select_crosline(i),
                line_offset: d.select_inline(i),
                line_end: 0,
            };
            let qx = (i % SIZE1) / QUAD_SIZE;
            let qy = (i / SIZE1) / QUAD_SIZE;
            quads[d as usize][qy * QUADS_SIZE + qx]
                .push(lines[d as usize].len())
                .unwrap();
            prev_qy = qy;
            prev_qx = qx;
        } else {
            let qx = (next % SIZE1) / QUAD_SIZE;
            let qy = (next / SIZE1) / QUAD_SIZE;

            if prev_qy != qy || prev_qx != qx {
                cur_line.line_end = d.select_crosline(i);
                lines[d as usize].push(cur_line).unwrap();

                cur_line = Line {
                    start_ns: ns + 1,
                    line_start: d.select_crosline(next),
                    line_offset: d.select_inline(next),
                    line_end: 0,
                };
                let qx = (next % SIZE1) / QUAD_SIZE;
                let qy = (next / SIZE1) / QUAD_SIZE;
                quads[d as usize][qy * QUADS_SIZE + qx]
                    .push(lines[d as usize].len())
                    .unwrap();
            }

            i = next;
            ns += 1;

            prev_qy = qy;
            prev_qx = qx;
        }

        let x = i % SIZE1;
        let y = i / SIZE1;

        let qx = x / QUAD_SIZE;
        let qy = y / QUAD_SIZE;

        for qx in
            qx.saturating_sub(QUADS_NEEDED)..qx.saturating_add(QUADS_NEEDED + 1).min(QUADS_SIZE)
        {
            for qy in
                qy.saturating_sub(QUADS_NEEDED)..qy.saturating_add(QUADS_NEEDED + 1).min(QUADS_SIZE)
            {
                let x = x as i16;
                let y = y as i16;

                for line_i in &quads[Dir::N as usize][qy * QUADS_SIZE + qx] {
                    let Some(line) = lines[Dir::N as usize].get(*line_i) else {
                        continue;
                    };
                    let dist = x.abs_diff(line.line_offset as i16) as i16;
                    if dist > 20 {
                        continue;
                    }
                    let dist_from_start = line.line_start as i16 - y;

                    let ns_at_intersection = line.start_ns as i16 + dist_from_start;
                    let diff_at_intersection = ns as i16 - ns_at_intersection;
                    if diff_at_intersection - dist < MIN_CHEAT as i16 {
                        continue;
                    }
                    let cheat_left = (diff_at_intersection - dist) - MIN_CHEAT as i16;

                    let cheat_start = (y + (20 - dist)).min(line.line_start as i16);
                    let cheat_end = (y - (20 - dist))
                        .max(y - cheat_left / 2)
                        .max(line.line_end as i16);

                    if cheat_start >= cheat_end {
                        sum += (cheat_start - cheat_end) as u32 + 1;
                    }
                }
                for line_i in &quads[Dir::E as usize][qy * QUADS_SIZE + qx] {
                    let Some(line) = lines[Dir::E as usize].get(*line_i) else {
                        continue;
                    };
                    let dist = y.abs_diff(line.line_offset as i16) as i16;
                    if dist > 20 {
                        continue;
                    }
                    let dist_from_start = x - line.line_start as i16;

                    let ns_at_intersection = line.start_ns as i16 + dist_from_start;
                    let diff_at_intersection = ns as i16 - ns_at_intersection;
                    if diff_at_intersection - dist < MIN_CHEAT as i16 {
                        continue;
                    }
                    let cheat_left = (diff_at_intersection - dist) - MIN_CHEAT as i16;

                    let cheat_start = (x - (20 - dist)).max(line.line_start as i16);
                    let cheat_end = (x + (20 - dist))
                        .min(x + cheat_left / 2)
                        .min(line.line_end as i16);

                    if cheat_end >= cheat_start {
                        sum += (cheat_end - cheat_start) as u32 + 1;
                    }
                }
                for line_i in &quads[Dir::S as usize][qy * QUADS_SIZE + qx] {
                    let Some(line) = lines[Dir::S as usize].get(*line_i) else {
                        continue;
                    };
                    let dist = x.abs_diff(line.line_offset as i16) as i16;
                    if dist > 20 {
                        continue;
                    }
                    let dist_from_start = y - line.line_start as i16;

                    let ns_at_intersection = line.start_ns as i16 + dist_from_start;
                    let diff_at_intersection = ns as i16 - ns_at_intersection;
                    if diff_at_intersection - dist < MIN_CHEAT as i16 {
                        continue;
                    }
                    let cheat_left = (diff_at_intersection - dist) - MIN_CHEAT as i16;

                    let cheat_start = (y - (20 - dist)).max(line.line_start as i16);
                    let cheat_end = (y + (20 - dist))
                        .min(y + cheat_left / 2)
                        .min(line.line_end as i16);

                    if cheat_end >= cheat_start {
                        sum += (cheat_end - cheat_start) as u32 + 1;
                    }
                }
                for line_i in &quads[Dir::W as usize][qy * QUADS_SIZE + qx] {
                    let Some(line) = lines[Dir::W as usize].get(*line_i) else {
                        continue;
                    };
                    let dist = y.abs_diff(line.line_offset as i16) as i16;
                    if dist > 20 {
                        continue;
                    }
                    let dist_from_start = line.line_start as i16 - x;

                    let ns_at_intersection = line.start_ns as i16 + dist_from_start;
                    let diff_at_intersection = ns as i16 - ns_at_intersection;
                    if diff_at_intersection - dist < MIN_CHEAT as i16 {
                        continue;
                    }
                    let cheat_left = (diff_at_intersection - dist) - MIN_CHEAT as i16;

                    let cheat_start = (x + (20 - dist)).min(line.line_start as i16);
                    let cheat_end = (x - (20 - dist))
                        .max(x - cheat_left / 2)
                        .max(line.line_end as i16);

                    if cheat_start >= cheat_end {
                        sum += (cheat_start - cheat_end) as u32 + 1;
                    }
                }
            }
        }
    }
    // dbg!(&quads);
    // dbg!(&lines);

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
