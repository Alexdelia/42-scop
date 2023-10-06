pub struct Speed(pub f32, pub bool);

impl Default for Speed {
    fn default() -> Self {
        Self(0.002, true)
    }
}

impl Speed {
    pub fn get(&self) -> f32 {
        if self.1 {
            self.0
        } else {
            0.0
        }
    }

    pub fn inc(&mut self) {
        self.0 *= 2.0;
    }

    pub fn dec(&mut self) {
        self.0 /= 2.0;
    }

    pub fn pause(&mut self) {
        self.1 = !self.1;
    }

    pub fn revert(&mut self) {
        self.0 = -self.0;
    }
}
