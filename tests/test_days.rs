use paste::paste;

/// Get input for a single day
macro_rules! get_day_input {
    ($day_num:literal) => {
        include_str!(concat!("../input/2024/day", $day_num, ".txt"))
    };
}

/// Create benchmarks for included days
macro_rules! benches {
    ($($day_num:literal => ($p1_sol:literal, $p2_sol:literal),)*) => {
        paste! {
            $(
                use advent_of_code_2024::[<day $day_num>]; // Replace `aoc24` with your crate name

                #[test]
                pub fn [<test_day $day_num _part1>]() {
                    let input = get_day_input!($day_num);

                    assert_eq!([<day $day_num>]::part1(input).to_string(), $p1_sol.to_string());
                }

                #[test]
                pub fn [<test_day $day_num _part2>]() {
                    let input = get_day_input!($day_num);

                    assert_eq!([<day $day_num>]::part2(input).to_string(), $p2_sol.to_string());
                }
            )*
        }
    };
}

benches!(
    1 => ("2000468", "18567089"),
    2 => ("220", "296"),
    3 => ("165225049", "108830766"),
    4 => ("2613", "1905"),
    5 => ("5713", "5180"),
    6 => ("4988", "1697"),
    7 => ("538191549061", "34612812972206"),
    8 => ("394", "1277"),
    9 => ("6283170117911", "6307653242596"),
    10 => ("607", "1384"),
);
