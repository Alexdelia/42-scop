// mod usemtl;
// use usemtl::usemtl;

use crate::prelude::*;

use super::mtl;
use crate::obj::Material;
use crate::setting::OBJ_PATH;
use crate::Object;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn parse(obj_path: &Path, mtl_path: &Vec<PathBuf>) -> Result<Object> {
    mtl::parse(obj_path);
    todo!()
}
