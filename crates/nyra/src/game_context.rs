use crate::{Cursor, Input, Mouse, render::Camera};

pub struct GameContext<'a> {
    pub input: &'a Input,
    pub mouse: &'a Mouse,
    pub cursor: Cursor<'a>,
    pub camera: &'a mut Camera,
    pub delta_time: f32,
}
