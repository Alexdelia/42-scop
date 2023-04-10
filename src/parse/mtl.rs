use crate::prelude::*;

use super::FileData;
use crate::obj::Material;

use std::path::Path;

pub fn parse(path: &Path) -> Result<Material> {
    let f = FileData::new(path)?;
    todo!()
}

// fn parse_newmtl(
