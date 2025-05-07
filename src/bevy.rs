use crate::BinaryImage;

use bevy_image::prelude::Image as BevyImage;
use bevy_render::render_resource::TextureFormat;
use derive_more::derive::{Display, Error};
use image::{GrayAlphaImage, ImageBuffer, LumaA, Rgba, RgbaImage};

impl TryFrom<BevyImage> for BinaryImage {
    type Error = IntoBinaryImageError;
    fn try_from(image: BevyImage) -> Result<BinaryImage, Self::Error> {
        let (width, height): (u32, u32) = (image.width(), image.height());
        match image.texture_descriptor.format {
            TextureFormat::Rg8Unorm => {
                let image: Option<GrayAlphaImage> = image
                    .data
                    .and_then(|data| ImageBuffer::from_raw(width, height, data));
                image.map(Self::from)
            }
            TextureFormat::Rgba8UnormSrgb
            | TextureFormat::Bgra8UnormSrgb
            | TextureFormat::Bgra8Unorm => {
                let image: Option<RgbaImage> = image
                    .data
                    .and_then(|data| ImageBuffer::from_raw(width, height, data));
                image.map(Self::from)
            }
            TextureFormat::R8Unorm => image.data.map(|data| Self::from_raw(width, height, &data)),
            // Throw and error if conversion isn't supported
            texture_format => return Err(IntoBinaryImageError::UnsupportedFormat(texture_format)),
        }
        .ok_or(IntoBinaryImageError::UnknownConversionError(
            image.texture_descriptor.format,
        ))
    }
}

impl TryFrom<&BevyImage> for BinaryImage {
    type Error = IntoBinaryImageError;
    fn try_from(image: &BevyImage) -> Result<BinaryImage, Self::Error> {
        let (width, height, data): (u32, u32, Option<&[u8]>) =
            (image.width(), image.height(), image.data.as_deref());

        match image.texture_descriptor.format {
            TextureFormat::Rg8Unorm => {
                let image: Option<ImageBuffer<LumaA<u8>, &[u8]>> =
                    data.and_then(|data| ImageBuffer::from_raw(width, height, data));
                image.map(Self::from)
            }
            TextureFormat::Rgba8UnormSrgb
            | TextureFormat::Bgra8UnormSrgb
            | TextureFormat::Bgra8Unorm => {
                let image: Option<ImageBuffer<Rgba<u8>, &[u8]>> =
                    data.and_then(|data| ImageBuffer::from_raw(width, height, data));
                image.map(Self::from)
            }
            TextureFormat::R8Unorm => data.map(|data| Self::from_raw(width, height, data)),
            // Throw and error if conversion isn't supported
            texture_format => return Err(IntoBinaryImageError::UnsupportedFormat(texture_format)),
        }
        .ok_or(IntoBinaryImageError::UnknownConversionError(
            image.texture_descriptor.format,
        ))
    }
}

#[non_exhaustive]
#[derive(Error, Display, Debug)]
pub enum IntoBinaryImageError {
    /// Conversion into binary image not supported for source format.
    #[display("Conversion into binary image not supported for {_0:?}.")]
    #[error(ignore)]
    UnsupportedFormat(TextureFormat),

    /// Encountered an unknown error during conversion.
    #[display("Failed to convert into {_0:?}.")]
    #[error(ignore)]
    UnknownConversionError(TextureFormat),
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_image::{
        prelude::Image as BevyImage, CompressedImageFormats, ImageFormat, ImageSampler, ImageType,
    };
    use bevy_render::render_asset::RenderAssetUsages;

    #[test]
    fn test_conversion_rgba() {
        let bevy_image = BevyImage::from_buffer(
            include_bytes!("../assets/car.png"), // buffer
            ImageType::Format(
                ImageFormat::from_image_crate_format(image::ImageFormat::Png)
                    .expect("PNG is unsupported"),
            ),
            CompressedImageFormats::default(),
            true,
            ImageSampler::default(),
            RenderAssetUsages::default(),
        )
        .unwrap();

        let binary_image: BinaryImage =
            BinaryImage::try_from(&bevy_image).expect("Conversion failed");

        assert_eq!(binary_image.width, bevy_image.width());
        assert_eq!(binary_image.height, bevy_image.height());
        // Additional checks on pixel data can be added here
    }
}
