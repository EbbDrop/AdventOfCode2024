use aoc_runner_derive::aoc;

enum SimpleParser {
    None,
    M,
    MU,
    MUL,
    MULb,
    MULbp(u16),
    MULbpp(u16),
    MULbppp(u16),
    MULbc(u16),
    MULbcp(u16, u16),
    MULbcpp(u16, u16),
    MULbcppp(u16, u16),
}

impl SimpleParser {
    fn is_next_ok(&self, c: u8) -> bool {
        match self {
            Self::None => false, // Hack for Self::advance
            Self::M => c == b'u',
            Self::MU => c == b'l',
            Self::MUL => c == b'(',
            Self::MULb => c.is_ascii_digit(),
            Self::MULbp(_) => c.is_ascii_digit() || c == b',',
            Self::MULbpp(_) => c.is_ascii_digit() || c == b',',
            Self::MULbppp(_) => c == b',',
            Self::MULbc(_) => c.is_ascii_digit(),
            Self::MULbcp(_, _) => c.is_ascii_digit() || c == b')',
            Self::MULbcpp(_, _) => c.is_ascii_digit() || c == b')',
            Self::MULbcppp(_, _) => c == b')',
        }
    }

    fn advance(&mut self, c: u8) -> u32 {
        if !self.is_next_ok(c) {
            *self = if c == b'm' { Self::M } else { Self::None };
            return 0;
        }
        *self = match &self {
            Self::None => Self::None, // unreachable
            Self::M => Self::MU,
            Self::MU => Self::MUL,
            Self::MUL => Self::MULb,
            Self::MULb => Self::MULbp((c - b'0') as u16),
            Self::MULbp(p) => {
                if c == b',' {
                    Self::MULbc(*p)
                } else {
                    Self::MULbpp(*p * 10 + (c - b'0') as u16)
                }
            }
            Self::MULbpp(p) => {
                if c == b',' {
                    Self::MULbc(*p)
                } else {
                    Self::MULbppp(*p * 10 + (c - b'0') as u16)
                }
            }
            Self::MULbppp(p) => Self::MULbc(*p),
            Self::MULbc(p) => Self::MULbcp(*p, (c - b'0') as u16),
            Self::MULbcp(f, p) => {
                if c == b')' {
                    let v = *f as u32 * *p as u32;
                    *self = Self::None;
                    return v;
                } else {
                    Self::MULbcpp(*f, *p * 10 + (c - b'0') as u16)
                }
            }
            Self::MULbcpp(f, p) => {
                if c == b')' {
                    let v = *f as u32 * *p as u32;
                    *self = Self::None;
                    return v;
                } else {
                    Self::MULbcppp(*f, *p * 10 + (c - b'0') as u16)
                }
            }
            Self::MULbcppp(f, p) => {
                let v = *f as u32 * *p as u32;
                *self = Self::None;
                return v;
            }
        };
        return 0;
    }
}

#[aoc(day3, part1)]
pub fn part1(s: &str) -> u32 {
    let mut sum = 0;

    let mut state = SimpleParser::None;
    for c in s.bytes() {
        sum += state.advance(c);
    }

    return sum;
}

enum OnParser {
    None,
    M,
    MU,
    MUL,
    MULb,
    MULbp(u16),
    MULbpp(u16),
    MULbppp(u16),
    MULbc(u16),
    MULbcp(u16, u16),
    MULbcpp(u16, u16),
    MULbcppp(u16, u16),
    D,
    DO,
    DON,
    DONq,
    DONqT,
    DONqTb,
}

enum OffParser {
    None,
    D,
    DO,
    DOb,
}

impl OffParser {
    fn is_next_ok(&self, c: u8) -> bool {
        match self {
            Self::None => false, // Hack for Self::advance
            Self::D => c == b'o',
            Self::DO => c == b'(',
            Self::DOb => c == b')',
        }
    }

