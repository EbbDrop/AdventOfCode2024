use std::hint::assert_unchecked;

use aoc_runner_derive::aoc;

#[cfg(test)]
const WIDTH: u16 = 11;
#[cfg(test)]
const HIGHT: u16 = 7;

#[cfg(not(test))]
const WIDTH: u16 = 101;
#[cfg(not(test))]
const HIGHT: u16 = 103;

static LUT: [u32; (WIDTH as usize) << 7 | HIGHT as usize] = const {
    let mut lut = [0; (WIDTH as usize) << 7 | HIGHT as usize];

    let mut x = 0;
    while x < WIDTH as u64 {
        let mut y = 0;
        while y < HIGHT as u64 {
            // From wlfram alpha
            let ticks = (x + (52 * x + 51 * y) * 101) % (101 * 103);
            lut[(x << 7 | y) as usize] = ticks as u32;

            y += 1;
        }
        x += 1;
    }

    lut
};

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
            x += (s[i] - b'0') as i16;
            i += 1;
        }
        i += 1;

        let mut y = 0;
        while s[i] != b' ' {
            y *= 10;
            y += (s[i] - b'0') as i16;
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
            vx += (s[i] - b'0') as i16;
            i += 1;
        }
        if neg {
            vx = WIDTH as i16 - vx;
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
            vy += (s[i] - b'0') as i16;
            i += 1;
        }
        if neg {
            vy = HIGHT as i16 - vy;
        }
        i += 3;

        unsafe {
            assert_unchecked((x + vx * 100) >= 0);
            assert_unchecked((y + vy * 100) >= 0);
        }

        let x = (x + vx * 100) as u16 % WIDTH;
        let y = (y + vy * 100) as u16 % HIGHT;

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
pub fn part2(s: &str) -> u32 {
    unsafe { inner_part2(s.as_bytes()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner_part2(s: &[u8]) -> u32 {
    let mut a = [(0, 0, 0, 0); 500];

    let mut i = 2;
    for k in 0..500 {
        let mut x = 0;
        while *s.get_unchecked(i) != b',' {
            x *= 10;
            x += (*s.get_unchecked(i) - b'0') as u8;
            i += 1;
        }
        i += 1;

        let mut y = 0;
        while *s.get_unchecked(i) != b' ' {
            y *= 10;
            y += (*s.get_unchecked(i) - b'0') as u8;
            i += 1;
        }
        i += 3;

        let neg = if *s.get_unchecked(i) == b'-' {
            i += 1;
            true
        } else {
            false
        };
        i += 1;
        let mut vx = (*s.get_unchecked(i - 1) - b'0') as i16;
        while *s.get_unchecked(i) != b',' {
            vx *= 10;
            vx += (*s.get_unchecked(i) - b'0') as i16;
            i += 1;
        }
        if neg {
            vx = WIDTH as i16 - vx;
        }
        i += 1;

        let neg = if *s.get_unchecked(i) == b'-' {
            i += 1;
            true
        } else {
            false
        };
        i += 1;
        let mut vy = (*s.get_unchecked(i - 1) - b'0') as i16;
        while *s.get_unchecked(i) != b'\n' {
            vy *= 10;
            vy += (*s.get_unchecked(i) - b'0') as i16;
            i += 1;
        }
        if neg {
            vy = HIGHT as i16 - vy;
        }
        i += 3;

        a[k] = (x as u8, y as u8, vx as u8, vy as u8);
    }

    unsafe {
        let mut xs = [0u8; 103];
        let mut ys = [0u8; 103];

        let mut s = 0;
        let (x, y) = 'outer: loop {
            for (x, y, vx, vy) in a {
                let x = ((x as u16).unchecked_add((vx as u16).unchecked_mul(s))) % WIDTH;
                let y = ((y as u16).unchecked_add((vy as u16).unchecked_mul(s))) % HIGHT;

                *xs.get_unchecked_mut(x as usize) += 1;
                if *xs.get_unchecked_mut(x as usize) >= 20 {
                    if *ys.get_unchecked_mut(y as usize) >= 20 {
                        break 'outer (s, s);
                    }

                    let x = s;

                    loop {
                        ys.fill(0);
                        s += 1;
                        for (_, y, _, vy) in a {
                            let y =
                                ((y as u16).unchecked_add((vy as u16).unchecked_mul(s))) % HIGHT;

                            *ys.get_unchecked_mut(y as usize) += 1;
                            if *ys.get_unchecked_mut(y as usize) >= 20 {
                                break 'outer (x, s);
                            }
                        }
                    }
                }
                *ys.get_unchecked_mut(y as usize) += 1;
                if *ys.get_unchecked_mut(y as usize) >= 20 {
                    if *xs.get_unchecked_mut(x as usize) >= 20 {
                        break 'outer (s, s);
                    }
                    let y = s;

                    loop {
                        xs.fill(0);
                        s += 1;
                        for (x, _, vx, _) in a {
                            let x =
                                ((x as u16).unchecked_add((vx as u16).unchecked_mul(s))) % WIDTH;

                            *xs.get_unchecked_mut(x as usize) += 1;
                            if *xs.get_unchecked_mut(x as usize) >= 20 {
                                break 'outer (s, y);
                            }
                        }
                    }
                }
            }

            s += 1;
            xs.fill(0);
            ys.fill(0);
        };

        *LUT.get_unchecked((x as usize) << 7 | y as usize)
    }
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
