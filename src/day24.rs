use std::hint::unreachable_unchecked;

use aoc_runner_derive::aoc;

#[aoc(day24, part1)]
pub fn part1(s: &str) -> u64 {
    let s = s.as_bytes();
    part1_inner(s)

    // println!("digraph G {{");

    // let mut lines = s.lines();
    // for l in &mut lines {
    //     if l.is_empty() {
    //         break;
    //     }
    //     println!("  {0} [label=\"{0}: {1}\"]", &l[..3], &l[5..6]);
    // }

    // // let mut i = 0;
    // for l in &mut lines {
    //     let a = &l[..3];
    //     let (op, len) = match &l[4..5] {
    //         "O" => ("OR", 2),
    //         "A" => ("AND", 3),
    //         "X" => ("XOR", 3),
    //         _ => unreachable!(),
    //     };
    //     let b = &l[len + 5..len + 8];
    //     let to = &l[len + 12..len + 15];
    //     println!("  {} [label=\"{}\"]", to, op);
    //     println!("  {} -> {}", a, to);
    //     println!("  {} -> {}", b, to);
    // }

    // println!("}}");
    // 2024
}

#[cfg(test)]
const INPUTS: usize = 5;
#[cfg(not(test))]
const INPUTS: usize = 45;

const YSTART: usize = INPUTS * 7;
const CSTART: usize = INPUTS * 2 * 7 + 1;

#[derive(Debug, Clone, Copy)]
enum State {
    Empty = 0,
    And,
    Or,
    Xor,
    EmptyInp1Set,
    AndInp1Set,
    OrInp1Set,
    XorInp1Set,
}

#[derive(Debug, Clone, Copy)]
struct Gate {
    inp_1: bool,
    out_1: u16,
    out_2: u16,
    state: State,
}

impl Gate {
    /// Returns `false` if input already set.
    fn set_inp(&mut self, b: bool) -> bool {
        match self.state {
            State::Empty => {
                self.inp_1 = b;
                self.state = State::EmptyInp1Set;
            }
            State::And => {
                self.inp_1 = b;
                self.state = State::AndInp1Set;
            }
            State::Or => {
                self.inp_1 = b;
                self.state = State::OrInp1Set;
            }
            State::Xor => {
                self.inp_1 = b;
                self.state = State::XorInp1Set;
            }
            _ => return false,
        }
        true
    }

    fn add_out(&mut self, out: u16) {
        if self.out_1 == 0 {
            self.out_1 = out;
        } else {
            debug_assert!(self.out_2 == 0);
            self.out_2 = out;
        }
    }
}

