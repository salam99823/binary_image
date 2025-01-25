#![allow(clippy::module_name_repetitions)]
use std::ops::Deref;

use image::{GenericImageView, Pixel};

use crate::BinaryImage;

#[derive(Debug, Clone, Copy)]
pub enum BinaryView<'a, I: GenericImageView> {
    Ref(&'a I),
    Image(I),
}

impl<I, P> GenericImageView for BinaryView<'_, I>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel,
    crate::Bit: From<P>,
{
    type Pixel = crate::Bit;
    #[inline]
    unsafe fn unsafe_get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        crate::Bit::from(self.deref().unsafe_get_pixel(x, y))
    }
    #[inline]
    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        debug_assert!(self.deref().in_bounds(x, y), "Pixel out of bounds");
        unsafe { self.unsafe_get_pixel(x, y) }
    }
    #[inline]
    fn dimensions(&self) -> (u32, u32) {
        self.deref().dimensions()
    }
    #[inline]
    fn height(&self) -> u32 {
        self.deref().height()
    }
    #[inline]
    fn width(&self) -> u32 {
        self.deref().width()
    }
}

impl<I, P> Deref for BinaryView<'_, I>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel,
    crate::Bit: From<P>,
{
    type Target = I;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(image) => image,
            Self::Image(image) => image,
        }
    }
}

impl<'a, I, P> From<BinaryView<'a, I>> for crate::BinaryImage
where
    I: GenericImageView<Pixel = P>,
    P: Pixel,
    crate::Bit: From<P>,
{
    fn from(view: BinaryView<'a, I>) -> BinaryImage {
        BinaryImage {
            height: view.height(),
            width: view.width(),
            buffer: view.pixels().map(|(_, _, pixel)| *pixel).collect(),
        }
    }
}
