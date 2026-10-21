use glam::Quat;

use crate::GameContext;

pub struct FreeFlyController {
    move_speed: f32,
    sensitivity: f32,
    yaw: f32,
    pitch: f32,

    initialized: bool,
}

impl Default for FreeFlyController {
    fn default() -> Self {
        Self {
            move_speed: 4.0,
            sensitivity: 0.003,
            yaw: 0.0,
            pitch: 0.0,
            initialized: true,
        }
    }
}

impl FreeFlyController {
    pub fn new(move_speed: f32, sensitivity: f32) -> Self {
        Self {
            move_speed,
            sensitivity,
            yaw: 0.0,
            pitch: 0.0,
            initialized: true,
        }
    }

    pub fn update(&mut self, game: &mut GameContext<'_>) {
        if self.initialized {
            self.setup(game);
            self.initialized = false;
        }

        self.yaw += game.mouse.delta.x * self.sensitivity;
        self.pitch -= game.mouse.delta.y * self.sensitivity;

        self.pitch = self
            .pitch
            .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

        game.camera.transform.rotation =
            Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch);

        game.camera
            .transform
            .translate_local(game.input.wasd_direction_3d() * game.delta_time * self.move_speed);
    }

    fn setup(&mut self, game: &mut GameContext<'_>) {
        game.cursor.set_locked(true);
        game.cursor.set_visible(false);
    }
}
