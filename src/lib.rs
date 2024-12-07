#![doc = include_str!("../README.md")]

use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, From, Into};
use image::{GenericImageView, Pixel};
use num_traits::Zero;
pub use pixel::Bit;

mod pixel;
#[cfg(test)]
mod tests;
mod utils;

bitflags::bitflags! {
    /// Neighbor constants for 8-connectivity pixel access.
    #[repr(transparent)]
    #[derive(
        Clone,
        Copy,
        Debug,
        Display,
        Default,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Deref,
        DerefMut,
        AsMut,
        AsRef,
        Into,
        From,
    )]
    pub struct Neighbors: u8  {
        const NORTH     = 1 << 7;
        const SOUTH     = 1 << 6;
        const EAST      = 1 << 5;
        const WEST      = 1 << 4;
        const NORTHEAST = 1 << 3;
        const NORTHWEST = 1 << 2;
        const SOUTHEAST = 1 << 1;
        const SOUTHWEST = 1;
    }
}

impl Neighbors {
    pub fn get_neighbors<I>(image: &I, x: u32, y: u32) -> Self
    where
        I: GenericImageView<Pixel = Bit>,
    {
        let mut neighbors = Neighbors::empty();
        if y < u32::MAX && *image.get_pixel(x, y + 1) {
            neighbors |= Neighbors::NORTH;
        }
        if y > u32::MIN && *image.get_pixel(x, y - 1) {
            neighbors |= Neighbors::SOUTH;
        }
        if x < u32::MAX && *image.get_pixel(x + 1, y) {
            neighbors |= Neighbors::EAST;
        }
        if x > u32::MIN && *image.get_pixel(x - 1, y) {
            neighbors |= Neighbors::WEST;
        }
        if x < u32::MAX && y < u32::MAX && *image.get_pixel(x + 1, y + 1) {
            neighbors |= Neighbors::NORTHEAST;
        }
        if x > u32::MIN && y < u32::MAX && *image.get_pixel(x - 1, y + 1) {
            neighbors |= Neighbors::NORTHWEST;
        }
        if x < u32::MAX && y > u32::MIN && *image.get_pixel(x + 1, y - 1) {
            neighbors |= Neighbors::SOUTHEAST;
        }
        if x > u32::MIN && y > u32::MIN && *image.get_pixel(x - 1, y - 1) {
            neighbors |= Neighbors::SOUTHWEST;
        }
        neighbors
    }

    #[inline]
    #[must_use]
    pub fn is_corner<I>(image: &I, x: u32, y: u32) -> bool
    where
        I: GenericImageView<Pixel = Bit>,
    {
        *image.get_pixel(x, y) && utils::is_corner(Self::get_neighbors(image, x, y).bits())
    }
}

#[derive(Debug, Clone)]
pub struct BinaryImage {
    data: bit_vec::BitVec,
    height: u32,
    width: u32,
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

#[derive(Debug, Clone, DerefMut, Deref)]
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
        pixel::Bit::from(channels ^ alpha_exist | alphas)
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

#[derive(Debug, Clone)]
pub struct BinaryRawView<'a, T>
where
    T: num_traits::Zero,
{
    data: &'a [T],
    height: u32,
    width: u32,
}

impl<'a, T> BinaryRawView<'a, T>
where
    T: num_traits::Zero,
{
    pub fn new(height: u32, width: u32, data: &'a [T]) -> Self {
        Self {
            data,
            height,
            width,
        }
    }
}

impl<'a, T> GenericImageView for BinaryRawView<'a, T>
where
    T: num_traits::Zero,
{
    type Pixel = pixel::Bit;
    #[inline]
    unsafe fn unsafe_get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        let size = self.data.len() / (self.height * self.width) as usize;
        let start = (y * self.width + x) as usize;
        let end = start + size;
        pixel::Bit::from(
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
