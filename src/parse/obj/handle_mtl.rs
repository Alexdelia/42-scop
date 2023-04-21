use crate::prelude::*;

use super::rule::{MTLLIB, USEMTL};
use crate::obj::Material;
use crate::setting::OBJ_PATH;

use ansi::abbrev::{B, BLU, D, Y};
use spof::{FoundLine, SpofedFile};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn get_mtl(f: &SpofedFile, mtl_path: &Vec<PathBuf>) -> Result<Option<PathBuf>> {
    let Some(mtl) = f.get(MTLLIB) else {
		return Ok(None);
	};

    let pl = mtl.get_once();
    let mut path = PathBuf::from(OBJ_PATH);
    path.push(&pl.0[0]);

    if !mtl_path.contains(&path) {
        pfe!(
            f!("cannot find {B}{Y}{path}{D} for {B}{Y}{obj}{D}", path = path.display(), obj = f.path.display()),
            h:"make sure you have a valid {B}{G}.mtl{D} file in the {G}same directory{D} as the {B}{BLU}.obj{D} file",
            f: f.path.to_string_lossy().to_string(),
            l:pl.clone().into(),
        )?;
    }

    Ok(Some(path))
}

pub fn check_usemtl(f: &SpofedFile, mtl: &Option<Material>) -> Result<bool> {
    let Some(mtl) = mtl else {
		return Ok(false);
	};

    let Some(fl) = f.get(USEMTL) else {
		return Ok(false);
	};

    let usemtl = fl.get_first_token();

    if *usemtl != mtl.name {
        pfe!(
            f!("expected use of material {B}{Y}{mtl}{D} but found {B}{Y}{usemtl}{D}", mtl = mtl.name, usemtl = usemtl),
            h:f!(
                "{B}{BLU}{path}{D} uses material {B}{Y}{usemtl}{D} but the material name in {B}{BLU}{mtl_path}{D} is {B}{Y}{mtl}{D}",
                path = f.path.display(),
                mtl_path = f.get(MTLLIB).unwrap().get_first_token(),
                mtl = mtl.name
            ),
            f:f.path.to_string_lossy().to_string(),
            l:fl.get_once().clone().into(),
        )?;
    }

    Ok(true)
}
