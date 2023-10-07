type SpeedPrecision = f32;

pub struct Speed(pub SpeedPrecision, pub bool);

const CHANGE_RATIO: SpeedPrecision = 1.5;

impl Default for Speed {
    fn default() -> Self {
        Self(0.002, true)
    }
}

impl Speed {
    pub fn get(&self) -> SpeedPrecision {
        if self.1 {
            self.0
        } else {
            0.0
        }
    }

    pub fn inc(&mut self) {
        self.0 *= CHANGE_RATIO;
    }

    pub fn dec(&mut self) {
        self.0 /= CHANGE_RATIO;
    }

    pub fn pause(&mut self) {
        self.1 = !self.1;
    }

    pub fn revert(&mut self) {
        self.0 = -self.0;
    }
}
