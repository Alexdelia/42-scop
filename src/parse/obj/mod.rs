// mod usemtl;
// use usemtl::usemtl;
mod rule;

use crate::prelude::*;

use super::mtl;
use crate::obj::Material;
use crate::setting::OBJ_PATH;
use crate::Object;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn parse(obj_path: &Path, mtl_path: &Vec<PathBuf>) -> Result<Object> {
    let m = spof::spof(obj_path, Some(rule::COMMENT), rule::obj_rule())?;
    mtl::parse(obj_path);
    todo!()
}
