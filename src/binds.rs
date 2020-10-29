use glutin::event::VirtualKeyCode as VK;

#[derive(Copy, Clone)]
pub enum Action {
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    NextImage,
    PrevImage,
    Quit,
    Reset,
    RotateLeft,
    RotateRight,
    ZoomIn,
    ZoomOut,
    ToggleFullscreen,
}

#[derive(Clone)]
pub struct Binds {
    binds: Vec<(Option<char>, Option<VK>, Action)>,
}

impl Default for Binds {
    fn default() -> Binds {
        Binds {
            binds: vec![
                (Some('='), None, Action::Reset),
                (Some('h'), None, Action::MoveLeft),
                (Some('j'), None, Action::MoveDown),
                (Some('k'), None, Action::MoveUp),
                (Some('l'), None, Action::MoveRight),
                (Some('q'), None, Action::Quit),
                (Some('+'), None, Action::ZoomIn),
                (Some('-'), None, Action::ZoomOut),
                (Some('f'), None, Action::ToggleFullscreen),
                (Some('n'), None, Action::NextImage),
                (Some('p'), None, Action::PrevImage),
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
