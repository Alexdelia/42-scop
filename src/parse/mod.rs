mod load_dir;
pub use load_dir::load_dir;
mod file_data;

mod mtl;
mod obj;

use crate::prelude::*;
use crate::setting::OBJ_PATH;
use crate::Object;

use ansi::abbrev::{B, BLU, D, G, M, Y};

use std::path::{Path, PathBuf};

pub fn parse() -> Result<Vec<Object>> {
    let (obj, mtl) = get_file(OBJ_PATH)?;
    let mut ret = Vec::new();

    for o in obj {
        match obj::parse(&o, &mtl) {
            Ok(obj) => ret.push(obj),
            Err(e) => warn!("cannot load {B}{M}{}{D} caused by:\n{Y}{e}{D}", o.display()),
        }
    }

    if ret.is_empty() {
        pfe!(
            f!("did not manage to load any {Y}object{D}"),
            h: f!("make sure you have at least one {G}valid {B}{BLU}.obj{D} file in {B}{M}{OBJ_PATH}{D}")
        )?
    } else {
        Ok(ret)
    }
}

fn get_file(path: impl AsRef<Path>) -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let dir = load_dir(path)?;

    let (mut obj, mut mtl) = (Vec::new(), Vec::new());

    for p in dir {
        match check_ext(&p) {
            Ext::Obj => obj.push(p),
            Ext::Mtl => mtl.push(p),
            Ext::None => warn!(
                "{} is not a {B}{BLU}.obj{D}/{B}{BLU}.mtl{D} file",
                p.display()
            ),
        }
    }

    Ok((obj, mtl))
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
