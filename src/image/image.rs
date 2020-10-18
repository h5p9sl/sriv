use crate::image::{ImageQuad, LoadedImage};
use ::image::Frame;

pub struct Image<'a> {
    quad: Option<ImageQuad>,
    frames: std::iter::Peekable<std::slice::Iter<'a, Frame>>,
    last_frame_instant: std::time::Instant,
}

impl From<LoadedImage> for Image<'_> {
    fn from(_image: LoadedImage) -> Self {
        unimplemented!();
    }
}

impl Image<'_> {
    pub fn draw<S: glium::Surface>(&self, surface: &mut S) -> Result<(), glium::DrawError> {
        self.quad
            .as_ref()
            .expect("Quad is not yet initialized")
            .draw(surface)
    }

    pub fn time_next_frame(&mut self) -> Option<std::time::Instant> {
        if let Some(next) = &self.frames.peek() {
            use std::time::Duration;
            let delay = next.delay().numer_denom_ms();
            self.last_frame_instant
                .checked_add(Duration::from_millis((delay.0 / delay.1).into()))
        } else {
            None
        }
    }

    pub fn next_frame(&mut self) -> Option<&Frame> {
        self.last_frame_instant = std::time::Instant::now();
        self.frames.next()
    }

    pub fn quad(&mut self) -> Option<&mut self::ImageQuad> {
        self.quad.as_mut()
    }
}