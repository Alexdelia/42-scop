use crate::prelude::*;

use ansi::abbrev::{B, BLU, D, G, Y};
use spof::{get_line, ExpectedLine, ExpectedSize, FileData, Format, Keyword, Occurence};

pub fn ka(f: &FileData) -> Result<String> {
    let el = ExpectedLine::new(
        Keyword::new("Ka", f!("the ambiant color")),
        Format::new("R G B", ExpectedSize::Fixed),
        Occurence::Once,
    );

    Ok(get_line(f, el)?)
}
