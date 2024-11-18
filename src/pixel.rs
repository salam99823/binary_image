use image::Primitive;
use num_traits::Zero;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct BinaryPixel(pub [BinarySubPixel; 1]);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BinarySubPixel(bool);

impl std::ops::Add for BinarySubPixel {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.0 |= rhs.0;
        self
    }
}

impl std::ops::AddAssign for BinarySubPixel {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for BinarySubPixel {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self, rhs);
        Self(false)
    }
}

impl std::ops::Mul for BinarySubPixel {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self.0 &= rhs.0;
        self
    }
}

impl std::ops::Div for BinarySubPixel {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        self.0 ^= !rhs.0;
        self
    }
}

impl std::ops::Rem for BinarySubPixel {
    type Output = Self;
    fn rem(mut self, rhs: Self) -> Self::Output {
        self.0 ^= rhs.0;
        self
    }
}

impl std::ops::Not for BinarySubPixel {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        self.0 = !self.0;
        self
    }
}

impl num_traits::Zero for BinarySubPixel {
    fn is_zero(&self) -> bool {
        !self.0
    }
    fn set_zero(&mut self) {
        self.0 = false;
    }
    fn zero() -> Self {
        Self(false)
    }
}

impl num_traits::One for BinarySubPixel {
    fn is_one(&self) -> bool {
        self.0
    }
    fn set_one(&mut self) {
        self.0 = true;
    }
    fn one() -> Self {
        Self(true)
    }
}

impl num_traits::Num for BinarySubPixel {
    type FromStrRadixErr = ();
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if str.chars().all(|ch| ch.is_digit(radix)) {
            for ch in str.chars() {
                if !matches!(ch, '0') {
                    return Ok(Self(true));
                }
            }
            Ok(Self(false))
        } else {
            Err(())
        }
    }
}

impl num_traits::NumCast for BinarySubPixel {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Some(if let Some(n) = n.to_isize() {
            Self(n != 0)
        } else if let Some(n) = n.to_usize() {
            Self(n > 0)
        } else {
            return None;
        })
    }
}

impl num_traits::ToPrimitive for BinarySubPixel {
    fn to_i64(&self) -> Option<i64> {
        self.to_u64().map(|n| n as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        if self.0 {
            Some(1)
        } else {
            Some(0)
        }
    }
}

impl num_traits::Bounded for BinarySubPixel {
    fn min_value() -> Self {
        Self(false)
    }
    fn max_value() -> Self {
        Self(true)
    }
}

impl Primitive for BinarySubPixel {
    const DEFAULT_MAX_VALUE: Self = Self(true);
    const DEFAULT_MIN_VALUE: Self = Self(false);
}

impl image::Pixel for BinaryPixel {
    type Subpixel = BinarySubPixel;
    const CHANNEL_COUNT: u8 = 1;
    const COLOR_MODEL: &'static str = "BLACKANDWHITE";

    #[inline(always)]
    fn channels(&self) -> &[Self::Subpixel] {
        &self.0
    }

    #[inline(always)]
    fn channels_mut(&mut self) -> &mut [Self::Subpixel] {
        &mut self.0
    }

    fn from_slice(slice: &[Self::Subpixel]) -> &Self {
        assert_eq!(slice.len(), 1);
        unsafe { &*(slice.as_ptr() as *const Self) }
    }

    fn from_slice_mut(slice: &mut [Self::Subpixel]) -> &mut Self {
        assert_eq!(slice.len(), 1);
        unsafe { &mut *(slice.as_mut_ptr() as *mut Self) }
    }

    fn to_rgb(&self) -> image::Rgb<BinarySubPixel> {
        image::Rgb([Zero::zero(), Zero::zero(), Zero::zero()])
    }

    fn to_rgba(&self) -> image::Rgba<BinarySubPixel> {
        image::Rgba([Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero()])
    }

    fn to_luma(&self) -> image::Luma<Self::Subpixel> {
        image::Luma([Zero::zero()])
    }

    fn to_luma_alpha(&self) -> image::LumaA<Self::Subpixel> {
        image::LumaA([Zero::zero(), Zero::zero()])
    }

    fn map<F>(&self, f: F) -> Self
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        let mut this = (*self).clone();
        this.apply(f);
        this
    }

    fn apply<F>(&mut self, mut f: F)
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        for v in &mut self.0 {
            *v = f(*v)
        }
    }

    fn map_with_alpha<F, G>(&self, f: F, _: G) -> Self
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
        G: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        let mut this = (*self).clone();
        this.apply(f);
        this
    }

    fn apply_with_alpha<F, G>(&mut self, mut f: F, _: G)
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
        G: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        for v in &mut self.0 {
            *v = f(*v)
        }
    }

    fn map2<F>(&self, other: &Self, f: F) -> Self
    where
        F: FnMut(Self::Subpixel, Self::Subpixel) -> Self::Subpixel,
    {
        let mut this = (*self).clone();
        this.apply2(other, f);
        this
    }

    fn apply2<F>(&mut self, other: &Self, mut f: F)
    where
        F: FnMut(Self::Subpixel, Self::Subpixel) -> Self::Subpixel,
    {
        for (a, &b) in self.0.iter_mut().zip(other.0.iter()) {
            *a = f(*a, b)
        }
    }

    fn invert(&mut self) {
        self.0[0] = !self.0[0];
    }

    fn blend(&mut self, other: &Self) {
        self.0[0] += other.0[0];
    }

    fn channels4(
        &self,
    ) -> (
        Self::Subpixel,
        Self::Subpixel,
        Self::Subpixel,
        Self::Subpixel,
    ) {
        let mut channels = [Self::Subpixel::DEFAULT_MAX_VALUE; 4];
        channels[0..1].copy_from_slice(&self.0);
        (channels[0], channels[1], channels[2], channels[3])
    }

    fn map_without_alpha<F>(&self, f: F) -> Self
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        let mut new = *self;
        new.apply_without_alpha(f);
        new
    }

    fn apply_without_alpha<F>(&mut self, f: F)
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        self.apply(f)
    }

    fn from_channels(
        a: Self::Subpixel,
        b: Self::Subpixel,
        c: Self::Subpixel,
        d: Self::Subpixel,
    ) -> Self {
        Self([a + b + c + d])
    }
}
