#![doc = include_str!("../README.md")]

use image::GenericImageView;

pub use neigbors::Neighbors;
pub use pixel::Bit;
pub use view::{raw::BinaryRawView, BinaryView};

mod neigbors;
mod pixel;
#[cfg(test)]
mod tests;
mod view;

#[derive(Debug, Clone)]
pub struct BinaryImage {
    height: u32,
    width: u32,
    data: bit_vec::BitVec,
}

impl BinaryImage {
    #[must_use]
    pub fn from_raw<T>(height: u32, width: u32, data: &[T]) -> Self
    where
        T: num_traits::Zero,
    {
        debug_assert!(
            data.len() >= (height * width) as usize,
            "Data must not be smaller than image dimensions"
        );
        let compress_step = data.len() / (height * width) as usize;
        Self {
            data: data
                .chunks(compress_step)
                .map(|pixel| pixel.iter().any(|channel| !channel.is_zero()))
                .collect(),
            height,
            width,
        }
    }
}

impl GenericImageView for BinaryImage {
    type Pixel = pixel::Bit;
    #[inline]
    unsafe fn unsafe_get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        Bit::from(self.data.get_unchecked((y * self.width + x) as usize))
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
        self.data.set((y * self.width + x) as usize, *pixel);
    }
    #[inline]
    fn put_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        debug_assert!(self.in_bounds(x, y));
        unsafe { self.unsafe_put_pixel(x, y, pixel) }
    }
    fn blend_pixel(&mut self, _: u32, _: u32, _: Self::Pixel) {
        unimplemented!()
    }
    fn get_pixel_mut(&mut self, _: u32, _: u32) -> &mut Self::Pixel {
        unimplemented!()
    }
}

impl<I: GenericImageView<Pixel = Bit>> From<&I> for BinaryImage {
    fn from(view: &I) -> Self {
        BinaryImage {
            height: view.height(),
            width: view.width(),
            data: view.pixels().map(|(_, _, pixel)| *pixel).collect(),
        }
    }
}
