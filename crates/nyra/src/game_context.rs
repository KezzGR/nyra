use crate::{Input, render::Camera};

pub struct GameContext<'a> {
    pub input: &'a Input,
    pub camera: &'a mut Camera,
    pub delta_time: f32,
}
