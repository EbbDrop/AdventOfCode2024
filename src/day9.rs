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
    // println!(
    //     "at {}, got: \"{}\" ({})",
    //     position,
    //     std::iter::repeat((block_id as u8 + b'0') as char)
    //         .take(size as usize)
    //         .collect::<String>(),
    //     size
    // );
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

    let mut sizes = [0; INPUT_SIZE];

    for i in 0..INPUT_SIZE {
        sizes[i] = s[i] - b'0';
    }

    let mut i = INPUT_SIZE - 1;

    let mut sum = 0;
    loop {
        let block_size = s[i] - b'0';

        let mut empty_pointer = 1;
        let mut position = 0;
        while empty_pointer < i {
            position += sizes[empty_pointer - 1] as u32;

            let empty_size = sizes[empty_pointer];
            if empty_size >= block_size {
                sizes[empty_pointer as usize] -= block_size;
                sizes[empty_pointer as usize - 1] += block_size;

                sum += get_checksum(i / 2, position, block_size as u32);
                break;
            }
            position += empty_size as u32;
            empty_pointer += 2;
        }
        if empty_pointer >= i {
            sum += get_checksum(i / 2, position, block_size as u32);
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
