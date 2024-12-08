use aoc_runner_derive::aoc;

#[cfg(not(test))]
const SIZE: usize = 50;
#[cfg(test)]
const SIZE: usize = 12;

const SIZE1: usize = SIZE + 1;

const FREQ_RANGE: usize = (b'z' - b'0' + 1) as usize;

#[aoc(day8, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        part1_inner(s)
    }
}

fn part1_inner(s: &str) -> u64 {
    let mut masts: [Vec<usize>; FREQ_RANGE] = [const { Vec::new() }; FREQ_RANGE];

    let mut antinodes = [false; SIZE * SIZE];
    let mut total_antinotedes = 0;

    for (i, f) in s
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, c)| **c != b'.' && **c != b'\n')
    {
        let f = f - b'0';

        let new_x = i % SIZE1;
        let new_y = i / SIZE1;
        for mast_i in &masts[f as usize] {
            let mast_x = mast_i % SIZE1;
            let mast_y = mast_i / SIZE1;

            let diff_x = new_x.abs_diff(mast_x);
            let diff_y = new_y.abs_diff(mast_y);

            if new_x > mast_x {
                if mast_x >= diff_x && mast_y >= diff_y {
                    let node_x = mast_x - diff_x;
                    let node_y = mast_y - diff_y;

                    if !antinodes[node_y * SIZE + node_x] {
                        total_antinotedes += 1;
                        antinodes[node_y * SIZE + node_x] = true;
                    }
                }

                if new_x + diff_x < SIZE && new_y + diff_y < SIZE {
                    let node_x = new_x + diff_x;
                    let node_y = new_y + diff_y;

                    if !antinodes[node_y * SIZE + node_x] {
                        total_antinotedes += 1;
                        antinodes[node_y * SIZE + node_x] = true;
                    }
                }
            } else {
                if mast_x + diff_x < SIZE && mast_y >= diff_y {
                    let node_x = mast_x + diff_x;
                    let node_y = mast_y - diff_y;

                    if !antinodes[node_y * SIZE + node_x] {
                        total_antinotedes += 1;
                        antinodes[node_y * SIZE + node_x] = true;
                    }
                }

                if new_x >= diff_x && new_y + diff_y < SIZE {
                    let node_x = new_x - diff_x;
                    let node_y = new_y + diff_y;

                    if !antinodes[node_y * SIZE + node_x] {
                        total_antinotedes += 1;
                        antinodes[node_y * SIZE + node_x] = true;
                    }
                }
            }
        }

        masts[f as usize].push(i);
    }

    total_antinotedes
}

#[aoc(day8, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        part2_inner(s)
    }
}

fn part2_inner(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut masts: [Vec<usize>; FREQ_RANGE] = [const { Vec::new() }; FREQ_RANGE];

    let mut antinodes = [false; SIZE * SIZE];
    let mut total_antinotedes = 0;

    for (i, f) in s
        .iter()
        .enumerate()
        .filter(|(_, c)| **c != b'.' && **c != b'\n')
    {
        let f = f - b'0';

        let new_x = i % SIZE1;
        let new_y = i / SIZE1;
        for mast_i in &masts[f as usize] {
            let mast_x = mast_i % SIZE1;
            let mast_y = mast_i / SIZE1;

            let o_diff_x = new_x.abs_diff(mast_x);
            let o_diff_y = new_y.abs_diff(mast_y);

            for k in 0.. {
                let diff_x = o_diff_x * k;
                let diff_y = o_diff_y * k;

                let mut new_node = false;

                if new_x > mast_x {
                    if mast_x >= diff_x && mast_y >= diff_y {
                        let node_x = mast_x - diff_x;
                        let node_y = mast_y - diff_y;
                        new_node = true;

                        if !antinodes[node_y * SIZE + node_x] {
                            total_antinotedes += 1;
                            antinodes[node_y * SIZE + node_x] = true;
                        }
                    }

                    if new_x + diff_x < SIZE && new_y + diff_y < SIZE {
                        let node_x = new_x + diff_x;
                        let node_y = new_y + diff_y;
                        new_node = true;

                        if !antinodes[node_y * SIZE + node_x] {
                            total_antinotedes += 1;
                            antinodes[node_y * SIZE + node_x] = true;
                        }
                    }
                } else {
                    if mast_x + diff_x < SIZE && mast_y >= diff_y {
                        let node_x = mast_x + diff_x;
                        let node_y = mast_y - diff_y;
                        new_node = true;

                        if !antinodes[node_y * SIZE + node_x] {
                            total_antinotedes += 1;
                            antinodes[node_y * SIZE + node_x] = true;
                        }
                    }

                    if new_x >= diff_x && new_y + diff_y < SIZE {
                        let node_x = new_x - diff_x;
                        let node_y = new_y + diff_y;
                        new_node = true;

                        if !antinodes[node_y * SIZE + node_x] {
                            total_antinotedes += 1;
                            antinodes[node_y * SIZE + node_x] = true;
                        }
                    }
                }
                if !new_node {
                    break;
                }
            }
        }

        masts[f as usize].push(i);
    }

    total_antinotedes
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 14);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 34);
    }
}
