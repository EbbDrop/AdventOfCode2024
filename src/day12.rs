use aoc_runner_derive::aoc;

#[cfg(not(test))]
const SIZE: usize = 140;
#[cfg(test)]
const SIZE: usize = 10;

const SIZE1: usize = SIZE + 1;

#[aoc(day12, part1)]
pub fn part1(s: &str) -> u32 {
    unsafe { part1_inner(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_inner(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut id_map = [0u16; SIZE * SIZE1];
    let mut next_id = 1;

    let mut merges = const {
        let mut merges = [0u16; 2048];
        let mut i = 0;
        while i < merges.len() {
            merges[i] = i as u16;
            i += 1;
        }
        merges
    };
    let mut area = [0u16; 2048];
    let mut perimiter = [0u16; 2048];

    for i in 0..SIZE * SIZE1 {
        let c = *s.get_unchecked(i);
        let prev = *s.get_unchecked(i.wrapping_sub(1).min(SIZE * SIZE1 - 1));
        let up = *s.get_unchecked(i.wrapping_sub(SIZE1).min(SIZE * SIZE1 - 1));
        let prev_id = *merges
            .get_unchecked(*id_map.get_unchecked(i.wrapping_sub(1).min(SIZE * SIZE1 - 1)) as usize);
        let up_id = *merges.get_unchecked(
            *id_map.get_unchecked(i.wrapping_sub(SIZE1).min(SIZE * SIZE1 - 1)) as usize,
        );

        if prev == c && c == up && prev_id == up_id {
            *id_map.get_unchecked_mut(i) = prev_id;

            *area.get_unchecked_mut(prev_id as usize) += 1;
        } else if prev == c && c == up {
            *id_map.get_unchecked_mut(i) = prev_id;

            *merges.get_unchecked_mut(up_id as usize) = prev_id;

            *area.get_unchecked_mut(prev_id as usize) += 1;
        } else if prev == c {
            *id_map.get_unchecked_mut(i) = prev_id;

            *area.get_unchecked_mut(prev_id as usize) += 1;
            *perimiter.get_unchecked_mut(prev_id as usize) += 1;
            *perimiter.get_unchecked_mut(up_id as usize) += 1;
        } else if up == c {
            *id_map.get_unchecked_mut(i) = up_id;

            *area.get_unchecked_mut(up_id as usize) += 1;
            *perimiter.get_unchecked_mut(prev_id as usize) += 1;
            *perimiter.get_unchecked_mut(up_id as usize) += 1;
        } else {
            *id_map.get_unchecked_mut(i) = next_id;

            *area.get_unchecked_mut(next_id as usize) += 1;
            *perimiter.get_unchecked_mut(next_id as usize) += 2;
            *perimiter.get_unchecked_mut(prev_id as usize) += 1;
            *perimiter.get_unchecked_mut(up_id as usize) += 1;

            next_id += 1
        }
    }
    for x in 0..SIZE {
        *perimiter.get_unchecked_mut(*id_map.get_unchecked((SIZE - 1) * SIZE1 + x) as usize) += 1;
    }

    // for id in 0..next_id {
    //     println!(
    //         "{id:2}(a: {:2}, p: {:2}) -> {:2}",
    //         area[id as usize], perimiter[id as usize], merges[id as usize]
    //     );
    // }
    // println!("");
    // for y in 0..SIZE {
    //     for x in 0..SIZE {
    //         if s[y * SIZE1 + x] == b'A' {
    //             print!(" {} ", id_map[y * SIZE1 + x]);
    //         } else {
    //             print!("({})", id_map[y * SIZE1 + x]);
    //         }
    //     }
    //     println!("");
    // }

    let mut sum = 0;

    for id in 1..next_id {
        let id = id as usize;

        if *merges.get_unchecked(id) as usize != id {
            let mut real_id = *merges.get_unchecked(id) as usize;
            while real_id < id {
                let new_real_id = *merges.get_unchecked(real_id as usize) as usize;
                if new_real_id == real_id {
                    break;
                }
                real_id = new_real_id;
            }
            *area.get_unchecked_mut(real_id) += *area.get_unchecked(id);
            *perimiter.get_unchecked_mut(real_id) += *perimiter.get_unchecked(id);
            *perimiter.get_unchecked_mut(id) = 0
        }
    }

    for id in 1..next_id {
        let id = id as usize;
        sum += *area.get_unchecked(id) as u32 * *perimiter.get_unchecked(id) as u32;
    }

    sum
}

#[aoc(day12, part2)]
pub fn part2(s: &str) -> u32 {
    unsafe { part2_inner(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_inner(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut id_map = [0u16; SIZE * SIZE1];
    let mut next_id = 1;

    let mut merges = const {
        let mut merges = [0u16; 2048];
        let mut i = 0;
        while i < merges.len() {
            merges[i] = i as u16;
            i += 1;
        }
        merges
    };
    let mut area = [0u16; 2048];
    let mut perimiter = [0u16; 2048];

    for i in 0..SIZE * SIZE1 {
        let c = *s.get_unchecked(i);
        let prev = *s.get_unchecked(i.wrapping_sub(1).min(SIZE * SIZE1 - 1));
        let up = *s.get_unchecked(i.wrapping_sub(SIZE1).min(SIZE * SIZE1 - 1));
        let prevup = *s.get_unchecked(i.wrapping_sub(SIZE1 + 1).min(SIZE * SIZE1 - 1));
        let prev_id = *merges
            .get_unchecked(*id_map.get_unchecked(i.wrapping_sub(1).min(SIZE * SIZE1 - 1)) as usize);
        let up_id = *merges.get_unchecked(
            *id_map.get_unchecked(i.wrapping_sub(SIZE1).min(SIZE * SIZE1 - 1)) as usize,
        );

        if prev == c && c == up && prev_id == up_id {
            // ? A
            // A A
            *id_map.get_unchecked_mut(i) = prev_id;

            *area.get_unchecked_mut(prev_id as usize) += 1;
        } else if prev == c && c == up {
            // ? A
            // A A
            *id_map.get_unchecked_mut(i) = prev_id;

            *merges.get_unchecked_mut(up_id as usize) = prev_id;

            *area.get_unchecked_mut(prev_id as usize) += 1;
        } else if prev == c {
            // ? B
            // A A
            *id_map.get_unchecked_mut(i) = prev_id;

            *area.get_unchecked_mut(prev_id as usize) += 1;

            if prevup == prev {
                // A B
                // A A
                *perimiter.get_unchecked_mut(prev_id as usize) += 1;
                *perimiter.get_unchecked_mut(up_id as usize) += 1;
            } else if prevup == up {
                // B B
                // A A
            } else {
                // C B
                // A A
                *perimiter.get_unchecked_mut(up_id as usize) += 1;
            }
        } else if up == c {
            // ? A
            // B A
            *id_map.get_unchecked_mut(i) = up_id;

            *area.get_unchecked_mut(up_id as usize) += 1;

            if prevup == up {
                // A A
                // B A

                *perimiter.get_unchecked_mut(prev_id as usize) += 1;
                *perimiter.get_unchecked_mut(up_id as usize) += 1;
            } else if prevup == prev {
                // B A
                // B A
            } else {
                // C A
                // B A
                *perimiter.get_unchecked_mut(prev_id as usize) += 1;
            }
        } else {
            // ? C
            // B A
            *id_map.get_unchecked_mut(i) = next_id;

            *area.get_unchecked_mut(next_id as usize) += 1;
            *perimiter.get_unchecked_mut(next_id as usize) += 2;

            if up == prevup && prevup == prev {
                // B B
                // B A
                *perimiter.get_unchecked_mut(up_id as usize) += 1;
                *perimiter.get_unchecked_mut(prev_id as usize) += 1;
            }
            if prevup != up {
                // D C
                // B A
                *perimiter.get_unchecked_mut(up_id as usize) += 1;
            } else {
                // C C
                // B A
            }
            if prevup != prev {
                // D C
                // B A
                *perimiter.get_unchecked_mut(prev_id as usize) += 1;
            } else {
                // B C
                // B A
            }

            next_id += 1
        }
    }
    let mut prevup = b'\n';
    for x in 0..SIZE {
        let up = *s.get_unchecked((SIZE - 1) * SIZE1 + x);
        if prevup != up {
            *perimiter.get_unchecked_mut(*id_map.get_unchecked((SIZE - 1) * SIZE1 + x) as usize) +=
                1;
            prevup = up;
        }
    }

    // for id in 0..next_id {
    //     println!(
    //         "{id:2}(a: {:2}, p: {:2}) -> {:2}",
    //         area[id as usize], perimiter[id as usize], merges[id as usize]
    //     );
    // }
    // println!("");
    // for y in 0..SIZE {
    //     for x in 0..SIZE {
    //         print!("{}~{} ", id_map[y * SIZE1 + x], s[y * SIZE1 + x] as char);
    //     }
    //     println!("");
    // }

    let mut sum = 0;

    for id in 1..next_id {
        let id = id as usize;

        if *merges.get_unchecked(id) as usize != id {
            let mut real_id = *merges.get_unchecked(id) as usize;
            while real_id < id {
                let new_real_id = *merges.get_unchecked(real_id as usize) as usize;
                if new_real_id == real_id {
                    break;
                }
                real_id = new_real_id;
            }
            *area.get_unchecked_mut(real_id) += *area.get_unchecked(id);
            *perimiter.get_unchecked_mut(real_id) += *perimiter.get_unchecked(id);
            *perimiter.get_unchecked_mut(id) = 0
        }
    }

    for id in 1..next_id {
        let id = id as usize;
        sum += *area.get_unchecked(id) as u32 * *perimiter.get_unchecked(id) as u32;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    //     const EXAMPLE: &str = r"ACCCCCC
    // ACAAAAC
    // ACACCAC
    // ACAACAC
    // ACCCCAC
    // AAAAAAC
    // CCCCCCC
    // ";

    //     //   . . . . . .
    //     // A.C C C C C C .
    //     //     . . . .
    //     // A.C.A A A A.C .
    //     //       . .
    //     // A.C.A.C C.A.C .
    //     //       .
    //     // A.C.A A.C.A.C .
    //     //     . .
    //     // A.C C C C.A.C .
    //     //   . . . .
    //     // A A A A A A.C .
    //     // . . . . . .
    //     //.C C C C C C C .
    //     // . . . . . . .

    //     const EXAMPLE_S: &str = r"AAAAAA
    // AAABBA
    // AAABBA
    // ABBAAA
    // ABBAAA
    // AAAAAA";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 1930);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 1206);
    }
}
