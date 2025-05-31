#![doc = include_str!("../README.md")]

use bit_vec::BitVec;
use image::{GenericImage, GenericImageView, Pixel};

pub use pixel::Bit;
pub use view::BinaryView;

#[cfg(feature = "bevy")]
pub mod bevy;
mod pixel;
#[cfg(test)]
mod tests;
mod view;

#[derive(Debug, Clone, Default)]
pub struct BinaryImage {
    width: u32,
    height: u32,
    buffer: bit_vec::BitVec,
}

impl BinaryImage {
    #[inline]
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: BitVec::from_elem((width * height) as usize, false),
        }
    }

    #[must_use]
    pub fn from_raw<T>(width: u32, height: u32, buffer: &[T]) -> Self
    where
        T: num_traits::Zero,
    {
        let image_size = (width * height) as usize;
        debug_assert!(
            buffer.len() >= image_size,
            "Buffer must not be smaller than image dimensions"
        );
        let compress_step = buffer.len() / image_size;
        Self {
            buffer: buffer
                .chunks(compress_step)
                .map(|pixel| !pixel.iter().any(num_traits::Zero::is_zero))
                .collect(),
            height,
            width,
        }
    }

    #[must_use]
    pub fn from_bitvec(width: u32, height: u32, buffer: bit_vec::BitVec) -> Self {
        let image_size = (width * height) as usize;
        debug_assert!(
            buffer.len() >= image_size,
            "Buffer must not be smaller than image dimensions"
        );
        Self {
            width,
            height,
            buffer,
        }
    }

    /// Flip an image horizontally
    #[must_use]
    pub fn flip_horizontal(&self) -> Self {
        let (width, height) = self.dimensions();
        let mut out = Self::new(width, height);
        for y in 0..height {
            for x in 0..width {
                out.put_pixel(width - x - 1, y, self.get_pixel(x, y));
            }
        }
        out
    }

    /// Flip an image vertically
    #[must_use]
    pub fn flip_vertical(&self) -> Self {
        let (width, height) = self.dimensions();
        let mut out = Self::new(width, height);
        for y in 0..height {
            for x in 0..width {
                out.put_pixel(x, height - 1 - y, self.get_pixel(x, y));
            }
        }
        out
    }

    #[inline]
    #[must_use]
    pub fn get_pixel(&self, x: u32, y: u32) -> Bit {
        GenericImageView::get_pixel(self, x, y)
    }

    #[inline]
    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }
    #[inline]
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }
}

impl GenericImageView for BinaryImage {
    type Pixel = Bit;
    #[inline]
    unsafe fn unsafe_get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        Bit::from(self.buffer.get_unchecked((y * self.width + x) as usize))
    }
    #[inline]
    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        debug_assert!(self.in_bounds(x, y));
        unsafe { self.unsafe_get_pixel(x, y) }
    }
    #[inline]
    fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
    #[inline]
    fn height(&self) -> u32 {
        self.height
    }
    #[inline]
    fn width(&self) -> u32 {
        self.width
    }
}

impl image::GenericImage for BinaryImage {
    #[inline]
    unsafe fn unsafe_put_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        self.buffer.set((y * self.width + x) as usize, *pixel);
    }
    #[inline]
    fn put_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        debug_assert!(self.in_bounds(x, y));
        unsafe { self.unsafe_put_pixel(x, y, pixel) }
    }
    fn blend_pixel(&mut self, x: u32, y: u32, other: Self::Pixel) {
        let mut pixel = self.get_pixel(x, y);
        pixel.blend(&other);
        self.put_pixel(x, y, pixel);
    }
    fn get_pixel_mut(&mut self, _: u32, _: u32) -> &mut Self::Pixel {
        unimplemented!()
    }
}

impl From<image::DynamicImage> for BinaryImage {
    fn from(image: image::DynamicImage) -> Self {
        match image {
            image::DynamicImage::ImageRgb8(image) => Self::from(image),
            image::DynamicImage::ImageLuma8(image) => Self::from(image),
            image::DynamicImage::ImageRgba8(image) => Self::from(image),
            image::DynamicImage::ImageRgb16(image) => Self::from(image),
            image::DynamicImage::ImageLumaA8(image) => Self::from(image),
            image::DynamicImage::ImageLuma16(image) => Self::from(image),
            image::DynamicImage::ImageRgba16(image) => Self::from(image),
            image::DynamicImage::ImageRgb32F(image) => Self::from(image),
            image::DynamicImage::ImageLumaA16(image) => Self::from(image),
            image::DynamicImage::ImageRgba32F(image) => Self::from(image),
            _ => unimplemented!(),
        }
    }
}

impl<Container, P> From<image::ImageBuffer<P, Container>> for BinaryImage
where
    Container: std::ops::Deref<Target = [P::Subpixel]>,
    P: Pixel,
    Bit: From<P>,
{
    fn from(image: image::ImageBuffer<P, Container>) -> Self {
        let buffer = image.pixels().map(|pixel| Bit::from(*pixel).0).collect();
        BinaryImage::from_bitvec(image.width(), image.height(), buffer)
    }
}
