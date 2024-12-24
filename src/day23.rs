use core::str;

use aoc_runner_derive::aoc;

const MAX: usize = 26 * 26;

const T_START: u16 = (b't' - b'a') as u16 * 26;

const T_START_REM: u16 = (-(T_START as i16)).rem_euclid(MAX as i16) as u16;

#[aoc(day23, part1)]
pub fn part1(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut connections = [const { heapless::Vec::<u16, 16>::new() }; MAX];
    unsafe {
        let mut i = 0;
        while i < s.len() {
            let cp1 =
                (s.get_unchecked(i) - b'a') as u16 * 26 + (s.get_unchecked(i + 1) - b'a') as u16;
            let cp2 = (s.get_unchecked(i + 3) - b'a') as u16 * 26
                + (s.get_unchecked(i + 4) - b'a') as u16;
            let cp1 = (cp1 + T_START_REM) % MAX as u16;
            let cp2 = (cp2 + T_START_REM) % MAX as u16;

            connections
                .get_unchecked_mut(cp1 as usize)
                .push_unchecked(cp2);
            connections
                .get_unchecked_mut(cp2 as usize)
                .push_unchecked(cp1);

            i += 6;
        }

        for c in connections.iter_mut() {
            c.sort_unstable_by(|a, b| b.cmp(a));
        }
        // println!(
        //     "{}{}:",
        //     ((c / 26) as u8 + b'a') as char,
        //     ((c % 26) as u8 + b'a') as char,
        // );

        let mut sum = 0;
        for c in 0..26 {
            for con in &connections[c as usize] {
                if *con < c {
                    break;
                }
                for concon in &connections[*con as usize] {
                    if *concon < *con {
                        break;
                    }
                    for conconcon in &connections[*concon as usize] {
                        if *conconcon == c {
                            sum += 1;
                            break;
                        }
                    }
                }
            }
        }

        sum
    }
}

static mut SCRATCH: [u8; 16 * 3] = [0; 16 * 3];

#[aoc(day23, part2)]
pub fn part2(s: &str) -> &'static str {
    let s = s.as_bytes();

    let mut g = [[false; MAX]; MAX];

    let mut vertecies = const {
        let mut vs = [(0u16, 0); MAX];
        let mut i = 0;
        while i < MAX {
            vs[i].0 = i as u16;
            i += 1;
        }
        vs
    };

    unsafe {
        let mut i = 0;
        while i < s.len() {
            let cp1 =
                (s.get_unchecked(i) - b'a') as u16 * 26 + (s.get_unchecked(i + 1) - b'a') as u16;
            let cp2 = (s.get_unchecked(i + 3) - b'a') as u16 * 26
                + (s.get_unchecked(i + 4) - b'a') as u16;

            *g.get_unchecked_mut(cp2 as usize)
                .get_unchecked_mut(cp1 as usize) = true;
            *g.get_unchecked_mut(cp1 as usize)
                .get_unchecked_mut(cp2 as usize) = true;
            vertecies.get_unchecked_mut(cp1 as usize).1 += 1;
            vertecies.get_unchecked_mut(cp2 as usize).1 += 1;

            i += 6;
        }
    }

    vertecies.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    let mut i = vertecies.len() - 1;
    while vertecies[i].1 == 0 {
        i -= 1;
    }
    let vertecies = &mut vertecies[..i + 1];

    let max_degree = vertecies[0].1 as usize;

    for i in 0..max_degree {
        vertecies[i].1 = i as u16;
    }
    for i in max_degree..vertecies.len() {
        vertecies[i].1 = max_degree as u16;
    }

    let mut cs = [const { heapless::Vec::<u16, 16>::new() }; 16];

    let mut q = heapless::Vec::<u16, 16>::new();
    let mut q_max = heapless::Vec::<u16, 16>::new();

    expand(vertecies, &g, &mut q, &mut q_max, &mut cs);

    q_max.sort_unstable();

    unsafe {
        let mut i = 0;
        for p in q_max {
            SCRATCH[i + 0] = (p / 26) as u8 + b'a';
            SCRATCH[i + 1] = (p % 26) as u8 + b'a';
            SCRATCH[i + 2] = b',';
            i += 3;
        }

        str::from_utf8_unchecked(&SCRATCH[..i - 1])
    }
}

// Using this algorithm: https://web.archive.org/web/20160911054636/http://www.dcs.gla.ac.uk/~pat/jchoco/clique/indSetMachrahanish/papers/tomita2003.pdf
fn expand(
    mut r: &mut [(u16, u16)],
    g: &[[bool; MAX]; MAX],
    q: &mut heapless::Vec<u16, 16>,
    q_max: &mut heapless::Vec<u16, 16>,
    cs: &mut [heapless::Vec<u16, 16>; 16],
) {
    while let Some(((p, color), rest)) = r.split_last_mut() {
        let p = *p as usize;
        if q.len() + *color as usize + 1 > q_max.len() {
            q.push(p as u16).unwrap();

            let mut new_r = heapless::Vec::<(u16, u16), 16>::new();
            for (i, _) in rest.iter() {
                if g[p][*i as usize] {
                    new_r.push((*i, 0)).unwrap();
                }
            }

            if !new_r.is_empty() {
                number_sort(new_r.as_mut_slice(), g, cs);
                expand(&mut new_r, g, q, q_max, cs);
            } else if q.len() > q_max.len() {
                q_max.clone_from(q);
            }
            q.pop();
        } else {
            return;
        }
        r = rest;
    }
}

fn number_sort(
    r: &mut [(u16, u16)],
    g: &[[bool; MAX]; MAX],
    cs: &mut [heapless::Vec<u16, 16>; 16],
) {
    let mut maxno = 0;
    cs[0].clear();
    cs[1].clear();

    {
        let mut r = &*r;
        while let Some(((p, _), rest)) = r.split_first() {
            let p = *p as usize;
            let mut k = 0;

            'outer: loop {
                for i in &cs[k] {
                    if g[p][*i as usize] {
                        k += 1;
                        continue 'outer;
                    }
                }
                break;
            }
            if k > maxno {
                maxno = k;
                cs[maxno + 1].clear();
            }
            cs[k].push(p as u16).unwrap();

            r = rest;
        }
    }

    let mut i = 0;
    for k in 0..=maxno {
        for j in &cs[k] {
            r[i] = (*j, k as u16);
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 7);
    }
    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), "co,de,ka,ta");
    }
}
