use core::str;

use aoc_runner_derive::aoc;

// 2 4: bst b=b%8
// 1 5: bxl b=b^5
// 7 5: cdv c=a/2^b
// 1 6: bxl b=b^6
// 0 3: adv a=a/8
// 4 6: bxc b=b^c
// 5 5: out out b
// 3 0: jnz a!=0 -> 0

#[aoc(day17, part1)]
pub fn part1(s: &str) -> &'static str {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part1(s.as_bytes())
    }
}

static mut RESULT: [u8; 128] = [0; 128];

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part1(s: &[u8]) -> &'static str {
    let mut i = 13;

    let mut a = (s[12] - b'0') as u32;
    while s[i] != b'\n' {
        a *= 10;
        a += (s[i] - b'0') as u32;
        i += 1;
    }

    i += 14;
    let mut b = (s[i - 1] - b'0') as u32;
    while s[i] != b'\n' {
        b *= 10;
        b += (s[i] - b'0') as u32;
        i += 1;
    }

    i += 14;
    let mut c = (s[i - 1] - b'0') as u32;
    while s[i] != b'\n' {
        c *= 10;
        c += (s[i] - b'0') as u32;
        i += 1;
    }

    let code_start = i + 11;

    let result_ptr = (&raw mut RESULT).cast::<u8>();
    let mut out_ptr = result_ptr;
    let result_ptr = result_ptr.cast_const();

    let mut ip = 0;
    while ip * 2 + code_start < s.len() {
        let opcode = s[ip * 2 + code_start];
        let oper = s[ip * 2 + code_start + 2];

        let get_lit = || (oper - b'0') as u32;
        let get_combo = || match oper {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => a,
            b'5' => b,
            b'6' => c,
            _ => unreachable!(),
        };

        match opcode {
            b'0' => {
                a = a / 2u32.pow(get_combo());
                ip += 2;
            }
            b'1' => {
                b = b ^ get_lit();
                ip += 2;
            }
            b'2' => {
                b = get_combo() % 8;
                ip += 2;
            }
            b'3' => {
                if a != 0 {
                    ip = get_lit() as usize;
                } else {
                    ip += 2;
                }
            }
            b'4' => {
                b = b ^ c;
                ip += 2;
            }
            b'5' => unsafe {
                out_ptr.write((get_combo() % 8) as u8 + b'0');
                out_ptr.offset(1).write(b',');

                out_ptr = out_ptr.offset(2);
                ip += 2;
            },
            b'6' => {
                b = a / 2u32.pow(get_combo());
                ip += 2;
            }
            b'7' => {
                c = a / 2u32.pow(get_combo());
                ip += 2;
            }
            _ => unreachable!(),
        }
    }

    let out_len = unsafe { out_ptr.offset_from(result_ptr) };

    unsafe { str::from_utf8(&RESULT[..(out_len - 1) as usize]).unwrap() }
}

const fn find_a_r(p: &[u64], x: u64, y: u64, pa: u64) -> Option<u64> {
    let Some((last, rest)) = p.split_last() else {
        return Some(pa / 8);
    };

    let mut ia = 0;
    while ia < 8 {
        let a = pa | ia;

        let b = a % 8;
        let b = b ^ x;
        let c = a >> b;
        let b = b ^ y;
        let b = b ^ c;
        if b % 8 == *last {
            if let Some(a) = find_a_r(rest, x, y, a * 8) {
                return Some(a);
            }
        }

        ia += 1;
    }
    None
}

const fn find_a(p: [u64; 16], x: u64, y: u64) -> u64 {
    match find_a_r(&p, x, y, 0) {
        Some(a) => a,
        None => 0, // :pray:,
    }
}

