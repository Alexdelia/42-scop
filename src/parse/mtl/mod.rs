mod rule;

use crate::prelude::*;

use crate::obj::Material;

use std::path::Path;

pub fn parse(path: &Path) -> Result<Material> {
    let m = spof::spof(path, Some(rule::COMMENT), rule::mtl_rule())?;
    todo!()
}
