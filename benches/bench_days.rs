use criterion::{criterion_group, criterion_main, Criterion};
use paste::paste;
use std::{fmt::Display, hint::black_box};

/// Get input for a single day
macro_rules! get_day_input {
    ($day_num:literal) => {
        include_str!(concat!("../input/2024/day", $day_num, ".txt"))
    };
}

/// Define benchmarks for a single day with part1 and part2
macro_rules! benches_day {
    ($day_num:literal) => {
        paste! {
            use advent_of_code_2024::[<day $day_num>]; // Replace `aoc24` with your crate name

            pub fn [<bench_day $day_num>](c: &mut Criterion) {
                let mut group = c.benchmark_group(concat!("day", $day_num));
                let input = get_day_input!($day_num);

                #[inline(never)]
                fn routine_part1(input: &str) -> impl Display + '_ {
                    [<day $day_num>]::part1(black_box(input))
                }
                group.bench_with_input(format!("day{}_part1", $day_num), input, |b, i| b.iter(|| routine_part1(i)));
                // #[inline(never)]
                // fn routine_part2(input: &str) -> impl Display + '_ {
                //     [<day $day_num>]::part2(black_box(input))
                // }
                // group.bench_with_input(format!("day{}_part2", $day_num), input, |b, i| b.iter(|| routine_part2(i)));
            }
        }
    };
}

/// Create benchmarks for included days
macro_rules! benches {
    ($($day_num:literal),*) => {
        paste! {
            $(
                benches_day!($day_num);
            )*

            criterion_group!(benches, $([<bench_day $day_num>]),*);
            criterion_main!(benches);
        }
    };
}

benches!(25);
