use glutin::event::VirtualKeyCode as VK;

#[derive(Copy, Clone)]
pub enum Action {
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    Quit,
    Reset,
    RotateLeft,
    RotateRight,
    ZoomIn,
    ZoomOut,
}

#[derive(Clone)]
pub struct Binds {
    binds: Vec<(VK, Action)>,
}

impl Default for Binds {
    fn default() -> Binds {
        Binds {
            binds: vec![
                (VK::Q, Action::Quit),
                (VK::Add, Action::ZoomIn),
                (VK::Subtract, Action::ZoomOut),
                (VK::J, Action::MoveDown),
                (VK::K, Action::MoveUp),
                (VK::H, Action::MoveLeft),
                (VK::L, Action::MoveRight),
                (VK::Comma, Action::RotateLeft),
                (VK::Period, Action::RotateRight),
                (VK::Equals, Action::Reset),
            ],
        }
    }
}

impl Binds {
    pub fn get_action(&self, input: VK) -> Option<Action> {
        for action in &self.binds {
            if input == action.0 {
                return Some(action.1);
            }
        }
        None
    }
}
