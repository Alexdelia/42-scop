use crate::prelude::*;

use super::rule::MTLLIB;
use crate::setting::OBJ_PATH;

use ansi::abbrev::{B, D, Y};
use spof::FoundLine;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn get_mtl(
    m: &HashMap<String, FoundLine>,
    obj_path: &Path,
    mtl_path: &Vec<PathBuf>,
) -> Result<Option<PathBuf>> {
    let Some(mtl) = m.get(MTLLIB) else {
		return Ok(None);
	};

    let pl = mtl.get_once();
    let mut path = PathBuf::from(OBJ_PATH);
    path.push(&pl.0[0]);

    if !mtl_path.contains(&path) {
        pfe!(
            f!("cannot find {B}{Y}{path}{D} for {B}{Y}{obj}{D}", path = path.display(), obj = obj_path.display()),
            h:"make sure you have a valid {B}{G}.mtl{D} file in the {G}same directory{D} as the {B}{BLU}.obj{D} file",
            f:obj_path.to_string_lossy().to_string(),
            l:pl.clone().into(),
        )?;
    }

    Ok(Some(path))
}
