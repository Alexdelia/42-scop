mod face;
mod handle_mtl;
use handle_mtl::{check_usemtl, get_mtl};

use crate::prelude::*;

use super::mtl::parse as mtl_parse;
use crate::obj::{EFace, Face};
use crate::{Object, VertexPrecision};

use ansi::abbrev::{B, BLU, D, M};
use hmerr::ParseFileError;
use spof::{rule, FileDataKey, FoundLine, SpofedFile};

use std::path::{Path, PathBuf};

const COMMENT: &str = "#";

rule!(
    pub enum RuleObj {
        V => "v", "X Y Z [W]", (3, 4), ZeroOrMore, f!("vertex {B}{M}X{D} {B}{M}Y{D} {B}{M}Z{D} {B}{M}W{D}"),
        Vt => "vt", "U V [W]", (2, 3), ZeroOrMore, f!("texture coordinate {B}{M}U{D} {B}{M}V{D} {B}{M}W{D}"),
        Vn => "vn", "X Y Z", Fixed, ZeroOrMore, f!("vertex normal {B}{M}X{D} {B}{M}Y{D} {B}{M}Z{D}"),
        Vp => "vp", "U [V] [W]", (1, 3), ZeroOrMore, f!("parameter space vertices {B}{M}U{D} {B}{M}V{D} {B}{M}W{D}"),
        F => "f", "V1[/VT1][/VN1] V2[/VT2][/VN2] V3[/VT3][/VN3] ...", (3, usize::MAX), ZeroOrMore, f!("face {B}{M}V1{D} {B}{M}V2{D} {B}{M}V3{D} ..."),
        Mtllib => "mtllib", "file.mtl", Fixed, Optional, f!("the {B}{BLU}.mlt{D} file to use, only one definition supported"),
        Usemtl => "usemtl", "material_name", Fixed, Optional, f!("the {B}{BLU}.mlt{D} file to use, only one definition supported"),
        O => "o", "object_name", Fixed, Optional, f!("the {B}{M}name{D} of the {B}{BLU}object{D}, only one definition supported"),
        G => "g", "group_name", Fixed, Optional, f!("the {B}{M}name{D} of the {B}{BLU}group{D}, only one definition supported"),
        S => "s", "group_number", Fixed, Optional, f!("the {B}{M}number{D} of the {B}{BLU}smoothing group{D}, only one definition supported"),
    }
);

pub fn parse(obj_path: &Path, mtl_path: &[PathBuf]) -> Result<Object> {
    let f = SpofedFile::new(obj_path, Some(COMMENT), RuleObj::build())?;

    // let mtl = if let Some(p) = get_mtl(&f, mtl_path)? {
    //     Some(mtl_parse(&p)?)
    // } else {
    //     None
    // };
    let mtl = None;

    let usemtl = check_usemtl(&f, &mtl)?;
    let v = f.parse::<VertexPrecision>(RuleObj::V)?;
    let vn = f.parse::<VertexPrecision>(RuleObj::Vn)?;
    let vt = f.parse::<VertexPrecision>(RuleObj::Vt)?;
    let vp = f.parse::<VertexPrecision>(RuleObj::Vp)?;
    // let face = face::parse(&f)?;
    let face: Vec<Face> = f.parse::<EFace>(RuleObj::F)?;
    let name = f[RuleObj::O].data.first_token().map_or_else(
        || obj_path.file_stem().unwrap().to_str().unwrap().to_string(),
        |t| t.to_string(),
    );
    let group = f[RuleObj::G].data.first_token().map(|t| t.to_string());

    // TMP
    let v = v
        .into_iter()
        .map(|vertices| crate::Vertex {
            position: [vertices[0], vertices[1], vertices[2], 1.0],
            color: [1.0, 0.0, 0.0, 1.0],
            texture: [0.0, 0.0],
        })
        .collect();

    Ok(Object {
        name,
        group,
        vertex: v,
        // texture: vt,
        texture: vec![],
        // normal: vn,
        normal: vec![],
        // parameter_space: vp,
        face,
        material: if usemtl { mtl } else { None },
        smooth: false,
    })
}
