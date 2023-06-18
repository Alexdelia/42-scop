use crate::prelude::*;

use super::RuleObj;
use crate::obj::{EFace, Face};

use ansi::abbrev::{B, D, M, N_C, R, Y};
use spof::SpofedFile;

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

        let texture = indices
            .next()
            .map(|s| s.parse::<usize>().map_err(ParseFaceError::Int))
            .transpose()?;

        let normal = indices
            .next()
            .map(|s| s.parse::<usize>().map_err(ParseFaceError::Int))
            .transpose()?;

        Ok(Self {
            vertex,
            texture,
            normal,
        })
    }
}
