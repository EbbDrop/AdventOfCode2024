use aoc_runner_derive::aoc;

#[cfg(not(test))]
const INPUT_SIZE: usize = 19999;
#[cfg(test)]
const INPUT_SIZE: usize = 19;

#[aoc(day9, part1)]
pub fn part1(s: &str) -> u64 {
    unsafe { part1_inner(s) }
}

#[inline(always)]
fn get_checksum(block_id: usize, position: u32, size: u32) -> u64 {
    size as u64 * block_id as u64 * (2 * position as u64 + size as u64 - 1)
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part1_inner(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut front_pointer = 0;
    let mut back_pointer = INPUT_SIZE - 1;

    let mut sum = 0;

    let mut position = 0;

    let mut to_move_size = 0;
    let mut to_move_id = 0;

    while front_pointer <= back_pointer {
        let block_size = (*s.get_unchecked(front_pointer) - b'0') as u32;
        front_pointer += 1;

        sum += get_checksum(front_pointer / 2, position, block_size);
        position += block_size;

        let mut empty_size = (*s.get_unchecked(front_pointer) - b'0') as u32;
        front_pointer += 1;

        while empty_size > 0 {
            if to_move_size == 0 {
                to_move_size = (*s.get_unchecked(back_pointer) - b'0') as u32;
                to_move_id = back_pointer / 2;
                back_pointer -= 2;
            }

            let move_size = to_move_size.min(empty_size);
            sum += get_checksum(to_move_id, position, move_size);
            position += move_size;

            to_move_size -= move_size;
            empty_size -= move_size;
        }
    }
    sum += get_checksum(to_move_id, position, to_move_size);

    sum / 2
}

#[aoc(day9, part2)]
pub fn part2(s: &str) -> u64 {
    unsafe { part2_inner(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_inner(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut jump_table: [u16; INPUT_SIZE / 2 + 1] = const {
        let mut t = [0; INPUT_SIZE / 2 + 1];
        let mut i = 0;
        while i < INPUT_SIZE / 2 + 1 {
            t[i] = (i + 1) as u16;
            i += 1;
        }
        t
    };

    let mut sizes = [0; INPUT_SIZE / 2 + 1];
    let mut position_table = [0; INPUT_SIZE / 2 + 1];
    let mut position = 0u32;

    let mut prev_pointer = 0;
    for i in 0..INPUT_SIZE / 2 {
        *sizes.get_unchecked_mut(i + 1) = s.get_unchecked(i * 2 + 1) - b'0';

        position += (s.get_unchecked(i * 2) - b'0') as u32;

        *position_table.get_unchecked_mut(i + 1) = position;

        position += (s.get_unchecked(i * 2 + 1) - b'0') as u32;

        if s.get_unchecked(i * 2 + 1) - b'0' == 0 {
            *jump_table.get_unchecked_mut(prev_pointer) += 1;
        } else {
            prev_pointer = *jump_table.get_unchecked(prev_pointer) as usize;
        }
    }

    let mut i = INPUT_SIZE - 1;

    let mut sum = 0;
    loop {
        let block_size = s.get_unchecked(i) - b'0';

        let mut prev_pointer = 0;
        let mut pointer = *jump_table.get_unchecked(0) as usize;

        while pointer * 2 <= i {
            let empty_size = *sizes.get_unchecked(pointer);

            if empty_size >= block_size {
                sum += get_checksum(
                    i / 2,
                    *position_table.get_unchecked(pointer) as u32,
                    block_size as u32,
                );

                *sizes.get_unchecked_mut(pointer) -= block_size;
                if *sizes.get_unchecked(pointer) == 0 {
                    *jump_table.get_unchecked_mut(prev_pointer) =
                        *jump_table.get_unchecked(pointer);
                }
                *position_table.get_unchecked_mut(pointer) += block_size as u32;

                break;
            }
            prev_pointer = pointer;
            pointer = *jump_table.get_unchecked(pointer) as usize;
        }
        if pointer * 2 > i {
            sum += get_checksum(
                i / 2,
                *position_table.get_unchecked(i / 2) as u32 + *sizes.get_unchecked(i / 2) as u32,
                block_size as u32,
            );
        }
        if i == 0 {
            break;
        }
        i -= 2;
    }

    sum / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402\n";
    // const EXAMPLE: &str = "12345\n";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 1928);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 2858);
    }
}
