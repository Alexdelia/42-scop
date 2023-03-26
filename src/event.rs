use crate::env::Env;

use yahmrslib::hmerr::Result;

impl Env {
    pub fn event(&self) -> Result<bool> {
        Ok(true)
    }
}
