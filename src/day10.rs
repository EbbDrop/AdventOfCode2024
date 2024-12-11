use aoc_runner_derive::aoc;

const MAX_SIZE: usize = 50;
const BIG_SIZE: usize = MAX_SIZE * (MAX_SIZE + 2);

#[aoc(day10, part1)]
pub fn part1(s: &str) -> u32 {
    unsafe { part1_inner(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_inner(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut positions = [(0u16, [0u16; MAX_SIZE * MAX_SIZE / 9]); 9];
    let mut first_map = [0u64; BIG_SIZE];
    let mut zero_pos = 0;

    let mut y = 0;
    let mut x = 0;
    for i in 0..s.len() {
        let c = s[i];
        if c == b'\n' {
            y += 1;
            x = 0;
            continue;
        }

        let layer = (c - b'0') as usize;
        if layer == 0 {
            first_map[y * MAX_SIZE + x + MAX_SIZE] = 1 << zero_pos;
            zero_pos += 1;
            zero_pos %= 64;
        } else {
            let len = positions[layer - 1].0 as usize;
            positions[layer - 1].1[len] = (y * MAX_SIZE + x + MAX_SIZE) as u16;
            positions[layer - 1].0 += 1;
        }
        x += 1;
    }

    let mut sum = 0;

    let mut next = &mut [0u64; BIG_SIZE];
    let mut current = &mut first_map;

    for layer in 0..8 {
        let (len, positions) = positions[layer];

        for i in &positions[..len as usize] {
            let i = *i as usize;
            next[i] =
                current[i - 1] | current[i + 1] | current[i + MAX_SIZE] | current[i - MAX_SIZE];
        }

        std::mem::swap(&mut current, &mut next);
        next.fill(0);
    }

    let (len9, positions9) = positions[8];
    for i in &positions9[..len9 as usize] {
        let i = *i as usize;
        sum += (current[i - 1] | current[i + 1] | current[i + MAX_SIZE] | current[i - MAX_SIZE])
            .count_ones();
    }

    sum
}

#[aoc(day10, part2)]
pub fn part2(s: &str) -> u16 {
    unsafe { part2_inner(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_inner(s: &str) -> u16 {
    let s = s.as_bytes();

    let mut positions = [(0u16, [0u16; MAX_SIZE * MAX_SIZE / 9]); 9];
    let mut first_map = [0u16; BIG_SIZE];

    let mut y = 0;
    let mut x = 0;
    for i in 0..s.len() {
        let c = s[i];
        if c == b'\n' {
            y += 1;
            x = 0;
            continue;
        }

        let layer = (c - b'0') as usize;
        if layer == 0 {
            first_map[y * MAX_SIZE + x + MAX_SIZE] = 1;
        } else {
            let len = positions[layer - 1].0 as usize;
            positions[layer - 1].1[len] = (y * MAX_SIZE + x + MAX_SIZE) as u16;
            positions[layer - 1].0 += 1;
        }
        x += 1;
    }

    let mut sum = 0;

    let mut next = &mut [0u16; BIG_SIZE];
    let mut current = &mut first_map;

    for layer in 0..8 {
        let (len, positions) = positions[layer];

        for i in &positions[..len as usize] {
            let i = *i as usize;
            next[i] =
                current[i - 1] + current[i + 1] + current[i + MAX_SIZE] + current[i - MAX_SIZE];
        }

        std::mem::swap(&mut current, &mut next);
        next.fill(0);
    }

    let (len9, positions9) = positions[8];
    for i in &positions9[..len9 as usize] {
        let i = *i as usize;
        sum += current[i - 1] + current[i + 1] + current[i + MAX_SIZE] + current[i - MAX_SIZE];
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    //     const EXAMPLE: &str = r"..90..9
    // ...1.98
    // ...2..7
    // 6543456
    // 765.987
    // 876....
    // 987....";

    // const EXAMPLE: &str = "12345\n";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 36);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 81);
    }
}
