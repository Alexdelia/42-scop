use crate::prelude::*;

use super::RuleObj;
use crate::obj::Material;
use crate::setting::OBJ_PATH;

use ansi::abbrev::{B, BLU, D, Y};
use spof::{SpofedFile};


use std::path::{PathBuf};

pub fn get_mtl(f: &SpofedFile<RuleObj>, mtl_path: &[PathBuf]) -> Result<Option<PathBuf>> {
    let mtl = &f[RuleObj::Mtllib].data;
    if mtl.is_empty() {
        return Ok(None);
    }

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

pub fn check_usemtl(f: &SpofedFile<RuleObj>, mtl: &Option<Material>) -> Result<bool> {
    let Some(mtl) = mtl else {
		return Ok(false);
	};

    let fl = &f[RuleObj::Usemtl].data;
    if fl.is_empty() {
        return Ok(false);
    }

    let usemtl = fl.get_first_token();

    if *usemtl != mtl.name {
        pfe!(
            f!("expected use of material {B}{Y}{mtl}{D} but found {B}{Y}{usemtl}{D}", mtl = mtl.name, usemtl = usemtl),
            h:f!(
                "{B}{BLU}{path}{D} uses material {B}{Y}{usemtl}{D} but the material name in {B}{BLU}{mtl_path}{D} is {B}{Y}{mtl}{D}",
                path = f.path.display(),
                mtl_path = f[RuleObj::Mtllib].data.get_first_token(),
                mtl = mtl.name
            ),
            f:f.name(),
            l:fl.get_once().clone().into(),
        )?;
    }

    Ok(true)
}
