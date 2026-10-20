use glam::Quat;

use crate::GameContext;

pub struct FreeFlyController {
    sensitivity: f32,
    move_speed: f32,
    yaw: f32,
    pitch: f32,
}

impl Default for FreeFlyController {
    fn default() -> Self {
        Self {
            sensitivity: 0.003,
            move_speed: 4.0,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

impl FreeFlyController {
    pub fn update(&mut self, game: &mut GameContext<'_>) {
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
}
