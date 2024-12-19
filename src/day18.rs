use aoc_runner_derive::aoc;

#[cfg(not(test))]
const SIZE: u32 = 71;
#[cfg(test)]
const SIZE: u32 = 7;

const SIZEM1: u32 = SIZE - 1;
const SIZE1: u32 = SIZE + 1;

const START: u32 = 0;
const END: u32 = SIZE * SIZE - 1;

#[aoc(day18, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part1(s.as_bytes())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    full_cost: u32,
    cost: u32,
    i: u32,
}

fn succ(i: u32) -> [u32; 4] {
    let x = i % SIZE as u32;
    let y = i / SIZE as u32;
    match (x, y) {
        (0, 0) => [i + 1, i + SIZE, u32::MAX, u32::MAX],
        (0, SIZEM1) => [i + 1, i - SIZE, u32::MAX, u32::MAX],
        (SIZEM1, SIZEM1) => [i - 1, i - SIZE, u32::MAX, u32::MAX],
        (SIZEM1, 0) => [i - 1, i + SIZE, u32::MAX, u32::MAX],

        (0, _) => [i - SIZE, i + 1, i + SIZE, u32::MAX],
        (SIZEM1, _) => [i - SIZE, i - 1, i + SIZE, u32::MAX],
        (_, 0) => [i - 1, i + 1, i + SIZE, u32::MAX],
        (_, SIZEM1) => [i - SIZE, i - 1, i + 1, u32::MAX],

        (_, _) => [i - SIZE, i - 1, i + 1, i + SIZE],
    }
}

fn hueristic(i: u32) -> u32 {
    let x = i % SIZE as u32;
    let y = i / SIZE as u32;
    SIZE - x + SIZE - y
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part1(s: &[u8]) -> u64 {
    #[cfg(not(test))]
    const BYTES: u32 = 1024;
    #[cfg(test)]
    const BYTES: u32 = 12;

    let mut map = [false; (SIZE * SIZE) as usize];
    let mut i = 0;
    for _ in 0..BYTES {
        let mut x = s[i] - b'0';
        i += 1;
        let c2 = s[i];
        if c2 != b',' {
            x = x * 10 + (c2 - b'0');
            i += 1;
        }
        i += 1;
        let mut y = s[i] - b'0';
        i += 1;
        let c2 = s[i];
        if c2 != b'\n' {
            y = y * 10 + (c2 - b'0');
            i += 1;
        }
        i += 1;

        map[y as usize * SIZE as usize + x as usize] = true;
    }

    let mut to_see = heapless::BinaryHeap::<_, heapless::binary_heap::Min, 128>::new();
    to_see
        .push(State {
            cost: 0,
            full_cost: 0 + hueristic(START),
            i: START,
        })
        .unwrap();

    let mut costs = [u32::MAX; (SIZE * SIZE) as usize];
    costs[START as usize] = 0;

    while let Some(state) = to_see.pop() {
        if state.i == END {
            return state.cost as u64;
        }
        if costs[state.i as usize] < state.cost {
            continue;
        }
        for new_i in succ(state.i) {
            if new_i == u32::MAX {
                break;
            }
            if map[new_i as usize] {
                continue;
            }

            let new_cost = state.cost + 1;

            if new_cost < costs[new_i as usize] {
                costs[new_i as usize] = new_cost;
                let h = hueristic(new_i);
                to_see
                    .push(State {
                        full_cost: new_cost + h,
                        cost: new_cost,
                        i: new_i,
                    })
                    .unwrap();
            }
        }
    }

    0
}

#[aoc(day18, part2)]
pub fn part2(s: &str) -> String {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part2(s.as_bytes()).to_owned()
    }
}

