use aoc_runner_derive::aoc;

#[cfg(test)]
const WIDTH: i32 = 11;
#[cfg(test)]
const HIGHT: i32 = 7;

#[cfg(not(test))]
const WIDTH: i32 = 101;
#[cfg(not(test))]
const HIGHT: i32 = 103;

#[aoc(day14, part1)]
pub fn part1(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part1(s.as_bytes())
    }
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part1(s: &[u8]) -> u64 {
    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    let mut i = 2;
    while i < s.len() {
        let mut x = 0;
        while s[i] != b',' {
            x *= 10;
            x += (s[i] - b'0') as i32;
            i += 1;
        }
        i += 1;

        let mut y = 0;
        while s[i] != b' ' {
            y *= 10;
            y += (s[i] - b'0') as i32;
            i += 1;
        }
        i += 3;

        let mut vx = 0;
        let neg = if s[i] == b'-' {
            i += 1;
            true
        } else {
            false
        };
        while s[i] != b',' {
            vx *= 10;
            vx += (s[i] - b'0') as i32;
            i += 1;
        }
        if neg {
            vx *= -1;
        }
        i += 1;

        let mut vy = 0;
        let neg = if s[i] == b'-' {
            i += 1;
            true
        } else {
            false
        };
        while s[i] != b'\n' {
            vy *= 10;
            vy += (s[i] - b'0') as i32;
            i += 1;
        }
        if neg {
            vy *= -1;
        }
        i += 3;

        let x = (x + vx * 100).rem_euclid(WIDTH);
        let y = (y + vy * 100).rem_euclid(HIGHT);

        if x < WIDTH / 2 {
            if y < HIGHT / 2 {
                tl += 1;
            } else if y > HIGHT / 2 {
                bl += 1;
            }
        } else if x > WIDTH / 2 {
            if y < HIGHT / 2 {
                tr += 1;
            } else if y > HIGHT / 2 {
                br += 1;
            }
        }
    }

    tl * bl * tr * br
}

#[aoc(day14, part2)]
pub fn part2(s: &str) -> u64 {
    #[expect(unused_unsafe)]
    unsafe {
        inner_part2(s.as_bytes())
    }
}

// #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
fn inner_part2(s: &[u8]) -> u64 {
    let mut a = [(0, 0, 0, 0); 500];

    let mut i = 2;
    for k in 0..500 {
        let mut x = 0;
        while s[i] != b',' {
            x *= 10;
            x += (s[i] - b'0') as i32;
            i += 1;
        }
        i += 1;

        let mut y = 0;
        while s[i] != b' ' {
            y *= 10;
            y += (s[i] - b'0') as i32;
            i += 1;
        }
        i += 3;

        let mut vx = 0;
        let neg = if s[i] == b'-' {
            i += 1;
            true
        } else {
            false
        };
        while s[i] != b',' {
            vx *= 10;
            vx += (s[i] - b'0') as i32;
            i += 1;
        }
        if neg {
            vx *= -1;
        }
        i += 1;

        let mut vy = 0;
        let neg = if s[i] == b'-' {
            i += 1;
            true
        } else {
            false
        };
        while s[i] != b'\n' {
            vy *= 10;
            vy += (s[i] - b'0') as i32;
            i += 1;
        }
        if neg {
            vy *= -1;
        }
        i += 3;

        a[k] = (x, y, vx, vy);
    }

    let mut f = [0i32; WIDTH as usize * HIGHT as usize];
    'sycles: for s in 5000.. {
        for (x, y, vx, vy) in a {
            let x = (x + vx * s).rem_euclid(WIDTH);
            let y = (y + vy * s).rem_euclid(HIGHT);

            if f[(y * WIDTH + x) as usize] == s {
                continue 'sycles;
            }
            f[(y * WIDTH + x) as usize] = s;
        }
        return (s % 10403) as u64;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 12);
    }

    // #[test]
    // fn example_part2() {
    //     assert_eq!(part2(EXAMPLE), 875318608908);
    // }
}