#[inline(always)]
pub fn part1_inner(s: &[u8]) -> u64 {
    let mut gates_map = heapless::FnvIndexMap::<u16, u16, 512>::new();

    let mut gates = heapless::Vec::<Gate, 512>::from_slice(
        &[Gate {
            inp_1: false,
            out_1: 0,
            out_2: 0,
            state: State::Empty,
        }; 46],
    )
    .unwrap();

    let mut stack = heapless::Vec::<(u16, bool), 128>::new();

    let mut i = CSTART;
    unsafe {
        while i < s.len() {
            let (state, len) = match s.get_unchecked(i + 4) {
                b'X' => (State::Xor, 3),
                b'A' => (State::And, 3),
                b'O' => (State::Or, 2),
                _ => unreachable_unchecked(),
            };

            let this = if *s.get_unchecked(i + len + 12) == b'z' {
                (s.get_unchecked(i + len + 13) - b'0') as u16 * 10
                    + (s.get_unchecked(i + len + 14) - b'0') as u16
            } else {
                let this = (s.get_unchecked(i + len + 12) - b'a') as u16 * 26 * 26
                    + (s.get_unchecked(i + len + 13) - b'a') as u16 * 26
                    + (s.get_unchecked(i + len + 14) - b'a') as u16;

                match gates_map.entry(this) {
                    heapless::Entry::Occupied(occupied_entry) => *occupied_entry.get(),
                    heapless::Entry::Vacant(vacant_entry) => {
                        let i = gates.len() as u16;
                        gates.push_unchecked(Gate {
                            inp_1: false,
                            out_1: 0,
                            out_2: 0,
                            state: State::Empty,
                        });
                        vacant_entry.insert(i).unwrap_unchecked();
                        i
                    }
                }
            };

            if *s.get_unchecked(i) == b'x' {
                let from1 = (s.get_unchecked(i + 1) - b'0') as usize * 10
                    + (s.get_unchecked(i + 2) - b'0') as usize;
                let from2 = (s.get_unchecked(i + len + 6) - b'0') as usize * 10
                    + (s.get_unchecked(i + len + 7) - b'0') as usize;

                let x = *s.get_unchecked(from1 * 7 + 5) == b'1';
                let y = *s.get_unchecked(YSTART + from2 * 7 + 5) == b'1';

                gates.get_unchecked_mut(this as usize).state = state;
                debug_assert!(gates.get_unchecked_mut(this as usize).set_inp(x));
                stack.push_unchecked((this, y));
            } else if *s.get_unchecked(i) == b'y' {
                let from1 = (s.get_unchecked(i + 1) - b'0') as usize * 10
                    + (s.get_unchecked(i + 2) - b'0') as usize;
                let from2 = (s.get_unchecked(i + len + 6) - b'0') as usize * 10
                    + (s.get_unchecked(i + len + 7) - b'0') as usize;

                let x = *s.get_unchecked(from2 * 7 + 5) == b'1';
                let y = *s.get_unchecked(YSTART + from1 * 7 + 5) == b'1';

                gates.get_unchecked_mut(this as usize).state = state;
                debug_assert!(gates.get_unchecked_mut(this as usize).set_inp(x));
                stack.push_unchecked((this, y));
            } else {
                let from1 = (s.get_unchecked(i) - b'a') as u16 * 26 * 26
                    + (s.get_unchecked(i + 1) - b'a') as u16 * 26
                    + (s.get_unchecked(i + 2) - b'a') as u16;

                let from2 = (s.get_unchecked(i + len + 5) - b'a') as u16 * 26 * 26
                    + (s.get_unchecked(i + len + 6) - b'a') as u16 * 26
                    + (s.get_unchecked(i + len + 7) - b'a') as u16;
                let from1 = match gates_map.entry(from1) {
                    heapless::Entry::Occupied(occupied_entry) => *occupied_entry.get(),
                    heapless::Entry::Vacant(vacant_entry) => {
                        let i = gates.len() as u16;
                        gates.push_unchecked(Gate {
                            inp_1: false,
                            out_1: 0,
                            out_2: 0,
                            state: State::Empty,
                        });
                        vacant_entry.insert(i).unwrap_unchecked();
                        i
                    }
                };
                let from2 = match gates_map.entry(from2) {
                    heapless::Entry::Occupied(occupied_entry) => *occupied_entry.get(),
                    heapless::Entry::Vacant(vacant_entry) => {
                        let i = gates.len() as u16;
                        gates.push_unchecked(Gate {
                            inp_1: false,
                            out_1: 0,
                            out_2: 0,
                            state: State::Empty,
                        });
                        vacant_entry.insert(i).unwrap_unchecked();
                        i
                    }
                };

                gates.get_unchecked_mut(from1 as usize).add_out(this);
                gates.get_unchecked_mut(from2 as usize).add_out(this);
                gates.get_unchecked_mut(this as usize).state = state;
            }

            i += len + 16;
        }

        let mut zs = 0;
        while let Some((g, inp2)) = stack.pop() {
            let gate = gates.get_unchecked(g as usize);
            let out = match gate.state {
                State::AndInp1Set => gate.inp_1 & inp2,
                State::OrInp1Set => gate.inp_1 | inp2,
                State::XorInp1Set => gate.inp_1 ^ inp2,
                _ => unreachable_unchecked(),
            };

            if g < 46 {
                zs |= (out as u64) << g;
            } else {
                let out_1 = gate.out_1;
                let out_2 = gate.out_2;
                if !gates.get_unchecked_mut(out_1 as usize).set_inp(out) {
                    stack.push_unchecked((out_1, out));
                }
                if out_2 != 0 {
                    if !gates.get_unchecked_mut(out_2 as usize).set_inp(out) {
                        stack.push_unchecked((out_2, out));
                    }
                }
            }
        }
        zs
    }
}

#[allow(unused)]
fn tos(g: u16) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    if g >= ZSTART {
        write!(s, "z{:02}", g - ZSTART);
    } else {
        write!(
            s,
            "{}{}{}",
            ((g / 26 / 26) as u8 + b'a') as char,
            ((g / 26 % 26) as u8 + b'a') as char,
            ((g % 26) as u8 + b'a') as char,
        );
    }
    s
}

