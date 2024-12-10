#![doc = include_str!("../README.md")]

use bit_vec::BitVec;
use image::{GenericImage, GenericImageView};

pub use neigbors::Neighbors;
pub use pixel::Bit;
pub use view::BinaryView;

mod neigbors;
mod pixel;
#[cfg(test)]
mod tests;
mod view;

#[derive(Debug, Clone, Default)]
pub struct BinaryImage {
    width: u32,
    height: u32,
    buf: bit_vec::BitVec,
}

impl BinaryImage {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buf: BitVec::with_capacity((width * height) as usize),
        }
    }

    #[must_use]
    pub fn from_raw<T>(width: u32, height: u32, buf: &[T]) -> Self
    where
        T: num_traits::Zero,
    {
        let image_size = (width * height) as usize;
        debug_assert!(
            buf.len() >= image_size,
            "Buffer must not be smaller than image dimensions"
        );
        let compress_step = buf.len() / image_size;
        Self {
            buf: buf
                .chunks(compress_step)
                .map(|pixel| !pixel.iter().any(num_traits::Zero::is_zero))
                .collect(),
            height,
            width,
        }
    }

    #[must_use]
    pub fn from_bitvec(width: u32, height: u32, buf: bit_vec::BitVec) -> Self {
        let image_size = (width * height) as usize;
        debug_assert!(
            buf.len() >= image_size,
            "Buffer must not be smaller than image dimensions"
        );
        Self { width, height, buf }
    }
}

impl image::GenericImageView for BinaryImage {
    type Pixel = pixel::Bit;
    #[inline]
    unsafe fn unsafe_get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        Bit::from(self.buf.get_unchecked((y * self.width + x) as usize))
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
        self.buf.set((y * self.width + x) as usize, *pixel);
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

impl<I: image::GenericImageView<Pixel = Bit>> From<&I> for BinaryImage {
    fn from(view: &I) -> Self {
        BinaryImage {
            height: view.height(),
            width: view.width(),
            buf: view.pixels().map(|(_, _, pixel)| *pixel).collect(),
        }
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
    P: image::Pixel,
    Bit: From<P>,
{
    fn from(image: image::ImageBuffer<P, Container>) -> Self {
        let mut new = Self::new(image.width(), image.height());
        for x in 0..image.width() {
            for y in 0..image.height() {
                new.put_pixel(x, y, Bit::from(*image.get_pixel(x, y)));
            }
        }
        new
    }
}
