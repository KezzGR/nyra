use crate::{Input, render::Camera};

pub struct GameContext<'a> {
    pub input: &'a Input,
    pub delta_time: &'a f32,
    pub camera: &'a mut Camera,
}
