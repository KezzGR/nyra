use std::time::Instant;

pub struct Time {
    last_frame: Instant,
}

impl Time {
    pub fn delta_time(&mut self) -> f32 {
        let now = Instant::now();
        let delta_time = (now - self.last_frame).as_secs_f32();
        self.last_frame = now;

        delta_time
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            last_frame: Instant::now(),
        }
    }
}
