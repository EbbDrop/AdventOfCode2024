use std::{env, fs::File, io::Write, path::PathBuf};

use bytemuck::{cast_box, Pod, Zeroable};
use fxhash::FxHashMap as HashMap;

const LUT_SIZE: u64 = 100;

const LUT: [u64; LUT_SIZE as usize] = const {
    let mut lut = [0; LUT_SIZE as usize];

    let mut i = 0u64;
    while i < LUT_SIZE {
        let r = if i == 0 {
            1
        } else if i.ilog10() % 2 == 1 {
            let i_digits = i.ilog10() + 1;
            let tens = 10u64.pow(i_digits / 2);

            (i % tens) << 32 | (i / tens)
        } else {
            i * 2024
        };
        lut[i as usize] = r;
        i += 1;
    }

    lut
};

fn amount_of_stones(num: u64, blinks_left: u64, cach: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks_left == 0 {
        return 1;
    }
    if let Some(r) = cach.get(&(num, blinks_left)) {
        return *r;
    }
    const { assert!(LUT_SIZE == 100) };
    let r = match num {
        0 => amount_of_stones(1, blinks_left - 1, cach),
        1..=9 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        10..=99 => {
            let r = LUT[num as usize];
            amount_of_stones(r & (2u64.pow(32) - 1), blinks_left - 1, cach)
                + amount_of_stones((r >> 32) & (2u64.pow(32) - 1), blinks_left - 1, cach)
        }
        100..=999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        1000..=9999 => {
            amount_of_stones(num / 100, blinks_left - 1, cach)
                + amount_of_stones(num % 100, blinks_left - 1, cach)
        }
        10000..=99999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        100000..=999999 => {
            amount_of_stones(num / 1000, blinks_left - 1, cach)
                + amount_of_stones(num % 1000, blinks_left - 1, cach)
        }
        1000000..=9999999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        10000000..=99999999 => {
            amount_of_stones(num / 10000, blinks_left - 1, cach)
                + amount_of_stones(num % 10000, blinks_left - 1, cach)
        }
        100000000..=999999999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        1000000000..=9999999999 => {
            amount_of_stones(num / 100000, blinks_left - 1, cach)
                + amount_of_stones(num % 100000, blinks_left - 1, cach)
        }
        10000000000..=99999999999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        100000000000..=999999999999 => {
            amount_of_stones(num / 1000000, blinks_left - 1, cach)
                + amount_of_stones(num % 1000000, blinks_left - 1, cach)
        }
        1000000000000..=9999999999999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        10000000000000..=99999999999999 => {
            amount_of_stones(num / 10000000, blinks_left - 1, cach)
                + amount_of_stones(num % 10000000, blinks_left - 1, cach)
        }
        100000000000000..=999999999999999 => amount_of_stones(num * 2024, blinks_left - 1, cach),
        1000000000000000..=9999999999999999 => {
            amount_of_stones(num / 100000000, blinks_left - 1, cach)
                + amount_of_stones(num % 100000000, blinks_left - 1, cach)
        }
        10000000000000000..=99999999999999999 => {
            amount_of_stones(num * 2024, blinks_left - 1, cach)
        }
        100000000000000000..=999999999999999999 => {
            amount_of_stones(num / 1000000000, blinks_left - 1, cach)
                + amount_of_stones(num % 1000000000, blinks_left - 1, cach)
        }
        1000000000000000000..=9999999999999999999 => {
            amount_of_stones(num * 2024, blinks_left - 1, cach)
        }
        10000000000000000000..=u64::MAX => {
            amount_of_stones(num / 10000000000, blinks_left - 1, cach)
                + amount_of_stones(num % 10000000000, blinks_left - 1, cach)
        }
    };
    cach.insert((num, blinks_left), r);
    r
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
struct AlignSlice([u8; 8_000_000]);

unsafe impl Zeroable for AlignSlice {}
unsafe impl Pod for AlignSlice {}

fn main() {
    // Never rerun
    println!("cargo::rerun-if-changed=build.rs");

    let mut big_lut75 = Box::new([0u64; 1_000_000]);
    let mut big_lut25 = Box::new([0u64; 1_000_000]);

    let mut cach = HashMap::default();
    for i in 0..1_000_000u64 {
        big_lut25[i as usize] = amount_of_stones(i, 25, &mut cach);
    }
    let mut cach = HashMap::default();
    for i in 0..1_000_000u64 {
        big_lut75[i as usize] = amount_of_stones(i, 75, &mut cach);
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let out_file75 = out_dir.join("big_lut75.bin");
    let out_file25 = out_dir.join("big_lut25.bin");

    let mut file75 = File::create(out_file75).unwrap();
    let mut file25 = File::create(out_file25).unwrap();

    let big_lut75: Box<AlignSlice> = cast_box(big_lut75);
    let big_lut25: Box<AlignSlice> = cast_box(big_lut25);

    file75.write_all(big_lut75.0.as_slice()).unwrap();
    file25.write_all(big_lut25.0.as_slice()).unwrap();
}
