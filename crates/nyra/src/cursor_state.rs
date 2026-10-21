use winit::window::{CursorGrabMode, Window};

pub struct CursorState {
    pub visibility: bool,
    pub locked: bool,
    pub recapture_on_click: bool,
}

impl Default for CursorState {
    fn default() -> Self {
        Self {
            visibility: true,
            locked: false,
            recapture_on_click: false,
        }
    }
}

impl CursorState {
    pub fn on_escape(&mut self, window: &Window) {
        if !self.locked {
            return;
        }

        self.visibility = true;
        self.locked = false;

        window.set_cursor_visible(self.visibility);
        window.set_cursor_grab(CursorGrabMode::None).unwrap();
    }

    pub fn on_click(&mut self, window: &Window) {
        if !self.recapture_on_click || self.locked {
            return;
        }

        self.visibility = false;
        self.locked = true;

        window.set_cursor_visible(self.visibility);
        window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
    }
}
