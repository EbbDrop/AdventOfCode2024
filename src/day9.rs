use aoc_runner_derive::aoc;

#[cfg(not(test))]
const INPUT_SIZE: usize = 19999;
#[cfg(test)]
const INPUT_SIZE: usize = 19;

#[aoc(day9, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        part1_inner(s)
    }
}

#[inline(always)]
fn get_checksum(block_id: usize, position: u32, size: u32) -> u64 {
    // println!("at {:5}, got: {:5} x {}", position, block_id, size);
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
    let s = s.as_bytes();

    let mut jump_table: [usize; INPUT_SIZE / 2 + 1] = const {
        let mut t = [0; INPUT_SIZE / 2 + 1];
        let mut i = 0;
        while i < INPUT_SIZE / 2 + 1 {
            t[i] = i + 1;
            i += 1;
        }
        t
    };

    let mut sizes = [0; INPUT_SIZE / 2 + 1];
    let mut position_table = [0; INPUT_SIZE / 2 + 1];
    let mut or_position_table = [0; INPUT_SIZE / 2 + 1];
    let mut position = 0u32;
    for i in 0..INPUT_SIZE / 2 {
        sizes[i + 1] = s[i * 2 + 1] - b'0';

        position += (s[i * 2] - b'0') as u32;

        position_table[i + 1] = position;

        position += (s[i * 2 + 1] - b'0') as u32;

        or_position_table[i + 1] = position;
    }

    let mut i = INPUT_SIZE - 1;

    let mut sum = 0;
    loop {
        let hu = i / 2 == 5303;
        let block_size = s[i] - b'0';
        // for i in 0..INPUT_SIZE / 2 + 1 {
        //     println!(
        //         "j: {}, p: {}, s: {}",
        //         jump_table[i], position_table[i], sizes[i]
        //     );
        // }
        // dbg!(block_size);

        let mut prev_pointer = 0;
        let mut pointer = jump_table[0];

        while pointer * 2 + 1 < i {
            if hu {
                println!("{pointer}");
            }
            let empty_size = sizes[pointer];

            if empty_size >= block_size {
                sum += get_checksum(i / 2, position_table[pointer] as u32, block_size as u32);

                sizes[pointer] -= block_size;
                if sizes[pointer] == 0 {
                    jump_table[prev_pointer] = jump_table[pointer];
                }
                position_table[pointer] += block_size as u32;

                break;
            }
            prev_pointer = pointer;
            pointer = jump_table[pointer];
        }
        if pointer * 2 + 1 >= i {
            // println!("{:?}", or_position_table);
            sum += get_checksum(i / 2, or_position_table[i / 2] as u32, block_size as u32);
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

// at 00, got: "00" (2)
// at 02, got: "99" (2)
// at 04, got: "2" (1)
// at 05, got: "111" (3)
// at 08, got: "777" (3)
// at 12, got: "44" (2)
// at 15, got: "333" (3)
// at 22, got: "5555" (4)
// at 27, got: "6666" (4)
// at 36, got: "8888" (4)
