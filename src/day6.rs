use aoc_runner_derive::aoc;
use bitvec::array::BitArray;

#[cfg(test)]
const SIZE: usize = 10;
#[cfg(not(test))]
const SIZE: usize = 130;

#[aoc(day6, part1)]
pub fn part1(s: &str) -> usize {
    let s = s.as_bytes();

    let mut hor_grid = [[0u8; SIZE]; SIZE];
    let mut vert_grid = [[0u8; SIZE]; SIZE];
    let mut hor_visit = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];

    for i in memchr::memchr_iter(b'#', s) {
        let x = i % (SIZE + 1);
        let y = i / (SIZE + 1);

        hor_grid[y][x] = b'#';
        vert_grid[x][y] = b'#';
    }

    let start = memchr::memchr(b'^', s).unwrap();

    let mut x = start % (SIZE + 1);
    let mut y = start / (SIZE + 1);
    hor_visit[y].set(x, true);

    loop {
        // Up
        let Some(y_move) = memchr::memrchr(b'#', &vert_grid[x][..y]) else {
            for y in 0..y {
                hor_visit[y].set(x, true);
            }
            break;
        };
        for y in y_move + 1..y {
            hor_visit[y].set(x, true);
        }
        y = y_move + 1;

        // Right
        let Some(x_move) = memchr::memchr(b'#', &hor_grid[y][x + 1..]) else {
            hor_visit[y][x..SIZE].fill(true);
            break;
        };
        hor_visit[y][x..x + x_move + 1].fill(true);
        x += x_move;

        // Down
        let Some(y_move) = memchr::memchr(b'#', &vert_grid[x][y + 1..]) else {
            for y in y..SIZE {
                hor_visit[y].set(x, true);
            }
            break;
        };
        for y in y..y + y_move + 1 {
            hor_visit[y].set(x, true);
        }
        y += y_move;

        // Left
        let Some(x_move) = memchr::memrchr(b'#', &hor_grid[y][..x]) else {
            hor_visit[y][..x].fill(true);
            break;
        };
        hor_visit[y][x_move + 1..x].fill(true);
        x = x_move + 1;
    }

    // TODO: is there a smarter way to do this?
    // for x in 0..SIZE {
    //     for y in vert_visit[x].iter_ones() {
    //         hor_visit[y].set(x, true);
    //     }
    // }
    // for y in 0..SIZE {
    //     for x in 0..SIZE {
    //         if hor_visit[y][x] {
    //             print!("#");
    //         } else {
    //             print!(" ");
    //         }
    //     }
    //     println!("");
    // }

    hor_visit.iter().map(|v| v.count_ones()).sum()
}

fn try_opst_up(
    opst_x: usize,
    opst_y: usize,
    start_x: usize,
    start_y: usize,

    hor_grid: &mut [[u8; SIZE]; SIZE],
    vert_grid: &mut [[u8; SIZE]; SIZE],

    mut visited_vert: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
    mut visited_hor: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
) -> bool {
    hor_grid[opst_y][opst_x] = b'#';
    vert_grid[opst_x][opst_y] = b'#';

    let mut x = start_x;
    let mut y = start_y;

    let loops = loop {
        // Up
        let Some(y_move) = memchr::memrchr(b'#', &vert_grid[x][..y]) else {
            break false;
        };
        y = y_move + 1;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Right
        let Some(x_move) = memchr::memchr(b'#', &hor_grid[y][x + 1..]) else {
            break false;
        };
        x += x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Down
        let Some(y_move) = memchr::memchr(b'#', &vert_grid[x][y + 1..]) else {
            break false;
        };
        y += y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Left
        let Some(x_move) = memchr::memrchr(b'#', &hor_grid[y][..x]) else {
            break false;
        };
        x = x_move + 1;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);
    };

    hor_grid[opst_y][opst_x] = 0;
    vert_grid[opst_x][opst_y] = 0;

    loops
}

fn try_opst_right(
    opst_x: usize,
    opst_y: usize,
    start_x: usize,
    start_y: usize,

    hor_grid: &mut [[u8; SIZE]; SIZE],
    vert_grid: &mut [[u8; SIZE]; SIZE],

    mut visited_vert: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
    mut visited_hor: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
) -> bool {
    hor_grid[opst_y][opst_x] = b'#';
    vert_grid[opst_x][opst_y] = b'#';

    let mut x = start_x;
    let mut y = start_y;

    let loops = loop {
        // Right
        let Some(x_move) = memchr::memchr(b'#', &hor_grid[y][x + 1..]) else {
            break false;
        };
        x += x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Down
        let Some(y_move) = memchr::memchr(b'#', &vert_grid[x][y + 1..]) else {
            break false;
        };
        y += y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Left
        let Some(x_move) = memchr::memrchr(b'#', &hor_grid[y][..x]) else {
            break false;
        };
        x = x_move + 1;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Up
        let Some(y_move) = memchr::memrchr(b'#', &vert_grid[x][..y]) else {
            break false;
        };
        y = y_move + 1;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);
    };

    hor_grid[opst_y][opst_x] = 0;
    vert_grid[opst_x][opst_y] = 0;

    loops
}

