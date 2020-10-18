pub mod image;
pub mod imageloader;
pub mod imagequad;

pub use self::image::Image;
pub use self::imageloader::ImageLoader;
pub use self::imagequad::ImageQuad;

type LoadedImage = (Option<::image::DynamicImage>, Option<Vec<::image::Frame>>);
