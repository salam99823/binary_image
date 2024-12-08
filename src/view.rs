#![allow(clippy::module_name_repetitions)]
use derive_more::{Constructor, Deref, DerefMut, From};

#[derive(Debug, Clone, DerefMut, Deref, From, Constructor)]
pub struct BinaryView<'a, I: image::GenericImageView>(pub &'a I);

impl<'a, I, P> image::GenericImageView for BinaryView<'a, I>
where
    I: image::GenericImageView<Pixel = P>,
    P: image::Pixel,
    crate::Bit: From<P>,
{
    type Pixel = crate::Bit;
    #[inline]
    unsafe fn unsafe_get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        crate::Bit::from(self.0.unsafe_get_pixel(x, y))
    }
    #[inline]
    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        debug_assert!(self.0.in_bounds(x, y), "Pixel out of bounds");
        unsafe { self.unsafe_get_pixel(x, y) }
    }
    #[inline]
    fn dimensions(&self) -> (u32, u32) {
        self.0.dimensions()
    }
    #[inline]
    fn height(&self) -> u32 {
        self.0.height()
    }
    #[inline]
    fn width(&self) -> u32 {
        self.0.width()
    }
}
