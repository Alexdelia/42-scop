use crate::prelude::*;

use ansi::abbrev::{B, BLU, D, G, Y};
use spof::{rule, Rule};

pub fn mtl_rule() -> Rule {
    rule!(
        (
            "newmtl",
            "material_name",
            Fixed,
            Once,
            f!("the {B}{G}name{D} of the {B}{BLU}material{D}, only one definition supported")
        ),
        ("Ka", "R G B", Fixed, Once, "the ambiant color")
    )
}
