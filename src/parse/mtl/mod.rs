mod newmtl;
use newmtl::newmtl;
mod ka;
use ka::ka;

use crate::prelude::*;

use crate::obj::Material;

use spof::FileData;

use std::path::Path;

pub fn parse(path: &Path) -> Result<Material> {
    let f = FileData::new(path, Some("#"))?;

    let newmtl = newmtl(&f)?;
    let ka = ka(&f)?;
    todo!()
}
