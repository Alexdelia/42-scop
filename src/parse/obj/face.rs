use crate::prelude::*;

use crate::obj::EFace;

use ansi::abbrev::{B, D, M, N_C, R, Y};

use std::num::ParseIntError;
use std::str::FromStr;

pub enum ParseFaceError {
    Int(ParseIntError),
    Format(String),
}

impl FromStr for EFace {
    type Err = ParseFaceError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let expected = || {
            f!("expected format:
	texture and normal absent:
		{B}{M}x{D}
	texture present but normal absent:
		{B}{M}x{N_C}/{M}x{D}
	texture absent but normal present:
		{B}{M}x{N_C}//{M}x{D}
	texture and normal present:
		{B}{M}x{N_C}/{M}x{N_C}/{M}x{D}

{B}{M}x{D} is an indice (usize) of a vertex, texture or normal")
        };

        if s.matches('/').count() > 2 {
            return Err(ParseFaceError::Format(f!(
                "too many '{B}{R}/{D}' in {B}{Y}face{D}: {B}{R}{s}{D}\n{}",
                expected()
            )));
        }

        let mut indices = s.split('/');
        let vertex = indices
            .next()
            .ok_or_else(|| {
                ParseFaceError::Format(f!(
                    "missing {B}{R}vertex {Y}face{D} in: {B}{R}{s}{D}\n{}",
                    expected()
                ))
            })?
            .parse::<usize>()
            .map_err(ParseFaceError::Int)?;

        let texture = match indices.next() {
            Some(s) => {
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse::<usize>().map_err(ParseFaceError::Int)?)
                }
            }
            None => None,
        };

        let normal = match indices.next() {
            Some(s) => {
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse::<usize>().map_err(ParseFaceError::Int)?)
                }
            }
            None => None,
        };

        Ok(Self {
            vertex,
            texture,
            normal,
        })
    }
}
