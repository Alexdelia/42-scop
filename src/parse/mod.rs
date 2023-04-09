mod load_dir;
pub use load_dir::load_dir;
mod obj;

use crate::prelude::*;
use crate::setting::OBJ_PATH;
use crate::Object;

use std::path::{Path, PathBuf};

pub fn parse() -> Result<Vec<Object>> {
    let (obj, mtl) = group_file(load_dir(OBJ_PATH)?);
    let mut ret = Vec::new();

    for o in obj {
        match obj::parse(&o, &mtl) {
            Ok(obj) => ret.push(obj),
            Err(e) => eprintln!("ERROR with {}: {}", o.display(), e), // TODO: use hmerr
        }
    }

    if ret.is_empty() {
        todo!("return error: did not manage to load any object");
        // Err("did not manage to load any object")
    } else {
        Ok(ret)
    }
}

fn group_file(dir: Vec<PathBuf>) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let (mut obj, mut mtl) = (Vec::new(), Vec::new());

    // check if each file.obj has his file.mtl
    for p in dir {
        match check_ext(&p) {
            Ext::Obj => obj.push(p),
            Ext::Mtl => mtl.push(p),
            Ext::None => eprintln!(
                "WARNING with {}: only treating .obj/.mtl files",
                p.display()
            ),
        }
    }

    (obj, mtl)
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
    use std::path::{Path, PathBuf};

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

    #[test]
    fn test_group_file() {
        let dir = vec![
            PathBuf::from("test0.obj"),
            PathBuf::from("test1.obj"),
            PathBuf::from("test2.obj"),
            PathBuf::from("test0.mtl"),
            PathBuf::from("test1.mtl"),
            PathBuf::from("test.txt"),
            PathBuf::from("test"),
        ];

        let (obj, mtl) = group_file(dir);

        assert_eq!(
            obj,
            vec![
                PathBuf::from("test0.obj"),
                PathBuf::from("test1.obj"),
                PathBuf::from("test2.obj")
            ]
        );
        assert_eq!(
            mtl,
            vec![PathBuf::from("test0.mtl"), PathBuf::from("test1.mtl")]
        );
    }
}
