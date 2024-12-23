use core::str;
use std::{arch::x86_64::*, mem::transmute};

use aoc_runner_derive::aoc;

const MAX: u32 = 16777216;

static LUT_P1: [u32; MAX as usize] = unsafe { transmute(*include_bytes!("../day22_lut.bin")) };

#[aoc(day22, part1)]
pub fn part1(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut sum = 0;
    let mut i = 0;
    unsafe {
        while i < s.len() {
            #[cfg(not(test))]
            let mut sn = (*s.get_unchecked(i + 0) as u32) * 100000
                + (*s.get_unchecked(i + 1) as u32) * 10000
                + (*s.get_unchecked(i + 2) as u32) * 1000
                + (*s.get_unchecked(i + 3) as u32) * 100
                + (*s.get_unchecked(i + 4) as u32) * 10
                + (*s.get_unchecked(i + 5) as u32) * 1
                - (b'0' as u32 * 111_111);
            #[cfg(not(test))]
            {
                i += 6;
            }

            #[cfg(test)]
            let mut sn = 0;
            while *s.get_unchecked(i) != b'\n' {
                sn *= 10;
                sn += (s.get_unchecked(i) - b'0') as u32;
                i += 1;
            }
            i += 1;

            sum += LUT_P1[sn as usize] as u64;
        }
    }

    sum
}

const SEQUENCES: usize = 18 * 18 * 18 * 18;

#[inline(always)]
unsafe fn vmod10(a: __m256i) -> __m256i {
    let ab_hm = _mm256_mul_epu32(
        _mm256_srli_epi64::<32>(a),
        _mm256_set1_epi32(3435973837u32 as i32),
    );
    let ab_hm = _mm256_and_si256(ab_hm, _mm256_set1_epi64x(0xFFFFFFFF00000000u64 as i64));
    let ab_lm =
        _mm256_srli_epi64::<32>(_mm256_mul_epu32(a, _mm256_set1_epi32(3435973837u32 as i32)));

    let d = _mm256_or_si256(ab_lm, ab_hm);

    let d = _mm256_srli_epi32::<3>(d);
    let c = _mm256_mullo_epi32(d, _mm256_set1_epi32(10));
    _mm256_sub_epi32(a, c)
}

#[inline(always)]
unsafe fn vmod104976(a: __m256i) -> __m256i {
    // Algo from LLVM
    let ab_hm = _mm256_mul_epu32(
        _mm256_srli_epi64::<32>(a),
        _mm256_set1_epi32(2681326939u32 as i32),
    );
    let ab_hm = _mm256_and_si256(ab_hm, _mm256_set1_epi64x(0xFFFFFFFF00000000u64 as i64));
    let ab_lm =
        _mm256_srli_epi64::<32>(_mm256_mul_epu32(a, _mm256_set1_epi32(2681326939u32 as i32)));
    let d = _mm256_or_si256(ab_lm, ab_hm);

    let d = _mm256_srli_epi32::<16>(d);
    let c = _mm256_mullo_epi32(d, _mm256_set1_epi32(104976));
    _mm256_sub_epi32(a, c)
}

static mut DONE: [[u16; 104976]; 8] = [[0u16; SEQUENCES]; 8];

