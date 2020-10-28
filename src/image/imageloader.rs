use crate::image::LoadedImage;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ImageLoaderError {
    IOError(Box<std::io::Error>),
    ImageError(Box<::image::ImageError>),
    InvalidPath(std::path::PathBuf),
    NoImageFound,
}

impl std::error::Error for ImageLoaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::ImageLoaderError::*;
        match self {
            IOError(e) => Some(e),
            ImageError(e) => Some(e),
            _ => None,
        }
    }
}
impl std::fmt::Display for ImageLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::ImageLoaderError::*;
        match self {
            IOError(e) => write!(f, "{:?}", e),
            ImageError(e) => write!(f, "{:?}", e),
            InvalidPath(p) => write!(f, "Invalid Path: \"{}\"", p.to_str().unwrap_or("<???>")),
            NoImageFound => write!(f, "No valid images found"),
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
    current_image: Option<usize>,
}

impl ImageLoader {
    fn is_image(path: &PathBuf) -> Result<bool, ::image::ImageError> {
        use ::image::io::Reader;
        Ok(Reader::open(path)?.format().is_some())
    }

    fn find_images(path: &PathBuf, recurse: bool) -> Option<Vec<PathBuf>> {
        let mut images = Vec::new();

        assert_eq!(path.is_dir(), true);

        let directory = path.read_dir();

        if let Ok(directory) = directory {
            for entry in directory {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let t = entry.file_type().unwrap();

                    if t.is_dir() && recurse {
                        debug!("Recursing subirectory {:?}", &path);
                        if let Some(mut i) = Self::find_images(&path, true) {
                            images.append(&mut i);
                        }
                    } else if t.is_file() && Self::is_image(&path).unwrap_or(false) {
                        debug!("Found image: {:?}", &path);
                        images.push(path);
                    }
                } else {
                    warn!("{}", entry.err().unwrap());
                }
            }
            Some(images)
        } else {
            warn!("{}: {}", path.to_str().unwrap(), directory.err().unwrap());
            None
        }
    }

    pub fn from_paths<P>(paths: &[P], recurse: bool) -> Result<Self, ImageLoaderError>
    where
        P: Into<PathBuf> + AsRef<std::ffi::OsStr>,
    {
        let mut images = Vec::new();

        for path in paths.iter() {
            let path: PathBuf = PathBuf::from(&path);

            if !path.exists() {
                warn!("{}", ImageLoaderError::InvalidPath(path));
            } else if path.is_file() {
                images.push(path);
            } else if path.is_dir() {
                if let Some(mut i) = Self::find_images(&path, recurse) {
                    images.append(&mut i);
                } else {
                    warn!("No images found in {}", path.to_str().unwrap());
                }
            } else {
                warn!("{}", ImageLoaderError::InvalidPath(path));
            }
        }

        if images.is_empty() {
            Err(ImageLoaderError::NoImageFound)
        } else {
            Ok(Self {
                images,
                current_image: None,
            })
        }
    }
}

impl ImageLoader {
    fn load_image<P: Into<std::path::PathBuf> + AsRef<std::ffi::OsStr>>(
        path: &P,
    ) -> Result<LoadedImage, ImageLoaderError> {
        use ::image::{gif::GifDecoder, io::Reader, AnimationDecoder, ImageFormat};

        let path: std::path::PathBuf = path.into();
        let reader = Reader::open(path)?.with_guessed_format()?;
        let format = reader.format();

        match format {
            Some(ImageFormat::Gif) => Ok((
                None,
                Some(
                    GifDecoder::new(reader.into_inner())?
                        .into_frames()
                        .collect_frames()?,
                ),
            )),
            Some(_) | None => Ok((Some(reader.decode()?), None)),
        }
    }

    /// Move the iterator by `dir` amount
    /// Returns `None` if no more images are available.
    fn iterate(&mut self, dir: isize) -> Option<Result<LoadedImage, ImageLoaderError>> {
        let next: usize = self
            .current_image
            .and_then(|current| {
                use std::convert::TryInto;
                (current as isize).saturating_add(dir).try_into().ok()
            })
            .unwrap_or(0);

        if next < self.images.len() {
            self.current_image = Some(next);
            info!("Image Index: {}", next);
            let image_path = self.images.get(next).unwrap();
            let loadedimage = Self::load_image(&image_path);
            if let Ok(image) = loadedimage {
                Some(Ok(image))
            } else {
                use std::error::Error;
                warn!(
                    "{}: {}",
                    image_path.to_str().unwrap(),
                    loadedimage.as_ref().err().unwrap().source().unwrap()
                );
                Some(Err(loadedimage.err().unwrap()))
            }
        } else {
            None
        }
    }
}

impl std::iter::Iterator for ImageLoader {
    type Item = LoadedImage;

    /// Reads the next valid image into memory
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(image) = self.iterate(1) {
            if let Ok(image) = image {
                return Some(image);
            }
        }
        None
    }
}

impl std::iter::DoubleEndedIterator for ImageLoader {
    /// Reads the previous image into memory
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(image) = self.iterate(-1) {
            if let Ok(image) = image {
                return Some(image);
            }
        }
        None
    }
}
