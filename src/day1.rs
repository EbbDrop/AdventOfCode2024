use aoc_runner_derive::aoc;

fn u32_from_bytes(&[a, b, c, d, e]: &[u8; 5]) -> u32 {
    (a - b'0') as u32 * 10000
        + (b - b'0') as u32 * 1000
        + (c - b'0') as u32 * 100
        + (d - b'0') as u32 * 10
        + (e - b'0') as u32
}

#[aoc(day1, part1)]
fn part1(s: &str) -> u32 {
    let mut a: Vec<u32> = Vec::with_capacity(1000);
    let mut b: Vec<u32> = Vec::with_capacity(1000);

    for line in s.as_bytes().chunks(14) {
        a.push(u32_from_bytes(&line[0..5].try_into().unwrap()));
        b.push(u32_from_bytes(&line[8..13].try_into().unwrap()));
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut sum = 0;
    for (a, b) in a.iter().zip(&b) {
        sum += a.abs_diff(*b);
    }

    sum
}

#[aoc(day1, part2)]
pub fn part2(s: &str) -> u32 {
    let mut a: Vec<u32> = Vec::with_capacity(1000);
    // let mut b: FxHashMap<u32, u32> = FxHashMap::default();
    let mut b: [u8; 99999 - 10000] = [0; 99999 - 10000];

    for line in s.as_bytes().chunks(14) {
        a.push(u32_from_bytes(&line[0..5].try_into().unwrap()));
        let b_num = u32_from_bytes(&line[8..13].try_into().unwrap());

        // *b.entry(b_num).or_insert(0) += 1;
        b[(b_num - 10000) as usize] += 1;
    }

    let mut sum = 0;

    for a in a {
        // sum += a * b.get(&a).cloned().unwrap_or(0);
        sum += a * b[(a - 10000) as usize] as u32;
    }

    sum
}
