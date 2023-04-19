use crate::prelude::*;

use ansi::abbrev::{B, BLU, G};
use ansi::RESET as DEF;
use spof::{rule, Rule};

pub const COMMENT: &str = "#";

pub const NEWMTL: &str = "newmtl";
pub const KA: &str = "Ka";
pub const KD: &str = "Kd";
pub const KS: &str = "Ks";
pub const ILLUM: &str = "illum";
pub const NS: &str = "Ns";
pub const D: &str = "d";
pub const TR: &str = "Tr";
// pub const NI: &str = "Ni";
// pub const MAP_KA: &str = "map_Ka";
// pub const MAP_KD: &str = "map_Kd";
// pub const MAP_KS: &str = "map_Ks";
// pub const MAP_NS: &str = "map_Ns";
// pub const MAP_D: &str = "map_d";
// pub const MAP_BUMP: &str = "map_bump";
// pub const BUMP: &str = "bump";
// pub const DISP: &str = "disp";
// pub const DECAL: &str = "decal";
// pub const REFL: &str = "refl";

pub fn mtl_rule() -> Rule {
    rule!(
        (
            NEWMTL,
            "material_name",
            Fixed,
            Once,
            f!("the {B}{G}name{DEF} of the {B}{BLU}material{DEF}, only one definition supported")
        ),
        (KA, "R G B", Fixed, Once, "the ambiant color"),
        (KD, "R G B", Fixed, Once, "the diffuse color"),
        (KS, "R G B", Fixed, Once, "the specular color"),
        (
            ILLUM,
            "illumination_model",
            Fixed,
            Once,
            "the illumination model, 0-10"
        ),
        (
            NS,
            "specular_exponent",
            Fixed,
            Once,
            "the specular exponent, 0.0-1000.0"
        ),
        (D, "dissolve", Fixed, Optional, "the dissolve, 0.0-1.0"),
        (TR, "dissolve", Fixed, Optional, "the dissolve, 0.0-1.0"),
        // (
        //     NI,
        //     "optical_density",
        //     Fixed,
        //     Once,
        //     "the optical density, 1.0-10.0"
        // ),
        // (MAP_KA, "texture_file", Fixed, Once, "the ambiant texture"),
        // (MAP_KD, "texture_file", Fixed, Once, "the diffuse texture"),
        // (
        //     MAP_KS,
        //     "texture_file",
        //     Fixed,
        //     Once,
        //     "the specular texture"
        // ),
        // (
        //     MAP_NS,
        //     "texture_file",
        //     Fixed,
        //     Once,
        //     "the specular exponent texture"
        // ),
        // (MAP_D, "texture_file", Fixed, Once, "the dissolve texture"),
        // (MAP_BUMP, "texture_file", Fixed, Once, "the bump texture"),
        // (BUMP, "texture_file", Fixed, Once, "the bump texture"),
        // (
        //     DISP,
        //     "texture_file",
        //     Fixed,
        //     Once,
        //     "the displacement texture"
        // ),
        // (DECAL, "texture_file", Fixed, Once, "the decal texture"),
        // (
        //     REFL,
        //     "texture_file",
        //     Fixed,
        //     Once,
        //     "the reflection texture"
        // ),
    )
}
