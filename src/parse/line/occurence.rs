use std::fmt::{Debug, Display};

use yahmrslib::ansi::abbrev::{B, D, G, I, R};

#[derive(Default)]
pub enum Occurence {
    #[default]
    Once, // 1
    Optional,            // 0 or 1
    ZeroOrMore,          // 0 or more
    OneOrMore,           // 1 or more
    Exactly(usize),      // exactly n
    Range(usize, usize), // between start and end
}

impl Display for Occurence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Occurence::Once => write!(f, "once"),
            Occurence::Optional => write!(f, "optional"),
            Occurence::ZeroOrMore => write!(f, "zero or more"),
            Occurence::OneOrMore => write!(f, "one or more"),
            Occurence::Exactly(x) => write!(f, "exactly {x}"),
            Occurence::Range(start, end) => write!(f, "between {start} and {end}"),
        }
    }
}

impl Debug for Occurence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Occurence::Once => write!(f, "n == 1"),
            Occurence::Optional => write!(f, "n <= 1"),
            Occurence::ZeroOrMore => write!(f, "n >= 0"),
            Occurence::OneOrMore => write!(f, "n >= 1"),
            Occurence::Exactly(x) => write!(f, "n == {x}"),
            Occurence::Range(start, end) => write!(f, "n >= {start} && n <= {end}"),
        }
    }
}

impl Occurence {
    pub fn in_range(&self, n: usize) -> bool {
        match self {
            Occurence::Once => n == 1,
            Occurence::Optional => n <= 1,
            Occurence::ZeroOrMore => n >= 0,
            Occurence::OneOrMore => n >= 1,
            Occurence::Exactly(x) => n == *x,
            Occurence::Range(start, end) => n >= *start && n <= *end,
        }
    }

    pub fn check(&self, n: usize) -> Result<(), String> {
        if self.in_range(n) {
            Ok(())
        } else {
            Err(format!(
                "expected to be {B}{G}{self} {I}({self:?}){D}, but it occured {B}{R}{n}{D} times",
            ))
        }
    }
}
