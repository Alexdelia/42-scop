use crate::prelude::*;

use ansi::abbrev::{B, BLU, D, G, Y};
use spof::{get_line, ExpectedLine, ExpectedSize, FileData, Format, Keyword, Occurence};

pub fn newmtl(f: &FileData) -> Result<String> {
    let el = ExpectedLine::new(
        Keyword::new(
            "newmtl",
            f!("the {B}{G}name{D} of the {B}{BLU}material{D}, only one definition supported"),
        ),
        Format::new("material_name", ExpectedSize::Fixed),
        Occurence::Once,
    );

    Ok(get_line(f, el)?.take_first_token())
}
