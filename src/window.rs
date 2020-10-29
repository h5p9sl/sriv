use clap::crate_name;
use glium::{glutin::ContextBuilder, Display, Frame, SwapBuffersError};
use glutin::{
    dpi,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, GlRequest,
};

pub struct Window {
    display: Display,
}

impl Window {
    pub fn new(el: &EventLoop<()>) -> Result<Window, String> {
        let wb = WindowBuilder::new()
            .with_title(crate_name!())
            .with_inner_size(dpi::LogicalSize::new(800.0, 600.0));

        let cb = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_gl_profile(glutin::GlProfile::Compatibility)
            .with_srgb(true);

        let display = glium::Display::new(wb, cb, el).unwrap();

        Ok(Window { display })
    }

    pub fn draw<F>(&mut self, f: F) -> Result<(), SwapBuffersError>
    where
        F: Fn(&mut Frame),
    {
        let mut frame = self.display.draw();
        f(&mut frame);
        frame.finish()
    }

    pub fn handle(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        if let WindowEvent::CloseRequested = event {
            *control_flow = ControlFlow::Exit;
        }
    }

    pub fn request_redraw(&mut self) {
        use std::borrow::BorrowMut;
        self.display
            .gl_window()
            .borrow_mut()
            .borrow_mut()
            .window()
            .request_redraw();
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn toggle_fullscreen(&mut self) {
        use std::borrow::BorrowMut;
        let glw = &mut self.display.gl_window();
        let wc = glw.borrow_mut().borrow_mut();
        wc.window().set_fullscreen({
            if wc.window().fullscreen().is_some() {
                None
            } else {
                use ::glutin::window::Fullscreen;
                Some(Fullscreen::Borderless(None))
            }
        });
    }
}
