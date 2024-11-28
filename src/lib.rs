use image::GenericImageView;
pub use pixel::{Bit, Pixel as BinaryPixel};

mod pixel;
mod utils;

bitflags::bitflags! {
    /// Neighbor constants for 8-connectivity pixel access.
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

#[derive(Debug)]
pub struct BinaryImage {
    data: bit_vec::BitVec,
    height: u32,
    width: u32,
}

impl BinaryImage {
    /// Gets the values of the neighboring pixels (8-connectivity) around the given coordinate.
    ///
    /// # Returns
    ///
    /// An byte representing the state of the neighboring pixels.
    #[inline]
    #[must_use]
    pub fn get_neighbors(&self, x: u32, y: u32) -> Neighbors {
        let mut neighbors = Neighbors::empty();
        if y < u32::MAX && *self.get_pixel(x, y + 1) {
            neighbors |= Neighbors::NORTH;
        }
        if y > u32::MIN && *self.get_pixel(x, y - 1) {
            neighbors |= Neighbors::SOUTH;
        }
        if x < u32::MAX && *self.get_pixel(x + 1, y) {
            neighbors |= Neighbors::EAST;
        }
        if x > u32::MIN && *self.get_pixel(x - 1, y) {
            neighbors |= Neighbors::WEST;
        }
        if x < u32::MAX && y < u32::MAX && *self.get_pixel(x + 1, y + 1) {
            neighbors |= Neighbors::NORTHEAST;
        }
        if x > u32::MIN && y < u32::MAX && *self.get_pixel(x - 1, y + 1) {
            neighbors |= Neighbors::NORTHWEST;
        }
        if x < u32::MAX && y > u32::MIN && *self.get_pixel(x + 1, y - 1) {
            neighbors |= Neighbors::SOUTHEAST;
        }
        if x > u32::MIN && y > u32::MIN && *self.get_pixel(x - 1, y - 1) {
            neighbors |= Neighbors::SOUTHWEST;
        }
        neighbors
    }

    #[inline]
    #[must_use]
    pub fn is_corner(&self, x: u32, y: u32) -> bool {
        *self.get_pixel(x, y) && utils::is_corner(self.get_neighbors(x, y).bits())
    }
}

impl GenericImageView for BinaryImage {
    type Pixel = pixel::Pixel;

    #[inline]
    unsafe fn unsafe_get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        pixel::Pixel::from(Bit::from(
            self.data.get((y * self.width + x) as usize).unwrap(),
        ))
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
        self.data.set((y * self.width + x) as usize, pixel.into());
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
