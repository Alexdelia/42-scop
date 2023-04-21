// mod usemtl;
// use usemtl::usemtl;
mod handle_mtl;
use handle_mtl::{check_usemtl, get_mtl};
mod rule;
mod vertex;

use crate::{prelude::*, VertexPrecision};

use super::mtl::parse as mtl_parse;
use crate::Object;

use spof::{FoundLine, SpofedFile};

use std::path::{Path, PathBuf};

pub fn parse(obj_path: &Path, mtl_path: &Vec<PathBuf>) -> Result<Object> {
    let f = SpofedFile::new(obj_path, Some(rule::COMMENT), rule::obj_rule())?;

    let mtl = if let Some(p) = get_mtl(&f, mtl_path)? {
        Some(mtl_parse(&p)?)
    } else {
        None
    };

    let usemtl = check_usemtl(&f, &mtl)?;

    // parse vertices

    todo!()
}
