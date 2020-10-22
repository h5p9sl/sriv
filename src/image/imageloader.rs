use crate::image::LoadedImage;

#[derive(Debug)]
pub enum ImageLoaderError {
    InvalidPath(std::path::PathBuf),
    ImageError(Box<::image::ImageError>),
    IOError(Box<std::io::Error>),
}

impl std::error::Error for ImageLoaderError {}
impl std::fmt::Display for ImageLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::ImageLoaderError::*;
        match self {
            InvalidPath(p) => write!(f, "Invalid Path: \"{}\"", p.to_str().unwrap_or("<???>")),
            ImageError(e) => write!(f, "{:?}", e),
            IOError(e) => write!(f, "{:?}", e),
        }
    }
}

impl From<std::io::Error> for ImageLoaderError {
    fn from(error: std::io::Error) -> Self {
        ImageLoaderError::IOError(Box::new(error))
    }
}

impl From<::image::ImageError> for ImageLoaderError {
    fn from(error: ::image::ImageError) -> Self {
        ImageLoaderError::ImageError(Box::new(error))
    }
}

// TODO: Add documentation here
pub struct ImageLoader {
    images: Vec<std::path::PathBuf>,
    current_image: usize,
}

impl ImageLoader {
    pub fn from_image_path<P: Into<std::path::PathBuf>>(path: P) -> Result<Self, ImageLoaderError> {
        let path: std::path::PathBuf = path.into();

        let mut images = Vec::new();

        if !path.exists() {
            Err(ImageLoaderError::InvalidPath(path))
        } else if path.is_file() {
            images.push(path);
            Ok(ImageLoader {
                images,
                current_image: 0,
            })
        } else if path.is_dir() {
            todo!(); // TODO
            Ok(ImageLoader {
                images,
                current_image: 0,
            })
        } else {
            Err(ImageLoaderError::InvalidPath(path))
        }
    }
}

impl ImageLoader {
    fn load_image<P: Into<std::path::PathBuf>>(path: P) -> Result<LoadedImage, ImageLoaderError> {
        use ::image::{gif::GifDecoder, io::Reader, AnimationDecoder, ImageFormat};

        let reader = Reader::open(path.into())?.with_guessed_format()?;
        let format = reader.format().unwrap();

        match format {
            ImageFormat::Gif => Ok((
                None,
                Some(
                    GifDecoder::new(reader.into_inner())?
                        .into_frames()
                        .collect_frames()?,
                ),
            )),
            _ => Ok((Some(reader.decode()?), None)),
        }
    }
}

impl std::iter::Iterator for ImageLoader {
    type Item = LoadedImage;

    /// Reads the next image into memory
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(path) = self.images.get(self.current_image) {
            self.current_image += 1;
            Some(Self::load_image(path).unwrap())
        } else {
            self.current_image = 0;
            None
        }
    }
}