const fn connections(map: [bool; 9], x: usize, y: usize) -> [bool; 9] {
    if map[y * 3 + x] {
        return [false; 9];
    }
    let mut cons = [false; 9];

    let mut stack = [(0, 0); 128];
    let mut stack_i = 1;
    stack[0] = (x, y);

    while stack_i > 0 {
        let (tx, ty) = stack[stack_i - 1];
        stack_i -= 1;

        if map[ty * 3 + tx] {
            continue;
        }
        if !cons[ty * 3 + tx] {
            cons[ty * 3 + tx] = true;
            match (ty, tx) {
                (0, 0) => {
                    stack[stack_i] = (1, 0);
                    stack[stack_i + 1] = (0, 1);
                    stack_i += 2;
                }
                (0, 1) => {
                    stack[stack_i] = (1, 1);
                    stack[stack_i + 1] = (0, 0);
                    stack[stack_i + 2] = (0, 2);
                    stack_i += 3;
                }
                (0, 2) => {
                    stack[stack_i] = (1, 2);
                    stack[stack_i + 1] = (0, 1);
                    stack_i += 2;
                }
                (1, 0) => {
                    stack[stack_i] = (0, 0);
                    stack[stack_i + 1] = (2, 0);
                    stack[stack_i + 2] = (1, 1);
                    stack_i += 3;
                }
                (1, 1) => {
                    stack[stack_i] = (0, 1);
                    stack[stack_i + 1] = (2, 1);
                    stack[stack_i + 2] = (1, 0);
                    stack[stack_i + 3] = (1, 2);
                    stack_i += 4;
                }
                (1, 2) => {
                    stack[stack_i] = (0, 2);
                    stack[stack_i + 1] = (2, 2);
                    stack[stack_i + 2] = (1, 1);
                    stack_i += 3;
                }
                (2, 0) => {
                    stack[stack_i] = (1, 0);
                    stack[stack_i + 1] = (2, 1);
                    stack_i += 2;
                }
                (2, 1) => {
                    stack[stack_i] = (1, 1);
                    stack[stack_i + 1] = (2, 0);
                    stack[stack_i + 2] = (2, 2);
                    stack_i += 3;
                }
                (2, 2) => {
                    stack[stack_i] = (1, 2);
                    stack[stack_i + 1] = (2, 1);
                    stack_i += 2;
                }
                (_, _) => unreachable!(),
            }
        }
    }

    cons
}

// Returns true if adding a wall in the middle of the map would change the situation of a bigger map
const fn will_change_sit(map: [bool; 9]) -> bool {
    let new_map = [
        map[0], map[1], map[2], map[3], true, map[5], map[6], map[7], map[8],
    ];

    let mut x = 0;
    while x < 3 {
        let mut y = 0;
        while y < 3 {
            if x == 1 && y == 1 {
                y += 1;
                continue;
            }
            let or_cons = connections(map, x, y);
            let new_cons = connections(new_map, x, y);
            if or_cons[0] != new_cons[0]
                || or_cons[1] != new_cons[1]
                || or_cons[2] != new_cons[2]
                || or_cons[3] != new_cons[3]
                || or_cons[5] != new_cons[5]
                || or_cons[6] != new_cons[6]
                || or_cons[7] != new_cons[7]
                || or_cons[8] != new_cons[8]
            {
                return true;
            }

            y += 1;
        }
        x += 1;
    }
    false
}

const LUT: [bool; 256] = const {
    let mut lut = [false; 256];
    let mut i = 0;
    while i < 256 {
        let map = [
            i >> 7 & 0b1 != 0,
            i >> 6 & 0b1 != 0,
            i >> 5 & 0b1 != 0,
            i >> 4 & 0b1 != 0,
            false,
            i >> 3 & 0b1 != 0,
            i >> 2 & 0b1 != 0,
            i >> 1 & 0b1 != 0,
            i >> 0 & 0b1 != 0,
        ];
        lut[i] = will_change_sit(map);
        i += 1;
    }
    lut
};

