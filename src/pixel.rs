#![allow(clippy::suspicious_arithmetic_impl, clippy::module_name_repetitions)]
use num_traits::{One, Zero};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Pixel(pub [Bit; 1]);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Bit(bool);

impl std::ops::AddAssign for Bit {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for Bit {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::MulAssign for Bit {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign for Bit {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::Add for Bit {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.0 |= *rhs;
        self
    }
}

impl std::ops::Sub for Bit {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.0 ^= *rhs;
        self
    }
}

impl std::ops::Mul for Bit {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self.0 &= *rhs;
        self
    }
}

impl std::ops::Div for Bit {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        debug_assert!(*rhs, "Cannot divide by zero");
        self.0 &= *rhs;
        self
    }
}

impl std::ops::Rem for Bit {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        debug_assert!(*rhs, "Cannot divide by zero");
        Self(false)
    }
}

impl std::ops::Not for Bit {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        self.0 = !*self;
        self
    }
}

impl Zero for Bit {
    fn is_zero(&self) -> bool {
        !**self
    }
    fn set_zero(&mut self) {
        self.0 = false;
    }
    fn zero() -> Self {
        Self(false)
    }
}

impl One for Bit {
    fn is_one(&self) -> bool {
        **self
    }
    fn set_one(&mut self) {
        self.0 = true;
    }
    fn one() -> Self {
        Self(true)
    }
}

impl num_traits::Num for Bit {
    type FromStrRadixErr = ();
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if str.chars().all(|ch| ch.is_digit(radix)) {
            Ok(Self(str.chars().all(|ch| !matches!(ch, '0'))))
        } else {
            Err(())
        }
    }
}

impl num_traits::NumCast for Bit {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        n.to_usize()
            .map(|n| Self(n != 0))
            .or_else(|| n.to_isize().map(|n| Self(n != 0)))
    }
}

impl num_traits::ToPrimitive for Bit {
    #[allow(clippy::cast_possible_wrap)]
    fn to_i64(&self) -> Option<i64> {
        Some(self.0.into())
    }

    fn to_u64(&self) -> Option<u64> {
        Some(self.0.into())
    }
}

impl num_traits::Bounded for Bit {
    fn min_value() -> Self {
        Self(false)
    }
    fn max_value() -> Self {
        Self(true)
    }
}

impl image::Primitive for Bit {
    const DEFAULT_MAX_VALUE: Self = Self(true);
    const DEFAULT_MIN_VALUE: Self = Self(false);
}

impl image::Pixel for Pixel {
    type Subpixel = Bit;
    const CHANNEL_COUNT: u8 = 1;
    const COLOR_MODEL: &'static str = "BLACKANDWHITE";
    fn channels(&self) -> &[Self::Subpixel] {
        self.as_ref()
    }
    fn channels_mut(&mut self) -> &mut [Self::Subpixel] {
        self.as_mut()
    }
    fn from_slice(slice: &[Self::Subpixel]) -> &Self {
        assert_eq!(slice.len(), 1);
        unsafe { &*slice.as_ptr().cast() }
    }
    fn from_slice_mut(slice: &mut [Self::Subpixel]) -> &mut Self {
        assert_eq!(slice.len(), 1);
        unsafe { &mut *slice.as_mut_ptr().cast() }
    }
    fn to_rgb(&self) -> image::Rgb<Bit> {
        image::Rgb([if **self { One::one() } else { Zero::zero() }; 3])
    }
    fn to_rgba(&self) -> image::Rgba<Bit> {
        image::Rgba([if **self { One::one() } else { Zero::zero() }; 4])
    }
    fn to_luma(&self) -> image::Luma<Self::Subpixel> {
        image::Luma([if **self { One::one() } else { Zero::zero() }])
    }
    fn to_luma_alpha(&self) -> image::LumaA<Self::Subpixel> {
        image::LumaA([if **self { One::one() } else { Zero::zero() }; 2])
    }
    fn map<F>(&self, f: F) -> Self
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        let mut this = *self;
        this.apply(f);
        this
    }
    fn map_with_alpha<F, G>(&self, f: F, _: G) -> Self
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
        G: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        let mut this = *self;
        this.apply(f);
        this
    }
    fn apply<F>(&mut self, mut f: F)
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        self.0[0] = f(self.0[0]);
    }
    fn apply_with_alpha<F, G>(&mut self, f: F, _: G)
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
        G: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        self.apply(f);
    }
    fn map2<F>(&self, other: &Self, f: F) -> Self
    where
        F: FnMut(Self::Subpixel, Self::Subpixel) -> Self::Subpixel,
    {
        let mut this = *self;
        this.apply2(other, f);
        this
    }
    fn apply2<F>(&mut self, other: &Self, mut f: F)
    where
        F: FnMut(Self::Subpixel, Self::Subpixel) -> Self::Subpixel,
    {
        self.0[0] = f(self.0[0], other.0[0]);
    }
    fn invert(&mut self) {
        self.0[0] = !self.0[0];
    }
    fn blend(&mut self, other: &Self) {
        self.0[0] -= other.0[0];
    }
    fn channels4(
        &self,
    ) -> (
        Self::Subpixel,
        Self::Subpixel,
        Self::Subpixel,
        Self::Subpixel,
    ) {
        (self.0[0], self.0[0], self.0[0], self.0[0])
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
        self.apply(f);
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

impl std::ops::Deref for Bit {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for Pixel {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0[0]
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<Bit> for Pixel {
    fn from(value: Bit) -> Self {
        Self([value])
    }
}

impl From<Bit> for bool {
    fn from(value: Bit) -> Self {
        *value
    }
}

impl From<Pixel> for bool {
    fn from(value: Pixel) -> Self {
        *value
    }
}

impl AsRef<[Bit]> for Pixel {
    fn as_ref(&self) -> &[Bit] {
        &self.0
    }
}

impl AsMut<[Bit]> for Pixel {
    fn as_mut(&mut self) -> &mut [Bit] {
        &mut self.0
    }
}
