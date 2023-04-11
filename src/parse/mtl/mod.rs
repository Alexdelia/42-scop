use crate::prelude::*;

use super::FileData;
use crate::obj::Material;

use std::path::Path;

pub fn parse(path: &Path) -> Result<Material> {
    let f = FileData::new(path)?;

    let newmtl = parse_newmtl(&f)?;
    todo!()
}

fn parse_newmtl(f: &FileData) -> Result<String> {
    todo!()
}