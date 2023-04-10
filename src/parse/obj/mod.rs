mod get_material;
use get_material::get_material;

use crate::prelude::*;

use super::{mtl, FileData};
use crate::obj::Material;
use crate::setting::OBJ_PATH;
use crate::Object;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn parse(obj_path: &Path, mtl_path: &Vec<PathBuf>) -> Result<Object> {
    let f = FileData::new(obj_path)?;

    let mtl = if let Some(mtl) = get_material(&f, mtl_path)? {
        Some(mtl::parse(&mtl)?)
    } else {
        None
    };

    todo!()
}
