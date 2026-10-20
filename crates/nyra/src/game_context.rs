use crate::{Input, Mouse, render::Camera};

pub struct GameContext<'a> {
    pub input: &'a Input,
    pub mouse: &'a Mouse,
    pub camera: &'a mut Camera,
    pub delta_time: f32,
}
