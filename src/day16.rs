use aoc_runner_derive::aoc;

#[cfg(test)]
const SIZE: usize = 15;
#[cfg(not(test))]
const SIZE: usize = 141;

const SIZE1: usize = SIZE + 1;
const MAX_INDX: usize = SIZE * SIZE1;

const START: u32 = ((SIZE - 2) * SIZE1 + 1) as u32;
const END: u32 = (SIZE1 + SIZE - 2) as u32;

#[aoc(day16, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part1(s.as_bytes())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Direction {
    fn all_not_eq(&self) -> [Direction; 3] {
        match self {
            Direction::N => [Direction::N, Direction::E, Direction::W],
            Direction::E => [Direction::N, Direction::E, Direction::S],
            Direction::S => [Direction::E, Direction::S, Direction::W],
            Direction::W => [Direction::N, Direction::S, Direction::W],
        }
    }

    fn step(&self) -> i32 {
        match self {
            Direction::N => -(SIZE1 as i32),
            Direction::E => 1,
            Direction::S => SIZE1 as i32,
            Direction::W => -1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    full_cost: u32,
    cost: u32,
    i: u32,
    d: Direction,
}

fn hueristic(i: u32, d: Direction) -> u32 {
    let x = i % SIZE1 as u32;
    let y = i / SIZE1 as u32;
    (((SIZE as u32 - 1) - x) + y - 1) as u32
        + match d {
            Direction::N => 0,
            Direction::E => 0,
            Direction::S => 1000,
            Direction::W => 1000,
        }
}

fn get_succ(i: u32, dir: Direction, map: &[u8]) -> Option<(u32, Direction, u32)> {
    let new_i = (i as i32 + dir.step()) as u32;

    if map[new_i as usize] == b'#' {
        return None;
    }
    Some((new_i, dir, 1))
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part1(s: &[u8]) -> u64 {
    let mut to_see = heapless::BinaryHeap::<_, heapless::binary_heap::Min, 128>::new();
    to_see
        .push(State {
            cost: 0,
            full_cost: hueristic(START, Direction::E),
            i: START,
            d: Direction::E,
        })
        .unwrap();

    let mut costs = [u32::MAX; SIZE * SIZE1];
    costs[START as usize] = 0;

    while let Some(state) = to_see.pop() {
        if state.i == END {
            // let mut path = Vec::new();
            // let mut i = END;
            // while i != START {
            //     path.push(i);
            //     i = prev[i];
            // }
            // for i in 0..(SIZE * SIZE1) - 1 {
            //     if path.contains(&i) {
            //         print!("o");
            //     } else {
            //         print!("{}", s[i] as char);
            //     }
            // }

            return state.cost as u64;
        }
        if costs[state.i as usize] < state.cost {
            continue;
        }
        for dir in state.d.all_not_eq() {
            if let Some((new_i, new_d, move_cost)) = get_succ(state.i, dir, s) {
                let new_cost = state.cost + move_cost + if dir != state.d { 1000 } else { 0 };

                if new_cost < costs[new_i as usize] {
                    costs[new_i as usize] = new_cost;
                    // prev[new_i] = state.i;
                    let h = hueristic(new_i, new_d);
                    to_see
                        .push(State {
                            full_cost: new_cost + h,
                            cost: new_cost,
                            i: new_i,
                            d: new_d,
                        })
                        .unwrap();
                }
            }
        }
    }

    0
}

#[aoc(day16, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part2(s.as_bytes())
    }
}

fn get_idx(i: u32, d: Direction) -> usize {
    (d as usize) * MAX_INDX + i as usize
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part2(s: &[u8]) -> u64 {
    let mut to_see = heapless::BinaryHeap::<_, heapless::binary_heap::Min, 1024>::new();
    to_see
        .push(State {
            cost: 0,
            full_cost: hueristic(START, Direction::E),
            i: START,
            d: Direction::E,
        })
        .unwrap();

    let mut costs = [u32::MAX; MAX_INDX * 4];
    costs[get_idx(START, Direction::E)] = 0;
    let mut min_cost = u32::MAX;

    let mut prev = [[0u32; 3]; MAX_INDX * 4];

    while let Some(state) = to_see.pop() {
        // println!("{}, {:?}", state.i, state.d);
        if state.full_cost > min_cost {
            break;
        }
        if state.i == END {
            min_cost = state.cost;
        }
        if costs[get_idx(state.i, state.d)] < state.cost {
            continue;
        }
        for dir in state.d.all_not_eq() {
            if let Some((new_i, new_d, move_cost)) = get_succ(state.i, dir, s) {
                let new_cost = state.cost + move_cost + if dir != state.d { 1000 } else { 0 };

                if new_cost < costs[get_idx(new_i, new_d)] {
                    costs[get_idx(new_i, new_d)] = new_cost;

                    prev[get_idx(new_i, new_d)] = [get_idx(state.i, state.d) as u32, 0, 0];

                    let h = hueristic(new_i, new_d);
                    to_see
                        .push(State {
                            full_cost: new_cost + h,
                            cost: new_cost,
                            i: new_i,
                            d: new_d,
                        })
                        .unwrap();
                } else if new_cost == costs[get_idx(new_i, new_d)] {
                    for p in &mut prev[get_idx(new_i, new_d)] {
                        if *p == get_idx(state.i, state.d) as u32 {
                            break;
                        } else if *p == 0 {
                            *p = get_idx(state.i, state.d) as u32;
                            break;
                        }
                    }
                    // println!(
                    //     "eq cost {}: {},{}: {:?}",
                    //     new_i,
                    //     new_i % SIZE1,
                    //     new_i / SIZE1,
                    //     prev[get_idx(new_i, new_d)]
                    // );
                }
            }
        }
    }

    let mut visited_small = [false; MAX_INDX];

    let mut stack = heapless::Vec::<u32, 64>::new();
    stack.push(get_idx(END, Direction::E) as u32).unwrap();
    stack.push(get_idx(END, Direction::N) as u32).unwrap();

    let mut sum = 0;
    while let Some(i) = stack.pop() {
        if !visited_small[i as usize % MAX_INDX] {
            visited_small[i as usize % MAX_INDX] = true;
            sum += 1;
        }

        for p in prev[i as usize] {
            if p != 0 {
                stack.push(p).unwrap();
            } else {
                break;
            }
        }
        prev[i as usize] = [0, 0, 0];
    }

    // for i in 0..(SIZE * SIZE1) - 1 {
    //     if visited[i] {
    //         print!("o");
    //     } else {
    //         print!("{}", s[i] as char);
    //     }
    // }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 7036);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 45);
    }
}
