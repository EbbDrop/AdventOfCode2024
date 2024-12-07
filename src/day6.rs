use aoc_runner_derive::aoc;
use bitvec::array::BitArray;

#[cfg(test)]
const SIZE: usize = 10;
#[cfg(not(test))]
const SIZE: usize = 130;

#[aoc(day6, part1)]
pub fn part1(s: &str) -> usize {
    let s = s.as_bytes();

    let mut hor_grid = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];
    let mut vert_grid = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];
    let mut hor_visit = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];
    // let mut vert_visit = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];

    for i in memchr::memchr_iter(b'#', s) {
        let x = i % (SIZE + 1);
        let y = i / (SIZE + 1);

        hor_grid[y].set(x, true);
        vert_grid[x].set(y, true);
    }

    let start = memchr::memchr(b'^', s).unwrap();

    let mut x = start % (SIZE + 1);
    let mut y = start / (SIZE + 1);
    hor_visit[y].set(x, true);

    loop {
        // Up
        let y_move = vert_grid[x][..y].trailing_zeros();
        // vert_visit[x][..y][y - y_move..].fill(true);
        for y in y - y_move..y {
            hor_visit[y].set(x, true);
        }
        if y_move >= y {
            break;
        }
        y -= y_move;

        // Right
        let x_move = hor_grid[y][x + 1..].leading_zeros();
        if x + x_move > SIZE {
            hor_visit[y][x..SIZE].fill(true);
            break;
        }
        hor_visit[y][x..SIZE][..x_move + 1].fill(true);
        x += x_move;

        // Down
        let y_move = vert_grid[x][y + 1..].leading_zeros();
        if y + y_move > SIZE {
            for y in y..SIZE {
                hor_visit[y].set(x, true);
            }
            break;
        }
        for y in y..y + y_move + 1 {
            hor_visit[y].set(x, true);
        }
        y += y_move;

        // Left
        let x_move = hor_grid[y][..x].trailing_zeros();
        hor_visit[y][..x][x - x_move..].fill(true);
        if x_move >= x {
            break;
        }
        x -= x_move;
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

    hor_grid: &mut [BitArray<[u64; SIZE.div_ceil(64)]>; SIZE],
    vert_grid: &mut [BitArray<[u64; SIZE.div_ceil(64)]>; SIZE],

    mut visited_vert: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
    mut visited_hor: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
) -> bool {
    hor_grid[opst_y].set(opst_x, true);
    vert_grid[opst_x].set(opst_y, true);

    let mut x = start_x;
    let mut y = start_y;

    let loops = loop {
        // Up
        let y_move = vert_grid[x][..y].trailing_zeros();
        if y_move >= y {
            break false;
        }
        y -= y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Right
        let x_move = hor_grid[y][x + 1..].leading_zeros();
        if x + x_move > SIZE {
            break false;
        }
        x += x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Down
        let y_move = vert_grid[x][y + 1..].leading_zeros();
        if y + y_move > SIZE {
            break false;
        }
        y += y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Left
        let x_move = hor_grid[y][..x].trailing_zeros();
        if x_move >= x {
            break false;
        }
        x -= x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);
    };

    hor_grid[opst_y].set(opst_x, false);
    vert_grid[opst_x].set(opst_y, false);

    loops
}

fn try_opst_right(
    opst_x: usize,
    opst_y: usize,
    start_x: usize,
    start_y: usize,

    hor_grid: &mut [BitArray<[u64; SIZE.div_ceil(64)]>; SIZE],
    vert_grid: &mut [BitArray<[u64; SIZE.div_ceil(64)]>; SIZE],

    mut visited_vert: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
    mut visited_hor: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
) -> bool {
    hor_grid[opst_y].set(opst_x, true);
    vert_grid[opst_x].set(opst_y, true);

    let mut x = start_x;
    let mut y = start_y;

    let loops = loop {
        // Right
        let x_move = hor_grid[y][x + 1..].leading_zeros();
        if x + x_move > SIZE {
            break false;
        }
        x += x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Down
        let y_move = vert_grid[x][y + 1..].leading_zeros();
        if y + y_move > SIZE {
            break false;
        }
        y += y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Left
        let x_move = hor_grid[y][..x].trailing_zeros();
        if x_move >= x {
            break false;
        }
        x -= x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Up
        let y_move = vert_grid[x][..y].trailing_zeros();
        if y_move >= y {
            break false;
        }
        y -= y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);
    };

    hor_grid[opst_y].set(opst_x, false);
    vert_grid[opst_x].set(opst_y, false);

    loops
}

