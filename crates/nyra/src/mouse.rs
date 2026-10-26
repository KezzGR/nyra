use nyra_math::Vec2;

#[derive(Default)]
pub struct Mouse {
    pub delta: Vec2,
}

impl Mouse {
    pub fn save_delta(&mut self, delta: Vec2) {
        self.delta += delta;
    }

    pub fn end_frame(&mut self) {
        self.delta = Vec2::ZERO;
    }
}