#[aoc(day24, part2)]
pub fn part2(s: &str) -> &'static str {
    let s = s.as_bytes();
    part2_inner(s)
}

const ZSTART: u16 = 26 * 26 * 26;

#[inline(always)]
pub fn part2_inner(s: &[u8]) -> &'static str {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum State {
        Or,
        And,
        Xor,
    }

    #[derive(Debug, Clone, Copy)]
    struct Gate {
        out_1: u16,
        out_2: u16,
        state: State,
    }

    impl Gate {
        fn add_out(&mut self, out: u16) {
            if self.out_1 == 0 {
                self.out_1 = out;
            } else {
                debug_assert_eq!(self.out_2, 0);
                self.out_2 = out;
            }
        }
    }

    let mut gates = [Gate {
        out_1: 0,
        out_2: 0,
        state: State::Or,
    }; 26 * 26 * 26 + 46];

    let mut inputs = [(0u16, 0u16); 45];

    let mut i = CSTART;
    unsafe {
        while i < s.len() {
            let (state, len) = match s.get_unchecked(i + 4) {
                b'X' => (State::Xor, 3),
                b'A' => (State::And, 3),
                b'O' => (State::Or, 2),
                _ => unreachable_unchecked(),
            };

            let this = if *s.get_unchecked(i + len + 12) == b'z' {
                (s.get_unchecked(i + len + 13) - b'0') as u16 * 10
                    + (s.get_unchecked(i + len + 14) - b'0') as u16
                    + 26 * 26 * 26
            } else {
                (s.get_unchecked(i + len + 12) - b'a') as u16 * 26 * 26
                    + (s.get_unchecked(i + len + 13) - b'a') as u16 * 26
                    + (s.get_unchecked(i + len + 14) - b'a') as u16
            };

            if *s.get_unchecked(i) == b'x' || *s.get_unchecked(i) == b'y' {
                let from1 = (s.get_unchecked(i + 1) - b'0') as usize * 10
                    + (s.get_unchecked(i + 2) - b'0') as usize;
                let from2 = (s.get_unchecked(i + len + 6) - b'0') as usize * 10
                    + (s.get_unchecked(i + len + 7) - b'0') as usize;
                debug_assert_eq!(from1, from2);

                match state {
                    State::Or => unreachable_unchecked(),
                    State::And => {
                        debug_assert_eq!(inputs[from1].0, 0);
                        inputs.get_unchecked_mut(from1).0 = this;
                    }
                    State::Xor => {
                        debug_assert_eq!(inputs[from1].1, 0);
                        inputs.get_unchecked_mut(from1).1 = this;
                    }
                }
            } else {
                let from1 = (s.get_unchecked(i) - b'a') as u16 * 26 * 26
                    + (s.get_unchecked(i + 1) - b'a') as u16 * 26
                    + (s.get_unchecked(i + 2) - b'a') as u16;

                let from2 = (s.get_unchecked(i + len + 5) - b'a') as u16 * 26 * 26
                    + (s.get_unchecked(i + len + 6) - b'a') as u16 * 26
                    + (s.get_unchecked(i + len + 7) - b'a') as u16;

                gates.get_unchecked_mut(from1 as usize).add_out(this);
                gates.get_unchecked_mut(from2 as usize).add_out(this);
                gates.get_unchecked_mut(this as usize).state = state;
            }
            gates.get_unchecked_mut(this as usize).state = state;

            i += len + 16;
        }

        let mut to_swap = heapless::Vec::<u16, 8>::new();

        let (and1, xor1) = inputs[0];
        let mut carry = if xor1 != ZSTART + 0 {
            // Asuming its a swap is with the carry
            debug_assert_eq!(and1, ZSTART);
            // println!("Swaping start: {} - {}", tos(and1), tos(ZSTART));
            to_swap.push_unchecked(xor1);
            to_swap.push_unchecked(and1);
            xor1
        } else {
            and1
        };

        for i in 1..45 {
            let (and1, xor1) = inputs[i as usize];
            debug_assert_ne!(gates[xor1 as usize].out_1, 0);
            let (and1, xor1) = if gates.get_unchecked(xor1 as usize).out_2 == 0 {
                to_swap.push_unchecked(and1);
                to_swap.push_unchecked(xor1);

                (xor1, and1)
            } else {
                (and1, xor1)
            };

            let next1 = gates.get_unchecked(xor1 as usize).out_1;
            let next2 = gates.get_unchecked(xor1 as usize).out_2;

            let or = if and1 == ZSTART + i {
                to_swap.push_unchecked(ZSTART + i);

                if gates.get_unchecked(next1 as usize).state == State::Xor {
                    to_swap.push_unchecked(next1);
                } else if gates.get_unchecked(next2 as usize).state == State::Xor {
                    to_swap.push_unchecked(next2);
                } else {
                    unreachable_unchecked()
                }

                // TODO: is it correct to asume this?
                let or = gates.get_unchecked(next1 as usize).out_1;
                or
            } else {
                debug_assert_eq!(gates[and1 as usize].out_2, 0);
                let or_from_and1 = gates.get_unchecked(and1 as usize).out_1;

                if or_from_and1 == ZSTART + i {
                    to_swap.push(ZSTART + i).unwrap();
                    if gates[next1 as usize].state == State::Xor {
                        to_swap.push_unchecked(next1);
                        next1
                    } else if gates[next2 as usize].state == State::Xor {
                        to_swap.push_unchecked(next2);
                        next2
                    } else {
                        unreachable_unchecked()
                    }
                } else {
                    if gates.get_unchecked((ZSTART + i) as usize).state != State::Xor {
                        to_swap.push_unchecked(next1);
                        to_swap.push_unchecked(next2);
                    }

                    or_from_and1
                }
            };

            carry = or;
        }

        debug_assert_eq!(carry, ZSTART + 45);

        to_swap.sort_unstable();
        debug_assert_eq!(to_swap.len(), 8);

        static mut OUTPUT: [u8; 8 * 4 - 1] = [b','; 8 * 4 - 1];
        let mut j = 0;
        while j < 8 {
            let w = *to_swap.get_unchecked(j);
            if w >= ZSTART {
                let w = w - ZSTART;
                OUTPUT[j * 4] = b'z';
                OUTPUT[j * 4 + 1] = (w / 10) as u8 + b'0';
                OUTPUT[j * 4 + 2] = (w % 10) as u8 + b'0';
            } else {
                OUTPUT[j * 4 + 0] = (w / 26 / 26) as u8 + b'a';
                OUTPUT[j * 4 + 1] = (w / 26 % 26) as u8 + b'a';
                OUTPUT[j * 4 + 2] = (w % 26) as u8 + b'a';
            }
            j += 1;
        }

        std::str::from_utf8_unchecked(&*(&raw const OUTPUT))
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    //     const EXAMPLE: &str = r"x00: 1
    // x01: 0
    // x02: 1
    // x03: 1
    // x04: 0
    // y00: 1
    // y01: 1
    // y02: 1
    // y03: 1
    // y04: 1

    // ntg XOR fgs -> mjb
    // y02 OR x01 -> tnw
    // kwq OR kpj -> z05
    // x00 OR x03 -> fst
    // tgd XOR rvg -> z01
    // vdt OR tnw -> bfw
    // bfw AND frj -> z10
    // ffh OR nrd -> bqk
    // y00 AND y03 -> djm
    // y03 OR y00 -> psh
    // bqk OR frj -> z08
    // tnw OR fst -> frj
    // gnj AND tgd -> z11
    // bfw XOR mjb -> z00
    // x03 OR x00 -> vdt
    // gnj AND wpb -> z02
    // x04 AND y00 -> kjc
    // djm OR pbm -> qhw
    // nrd AND vdt -> hwm
    // kjc AND fst -> rvg
    // y04 OR y02 -> fgs
    // y01 AND x02 -> pbm
    // ntg OR kjc -> kwq
    // psh XOR fgs -> tgd
    // qhw XOR tgd -> z09
    // pbm OR djm -> kpj
    // x03 XOR y03 -> ffh
    // x00 XOR y04 -> ntg
    // bfw OR bqk -> z06
    // nrd XOR fgs -> wpb
    // frj XOR qhw -> z04
    // bqk OR frj -> z07
    // y03 OR x01 -> nrd
    // hwm AND bqk -> z03
    // tgd XOR rvg -> z12
    // tnw OR pbm -> gnj
    // ";

    //     #[test]
    //     fn example_part1() {
    //         assert_eq!(part1(EXAMPLE), 2024);
    //     }

    // #[test]
    // fn example_part2() {
    //     assert_eq!(part2(EXAMPLE), "co,de,ka,ta");
    // }
}
