#![allow(clippy::module_name_repetitions)]
use derive_more::{Constructor, Deref, DerefMut, From};
use image::{GenericImageView, Pixel};
use num_traits::Zero;

use crate::pixel::Bit;

pub mod raw;

#[derive(Debug, Clone, DerefMut, Deref, From, Constructor)]
pub struct BinaryView<'a, I>(pub &'a I)
where
    I: GenericImageView;

impl<'a, I> GenericImageView for BinaryView<'a, I>
where
    I: GenericImageView,
{
    type Pixel = Bit;
    #[inline]
    unsafe fn unsafe_get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        let pixel = self.0.unsafe_get_pixel(x, y);
        let (mut channels, mut alphas, mut alpha_exist) = (false, false, false);
        pixel.map_with_alpha(
            |channel| {
                channels |= !channel.is_zero();
                channel
            },
            |alpha| {
                alpha_exist = true;
                alphas |= !alpha.is_zero();
                alpha
            },
        );
        Bit::from(channels ^ !alpha_exist | alphas)
    }
    #[inline]
    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        debug_assert!(self.0.in_bounds(x, y));
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
