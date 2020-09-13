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

        Ok(Window { display: display })
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
        use std::ops::Deref;
        match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(size) => self.display.gl_window().deref().deref().resize(size),
            _ => {}
        }
    }

    pub fn request_redraw(&mut self) {
        use std::ops::Deref;
        self.display
            .gl_window()
            .deref()
            .deref()
            .window()
            .request_redraw();
    }

    pub fn display(&self) -> &Display {
        &self.display
    }
}
