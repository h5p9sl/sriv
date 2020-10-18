use glium::texture;
use image::{gif::GifDecoder, io::Reader, AnimationDecoder, DynamicImage, Frame, ImageFormat};
use std::path::PathBuf;

pub struct Image {
    pub frames: Option<Vec<Frame>>,
    pub image: Option<DynamicImage>,
}

impl Image {
    pub fn new<F>(fp: F) -> Option<Image>
    where
        F: Into<PathBuf>,
    {
        let fp = fp.into();
        let reader = Reader::open(&fp)
            .expect("Failed to open file")
            .with_guessed_format()
            .expect("Unable to open file for parsing");

        let format = reader.format().expect("File doesn't have valid format");
        match format {
            ImageFormat::Gif => Self::load_into_frames(
                GifDecoder::new(reader.into_inner()).expect("Failed to create GifDecoder"),
            ),
            _ => Some(Image {
                image: reader.decode().ok(),
                frames: None,
            }),
        }
    }

    pub fn is_animated(&self) -> bool {
        self.frames.is_some()
    }

    fn load_into_frames<'a, D>(decoder: D) -> Option<Image>
    where
        D: AnimationDecoder<'a>,
    {
        let frames = decoder.into_frames().collect_frames();
        Some(Image {
            image: None,
            frames: frames.ok(),
        })
    }
}

#[deprecated]
pub fn texture_from_dynamic_image<F>(
    display: &F,
    image: &DynamicImage,
) -> Option<texture::SrgbTexture2d>
where
    F: glium::backend::Facade,
{
    use image::GenericImageView;

    enum RawImageData {
        Rgb8(Vec<u8>),
        Rgba8(Vec<u8>),
        Rgb16(Vec<u16>),
        Rgba16(Vec<u16>),
    };

    let raw_image_data = match image {
        DynamicImage::ImageLuma8(image) => RawImageData::Rgb8(image.to_vec()),
        DynamicImage::ImageLumaA8(image) => RawImageData::Rgba8(image.to_vec()),
        DynamicImage::ImageRgb8(image) => RawImageData::Rgb8(image.to_vec()),
        DynamicImage::ImageRgba8(image) => RawImageData::Rgba8(image.to_vec()),
        DynamicImage::ImageLuma16(image) => RawImageData::Rgba16(image.to_vec()),
        DynamicImage::ImageLumaA16(image) => RawImageData::Rgba16(image.to_vec()),
        DynamicImage::ImageRgb16(image) => RawImageData::Rgb16(image.to_vec()),
        DynamicImage::ImageRgba16(image) => RawImageData::Rgba16(image.to_vec()),
        _ => RawImageData::Rgba8(image.to_rgba().into_raw()),
    };

    let raw_image_data: (
        Option<texture::RawImage2d<u8>>,
        Option<texture::RawImage2d<u16>>,
    ) = match raw_image_data {
        RawImageData::Rgb8(vec) => (
            Some(texture::RawImage2d::from_raw_rgb(vec, image.dimensions())),
            None,
        ),
        RawImageData::Rgba8(vec) => (
            Some(texture::RawImage2d::from_raw_rgba(vec, image.dimensions())),
            None,
        ),
        RawImageData::Rgb16(vec) => (
            None,
            Some(texture::RawImage2d::from_raw_rgb(vec, image.dimensions())),
        ),
        RawImageData::Rgba16(vec) => (
            None,
            Some(texture::RawImage2d::from_raw_rgba(vec, image.dimensions())),
        ),
    };

    if let Some(data) = raw_image_data.0 {
        return texture::SrgbTexture2d::new(display, data).ok();
    }
    texture::SrgbTexture2d::new(display, raw_image_data.1.unwrap()).ok()
}
