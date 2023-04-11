use crate::prelude::*;

use super::Keyword;
use crate::parse::FileData;

use ansi::abbrev::{B, D, G, M, R, Y};

use std::fmt::{Display, Formatter};

pub struct Format {
    pub token: String,
    size: Size,
}

pub enum Size {
    Fixed(usize),        // expected_size == size
    Undefined,           // expected_size >= 0
    Range(usize, usize), // expected_size >= min && expected_size <= max
}

#[derive(Default)]
pub enum ExpectedSize {
    #[default]
    Fixed,
    Undefined,
    Range(usize, usize),
}

impl From<(ExpectedSize, &str)> for Size {
    fn from((in_size, token): (ExpectedSize, &str)) -> Self {
        match in_size {
            ExpectedSize::Fixed => Self::Fixed(
                token
                    .split_whitespace()
                    .into_iter()
                    .collect::<Vec<&str>>()
                    .len(),
            ),
            ExpectedSize::Undefined => Self::Undefined,
            ExpectedSize::Range(min, max) => Self::Range(min, max),
        }
    }
}

impl Format {
    pub fn new(token: impl Into<String>, expected_size: ExpectedSize) -> Self {
        let token: String = token.into();
        let size: Size = (expected_size, token.as_str()).into();
        Self { token, size }
    }

    pub fn check(&self, line: &str) -> std::result::Result<Vec<String>, (String, usize)> {
        let mut split: Vec<String> = line
            .split_whitespace()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        self.size.check(split.len())?;
        Ok(split)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fixed(expected_size) => write!(f, "{}", expected_size),
            Self::Range(min, max) => write!(f, "{}-{}", min, max),
            _ => write!(f, "undefined"),
        }
    }
}

impl Size {
    fn in_range(&self, size: usize) -> bool {
        match self {
            Self::Fixed(expected_size) => size == *expected_size,
            Self::Range(min, max) => size >= *min && size <= *max,
            _ => true,
        }
    }

    fn check(&self, size: usize) -> std::result::Result<(), (String, usize)> {
        if self.in_range(size) {
            Ok(())
        } else {
            Err((format!("{self}"), size))
        }
    }
}
