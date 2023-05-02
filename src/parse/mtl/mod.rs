use crate::prelude::*;

use crate::obj::Material;

use ansi::abbrev::{B, BLU, D, G};
use spof::{rule, FileDataKey, SpofedFile};

use std::path::Path;

const COMMENT: &str = "#";

rule! (
    enum RuleMtl {
        Newmtl => "newmtl", "material_name", Fixed, Once, f!("the {B}{G}name{D} of the {B}{BLU}material{D}, only one definition supported"),
        Ka => "Ka", "R G B", Fixed, Once, "the ambiant color",
        Kd => "Kd", "R G B", Fixed, Once, "the diffuse color",
        Ks => "Ks", "R G B", Fixed, Once, "the specular color",
        Illum => "illum", "illumination_model", Fixed, Once, "the illumination model, 0-10",
        Ns => "Ns", "specular_exponent", Fixed, Once, "the specular exponent, 0.0-1000.0",
        D => "d", "dissolve", Fixed, Optional, "the dissolve, 0.0-1.0",
        Tr => "Tr", "dissolve", Fixed, Optional, "the dissolve, 0.0-1.0",
        // Ni => "Ni", "optical_density", Fixed, Once, "the optical density, 1.0-10.0",
        // MapKa => "map_Ka", "texture_file", Fixed, Once, "the ambiant texture",
        // MapKd => "map_Kd", "texture_file", Fixed, Once, "the diffuse texture",
        // MapKs => "map_Ks", "texture_file", Fixed, Once, "the specular texture",
        // MapNs => "map_Ns", "texture_file", Fixed, Once, "the specular exponent texture",
        // MapD => "map_d", "texture_file", Fixed, Once, "the dissolve texture",
        // MapBump => "map_bump", "texture_file", Fixed, Once, "the bump texture",
        // Bump => "bump", "texture_file", Fixed, Once, "the bump texture",
        // Disp => "disp", "texture_file", Fixed, Once, "the displacement texture",
        // Decal => "decal", "texture_file", Fixed, Once, "the decal texture",
        // Refl => "refl", "texture_file", Fixed, Once, "the reflection texture",
    }
);

pub fn parse(path: &Path) -> Result<Material> {
    let f = SpofedFile::new(path, Some(COMMENT), RuleMtl::build())?;
    todo!()
}
