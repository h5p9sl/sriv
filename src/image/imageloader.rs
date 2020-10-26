use crate::image::LoadedImage;
use std::path::PathBuf;

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
    fn is_image(path: &PathBuf) -> bool {
        use ::image::io::Reader;
        Reader::open(path).unwrap().format().is_some()
    }

    fn find_images(path: &PathBuf, recurse: bool) -> Vec<PathBuf> {
        let mut images = Vec::new();

        assert_eq!(path.is_dir(), true);

        for entry in path.read_dir().unwrap() {
            if let Ok(entry) = entry {
                let t = entry.file_type().unwrap();
                let path = entry.path();

                if t.is_dir() && recurse {
                    debug!("Recursing subirectory {:?}", &path);
                    images.append(&mut Self::find_images(&path, true));
                } else if t.is_file() && Self::is_image(&path) {
                    debug!("Found image: {:?}", &path);
                    images.push(path);
                }
            } else {
                error!("{:?}", entry.err());
            }
        }
        images
    }

    pub fn from_paths<P>(paths: &[P], recurse: bool) -> Result<Self, ImageLoaderError>
    where
        P: Into<PathBuf> + AsRef<std::ffi::OsStr>,
    {
        let mut images = Vec::new();

        for path in paths.iter() {
            let path: PathBuf = PathBuf::from(&path);

            if !path.exists() {
                return Err(ImageLoaderError::InvalidPath(path));
            } else if path.is_file() {
                images.push(path);
            } else if path.is_dir() {
                images.append(&mut Self::find_images(&path, recurse));
            } else {
                return Err(ImageLoaderError::InvalidPath(path));
            }
        }

        Ok(Self {
            images,
            current_image: 0usize,
        })
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
