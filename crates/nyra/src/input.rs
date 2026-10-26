use std::collections::HashSet;

use nyra_math::Vec3;
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
pub struct Input {
    pressed_keys: HashSet<KeyCode>,
    holding_keys: HashSet<KeyCode>,
    released_keys: HashSet<KeyCode>,
}

impl Input {
    pub fn handle_keyboard_input(&mut self, key_event: &KeyEvent) {
        let PhysicalKey::Code(key) = key_event.physical_key else {
            return;
        };

        match key_event.state {
            ElementState::Pressed => {
                if self.holding_keys.insert(key) {
                    self.pressed_keys.insert(key);
                }
            }

            ElementState::Released => {
                if self.holding_keys.remove(&key) {
                    self.released_keys.insert(key);
                }
            }
        }
    }

    pub fn key_down(&self, key: KeyCode) -> bool {
        self.holding_keys.contains(&key)
    }

    pub fn key_just_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn key_just_released(&self, key: KeyCode) -> bool {
        self.released_keys.contains(&key)
    }

    pub fn end_frame(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();
    }

    pub fn clear_inputs(&mut self) {
        self.pressed_keys.clear();
        self.holding_keys.clear();
        self.released_keys.clear();
    }

    pub fn wasd_direction_3d(&self) -> Vec3 {
        let mut direction = Vec3::ZERO;

        if self.key_down(KeyCode::KeyW) {
            direction += Vec3::Z;
        }
        if self.key_down(KeyCode::KeyA) {
            direction += Vec3::NEG_X;
        }
        if self.key_down(KeyCode::KeyS) {
            direction += Vec3::NEG_Z;
        }
        if self.key_down(KeyCode::KeyD) {
            direction += Vec3::X;
        }

        direction.normalize_or_zero()
    }
}
