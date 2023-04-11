use super::Occurence;

use yahmrslib::ansi::MAGENTA;

pub struct Token {
    pub token: String,
    pub occurence: Occurence,
    pub color: String,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token: String::new(),
            occurence: Occurence::Once,
            color: MAGENTA.to_string(),
        }
    }
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        Self {
            token: s.to_string(),
            ..Default::default()
        }
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Self {
            token: s,
            ..Default::default()
        }
    }
}

impl Token {
    pub fn new(
        token: impl Into<String>,
        occurence: Occurence,
        color: impl Into<Option<String>>,
    ) -> Self {
        Self {
            token: token.into(),
            occurence,
            color: color.into().unwrap_or_else(|| MAGENTA.to_string()),
        }
    }
}
