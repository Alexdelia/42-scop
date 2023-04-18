use crate::prelude::*;

use ansi::abbrev::{B, BLU, D, G, M, Y};
use spof::{rule, Rule};

pub fn mtl_rule() -> Rule {
    rule!(
        (
            "v",
            "X Y Z [W]",
            (3, 4),
            ZeroOrMore,
            f!("vertex {B}{M}X{D} {B}{M}Y{D} {B}{M}Z{D} {B}{M}W{D}")
        ),
        (
            "vt",
            "U V [W]",
            (2, 3),
            ZeroOrMore,
            f!("texture coordinate {B}{M}U{D} {B}{M}V{D} {B}{M}W{D}")
        ),
        (
            "vn",
            "X Y Z",
            Fixed,
            ZeroOrMore,
            f!("vertex normal {B}{M}X{D} {B}{M}Y{D} {B}{M}Z{D}")
        ),
        (
            "vp",
            "U [V] [W]",
            (1, 3),
            ZeroOrMore,
            f!("parameter space vertices {B}{M}U{D} {B}{M}V{D} {B}{M}W{D}")
        ),
        (
            "f",
            "V1[/VT1][/VN1] V2[/VT2][/VN2] V3[/VT3][/VN3] ...",
            (3, usize::MAX),
            ZeroOrMore,
            f!("face {B}{M}V1{D} {B}{M}V2{D} {B}{M}V3{D} ...")
        ),
        (
            "mtllib",
            "file.mtl",
            Fixed,
            Optional,
            f!("the {B}{BLU}.mlt{D} file to use, only one definition supported")
        ),
        (
            "usemtl",
            "material_name",
            Fixed,
            Optional,
            f!("the {B}{BLU}.mlt{D} file to use, only one definition supported")
        ),
        (
            "o",
            "object_name",
            Fixed,
            Optional,
            f!("the {B}{M}name{D} of the {B}{BLU}object{D}, only one definition supported")
        ),
        (
            "g",
            "group_name",
            Fixed,
            Optional,
            f!("the {B}{M}name{D} of the {B}{BLU}group{D}, only one definition supported")
        ),
        (
            "s",
            "<off | 0 | 1 | on>",
            Fixed,
            Optional,
            f!("the smooth shading {B}{M}state{D}, only one definition supported")
        ),
    )
}
