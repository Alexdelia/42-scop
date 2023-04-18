use crate::prelude::*;

use ansi::abbrev::{B, BLU, D, GRE, M, Y};
use spof::{rule, Rule};

pub const COMMENT: &str = "#";

pub const V: &str = "v";
pub const VT: &str = "vt";
pub const VN: &str = "vn";
pub const VP: &str = "vp";
pub const F: &str = "f";
pub const MTLLIB: &str = "mtllib";
pub const USEMTL: &str = "usemtl";
pub const O: &str = "o";
pub const G: &str = "g";
pub const S: &str = "s";

pub fn obj_rule() -> Rule {
    rule!(
        (
            V,
            "X Y Z [W]",
            (3, 4),
            ZeroOrMore,
            f!("vertex {B}{M}X{D} {B}{M}Y{D} {B}{M}Z{D} {B}{M}W{D}")
        ),
        (
            VT,
            "U V [W]",
            (2, 3),
            ZeroOrMore,
            f!("texture coordinate {B}{M}U{D} {B}{M}V{D} {B}{M}W{D}")
        ),
        (
            VN,
            "X Y Z",
            Fixed,
            ZeroOrMore,
            f!("vertex normal {B}{M}X{D} {B}{M}Y{D} {B}{M}Z{D}")
        ),
        (
            VP,
            "U [V] [W]",
            (1, 3),
            ZeroOrMore,
            f!("parameter space vertices {B}{M}U{D} {B}{M}V{D} {B}{M}W{D}")
        ),
        (
            F,
            "V1[/VT1][/VN1] V2[/VT2][/VN2] V3[/VT3][/VN3] ...",
            (3, usize::MAX),
            ZeroOrMore,
            f!("face {B}{M}V1{D} {B}{M}V2{D} {B}{M}V3{D} ...")
        ),
        (
            MTLLIB,
            "file.mtl",
            Fixed,
            Optional,
            f!("the {B}{BLU}.mlt{D} file to use, only one definition supported")
        ),
        (
            USEMTL,
            "material_name",
            Fixed,
            Optional,
            f!("the {B}{BLU}.mlt{D} file to use, only one definition supported")
        ),
        (
            O,
            "object_name",
            Fixed,
            Optional,
            f!("the {B}{M}name{D} of the {B}{BLU}object{D}, only one definition supported")
        ),
        (
            G,
            "group_name",
            Fixed,
            Optional,
            f!("the {B}{M}name{D} of the {B}{BLU}group{D}, only one definition supported")
        ),
        (
            S,
            "<off | 0 | 1 | on>",
            Fixed,
            Optional,
            f!("the smooth shading {B}{M}state{D}, only one definition supported")
        ),
    )
}