fn is_connected(map: &[bool; SIZE1 as usize * SIZE as usize]) -> bool {
    let mut to_see = heapless::BinaryHeap::<_, heapless::binary_heap::Min, 1024>::new();
    to_see
        .push(State {
            cost: 0,
            full_cost: 0 + hueristic(START),
            i: START,
        })
        .unwrap();

    let mut costs = [u32::MAX; (SIZE1 * SIZE) as usize];
    costs[START as usize] = 0;

    while let Some(state) = to_see.pop() {
        if state.i == SIZE1 * SIZE - 2 {
            return true;
        }
        if costs[state.i as usize] < state.cost {
            continue;
        }
        for new_i in [
            state.i.wrapping_sub(SIZE1).min(map.len() as u32 - 1),
            state.i.wrapping_sub(1).min(map.len() as u32 - 1),
            state.i.wrapping_add(1).min(map.len() as u32 - 1),
            state.i.wrapping_add(SIZE1).min(map.len() as u32 - 1),
        ] {
            if map[new_i as usize] {
                continue;
            }

            let new_cost = state.cost + 1;

            if new_cost < costs[new_i as usize] {
                costs[new_i as usize] = new_cost;
                let h = hueristic(new_i);
                to_see
                    .push(State {
                        full_cost: new_cost + h,
                        cost: new_cost,
                        i: new_i,
                    })
                    .unwrap();
            }
        }
    }

    false
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part2(s: &[u8]) -> &str {
    let mut map = [false; (SIZE * SIZE1) as usize];
    for y in 0..SIZE {
        map[y as usize * SIZE1 as usize + SIZE as usize] = true;
    }

    let mut i = 0;
    loop {
        let start_i = i;
        let mut x = s[i] - b'0';
        i += 1;
        let c2 = s[i];
        if c2 != b',' {
            x = x * 10 + (c2 - b'0');
            i += 1;
        }
        i += 1;
        let mut y = s[i] - b'0';
        i += 1;
        let c2 = s[i];
        if c2 != b'\n' {
            y = y * 10 + (c2 - b'0');
            i += 1;
        }
        i += 1;

        let idx = y as usize * SIZE1 as usize + x as usize;

        let luti = (map[idx.wrapping_sub(SIZE1 as usize + 1).min(map.len() - 1)] as usize) << 7
            | (map[idx.wrapping_sub(SIZE1 as usize).min(map.len() - 1)] as usize) << 6
            | (map[idx.wrapping_sub(SIZE1 as usize - 1).min(map.len() - 1)] as usize) << 5
            | (map[idx.wrapping_sub(1).min(map.len() - 1)] as usize) << 4
            | (map[idx.wrapping_add(1).min(map.len() - 1)] as usize) << 3
            | (map[idx.wrapping_add(SIZE1 as usize - 1).min(map.len() - 1)] as usize) << 2
            | (map[idx.wrapping_add(SIZE1 as usize).min(map.len() - 1)] as usize) << 1
            | (map[idx.wrapping_add(SIZE1 as usize + 1).min(map.len() - 1)] as usize) << 0;

        let will_change = LUT[luti];
        // dbg!(will_change);

        map[idx] = true;

        // if will_change {
        // for py in 0..SIZE {
        //     for px in 0..SIZE {
        //         if py == y as u32 && px == x as u32 {
        //             print!("O");
        //         } else if map[py as usize * SIZE1 as usize + px as usize] {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("");
        // }
        // println!("");
        if !is_connected(&map) {
            return std::str::from_utf8(&s[start_i..i - 1]).unwrap();
        }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 22);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), "");
    }

    #[test]
    fn cons_test() {
        // #.#
        // ..#
        // ##.
        let map = [
            true, false, true, //
            false, false, true, //
            true, true, false,
        ];
        assert_eq!(
            connections(map, 1, 0),
            [false, true, false, true, true, false, false, false, false]
        );
        assert_eq!(
            connections(map, 1, 1),
            [false, true, false, true, true, false, false, false, false]
        );
        assert_eq!(
            connections(map, 0, 1),
            [false, true, false, true, true, false, false, false, false]
        );
        assert_eq!(
            connections(map, 2, 2),
            [false, false, false, false, false, false, false, false, true]
        );
    }

    #[test]
    fn change_sit_test() {
        // #.#
        // ..#
        // ##.
        let map = [
            true, false, true, //
            false, false, true, //
            true, true, false,
        ];
        assert_eq!(will_change_sit(map), true);
        // ..#
        // ..#
        // ##.
        let map = [
            false, false, true, //
            false, false, true, //
            true, true, false,
        ];
        assert_eq!(will_change_sit(map), false);
        // .#.
        // #.#
        // .#.
        let map = [
            false, true, false, //
            true, false, true, //
            false, true, false,
        ];
        assert_eq!(will_change_sit(map), false);
        // #.#
        // ...
        // #.#
        let map = [true, false, true, false, false, false, true, false, true];
        assert_eq!(will_change_sit(map), true);
    }
}
