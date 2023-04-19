// mod usemtl;
// use usemtl::usemtl;
mod handle_mtl;
use handle_mtl::{check_usemtl, get_mtl};
mod rule;
mod vertex;

use crate::{prelude::*, VertexPrecision};

use super::mtl::parse as mtl_parse;
use crate::Object;

use spof::FoundLine;

use std::path::{Path, PathBuf};

pub fn parse(obj_path: &Path, mtl_path: &Vec<PathBuf>) -> Result<Object> {
    let m = spof::spof(obj_path, Some(rule::COMMENT), rule::obj_rule())?;

    let mtl = if let Some(p) = get_mtl(&m, obj_path, mtl_path)? {
        Some(mtl_parse(&p)?)
    } else {
        None
    };

    let usemtl = check_usemtl(obj_path, &m, &mtl)?;

    let v = m
        .get(rule::V)
        .unwrap_or(FoundLine::new())
        .0
        .iter()
        .map(|pl| pl.0.iter().map(|t| t.parse::<VertexPrecision>().unwrap()))
        .collect::<Result<Vec<_>>>()?;

    todo!()
}