#[aoc(day22, part2)]
pub fn part2(s: &str) -> i32 {
    let s = s.as_bytes();

    let mut sequences = [0; SEQUENCES];

    let mut i = 0;
    let mut monky = 1;
    unsafe {
        for j in 0..8 {
            DONE[j].fill(0);
        }

        while i < s.len() {
            let mut sns: __m256i = _mm256_setzero_si256();
            let mut sns_len = 0;
            while i < s.len() && sns_len < 8 {
                #[cfg(not(test))]
                let mut sn = (*s.get_unchecked(i + 0) as u32) * 100000
                    + (*s.get_unchecked(i + 1) as u32) * 10000
                    + (*s.get_unchecked(i + 2) as u32) * 1000
                    + (*s.get_unchecked(i + 3) as u32) * 100
                    + (*s.get_unchecked(i + 4) as u32) * 10
                    + (*s.get_unchecked(i + 5) as u32) * 1
                    - (b'0' as u32 * 111_111);
                #[cfg(not(test))]
                {
                    i += 6;
                }

                #[cfg(test)]
                let mut sn = 0;
                while *s.get_unchecked(i) != b'\n' {
                    sn *= 10;
                    sn += (s.get_unchecked(i) - b'0') as u32;
                    i += 1;
                }
                i += 1;
                sns = _mm256_permutevar8x32_epi32(sns, _mm256_setr_epi32(7, 0, 1, 2, 3, 4, 5, 6));
                sns = _mm256_blend_epi32::<1>(sns, _mm256_set1_epi32(sn as i32));
                sns_len += 1;
            }

            let mut diffs = _mm256_setzero_si256();
            let mut prev = vmod10(sns);

            for _ in 0..3 {
                let i = _mm256_slli_epi32::<6>(sns);
                let i = _mm256_and_si256(i, _mm256_set1_epi32(16777152));
                sns = _mm256_xor_si256(i, sns);

                let i = _mm256_srli_epi32::<5>(sns);
                sns = _mm256_xor_si256(i, sns);
                let i = _mm256_slli_epi32::<11>(sns);
                let i = _mm256_and_si256(i, _mm256_set1_epi32(16777152));
                sns = _mm256_xor_si256(i, sns);

                let price = vmod10(sns);
                let diff = _mm256_sub_epi32(_mm256_add_epi32(price, _mm256_set1_epi32(9)), prev);
                diffs = _mm256_add_epi32(_mm256_mullo_epi32(diffs, _mm256_set1_epi32(18)), diff);

                prev = price;
            }

            for _ in 4..2000 {
                let i = _mm256_slli_epi32::<6>(sns);
                let i = _mm256_and_si256(i, _mm256_set1_epi32(16777152));
                sns = _mm256_xor_si256(i, sns);
                let i = _mm256_srli_epi32::<5>(sns);
                sns = _mm256_xor_si256(i, sns);
                let i = _mm256_slli_epi32::<11>(sns);
                let i = _mm256_and_si256(i, _mm256_set1_epi32(16777152));
                sns = _mm256_xor_si256(i, sns);

                let price = vmod10(sns);
                let diff = _mm256_sub_epi32(_mm256_add_epi32(price, _mm256_set1_epi32(9)), prev);
                diffs = _mm256_add_epi32(_mm256_mullo_epi32(diffs, _mm256_set1_epi32(18)), diff);
                diffs = vmod104976(diffs);

                let diff_i = _mm256_extract_epi32::<0>(diffs) as usize;
                std::hint::assert_unchecked(diff_i < SEQUENCES);
                if DONE[0][diff_i] != monky + 0 {
                    let price = _mm256_extract_epi32::<0>(price);
                    sequences[diff_i] += price;

                    DONE[0][diff_i] = monky + 0;
                }
                let diff_i = _mm256_extract_epi32::<1>(diffs) as usize;
                std::hint::assert_unchecked(diff_i < SEQUENCES);
                if DONE[1][diff_i] != monky + 1 {
                    let price = _mm256_extract_epi32::<1>(price);
                    sequences[diff_i] += price;

                    DONE[1][diff_i] = monky + 1;
                }
                let diff_i = _mm256_extract_epi32::<2>(diffs) as usize;
                std::hint::assert_unchecked(diff_i < SEQUENCES);
                if DONE[2][diff_i] != monky + 2 {
                    let price = _mm256_extract_epi32::<2>(price);
                    sequences[diff_i] += price;

                    DONE[2][diff_i] = monky + 2;
                }
                let diff_i = _mm256_extract_epi32::<3>(diffs) as usize;
                std::hint::assert_unchecked(diff_i < SEQUENCES);
                if DONE[3][diff_i] != monky + 3 {
                    let price = _mm256_extract_epi32::<3>(price);
                    sequences[diff_i] += price;

                    DONE[3][diff_i] = monky + 3;
                }
                let diff_i = _mm256_extract_epi32::<4>(diffs) as usize;
                std::hint::assert_unchecked(diff_i < SEQUENCES);
                if DONE[4][diff_i] != monky + 4 {
                    let price = _mm256_extract_epi32::<4>(price);
                    sequences[diff_i] += price;

                    DONE[4][diff_i] = monky + 4;
                }
                let diff_i = _mm256_extract_epi32::<5>(diffs) as usize;
                std::hint::assert_unchecked(diff_i < SEQUENCES);
                if DONE[5][diff_i] != monky + 5 {
                    let price = _mm256_extract_epi32::<5>(price);
                    sequences[diff_i] += price;

                    DONE[5][diff_i] = monky + 5;
                }
                let diff_i = _mm256_extract_epi32::<6>(diffs) as usize;
                std::hint::assert_unchecked(diff_i < SEQUENCES);
                if DONE[6][diff_i] != monky + 6 {
                    let price = _mm256_extract_epi32::<6>(price);
                    sequences[diff_i] += price;

                    DONE[6][diff_i] = monky + 6;
                }
                let diff_i = _mm256_extract_epi32::<7>(diffs) as usize;
                std::hint::assert_unchecked(diff_i < SEQUENCES);
                if DONE[7][diff_i] != monky + 7 {
                    let price = _mm256_extract_epi32::<7>(price);
                    sequences[diff_i] += price;

                    DONE[7][diff_i] = monky + 7;
                }

                prev = price;
            }
            monky += 8;
        }

        sequences.into_iter().max().unwrap_unchecked()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let s = r"1
10
100
2024
";

        assert_eq!(part1(s), 37327623);
    }

    #[test]
    fn example_part2() {
        let s = r"1
2
3
2024
";

        assert_eq!(part2(s), 23);
    }
}