fn try_opst_down(
    opst_x: usize,
    opst_y: usize,
    start_x: usize,
    start_y: usize,

    hor_grid: &mut [BitArray<[u64; SIZE.div_ceil(64)]>; SIZE],
    vert_grid: &mut [BitArray<[u64; SIZE.div_ceil(64)]>; SIZE],

    mut visited_vert: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
    mut visited_hor: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
) -> bool {
    hor_grid[opst_y].set(opst_x, true);
    vert_grid[opst_x].set(opst_y, true);

    let mut x = start_x;
    let mut y = start_y;

    let loops = loop {
        // Down
        let y_move = vert_grid[x][y + 1..].leading_zeros();
        if y + y_move > SIZE {
            break false;
        }
        y += y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Left
        let x_move = hor_grid[y][..x].trailing_zeros();
        if x_move >= x {
            break false;
        }
        x -= x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Up
        let y_move = vert_grid[x][..y].trailing_zeros();
        if y_move >= y {
            break false;
        }
        y -= y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Right
        let x_move = hor_grid[y][x + 1..].leading_zeros();
        if x + x_move > SIZE {
            break false;
        }
        x += x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);
    };

    hor_grid[opst_y].set(opst_x, false);
    vert_grid[opst_x].set(opst_y, false);

    loops
}

fn try_opst_left(
    opst_x: usize,
    opst_y: usize,
    start_x: usize,
    start_y: usize,

    hor_grid: &mut [BitArray<[u64; SIZE.div_ceil(64)]>; SIZE],
    vert_grid: &mut [BitArray<[u64; SIZE.div_ceil(64)]>; SIZE],

    mut visited_vert: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
    mut visited_hor: BitArray<[u64; (SIZE * SIZE).div_ceil(64)]>,
) -> bool {
    hor_grid[opst_y].set(opst_x, true);
    vert_grid[opst_x].set(opst_y, true);

    let mut x = start_x;
    let mut y = start_y;

    let loops = loop {
        // Left
        let x_move = hor_grid[y][..x].trailing_zeros();
        if x_move >= x {
            break false;
        }
        x -= x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Up
        let y_move = vert_grid[x][..y].trailing_zeros();
        if y_move >= y {
            break false;
        }
        y -= y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);

        // Right
        let x_move = hor_grid[y][x + 1..].leading_zeros();
        if x + x_move > SIZE {
            break false;
        }
        x += x_move;
        if visited_hor[y * SIZE + x] {
            break true;
        }
        visited_hor.set(y * SIZE + x, true);

        // Down
        let y_move = vert_grid[x][y + 1..].leading_zeros();
        if y + y_move > SIZE {
            break false;
        }
        y += y_move;
        if visited_vert[y * SIZE + x] {
            break true;
        }
        visited_vert.set(y * SIZE + x, true);
    };

    hor_grid[opst_y].set(opst_x, false);
    vert_grid[opst_x].set(opst_y, false);

    loops
}

#[aoc(day6, part2)]
pub fn part2(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut hor_grid = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];
    let mut vert_grid = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];

    let mut visited_vert = BitArray::<[u64; (SIZE * SIZE).div_ceil(64)]>::default();
    let mut visited_hor = BitArray::<[u64; (SIZE * SIZE).div_ceil(64)]>::default();
    let mut tried_opst = BitArray::<[u64; (SIZE * SIZE).div_ceil(64)]>::default();

    for i in memchr::memchr_iter(b'#', s) {
        let x = i % (SIZE + 1);
        let y = i / (SIZE + 1);

        hor_grid[y].set(x, true);
        vert_grid[x].set(y, true);
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
            if hor_grid[y - 1][x] {
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
            if hor_grid[y][x + 1] {
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
            if hor_grid[y + 1][x] {
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
            if hor_grid[y][x - 1] {
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
