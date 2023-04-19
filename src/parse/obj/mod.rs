// mod usemtl;
// use usemtl::usemtl;
mod get_mtl;
mod rule;
use get_mtl::get_mtl;

use crate::prelude::*;

use super::mtl;
use crate::Object;

use std::path::{Path, PathBuf};

pub fn parse(obj_path: &Path, mtl_path: &Vec<PathBuf>) -> Result<Object> {
    let m = spof::spof(obj_path, Some(rule::COMMENT), rule::obj_rule())?;

    let mtl = if let Some(p) = get_mtl(&m, obj_path, mtl_path)? {
        Some(mtl::parse(&p)?)
    } else {
        None
    };

    todo!()
}
