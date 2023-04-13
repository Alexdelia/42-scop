mod usemtl;
use usemtl::usemtl;

use crate::prelude::*;

use super::mtl;
use crate::obj::Material;
use crate::setting::OBJ_PATH;
use crate::Object;

use spof::FileData;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn parse(obj_path: &Path, mtl_path: &Vec<PathBuf>) -> Result<Object> {
    let f = FileData::new(obj_path, Some("#"))?;

    let mtl = if let Some(mtl) = usemtl(&f, mtl_path)? {
        Some(mtl::parse(&mtl)?)
    } else {
        None
    };

    todo!()
}
