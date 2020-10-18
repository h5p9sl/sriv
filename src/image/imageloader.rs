use crate::image::LoadedImage;
use core::slice::Iter;

#[derive(Debug)]
pub enum ImageLoaderError {
    InvalidPath(String),
    ImageError(::image::ImageError),
    IOError(std::io::Error),
}

impl std::error::Error for ImageLoaderError {}
impl std::fmt::Display for ImageLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!();
    }
}

impl From<std::io::Error> for ImageLoaderError {
    fn from(error: std::io::Error) -> Self {
        ImageLoaderError::IOError(error)
    }
}

impl From<::image::ImageError> for ImageLoaderError {
    fn from(error: ::image::ImageError) -> Self {
        ImageLoaderError::ImageError(error)
    }
}

// TODO: Add documentation here
pub struct ImageLoader<'a> {
    images: Iter<'a, std::path::PathBuf>,
}

impl ImageLoader<'_> {
    pub fn from_image_path<P: Into<std::path::PathBuf>>(path: P) -> Result<Self, ImageLoaderError> {
        let path: std::path::PathBuf = path.into();

        assert!(
            path.exists(),
            "{} does not exist.",
            path.to_str().unwrap_or("?")
        );

        let mut images = Vec::new();

        if path.is_file() {
            images.push(path.to_owned());
            Ok(ImageLoader {
                images: images.iter(),
            })
        } else if path.is_dir() {
            // TODO
            unimplemented!();
            Ok(ImageLoader {
                images: images.iter(),
            })
        } else {
            panic!("Invalid path: {}", path.to_str().unwrap_or("?"));
        }
    }
}

impl ImageLoader<'_> {
    fn load_image<P: Into<std::path::PathBuf>>(path: P) -> Result<LoadedImage, ImageLoaderError> {
        use ::image::{io::Reader, ImageFormat, gif::GifDecoder, AnimationDecoder};

        let reader = Reader::open(path.into())?.with_guessed_format()?;
        let format = reader.format().unwrap();

        match format {
            ImageFormat::Gif => Ok((
                None,
                Some(GifDecoder::new(reader.into_inner())?.into_frames().collect_frames()?),
            )),
            _ => Ok((Some(reader.decode()?), None)),
        }
    }
}

impl std::iter::Iterator for ImageLoader<'_> {
    type Item = LoadedImage;

    /// Reads the next image into memory
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(path) = self.images.next() {
            Some(Self::load_image(path).unwrap())
        } else {
            None
        }
    }
}
