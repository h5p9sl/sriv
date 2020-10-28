use crate::image::{ImageQuad, LoadedImage};
use ::image::Frame;

pub struct Image {
    quad: Option<ImageQuad>,
    frames: Vec<Frame>,
    current_frame: usize,
    last_frame_instant: std::time::Instant,
}

impl From<LoadedImage> for Image {
    fn from(image: LoadedImage) -> Self {
        let frames = {
            if let Some(frames) = image.1 {
                Some(frames)
            } else if let Some(dynimg) = image.0 {
                let mut v = Vec::new();
                v.push(::image::Frame::new(dynimg.to_rgba()));
                Some(v)
            } else {
                None
            }
        }
        .unwrap();
        Image {
            quad: None,
            frames,
            current_frame: 0,
            last_frame_instant: std::time::Instant::now(),
        }
    }
}

impl Image {
    pub fn draw<S: glium::Surface>(&self, surface: &mut S) -> Result<(), glium::DrawError> {
        self.quad
            .as_ref()
            .expect("Quad is not yet initialized")
            .draw(surface)
    }

    pub fn generate_quad<F: glium::backend::Facade>(&mut self, display: &F) {
        use glium::texture::{RawImage2d, SrgbTexture2d};
        let frame = self
            .frames
            .get(self.current_frame)
            .expect("No current frame")
            .buffer();
        let dimensions = frame.dimensions();
        let raw_image = RawImage2d::from_raw_rgba(frame.as_raw().to_owned(), dimensions);
        let texture = SrgbTexture2d::new(display, raw_image);
        self.quad = Some(ImageQuad::new(display, texture.unwrap()));
    }

    pub fn time_next_frame(&mut self) -> Option<std::time::Instant> {
        if let Some(next) = &self.frames.iter().peekable().peek() {
            use std::time::Duration;
            let delay = next.delay().numer_denom_ms();
            self.last_frame_instant
                .checked_add(Duration::from_millis((delay.0 / delay.1).into()))
        } else {
            None
        }
    }

    pub fn next_frame<F: glium::backend::Facade>(&mut self, display: &F) {
        self.last_frame_instant = std::time::Instant::now();
        // Next frame
        self.current_frame += 1;
        if self.current_frame > self.frames.len() {
            self.current_frame = 0;
        }
        // Update imagequad
        if let Some(frame) = self.frames.get(self.current_frame) {
            self.quad
                .as_mut()
                .expect("Cannot set next frame: no quad")
                .set_image(frame.buffer(), display);
        }
    }

    pub fn quad(&mut self) -> Option<&mut self::ImageQuad> {
        self.quad.as_mut()
    }
}
