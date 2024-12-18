use core::str;
use std::hint::unreachable_unchecked;

use aoc_runner_derive::aoc;

// 2 4: bst b=b%8
// 1 5: bxl b=b^5
// 7 5: cdv c=a/2^b
// 1 6: bxl b=b^6
// 0 3: adv a=a/8
// 4 6: bxc b=b^c
// 5 5: out out b
// 3 0: jnz a!=0 -> 0

static mut RESULT: [u8; 128] = [0; 128];

#[aoc(day17, part1)]
pub fn part1(s: &str) -> &'static str {
    unsafe {
        let s = s.as_bytes();
        let mut a = (*s.get_unchecked(12) as u64) * 10000000
            + (*s.get_unchecked(13) as u64) * 1000000
            + (*s.get_unchecked(14) as u64) * 100000
            + (*s.get_unchecked(15) as u64) * 10000
            + (*s.get_unchecked(16) as u64) * 1000
            + (*s.get_unchecked(17) as u64) * 100
            + (*s.get_unchecked(18) as u64) * 10
            + (*s.get_unchecked(19) as u64) * 1
            - (b'0' as u64 * 11111111);

        const I: usize = 59;

        let x = (*s.get_unchecked(I + 6) - b'0') as u64;

        let o1 = *s.get_unchecked(I + 12);
        let o3 = *s.get_unchecked(I + 20);

        let y = match (o1, o3) {
            (b'1', _) => *s.get_unchecked(I + 14) - b'0',
            (_, b'1') => *s.get_unchecked(I + 22) - b'0',
            _ => *s.get_unchecked(I + 18) - b'0',
        } as u64;

        let result_ptr = (&raw mut RESULT).cast::<u8>();
        let mut out_ptr = result_ptr;
        let result_ptr = result_ptr.cast_const();

        while a != 0 {
            let b = a % 8;
            let b = b ^ x;
            let c = a >> b;
            let b = b ^ y ^ c;

            out_ptr.write((b % 8) as u8 + b'0');
            out_ptr.offset(1).write(b',');
            out_ptr = out_ptr.offset(2);
            a = a / 8;
        }

        let out_len = out_ptr.offset_from(result_ptr);

        str::from_utf8_unchecked((*&raw const RESULT).get_unchecked(..(out_len - 1) as usize))
    }
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
    unsafe {
        let s = s.as_bytes();

        const I: usize = 59;

        let x = (*s.get_unchecked(I + 6) - b'0') as usize;

        let o1 = *s.get_unchecked(I + 12);
        let o3 = *s.get_unchecked(I + 20);

        let a = match (o1, o3) {
            (b'1', b'0') => {
                let y = (*s.get_unchecked(I + 14) - b'0') as usize;
                let z = (*s.get_unchecked(I + 18) - b'0') as usize;
                *LUT.get_unchecked(x * 512 + y * 64 + z * 8 + 0)
            }
            (b'1', b'5') => {
                let y = (*s.get_unchecked(I + 14) - b'0') as usize;
                let z = (*s.get_unchecked(I + 18) - b'0') as usize;
                *LUT.get_unchecked(x * 512 + y * 64 + z * 8 + 1)
            }
            (b'1', b'4') => {
                let y = (*s.get_unchecked(I + 14) - b'0') as usize;
                let z = (*s.get_unchecked(I + 22) - b'0') as usize;
                *LUT.get_unchecked(x * 512 + y * 64 + z * 8 + 2)
            }
            (b'0', b'4') => {
                let y = (*s.get_unchecked(I + 18) - b'0') as usize;
                let z = (*s.get_unchecked(I + 22) - b'0') as usize;
                *LUT.get_unchecked(x * 512 + y * 64 + z * 8 + 3)
            }
            (b'0', b'1') => {
                let y = (*s.get_unchecked(I + 22) - b'0') as usize;
                let z = (*s.get_unchecked(I + 18) - b'0') as usize;
                *LUT.get_unchecked(x * 512 + y * 64 + z * 8 + 4)
            }
            (b'4', b'0') => {
                let y = (*s.get_unchecked(I + 18) - b'0') as usize;
                let z = (*s.get_unchecked(I + 14) - b'0') as usize;
                *LUT.get_unchecked(x * 512 + y * 64 + z * 8 + 5)
            }
            (b'4', b'5') => {
                let y = (*s.get_unchecked(I + 18) - b'0') as usize;
                let z = (*s.get_unchecked(I + 14) - b'0') as usize;
                *LUT.get_unchecked(x * 512 + y * 64 + z * 8 + 6)
            }
            (b'4', b'1') => {
                let y = (*s.get_unchecked(I + 22) - b'0') as usize;
                let z = (*s.get_unchecked(I + 14) - b'0') as usize;
                *LUT.get_unchecked(x * 512 + y * 64 + z * 8 + 7)
            }
            _ => unreachable_unchecked(),
        };
        a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let s = r"Register A: 45483412
Register B: 0
Register C: 0

Program: 2,4,1,3,7,5,0,3,4,1,1,5,5,5,3,0
";

        assert_eq!(part1(s), "1,5,0,5,2,0,1,3,5");
        assert_eq!(part2(s), 236581108670061);
    }

    #[test]
    fn test2() {
        let s = r"Register A: 64751475
Register B: 0
Register C: 0

Program: 2,4,1,2,7,5,4,5,1,3,5,5,0,3,3,0
";

        assert_eq!(part1(s), "3,1,4,3,1,7,1,6,3");
        assert_eq!(part2(s), 37221270076916);
    }
}
