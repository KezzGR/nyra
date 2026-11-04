use std::time::Instant;

#[derive(Default)]
pub struct Time {
    last_frame: Option<Instant>,
}

impl Time {
    pub fn delta_time(&mut self) -> f32 {
        let now = Instant::now();

        let delta = match self.last_frame {
            Some(last) => (now - last).as_secs_f32(),
            None => 0.0,
        };

        self.last_frame = Some(now);

        delta
    }

    pub fn reset(&mut self) {
        self.last_frame = None;
    }
}
