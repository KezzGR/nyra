use winit::window::{CursorGrabMode, Window};

use crate::CursorState;

pub struct Cursor<'a> {
    window: &'a Window,
    state: &'a mut CursorState,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(window: &'a Window, state: &'a mut CursorState) -> Self {
        Self { window, state }
    }

    pub fn set_visible(&mut self, visibility: bool) {
        self.window.set_cursor_visible(visibility);

        self.state.visibility = visibility;
    }

    pub fn set_locked(&mut self, locked: bool) {
        if locked {
            self.window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
        } else {
            self.window.set_cursor_grab(CursorGrabMode::None).unwrap();
        }

        self.state.locked = locked;
        self.state.recapture_on_click = locked;
    }
}
