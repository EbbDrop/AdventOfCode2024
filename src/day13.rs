use std::arch::asm;

use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
pub fn part1(s: &str) -> u64 {
    unsafe { inner_part1(s) }
}

#[aoc(day13, part2)]
pub fn part2(s: &str) -> u64 {
    unsafe { inner_part2(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_part1(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut sum = 0;
    asm!(
        "2:",                                //
        "movzx {ax:e}, byte ptr [{s} + 12]", // ax = s[i + 12]
        "add   {ax:e}, {ax:e}",              // ax *= 2
        "lea   {ax:e}, [{ax} + 4*{ax}]",     // ax *= 5
        "add   {ax:l}, byte ptr [{s} + 13]", // ax += s[i + 13]
        "add   {ax:l}, -16",                 // ax -= 16
        "movzx {ax}, {ax:l}",                // ax = ax & 0xFF
        "movzx {ay:e}, byte ptr [{s} + 18]", // ay = s[i + 18]
        "add   {ay:e}, {ay:e}",              // ay *= 2
        "lea   {ay:e}, [{ay} + 4*{ay}]",     // ay *= 5
        "add   {ay:l}, byte ptr [{s} + 19]", // ay += s[i + 19]
        "add   {ay:l}, -16",                 // ay -= 16
        "movzx {ay}, {ay:l}",                // ay = ay & 0xFF

        "movzx {bx:e}, byte ptr [{s} + 33]", // bx = s[i + 33]
        "add   {bx:e}, {bx:e}",              // bx *= 2
        "lea   {bx:e}, [{bx} + 4*{bx}]",     // bx *= 5
        "add   {bx:l}, byte ptr [{s} + 34]", // bx += s[i + 34]
        "add   {bx:l}, -16",                 // bx -= 16
        "movzx {bx}, {bx:l}",                // bx = bx & 0xFF
        "movzx {by:e}, byte ptr [{s} + 39]", // by = s[i + 39]
        "add   {by:e}, {by:e}",              // by *= 2
        "lea   {by:e}, [{by} + 4*{by}]",     // by *= 5
        "add   {by:l}, byte ptr [{s} + 40]", // by += s[i + 40]
        "add   {by:l}, -16",                 // by -= 16
        "movzx {by}, {by:l}",                // by = by & 0xFF

        "movzx rax, byte ptr [{s} + 51]",    // x = s[i + 51]
        "add   rax, rax",                    // x *= 2
        "lea   rax, [rax + 4*rax]",          // x *= 5
        "movzx {t}, byte ptr [{s} + 52]",    // t = s[i + 52]
        "add   rax, {t}",                    // x += t
        "add   rax, rax",                    // x *= 2
        "lea   rax, [rax + 4*rax]",          // x *= 5
        "movzx {t}, byte ptr [{s} + 53]",    // t = s[i + 53]
        "add   rax, {t}",                    // x += t
        "add   rax, -111 * '0'",             // x -= 111 * b'0'
        "movzx {t}, byte ptr [{s} + 54]",    // x = s[i + 54]
        "cmp   {t}, ','",                    //
        "je    4f",                          // jump t == b','
        "add   rax, rax",                    // x *= 2
        "lea   rax, [rax + 4*rax]",          // x *= 5
        "lea   rax, [rax + {t} -'0']",       // x += t - b'0'
        "inc   {s}",                         // s += 1
        "movzx {t}, byte ptr [{s} + 54]",    // x = s[i + 54]
        "cmp   {t}, ','",                    //
        "je   4f",                           // jump t == b','
        "add   rax, rax",                    // x *= 2
        "lea   rax, [rax + 4*rax]",          // x *= 5
        "lea   rax, [rax + {t} -'0']",       // x += t - b'0'
        "inc   {s}",                         // s += 1
        "4:",

        "movzx {y}, byte ptr [{s} + 58]",    // y = s[i + 58]
        "add   {y}, {y}",                    // y *= 2
        "lea   {y}, [{y} + 4*{y}]",          // y *= 5
        "movzx {t}, byte ptr [{s} + 59]",    // t = s[i + 59]
        "add   {y}, {t}",                    // y += t
        "add   {y}, {y}",                    // y *= 2
        "lea   {y}, [{y} + 4*{y}]",          // y *= 5
        "movzx {t}, byte ptr [{s} + 60]",    // t = s[i + 60]
        "add   {y}, {t}",                    // y += t
        "add   {y}, -111 * '0'",             // y -= 111 * b'0'
        "movzx {t}, byte ptr [{s} + 61]",    // y = s[i + 61]
        "cmp   {t}, '\\n'",                  //
        "je    5f",                          // jump t == b','
        "add   {y}, {y}",                    // y *= 2
        "lea   {y}, [{y} + 4*{y}]",          // y *= 5
        "lea   {y}, [{y} + {t} -'0']",       // y += t - b'0'
        "inc   {s}",                         // s += 1
        "movzx {t}, byte ptr [{s} + 61]",    // y = s[i + 61]
        "cmp   {t}, '\\n'",                  //
        "je   5f",                           // jump t == b','
        "add   {y}, {y}",                    // y *= 2
        "lea   {y}, [{y} + 4*{y}]",          // y *= 5
        "lea   {y}, [{y} + {t} -'0']",       // y += t - b'0'
        "inc   {s}",                         // s += 1
        "5:",

        "imul  rax, {by}",                   // x *= by
        "mov   {t}, {y}",                    // t = y
        "imul  {t}, {bx}",                   // t *= bx
        "sub   rax, {t}",                    // x -= t       x = x * by - y * bx
        "imul  {bx}, {ay}",                  // bx *= ay
        "imul  {ax}, {by}",                  // ax *= by
        "sub   {ax}, {bx}",                  // ax -= bx     ax = by * ax - bx * ay

        "cqo",                               //
        "idiv  {ax}",                        // rax = x / ax, rdx = x % ax
        "test  rdx, rdx",                    //
        "jne   6f",                          // jump rdx != 0

        "mov   {t}, rax",                    // t = rax
        "imul  {ay}, rax",                   // ay *= rax
        "sub   {y}, {ay}",                   // y -= ay      y = y - ay * a
        "mov   rax, {y}",                    //
        "cqo",                               //
        "idiv  {by}",                        // rax = y / by, rdx = y % by
        "test  rdx, rdx",                    //
        "jne   6f",                          // jump rdx != 0

        "lea   {t}, [{t} + 2*{t}]",          // t = 3 * t
        "add   {sum}, {t}",                  // sum += t
        "add   {sum}, rax",                  // sum += rax

        "6:",
        "add   {s}, 63",                     // s += 63
        "cmp   {end}, {s}",                  //
        "jae   2b",                          // jump s > end
        "3:",
        s = inout(reg) s.as_ptr() => _,
        end = in(reg) s.as_ptr().offset(s.len() as isize),
        out("rdx") _,
        out("rax") _,
        t = out(reg) _,
        ax = out(reg) _,
        ay = out(reg) _,
        bx = out(reg) _,
        by = out(reg) _,
        y = out(reg) _,
        sum = inout(reg) sum,
    );

    sum
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_part2(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut sum = 0;

    let mut i = 0;
    while i < s.len() {
        let ax = (s
            .get_unchecked(i + 12)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 13))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i64;
        let ay = (s
            .get_unchecked(i + 18)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 19))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i64;

        let bx = (s
            .get_unchecked(i + 33)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 34))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i64;
        let by = (s
            .get_unchecked(i + 39)
            .wrapping_mul(10)
            .wrapping_add(*s.get_unchecked(i + 40))
            .wrapping_sub(const { b'0'.wrapping_mul(11) })) as i64;
        i += 51;

        let mut x = 0;
        while *s.get_unchecked(i) != b',' {
            x *= 10;
            x += (*s.get_unchecked(i) - b'0') as i64;
            i += 1;
        }
        x += 10000000000000;
        i += 4;

        let mut y = 0;
        while *s.get_unchecked(i) != b'\n' {
            y *= 10;
            y += (*s.get_unchecked(i) - b'0') as i64;
            i += 1;
        }
        y += 10000000000000;
        i += 2;

        let numerator = x * by - y * bx;
        let denominator = ax * by - ay * bx;
        std::hint::assert_unchecked(denominator != 0);
        std::hint::assert_unchecked(numerator != i64::MIN);
        std::hint::assert_unchecked(by != 0);
        std::hint::assert_unchecked(by != -1);

        let a = numerator / denominator;
        if numerator % denominator == 0 && (y - a * ay) % by == 0 {
            let b = (y - a * ay) / by;

            sum += (a * 3 + b) as u64;
        }
    }

    sum
}

// #[aoc(day13, part2)]
// pub fn part2(s: &str) -> u64 {
//     #[expect(unused_unsafe)]
//     unsafe {
//         part2_inner(s)
//     }
// }

// // #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
// fn part2_inner(s: &str) -> u64 {
//     let s = s.as_bytes();
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 480);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 875318608908);
    }
}
