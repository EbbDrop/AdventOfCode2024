use core::str;

use aoc_runner_derive::aoc;
use bitvec::array::BitArray;

const MAX: usize = 26 * 26;

const T_START: u16 = (b't' - b'a') as u16 * 26;

const T_START_REM: u16 = (-(T_START as i16)).rem_euclid(MAX as i16) as u16;

const MAX_C: usize = 13;

#[aoc(day23, part1)]
pub fn part1(s: &str) -> u64 {
    let s = s.as_bytes();

    let mut connections = [const { heapless::Vec::<u16, MAX_C>::new() }; MAX];
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

        // println!(
        //     "{}{}:",
        //     ((c / 26) as u8 + b'a') as char,
        //     ((c % 26) as u8 + b'a') as char,
        // );

        let mut sum = 0;
        for c in 0..26 {
            for con in &connections[c as usize] {
                if *con < c {
                    continue;
                }
                for concon in &connections[*con as usize] {
                    if *concon < *con {
                        continue;
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

static mut SCRATCH: [u8; MAX_C * 3] = [0; MAX_C * 3];

const BAL: usize = (MAX * MAX).div_ceil(64);

#[aoc(day23, part2)]
pub fn part2(s: &str) -> &'static str {
    let s = s.as_bytes();

    let mut g = BitArray::<[u64; BAL]>::default();

    let mut vertecies = const {
        let mut vs = [(0u16, 0); MAX];
        let mut i = 0;
        while i < MAX {
            vs[i].0 = i as u16;
            i += 1;
        }
        vs
    };
    let mut connections = [const { heapless::Vec::<u16, MAX_C>::new() }; MAX];

    unsafe {
        let mut i = 0;
        while i < s.len() {
            let cp1 =
                (s.get_unchecked(i) - b'a') as u16 * 26 + (s.get_unchecked(i + 1) - b'a') as u16;
            let cp2 = (s.get_unchecked(i + 3) - b'a') as u16 * 26
                + (s.get_unchecked(i + 4) - b'a') as u16;

            connections
                .get_unchecked_mut(cp1 as usize)
                .push_unchecked(cp2);
            connections
                .get_unchecked_mut(cp2 as usize)
                .push_unchecked(cp1);
            g.set(cp2 as usize * MAX + cp1 as usize, true);
            g.set(cp1 as usize * MAX + cp2 as usize, true);
            vertecies.get_unchecked_mut(cp1 as usize).1 += 1;
            vertecies.get_unchecked_mut(cp2 as usize).1 += 1;

            i += 6;
        }

        vertecies.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        let mut i = vertecies.len() - 1;
        while vertecies.get_unchecked(i).1 == 0 {
            i -= 1;
        }
        let vertecies = vertecies.get_unchecked_mut(..i + 1);

        let max_degree = vertecies.get_unchecked(0).1 as usize;

        for i in 0..max_degree {
            vertecies.get_unchecked_mut(i).1 = i as u16;
        }
        for i in max_degree..vertecies.len() {
            vertecies.get_unchecked_mut(i).1 = max_degree as u16;
        }

        let mut cs = [const { heapless::Vec::<u16, MAX_C>::new() }; MAX_C];

        let mut q = heapless::Vec::<u16, MAX_C>::new();
        let mut q_max = heapless::Vec::<u16, MAX_C>::new();

        expand_first(vertecies, &g, &connections, &mut q, &mut q_max, &mut cs);

        q_max.sort_unstable();

        let mut i = 0;
        for p in q_max {
            std::hint::assert_unchecked(i + 2 < 13 * 3);
            SCRATCH[i + 0] = (p / 26) as u8 + b'a';
            SCRATCH[i + 1] = (p % 26) as u8 + b'a';
            SCRATCH[i + 2] = b',';
            i += 3;
        }

        std::hint::assert_unchecked(i - 1 < 13 * 3);
        str::from_utf8_unchecked(&SCRATCH[..i - 1])
    }
}

// Using a modified version of this algorithm: https://web.archive.org/web/20160911054636/http://www.dcs.gla.ac.uk/~pat/jchoco/clique/indSetMachrahanish/papers/tomita2003.pdf
unsafe fn expand_first(
    mut r: &mut [(u16, u16)],
    g: &BitArray<[u64; BAL]>,
    cons: &[heapless::Vec<u16, MAX_C>; MAX],
    q: &mut heapless::Vec<u16, MAX_C>,
    q_max: &mut heapless::Vec<u16, MAX_C>,
    cs: &mut [heapless::Vec<u16, MAX_C>; MAX_C],
) {
    let mut r_map = [true; MAX];
    while let Some(((p, color), rest)) = r.split_last_mut() {
        let p = *p as usize;
        if q.len() + *color as usize + 1 > q_max.len() {
            q.push_unchecked(p as u16);

            let mut new_r = heapless::Vec::<(u16, u16), MAX_C>::new();
            for i in cons[p].iter() {
                if unsafe { *r_map.get_unchecked(*i as usize) } {
                    new_r.push_unchecked((*i, 0));
                }
            }

            if !new_r.is_empty() {
                unsafe { number_sort(new_r.as_mut_slice(), g, cs) };
                expand(&mut new_r, g, q, q_max, cs);
            } else if q.len() > q_max.len() {
                q_max.clone_from(q);
            }
            q.pop();
        } else {
            return;
        }
        *unsafe { r_map.get_unchecked_mut(p) } = false;
        r = rest;
    }
}

// Using this algorithm: https://web.archive.org/web/20160911054636/http://www.dcs.gla.ac.uk/~pat/jchoco/clique/indSetMachrahanish/papers/tomita2003.pdf
unsafe fn expand(
    mut r: &mut [(u16, u16)],
    g: &BitArray<[u64; BAL]>,
    q: &mut heapless::Vec<u16, MAX_C>,
    q_max: &mut heapless::Vec<u16, MAX_C>,
    cs: &mut [heapless::Vec<u16, MAX_C>; MAX_C],
) {
    while let Some(((p, color), rest)) = r.split_last_mut() {
        let p = *p as usize;
        if q.len() + *color as usize + 1 > q_max.len() {
            q.push_unchecked(p as u16);

            let mut new_r = heapless::Vec::<(u16, u16), MAX_C>::new();
            for (i, _) in rest.iter() {
                if unsafe { *g.get_unchecked(*i as usize * MAX + p) } {
                    new_r.push_unchecked((*i, 0));
                }
            }

            if !new_r.is_empty() {
                unsafe { number_sort(new_r.as_mut_slice(), g, cs) };
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

#[inline(always)]
unsafe fn number_sort(
    r: &mut [(u16, u16)],
    g: &BitArray<[u64; BAL]>,
    cs: &mut [heapless::Vec<u16, MAX_C>; MAX_C],
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
                for i in cs.get_unchecked(k) {
                    if *g.get_unchecked(*i as usize * MAX + p) {
                        k += 1;
                        continue 'outer;
                    }
                }
                break;
            }
            if k > maxno {
                maxno = k;
                cs.get_unchecked_mut(maxno + 1).clear();
            }
            cs.get_unchecked_mut(k).push_unchecked(p as u16);

            r = rest;
        }
    }

    let mut i = 0;
    for k in 0..=maxno {
        for j in cs.get_unchecked(k) {
            *r.get_unchecked_mut(i) = (*j, k as u16);
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
