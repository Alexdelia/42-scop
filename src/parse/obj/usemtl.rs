use crate::prelude::*;

use crate::setting::OBJ_PATH;

use ansi::abbrev::{B, BLU, D, Y};
use spof::{get_line, ExpectedLine, ExpectedSize, FileData, Format, Keyword, Occurence};

use std::path::PathBuf;

pub fn usemtl(f: &FileData, mtl_path: &Vec<PathBuf>) -> Result<Option<PathBuf>> {
    let el = ExpectedLine::new(
        Keyword::new(
            "usemtl",
            f!("the {B}{BLU}.mlt{D} file to use, only one definition supported"),
        ),
        Format::new("file.mtl", ExpectedSize::Fixed),
        Occurence::Optional,
    );

    let found = get_line(f, el)?;
    if found.is_empty() {
        return Ok(None);
    }
    let pl = &found.0[0];

    let mut path = PathBuf::from(OBJ_PATH);
    path.push(&pl.0[0]);
    let i = pl.1;

    if !mtl_path.contains(&path) {
        let unprocessed_line = f.content[i].clone();
        let l = unprocessed_line.len();

        pfe!(
            f!("cannot find {B}{Y}{path}{D} for {B}{Y}{obj}{D}", path = path.display(), obj = f.name),
            h:"make sure you have a valid {B}{G}.mtl{D} file in the {G}same directory{D} as the {B}{BLU}.obj{D} file",
            f:f.name.clone(),
            l:ple!(unprocessed_line, i:i, w:pwe!((0, l)))
        )?;
    }

    Ok(Some(path));

    Ok(None)
}
