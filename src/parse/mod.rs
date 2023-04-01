mod load_dir;
use std::path::PathBuf;

pub use load_dir::load_dir;
mod obj;

use yahmrslib::hmerr::Result;

use crate::setting::OBJ_PATH;
use crate::Object;

pub fn parse() -> Result<Vec<Object>> {
    let dir = group_file(load_dir(OBJ_PATH)?);

    if obj.is_empty() {
        Err("did not manage to load any object")
    } else {
        Ok(obj)
    }
}

fn group_file(dir: Vec<PathBuf>) -> Vec<(PathBuf, Option<PathBuf>)> {
    let mut ret: Vec<(PathBuf, Option<PathBuf>)> = Vec::new();

    // check if each file.obj has his file.mtl
    for obj in dir {
		if  && ext != "obj" && ext != "mtl" {
			eprintln!("WARNING with {}: only treating .obj/.mtl files", obj.display());
			continue;
		};
		if ext != "obj" && ext != "mtl" {


        let mut mtl = None;
        if let Some(ext) = obj.extension() {
            if ext == "obj" {
                let mut mtl_path = obj.clone();
                mtl_path.set_extension("mtl");
                if mtl_path.exists() {
                    mtl = Some(mtl_path);
                }
            }
        }
        ret.push((obj, mtl));
    }

    ret
}
