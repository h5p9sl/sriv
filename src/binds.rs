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
    binds: Vec<(Option<char>, Option<VK>, Action)>,
}

impl Default for Binds {
    fn default() -> Binds {
        Binds {
            binds: vec![
                (None, Some(VK::Add), Action::ZoomIn),
                (None, Some(VK::Equals), Action::Reset),
                (None, Some(VK::H), Action::MoveLeft),
                (None, Some(VK::J), Action::MoveDown),
                (None, Some(VK::K), Action::MoveUp),
                (None, Some(VK::L), Action::MoveRight),
                (None, Some(VK::Q), Action::Quit),
                (None, Some(VK::Subtract), Action::ZoomOut),
                (Some('<'), None, Action::RotateLeft),
                (Some('>'), None, Action::RotateRight),
            ],
        }
    }
}

impl Binds {
    pub fn get_action_char(&self, input: char) -> Option<Action> {
        for action in &self.binds {
            if Some(input) == action.0 {
                return Some(action.2);
            }
        }
        None
    }

    pub fn get_action(&self, input: VK) -> Option<Action> {
        for action in &self.binds {
            if Some(input) == action.1 {
                return Some(action.2);
            }
        }
        None
    }
}
