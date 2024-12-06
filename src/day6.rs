use aoc_runner_derive::aoc;
use bitvec::array::BitArray;

#[cfg(test)]
const SIZE: usize = 10;
#[cfg(not(test))]
const SIZE: usize = 130;

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// enum Dir {
//     Up,
//     Right,
//     Down,
//     Left,
// }

// impl Dir {
//     fn next(&self) -> Self {
//         match self {
//             Dir::Up => Dir::Right,
//             Dir::Right => Dir::Down,
//             Dir::Down => Dir::Left,
//             Dir::Left => Dir::Up,
//         }
//     }
// }

#[aoc(day6, part1)]
pub fn part1(s: &str) -> usize {
    let s = s.as_bytes();

    let mut hor_grid = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];
    let mut vert_grid = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];
    let mut hor_visit = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];
    let mut vert_visit = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];

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
        vert_visit[x][..y][y - y_move..].fill(true);
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
            vert_visit[x][y..SIZE].fill(true);
            break;
        }
        vert_visit[x][y..SIZE][..y_move + 1].fill(true);
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
    for x in 0..SIZE {
        for y in vert_visit[x].iter_ones() {
            hor_visit[y].set(x, true);
        }
    }
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

#[aoc(day6, part2)]
pub fn part2(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut hor_grid = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];
    let mut vert_grid = [BitArray::<[u64; SIZE.div_ceil(64)]>::default(); SIZE];
    let mut visited_vert = [false; SIZE * SIZE];
    let mut visited_hor = [false; SIZE * SIZE];

    for i in memchr::memchr_iter(b'#', s) {
        let x = i % (SIZE + 1);
        let y = i / (SIZE + 1);

        hor_grid[y].set(x, true);
        vert_grid[x].set(y, true);
    }

    let start = memchr::memchr(b'^', s).unwrap();

    let start_x = start % (SIZE + 1);
    let start_y = start / (SIZE + 1);

    let mut sum = 0;
    // TODO: Only test opst in original path
    for i in 0..SIZE * SIZE {
        let opst_x = i % SIZE;
        let opst_y = i / SIZE;

        if opst_x == start_x && opst_y == start_y {
            continue;
        }
        if hor_grid[opst_y][opst_x] {
            continue;
        }
        hor_grid[opst_y].set(opst_x, true);
        vert_grid[opst_x].set(opst_y, true);

        let mut x = start_x;
        let mut y = start_y;

        visited_hor.fill(false);
        visited_vert.fill(false);
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
            visited_vert[y * SIZE + x] = true;

            // Right
            let x_move = hor_grid[y][x + 1..].leading_zeros();
            if x + x_move > SIZE {
                break false;
            }
            x += x_move;
            if visited_hor[y * SIZE + x] {
                break true;
            }
            visited_hor[y * SIZE + x] = true;

            // Down
            let y_move = vert_grid[x][y + 1..].leading_zeros();
            if y + y_move > SIZE {
                break false;
            }
            y += y_move;
            if visited_vert[y * SIZE + x] {
                break true;
            }
            visited_vert[y * SIZE + x] = true;

            // Left
            let x_move = hor_grid[y][..x].trailing_zeros();
            if x_move >= x {
                break false;
            }
            x -= x_move;
            if visited_hor[y * SIZE + x] {
                break true;
            }
            visited_hor[y * SIZE + x] = true;
        };
        if loops {
            sum += 1;
        }

        hor_grid[opst_y].set(opst_x, false);
        vert_grid[opst_x].set(opst_y, false);
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
