#![allow(clippy::module_name_repetitions)]
use image::GenericImageView;

use crate::pixel::Bit;

#[derive(Debug, Clone)]
pub struct BinaryRawView<'a, T>
where
    T: num_traits::Zero,
{
    height: u32,
    width: u32,
    data: &'a [T],
}

impl<'a, T> BinaryRawView<'a, T>
where
    T: num_traits::Zero,
{
    #[must_use]
    pub fn new(height: u32, width: u32, data: &'a [T]) -> Self {
        debug_assert!(
            data.len() >= (height * width) as usize,
            "Data must not be smaller than image dimensions"
        );
        Self {
            height,
            width,
            data,
        }
    }
}

impl<'a, T> GenericImageView for BinaryRawView<'a, T>
where
    T: num_traits::Zero,
{
    type Pixel = Bit;
    #[inline]
    unsafe fn unsafe_get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        let size = self.data.len() / (self.height * self.width) as usize;
        let start = (y * self.width + x) as usize;
        let end = start + size;
        Bit::from(
            self.data
                .get_unchecked(start..end)
                .iter()
                .any(|channel| !channel.is_zero()),
        )
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
