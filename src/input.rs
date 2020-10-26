use crate::{
    binds::{Action, Binds},
    image::ImageQuad,
};
use glutin::{event::KeyboardInput, event_loop::ControlFlow};
use vek::mat::repr_c::column_major::mat4::Mat4;

pub struct Input {
    binds: Binds,
}

impl Input {
    pub fn new(binds: &Binds) -> Input {
        Input {
            binds: binds.clone(),
        }
    }

    pub fn perform_action(action: &Action, matrix: &mut Mat4<f32>, control_flow: &mut ControlFlow) {
        use std::f32::consts::PI;
        const ZOOM_FACTOR: f32 = 0.5;
        const MOVE_FACTOR: f32 = 0.5;
        match action {
            Action::MoveDown => matrix.translate_2d([0.0, MOVE_FACTOR]),
            Action::MoveLeft => matrix.translate_2d([MOVE_FACTOR, 0.0]),
            Action::MoveRight => matrix.translate_2d([-MOVE_FACTOR, 0.0]),
            Action::MoveUp => matrix.translate_2d([0.0, -MOVE_FACTOR]),
            Action::Quit => *control_flow = ControlFlow::Exit,
            Action::Reset => *matrix = Default::default(),
            Action::RotateLeft => matrix.rotate_z(PI / 2.0),
            Action::RotateRight => matrix.rotate_z(-PI / 2.0),
            Action::ZoomIn => matrix.scale_3d([1.0 / ZOOM_FACTOR, 1.0 / ZOOM_FACTOR, 1.0]),
            Action::ZoomOut => matrix.scale_3d([ZOOM_FACTOR, ZOOM_FACTOR, 1.0]),
            _ => {}
        }
    }

    pub fn handle_char(
        &self,
        character: char,
        image_quad: &mut ImageQuad,
        control_flow: &mut ControlFlow,
    ) -> bool {
        if let Some(action) = self.binds.get_action_char(character) {
            Self::perform_action(&action, image_quad.matrix_mut(), control_flow);
            true
        } else {
            false
        }
    }

    pub fn handle(
        &self,
        event: &KeyboardInput,
        image_quad: &mut ImageQuad,
        control_flow: &mut ControlFlow,
    ) -> bool {
        use glutin::event::ElementState::Pressed;
        if event.state == Pressed {
            if let Some(vk) = event.virtual_keycode {
                if let Some(action) = self.binds.get_action(vk) {
                    Self::perform_action(&action, image_quad.matrix_mut(), control_flow);
                    return true;
                }
            }
        }
        false
    }
}
