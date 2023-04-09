use crate::prelude::*;
use crate::Object;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn parse(obj_path: &Path, mtl_path: &Vec<PathBuf>) -> Result<Object> {
    let file = unwrap_reader(BufReader::new(File::open(obj_path)?))?;
    dbg!(file.len());

    todo!()
}

fn unwrap_reader(reader: BufReader<File>) -> Result<Vec<String>> {
    let mut ret = Vec::new();

    for line in reader.lines() {
        ret.push(line?);
    }

    Ok(ret)
}

fn get_material(file: Vec<String>) -> Option<String> {
    for line in file {
        if line.trim_start().starts_with("usemtl") {}
    }

    None
}