fn try_opst_down(
    opst_x: usize,
    opst_y: usize,
    start_x: usize,
    start_y: usize,

    hor_grid: &mut [[u8; SIZE]; SIZE],
    vert_grid: &mut [[u8; SIZE]; SIZE],

    mut visited_vert: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
    mut visited_hor: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
) -> bool {
    hor_grid[opst_y][opst_x] = b'#';
    vert_grid[opst_x][opst_y] = b'#';

    let mut x = start_x;
    let mut y = start_y;

    let loops = loop {
        // Down
        let Some(y_move) = memchr::memchr(b'#', &vert_grid[x][y + 1..]) else {
            break false;
        };
        y += y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Left
        let Some(x_move) = memchr::memrchr(b'#', &hor_grid[y][..x]) else {
            break false;
        };
        x = x_move + 1;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Up
        let Some(y_move) = memchr::memrchr(b'#', &vert_grid[x][..y]) else {
            break false;
        };
        y = y_move + 1;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Right
        let Some(x_move) = memchr::memchr(b'#', &hor_grid[y][x + 1..]) else {
            break false;
        };
        x += x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);
    };

    hor_grid[opst_y][opst_x] = 0;
    vert_grid[opst_x][opst_y] = 0;

    loops
}

fn try_opst_left(
    opst_x: usize,
    opst_y: usize,
    start_x: usize,
    start_y: usize,

    hor_grid: &mut [[u8; SIZE]; SIZE],
    vert_grid: &mut [[u8; SIZE]; SIZE],

    mut visited_vert: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
    mut visited_hor: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
) -> bool {
    hor_grid[opst_y][opst_x] = b'#';
    vert_grid[opst_x][opst_y] = b'#';

    let mut x = start_x;
    let mut y = start_y;

    let loops = loop {
        // Left
        let Some(x_move) = memchr::memrchr(b'#', &hor_grid[y][..x]) else {
            break false;
        };
        x = x_move + 1;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Up
        let Some(y_move) = memchr::memrchr(b'#', &vert_grid[x][..y]) else {
            break false;
        };
        y = y_move + 1;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Right
        let Some(x_move) = memchr::memchr(b'#', &hor_grid[y][x + 1..]) else {
            break false;
        };
        x += x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Down
        let Some(y_move) = memchr::memchr(b'#', &vert_grid[x][y + 1..]) else {
            break false;
        };
        y += y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);
    };

    hor_grid[opst_y][opst_x] = 0;
    vert_grid[opst_x][opst_y] = 0;

    loops
}

#[aoc(day6, part2)]
pub fn part2(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut hor_grid = [[0u8; SIZE]; SIZE];
    let mut vert_grid = [[0u8; SIZE]; SIZE];

    let mut visited_vert = BitArray::<[u64; (SIZE * SIZE).div_ceil(64)]>::default();
    let mut visited_hor = BitArray::<[u64; (SIZE * SIZE).div_ceil(64)]>::default();
    let mut tried_opst = BitArray::<[u64; (SIZE * SIZE).div_ceil(64)]>::default();

    for i in memchr::memchr_iter(b'#', s) {
        let x = i % (SIZE + 1);
        let y = i / (SIZE + 1);

        hor_grid[y][x] = b'#';
        vert_grid[x][y] = b'#';
    }

    let start = memchr::memchr(b'^', s).unwrap();

    let start_x = start % (SIZE + 1);
    let start_y = start / (SIZE + 1);

    let mut x = start_x;
    let mut y = start_y;

    let mut sum = 0;
    'outer: loop {
        // Up
        loop {
            if y == 0 {
                break 'outer;
            }
            if hor_grid[y - 1][x] != 0 {
                visited_vert.set(y * SIZE + x, true);
                break; // Turn
            } else {
                if !tried_opst[(y - 1) * SIZE + x] {
                    if try_opst_up(
                        x,
                        y - 1,
                        x,
                        y,
                        &mut hor_grid,
                        &mut vert_grid,
                        visited_vert.clone(),
                        visited_hor.clone(),
                    ) {
                        sum += 1;
                    }
                    tried_opst.set((y - 1) * SIZE + x, true);
                }
                y -= 1;
            }
        }

        // Right
        loop {
            if x == SIZE - 1 {
                break 'outer;
            }
            if hor_grid[y][x + 1] != 0 {
                visited_hor.set(y * SIZE + x, true);
                break; // Turn
            } else {
                if !tried_opst[y * SIZE + x + 1] {
                    if try_opst_right(
                        x + 1,
                        y,
                        x,
                        y,
                        &mut hor_grid,
                        &mut vert_grid,
                        visited_vert.clone(),
                        visited_hor.clone(),
                    ) {
                        sum += 1;
                    }
                    tried_opst.set(y * SIZE + x + 1, true);
                }
                x += 1;
            }
        }

        // Down
        loop {
            if y == SIZE - 1 {
                break 'outer;
            }
            if hor_grid[y + 1][x] != 0 {
                visited_vert.set(y * SIZE + x, true);
                break; // Turn
            } else {
                if !tried_opst[(y + 1) * SIZE + x] {
                    if try_opst_down(
                        x,
                        y + 1,
                        x,
                        y,
                        &mut hor_grid,
                        &mut vert_grid,
                        visited_vert.clone(),
                        visited_hor.clone(),
                    ) {
                        sum += 1;
                    }
                    tried_opst.set((y + 1) * SIZE + x, true);
                }
                y += 1;
            }
        }

        // Left
        loop {
            if x == 0 {
                break 'outer;
            }
            if hor_grid[y][x - 1] != 0 {
                visited_hor.set(y * SIZE + x, true);
                break; // Turn
            } else {
                if !tried_opst[y * SIZE + x - 1] {
                    if try_opst_left(
                        x - 1,
                        y,
                        x,
                        y,
                        &mut hor_grid,
                        &mut vert_grid,
                        visited_vert.clone(),
                        visited_hor.clone(),
                    ) {
                        sum += 1;
                    }
                    tried_opst.set(y * SIZE + x - 1, true);
                }
                x -= 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 41);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
