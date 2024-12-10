use aoc_runner_derive::aoc;

#[cfg(not(test))]
const INPUT_SIZE: usize = 19999;
#[cfg(test)]
const INPUT_SIZE: usize = 19;

// Assume a avrage of 5
const MAX_BLOCKS_SIZE: usize = INPUT_SIZE * 5;

#[aoc(day9, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        part1_inner(s)
    }
}

#[inline(always)]
fn get_checksum(block_id: usize, position: u32, size: u32) -> u64 {
    size as u64 * block_id as u64 * (2 * position as u64 + size as u64 - 1)
}

fn part1_inner(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut front_pointer = 0;
    let mut back_pointer = INPUT_SIZE - 1;

    let mut sum = 0;

    let mut position = 0;

    let mut to_move_size = 0;
    let mut to_move_id = 0;

    while front_pointer <= back_pointer {
        let block_size = (s[front_pointer] - b'0') as u32;
        front_pointer += 1;

        sum += get_checksum(front_pointer / 2, position, block_size);
        position += block_size;

        let mut empty_size = (s[front_pointer] - b'0') as u32;
        front_pointer += 1;

        while empty_size > 0 {
            if to_move_size == 0 {
                to_move_size = (s[back_pointer] - b'0') as u32;
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

    println!("");
    dbg!(to_move_id, to_move_size);
    dbg!(front_pointer, back_pointer);

    sum / 2
}

#[aoc(day9, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        part2_inner(s)
    }
}

fn part2_inner(s: &str) -> u64 {
    let input_map = s
        .as_bytes()
        .strip_suffix(&[b'\n'])
        .unwrap_or_else(|| s.as_bytes());

    // All blocks with their id, and empty space
    let mut full_input_blocks: [u32; MAX_BLOCKS_SIZE] = [0; MAX_BLOCKS_SIZE];
    // Like full but without empty space
    let mut input_blocks: [(usize, u32); INPUT_SIZE / 2 + 1] = [(0, 0); INPUT_SIZE / 2 + 1];

    let mut block_id = 0;
    let mut full_input_blocks_i: usize = 0;
    let mut input_blocks_i: usize = 0;

    for c in input_map.chunks(2) {
        let block_size = (c[0] - b'0') as usize;

        full_input_blocks[full_input_blocks_i..full_input_blocks_i + block_size].fill(block_id);

        input_blocks[input_blocks_i] = (full_input_blocks_i, block_size as u32);

        full_input_blocks_i += block_size;
        input_blocks_i += 1;

        if c.len() == 2 {
            let empty_size = (c[1] - b'0') as usize;
            full_input_blocks_i += empty_size;
        }

        block_id += 1;
    }

    let full_input_blocks_start: usize = (input_map[0] - b'0') as usize;

    for block_id in (1..input_blocks_i).rev() {
        let (position, size) = input_blocks[block_id];

        let mut i = full_input_blocks_start;
        let mut search_size = 0;
        while i < position {
            if full_input_blocks[i as usize] == 0 {
                search_size += 1;
                if search_size == size {
                    full_input_blocks[position..position + size as usize].fill(0);
                    full_input_blocks[i - size as usize + 1..i + 1].fill(block_id as u32);
                    break;
                }
            } else {
                search_size = 0;
            }
            i += 1;
        }
    }

    full_input_blocks
        .iter()
        .enumerate()
        .map(|(i, id)| i as u64 * *id as u64)
        .sum()
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
