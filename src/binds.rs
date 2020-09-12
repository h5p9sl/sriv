use glutin::event::VirtualKeyCode as VK;
type Bind = Option<VK>;

pub enum Action {
    Quit,
    ZoomIn,
    ZoomOut,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    RotateLeft,
    RotateRight,
}

pub struct Binds {
    pub quit: Bind,
    pub zoom_in: Bind,
    pub zoom_out: Bind,
    pub move_up: Bind,
    pub move_down: Bind,
    pub move_left: Bind,
    pub move_right: Bind,
    pub rotate_right: Bind,
    pub rotate_left: Bind,
}

impl Default for Binds {
    fn default() -> Binds {
        Binds {
            quit: Some(VK::Q),
            zoom_in: Some(VK::Add),
            zoom_out: Some(VK::Subtract),
            move_down: Some(VK::J),
            move_up: Some(VK::K),
            move_left: Some(VK::H),
            move_right: Some(VK::L),
            rotate_right: Some(VK::Comma),
            rotate_left: Some(VK::Period),
        }
    }
}

impl Binds {
    pub fn get_action(&self, input: VK) -> Option<Action> {
        if Some(input) == self.quit {
            return Some(Action::Quit);
        } else if Some(input) == self.zoom_in {
            return Some(Action::ZoomIn);
        } else if Some(input) == self.zoom_out {
            return Some(Action::ZoomOut);
        } else if Some(input) == self.move_up {
            return Some(Action::MoveUp);
        } else if Some(input) == self.move_down {
            return Some(Action::MoveDown);
        } else if Some(input) == self.move_left {
            return Some(Action::MoveLeft);
        } else if Some(input) == self.move_right {
            return Some(Action::MoveRight);
        } else if Some(input) == self.rotate_right {
            return Some(Action::RotateRight);
        } else if Some(input) == self.rotate_left {
            return Some(Action::RotateLeft);
        }
        None
    }
}