static LUT: [u64; 8 * 8 * 8 * 8] = const {
    let mut lut = [0u64; 8 * 8 * 8 * 8];
    let mut x = 0;
    while x < 8 {
        let mut y = 0;
        while y < 8 {
            let mut z = 0;
            while z < 8 {
                lut[x as usize * 512 + y as usize * 64 + z as usize * 8 + 0] =
                    find_a([2, 4, 1, x, 7, 5, 1, y, 4, z, 0, 3, 5, 5, 3, 0], x, y);
                lut[x as usize * 512 + y as usize * 64 + z as usize * 8 + 1] =
                    find_a([2, 4, 1, x, 7, 5, 1, y, 4, z, 5, 5, 0, 3, 3, 0], x, y);

                lut[x as usize * 512 + y as usize * 64 + z as usize * 8 + 2] =
                    find_a([2, 4, 1, x, 7, 5, 1, y, 0, 3, 4, z, 5, 5, 3, 0], x, y);

                lut[x as usize * 512 + y as usize * 64 + z as usize * 8 + 3] =
                    find_a([2, 4, 1, x, 7, 5, 0, 3, 1, y, 4, z, 5, 5, 3, 0], x, y);

                lut[x as usize * 512 + y as usize * 64 + z as usize * 8 + 4] =
                    find_a([2, 4, 1, x, 7, 5, 0, 3, 4, z, 1, y, 5, 5, 3, 0], x, y);

                lut[x as usize * 512 + y as usize * 64 + z as usize * 8 + 5] =
                    find_a([2, 4, 1, x, 7, 5, 4, z, 1, y, 0, 3, 5, 5, 3, 0], x, y);
                lut[x as usize * 512 + y as usize * 64 + z as usize * 8 + 6] =
                    find_a([2, 4, 1, x, 7, 5, 4, z, 1, y, 5, 5, 0, 3, 3, 0], x, y);

                lut[x as usize * 512 + y as usize * 64 + z as usize * 8 + 7] =
                    find_a([2, 4, 1, x, 7, 5, 4, z, 0, 3, 1, y, 5, 5, 3, 0], x, y);
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }

    lut
};

#[aoc(day17, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part2(s.as_bytes())
    }
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part2(s: &[u8]) -> u64 {
    let i = memchr::memchr(b',', s).unwrap() - 1;

    let a = find_a(
        [
            (s[i + 0] - b'0') as u64,
            (s[i + 2] - b'0') as u64,
            (s[i + 4] - b'0') as u64,
            (s[i + 6] - b'0') as u64,
            (s[i + 8] - b'0') as u64,
            (s[i + 10] - b'0') as u64,
            (s[i + 12] - b'0') as u64,
            (s[i + 14] - b'0') as u64,
            (s[i + 16] - b'0') as u64,
            (s[i + 18] - b'0') as u64,
            (s[i + 20] - b'0') as u64,
            (s[i + 22] - b'0') as u64,
            (s[i + 24] - b'0') as u64,
            (s[i + 26] - b'0') as u64,
            (s[i + 28] - b'0') as u64,
            (s[i + 30] - b'0') as u64,
        ],
        5,
        6,
    );

    a

    // let x = (s[i + 6] - b'0') as usize;

    // let o1 = s[i + 12];
    // let o2 = s[i + 16];
    // let o3 = s[i + 20];
    // let o4 = s[i + 24];

    // match (o1, o2, o3, o4) {
    //     (b'1', b'4', b'0', b'5') => {
    //         let y = (s[i + 14] - b'0') as usize;
    //         let z = (s[i + 18] - b'0') as usize;
    //         LUT[x * 512 + y * 64 + z * 8 + 0]
    //     }
    //     (b'1', b'4', b'5', b'0') => {
    //         let y = (s[i + 14] - b'0') as usize;
    //         let z = (s[i + 18] - b'0') as usize;
    //         LUT[x * 512 + y * 64 + z * 8 + 1]
    //     }
    //     (b'1', b'0', b'4', b'5') => {
    //         let y = (s[i + 14] - b'0') as usize;
    //         let z = (s[i + 22] - b'0') as usize;
    //         LUT[x * 512 + y * 64 + z * 8 + 0]
    //     }
    //     (b'0', b'1', b'4', b'5') => {
    //         let y = (s[i + 18] - b'0') as usize;
    //         let z = (s[i + 22] - b'0') as usize;
    //         LUT[x * 512 + y * 64 + z * 8 + 0]
    //     }
    //     (b'0', b'4', b'1', b'5') => {
    //         let y = (s[i + 18] - b'0') as usize;
    //         let z = (s[i + 22] - b'0') as usize;
    //         LUT[x * 512 + y * 64 + z * 8 + 0]
    //     }
    //     (b'4', b'1', b'0', b'5') => {
    //         let y = (s[i + 18] - b'0') as usize;
    //         let z = (s[i + 14] - b'0') as usize;
    //         LUT[x * 512 + y * 64 + z * 8 + 0]
    //     }
    //     (b'4', b'1', b'5', b'0') => {
    //         let y = (s[i + 18] - b'0') as usize;
    //         let z = (s[i + 14] - b'0') as usize;
    //         LUT[x * 512 + y * 64 + z * 8 + 0]
    //     }
    //     (b'4', b'0', b'1', b'5') => {
    //         let y = (s[i + 22] - b'0') as usize;
    //         let z = (s[i + 14] - b'0') as usize;
    //         LUT[x * 512 + y * 64 + z * 8 + 0]
    //     }
    //     _ => unreachable!(),
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r"Register A: 0
Register B: 0
Register C: 9

Program: 2,6,5,5
";

    #[test]
    fn example1() {
        assert_eq!(part1(EXAMPLE_1), "1");
    }

    const EXAMPLE_2: &str = r"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
";

    #[test]
    fn example2() {
        assert_eq!(part1(EXAMPLE_2), "0,1,2");
    }

    const EXAMPLE_3: &str = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    fn example3() {
        assert_eq!(part1(EXAMPLE_3), "4,2,5,6,7,7,7,7,3,1,0");
    }

    const EXAMPLE_4: &str = r"Register A: 0
Register B: 29
Register C: 0

Program: 1,7,5,0
";
    #[test]
    fn example4() {
        assert_eq!(part1(EXAMPLE_4), "0");
    }

    const EXAMPLE_5: &str = r"Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0,5,1
";
    #[test]
    fn example5() {
        assert_eq!(part1(EXAMPLE_5), "1");
    }

    const EXAMPLE_PART1: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_PART1), "4,6,3,5,6,3,5,2,1,0");
    }
}
