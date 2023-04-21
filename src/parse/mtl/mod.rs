mod rule;

use crate::prelude::*;

use crate::obj::Material;

use spof::SpofedFile;

use std::path::Path;

pub fn parse(path: &Path) -> Result<Material> {
    let f = SpofedFile::new(path, Some(rule::COMMENT), rule::mtl_rule())?;
    todo!()
}
