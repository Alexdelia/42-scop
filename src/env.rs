use crate::setting::Setting;

pub struct Env {
    pub setting: Setting,
}

impl Env {
    pub fn new() -> Self {
        Self {
            setting: Setting::default(),
        }
    }
}
