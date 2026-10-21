use crate::{Cursor, Input, Mouse, render::Camera, world::World};

pub struct GameContext<'a> {
    pub input: &'a Input,
    pub mouse: &'a Mouse,
    pub cursor: Cursor<'a>,
    pub camera: &'a mut Camera,
    pub world: &'a mut World,
    pub delta_time: f32,
}
