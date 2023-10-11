pub enum RotationType {
    Clockwise,
    CounterClockwise,
    None,
}

pub struct RotateAxis {
    pub x: RotationType,
    pub y: RotationType,
    pub z: RotationType,
}

impl RotationType {
    pub fn update(&self, angle: f32, speed: f32) -> f32 {
        match self {
            RotationType::Clockwise => (angle + speed) % 360.,
            RotationType::CounterClockwise => (angle - speed) % 360.,
            RotationType::None => angle,
        }
    }
}
