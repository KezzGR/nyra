use winit::{
    dpi::PhysicalPosition,
    window::{CursorGrabMode, Window},
};

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
            let size = self.window.inner_size();

            let center = PhysicalPosition::new(size.width / 2, size.height / 2);

            self.window.set_cursor_position(center).unwrap();

            self.window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
        } else {
            self.window.set_cursor_grab(CursorGrabMode::None).unwrap();
        }

        self.state.locked = locked;
        self.state.recapture_on_click = locked;
    }

    pub fn has_focus(&self) -> bool {
        self.window.has_focus()
    }

    pub fn is_locked(&self) -> bool {
        self.state.locked
    }
}