    fn advance(&mut self, c: u8) -> bool {
        if !self.is_next_ok(c) {
            *self = if c == b'd' { Self::D } else { Self::None };
            return false;
        }
        *self = match &self {
            Self::None => Self::None, // unreachable
            OffParser::D => OffParser::DO,
            OffParser::DO => OffParser::DOb,
            OffParser::DOb => {
                return true;
            }
        };
        return false;
    }
}

impl OnParser {
    fn is_next_ok(&self, c: u8) -> bool {
        match self {
            Self::None => false, // Hack for Self::advance
            Self::M => c == b'u',
            Self::MU => c == b'l',
            Self::MUL => c == b'(',
            Self::MULb => c.is_ascii_digit(),
            Self::MULbp(_) => c.is_ascii_digit() || c == b',',
            Self::MULbpp(_) => c.is_ascii_digit() || c == b',',
            Self::MULbppp(_) => c == b',',
            Self::MULbc(_) => c.is_ascii_digit(),
            Self::MULbcp(_, _) => c.is_ascii_digit() || c == b')',
            Self::MULbcpp(_, _) => c.is_ascii_digit() || c == b')',
            Self::MULbcppp(_, _) => c == b')',
            Self::D => c == b'o',
            Self::DO => c == b'n' || c == b'(',
            Self::DON => c == b'\'',
            Self::DONq => c == b't',
            Self::DONqT => c == b'(',
            Self::DONqTb => c == b')',
        }
    }

    fn advance(&mut self, c: u8) -> Option<u32> {
        if !self.is_next_ok(c) {
            *self = if c == b'm' {
                Self::M
            } else if c == b'd' {
                Self::D
            } else {
                Self::None
            };
            return Some(0);
        }
        *self = match &self {
            Self::None => Self::None, // unreachable
            Self::M => Self::MU,
            Self::MU => Self::MUL,
            Self::MUL => Self::MULb,
            Self::MULb => Self::MULbp((c - b'0') as u16),
            Self::MULbp(p) => {
                if c == b',' {
                    Self::MULbc(*p)
                } else {
                    Self::MULbpp(*p * 10 + (c - b'0') as u16)
                }
            }
            Self::MULbpp(p) => {
                if c == b',' {
                    Self::MULbc(*p)
                } else {
                    Self::MULbppp(*p * 10 + (c - b'0') as u16)
                }
            }
            Self::MULbppp(p) => Self::MULbc(*p),
            Self::MULbc(p) => Self::MULbcp(*p, (c - b'0') as u16),
            Self::MULbcp(f, p) => {
                if c == b')' {
                    let v = *f as u32 * *p as u32;
                    *self = Self::None;
                    return Some(v);
                } else {
                    Self::MULbcpp(*f, *p * 10 + (c - b'0') as u16)
                }
            }
            Self::MULbcpp(f, p) => {
                if c == b')' {
                    let v = *f as u32 * *p as u32;
                    *self = Self::None;
                    return Some(v);
                } else {
                    Self::MULbcppp(*f, *p * 10 + (c - b'0') as u16)
                }
            }
            Self::MULbcppp(f, p) => {
                let v = *f as u32 * *p as u32;
                *self = Self::None;
                return Some(v);
            }
            Self::D => Self::DO,
            Self::DO => Self::DON,
            Self::DON => Self::DONq,
            Self::DONq => Self::DONqT,
            Self::DONqT => Self::DONqTb,
            Self::DONqTb => return None,
        };
        return Some(0);
    }
}

#[aoc(day3, part2)]
pub fn part2(s: &str) -> u32 {
    let s = s.as_bytes();

    let mut sum = 0;

    let mut i = 0;
    while i < s.len() {
        let mut state = OnParser::None;
        while i < s.len() {
            if let Some(v) = state.advance(s[i]) {
                i += 1;
                sum += v;
            } else {
                i += 1;
                break;
            }
        }
        let mut state = OffParser::None;
        while i < s.len() {
            if state.advance(s[i]) {
                i += 1;
                break;
            } else {
                i += 1;
            }
        }
    }
    return sum;
}
