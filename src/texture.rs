use glium::texture;
use image::DynamicImage;
use std::path::Path;

pub fn dynamic_image_from_path<P>(fp: P) -> Option<DynamicImage>
where
    P: AsRef<Path>,
{
    let fp = fp.as_ref();
    let di = image::open(&fp);
    if let Some(e) = di.as_ref().err() {
        eprintln!(
            "Failed to open image \"{}\": {}...",
            fp.to_str().unwrap_or("null"),
            e
        );
    }
    di.ok()
}

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

    let raw_image_data: (Option<texture::RawImage2d::<u8>>, Option<texture::RawImage2d::<u16>>) = match raw_image_data {
        RawImageData::Rgb8(vec) => (Some(texture::RawImage2d::from_raw_rgb(vec, image.dimensions())), None),
        RawImageData::Rgba8(vec) => (Some(texture::RawImage2d::from_raw_rgba(vec, image.dimensions())), None),
        RawImageData::Rgb16(vec) => (None, Some(texture::RawImage2d::from_raw_rgb(vec, image.dimensions()))),
        RawImageData::Rgba16(vec) => (None, Some(texture::RawImage2d::from_raw_rgba(vec, image.dimensions()))),
    };

    if let Some(data) = raw_image_data.0 {
        return texture::SrgbTexture2d::new(display, data).ok();
    }
    texture::SrgbTexture2d::new(display, raw_image_data.1.unwrap()).ok()
}

/// Wrapper for dynamic_image_from_path and texture_from_dynamic_image
pub fn _texture_from_path<F, P>(display: &F, fp: P) -> Option<texture::SrgbTexture2d>
where
    F: glium::backend::Facade,
    P: AsRef<Path>,
{
    if let Some(image) = dynamic_image_from_path(fp) {
        return texture_from_dynamic_image(display, &image);
    }
    None
}
