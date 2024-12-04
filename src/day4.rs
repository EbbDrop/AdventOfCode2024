use aoc_runner_derive::aoc;
use itertools::Itertools;
use phf::phf_map;

const SIZE: usize = 140;
const FULL_SIZE: usize = SIZE * (SIZE + 1);

fn find_in_slice(b: &[u8]) -> usize {
    b.iter()
        .tuple_windows()
        .filter(|(a, b, c, d)| **a == b'X' && **b == b'M' && **c == b'A' && **d == b'S')
        .count()
}

fn find_in_slice_rev(b: &[u8]) -> usize {
    b.iter()
        .tuple_windows()
        .filter(|(a, b, c, d)| **a == b'S' && **b == b'A' && **c == b'M' && **d == b'X')
        .count()
}

#[aoc(day4, part1)]
pub fn part1(s: &str) -> usize {
    let normal_grid: [u8; FULL_SIZE - 1] = s.as_bytes()[0..FULL_SIZE - 1].try_into().unwrap();

    let mut vert_grid = [b'\n'; FULL_SIZE];
    for col in 0..SIZE {
        for row in 0..SIZE {
            vert_grid[col * (SIZE + 1) + row] = normal_grid[row * (SIZE + 1) + col];
        }
    }

    let mut diag_grid = [b'\n'; (SIZE * SIZE) + SIZE * 2 - 1];
    let mut pos = 0;
    for diag_left in 0..SIZE {
        for i in 0..SIZE - diag_left {
            diag_grid[pos] = normal_grid[(i + diag_left) * (SIZE + 1) + i];
            pos += 1;
        }
        pos += 1;
    }
    for diag_top in 1..SIZE {
        for i in 0..SIZE - diag_top {
            diag_grid[pos] = normal_grid[i * (SIZE + 1) + i + diag_top];
            pos += 1;
        }
        pos += 1;
    }

    let mut alt_diag_grid = [b'\n'; (SIZE * SIZE) + SIZE * 2 - 1];
    let mut pos = 0;
    for alt_diag_left in 0..SIZE {
        for i in 0..SIZE - alt_diag_left {
            alt_diag_grid[pos] = normal_grid[(i + alt_diag_left) * (SIZE + 1) + SIZE - i - 1];
            pos += 1;
        }
        pos += 1;
    }
    for alt_diag_top in 1..SIZE {
        for i in 0..SIZE - alt_diag_top {
            alt_diag_grid[pos] = normal_grid[i * (SIZE + 1) + SIZE - i - alt_diag_top - 1];
            pos += 1;
        }
        pos += 1;
    }

    // println!("{}\n", String::from_utf8_lossy(&normal_grid));
    // println!("{}", String::from_utf8_lossy(&alt_diag_grid));

    find_in_slice(&normal_grid)
        + find_in_slice(&vert_grid)
        + find_in_slice(&diag_grid)
        + find_in_slice(&alt_diag_grid)
        + find_in_slice_rev(&normal_grid)
        + find_in_slice_rev(&vert_grid)
        + find_in_slice_rev(&diag_grid)
        + find_in_slice_rev(&alt_diag_grid)
}

static MAP: phf::Map<[u8; 4], u32> = phf_map! {
    *b"MSMS" => 1,
    *b"MSSM" => 1,
    *b"SMMS" => 1,
    *b"SMSM" => 1,
};

#[aoc(day4, part2)]
pub fn part2(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut sum = 0;
    for i in memchr::memmem::find_iter(&s[SIZE + 2..FULL_SIZE - SIZE], b"A") {
        let i = i + SIZE + 2;

        sum += MAP
            .get(dbg!(&[
                s[i - SIZE - 2], // tl
                s[i + SIZE + 2], // br
                s[i - SIZE],     // tr
                s[i + SIZE],     // bl
            ]))
            .cloned()
            .unwrap_or(0);
    }

    sum
}
