mod load_dir;
pub use load_dir::load_dir;
mod obj;

use yahmrslib::hmerr::Result;

use crate::setting::OBJ_PATH;
use crate::Object;

use std::path::{Path, PathBuf};

pub fn parse() -> Result<Vec<Object>> {
    let dir = group_file(load_dir(OBJ_PATH)?);

    if dir.is_empty() {
        todo!("return error: did not manage to load any object");
        // Err("did not manage to load any object")
    } else {
        todo!("return Ok(dir)");
        // Ok(dir)
    }
}

fn group_file(dir: Vec<PathBuf>) -> Vec<(PathBuf, Option<PathBuf>)> {
    let mut ret: Vec<(PathBuf, Option<PathBuf>)> = Vec::new();

    // check if each file.obj has his file.mtl
    for obj in dir {
        match check_ext(&obj) {
            Ext::None => {
                eprintln!(
                    "WARNING with {}: only treating .obj/.mtl files",
                    obj.display()
                );
                continue;
            }
            Ext::Mtl => continue,
            Ext::Obj => {
                let mut mtl = obj.clone();
                mtl.set_extension("mtl");
                if mtl.exists() {
                    ret.push((obj, Some(mtl)));
                } else {
                    ret.push((obj, None));
                }
            }
        }
    }

    ret
}

enum Ext {
    Obj,
    Mtl,
    None,
}

fn check_ext(path: &Path) -> Ext {
    match path.extension() {
        Some(ext) if ext == "obj" => Ext::Obj,
        Some(ext) if ext == "mtl" => Ext::Mtl,
        _ => Ext::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // derive PartialEq for Ext
    impl PartialEq for Ext {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Ext::Obj, Ext::Obj) => true,
                (Ext::Mtl, Ext::Mtl) => true,
                (Ext::None, Ext::None) => true,
                _ => false,
            }
        }
    }
    impl std::fmt::Debug for Ext {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Ext::Obj => write!(f, "Ext::Obj"),
                Ext::Mtl => write!(f, "Ext::Mtl"),
                Ext::None => write!(f, "Ext::None"),
            }
        }
    }

    #[test]
    fn test_check_ext() {
        let path = Path::new("test.obj");
        assert_eq!(check_ext(path), Ext::Obj);

        let path = Path::new("test.mtl");
        assert_eq!(check_ext(path), Ext::Mtl);

        let path = Path::new("test.txt");
        assert_eq!(check_ext(path), Ext::None);

        let path = Path::new("test");
        assert_eq!(check_ext(path), Ext::None);
    }
}
