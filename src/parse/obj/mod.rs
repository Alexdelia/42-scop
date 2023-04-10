use crate::prelude::*;

use super::{mtl, NamedFile};
use crate::obj::Material;
use crate::setting::OBJ_PATH;
use crate::Object;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn parse(obj_path: &Path, mtl_path: &Vec<PathBuf>) -> Result<Object> {
    let file = unwrap_reader(BufReader::new(File::open(obj_path)?))?;
    let no_comment = remove_comment(file.clone());
    assert_eq!(file.len(), no_comment.len());
    let f: NamedFile = (obj_path.to_string_lossy().to_string(), file, no_comment);

    let mtl = if let Some(mtl) = get_material(&f, mtl_path)? {
        Some(mtl::parse(&mtl)?)
    } else {
        None
    };

    todo!()
}

fn unwrap_reader(reader: BufReader<File>) -> Result<Vec<String>> {
    let mut ret = Vec::new();

    for line in reader.lines() {
        ret.push(line?);
    }

    Ok(ret)
}

fn remove_comment(content: Vec<String>) -> Vec<String> {
    let mut ret = Vec::new();

    for line in content {
        if let Some(i) = line.find('#') {
            ret.push(line[..i].to_string());
        } else {
            ret.push(line);
        }
    }

    ret
}

fn get_material(f: &NamedFile, mtl_path: &Vec<PathBuf>) -> Result<Option<PathBuf>> {
    if let Some((i, s)) = get_material_line(f)? {
        let no_comment_size = s.len();

        let mut path = PathBuf::from(OBJ_PATH);
        path.push(s);

        if !mtl_path.contains(&path) {
            return Err(pfe!(
                "cannot find {B}{Y}{path}{D} for {B}{Y}{}{D}",
                h:"make sure you have a valid {B}{G}.mtl{D} file in the {G}same directory{D} as the {B}{BLU}.obj{D} file",
                f:f.0.clone(),
                l:ple!(f.1[i].clone(), i:i, w:pwe!((0, no_comment_size)))
            ))?;
        }

        return Ok(Some(path));
    }

    Ok(None)
}

fn get_material_line(f: &NamedFile) -> Result<Option<(usize, String)>> {
    for (i, line) in f.2.iter().enumerate() {
        if line.starts_with("usemtl") {
            let mut split: Vec<&str> = line.split_whitespace().into_iter().collect();

            if split.len() != 2 {
                return Err(pfe!(
                    if split.len() < 2 {
                        "expected file after {B}{Y}usemtl{D}"
                    } else {
                        "expected only one file after {B}{Y}usemtl{D}"
                    },
                    h:"{B}{Y}usemtl{D} define the {B}{BLU}.mlt{D} file to use\nthe line must follow the format: `{B}usemtl {M}file.mtl{D}`",
                    f:f.0.clone(),
                    l:ple!(f.1[i].clone(), i:i, w:pwe!((0, line.len())))
                ))?;
            }

            return Ok(Some((i, split[1].to_string())));
        }
    }

    Ok(None)
}
