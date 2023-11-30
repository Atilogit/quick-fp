use cfg_if::cfg_if;

use crate::Float;

impl<F, T> std::ops::Add<T> for Float<F>
where
    F: Copy + num_traits::NumCast,
    T: num_traits::NumCast,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: Self = num_traits::cast(rhs).unwrap();
        cfg_if! {
            if #[cfg(feature = "nightly")] {
                Self(unsafe { std::intrinsics::fadd_fast(self.0, rhs.0) })
            } else {
                Self(self.0 + rhs.0)
            }
        }
    }
}

macro_rules! add_rhs_impl {
    ($ty:ty) => {
        impl<F> std::ops::Add<Float<F>> for $ty
        where
            F: Copy,
            F: num_traits::NumCast,
        {
            type Output = Float<F>;

            fn add(self, rhs: Float<F>) -> Self::Output {
                let lhs: Float<F> = num_traits::cast(self).unwrap();
                lhs + rhs
            }
        }
    };
}

add_rhs_impl!(f32);
add_rhs_impl!(f64);
add_rhs_impl!(i8);
add_rhs_impl!(i16);
add_rhs_impl!(i32);
add_rhs_impl!(i64);
add_rhs_impl!(i128);
add_rhs_impl!(isize);
add_rhs_impl!(u8);
add_rhs_impl!(u16);
add_rhs_impl!(u32);
add_rhs_impl!(u64);
add_rhs_impl!(u128);
add_rhs_impl!(usize);

impl<F, T> std::ops::Sub<T> for Float<F>
where
    F: Copy + num_traits::NumCast,
    T: num_traits::NumCast,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Self = num_traits::cast(rhs).unwrap();
        cfg_if! {
            if #[cfg(feature = "nightly")] {
                Self(unsafe { std::intrinsics::fsub_fast(self.0, rhs.0) })
            } else {
                Self(self.0 - rhs.0)
            }
        }
    }
}

macro_rules! sub_rhs_impl {
    ($ty:ty) => {
        impl<F> std::ops::Sub<Float<F>> for $ty
        where
            F: Copy,
            F: num_traits::NumCast,
        {
            type Output = Float<F>;

            fn sub(self, rhs: Float<F>) -> Self::Output {
                let lhs: Float<F> = num_traits::cast(self).unwrap();
                lhs - rhs
            }
        }
    };
}

sub_rhs_impl!(f32);
sub_rhs_impl!(f64);
sub_rhs_impl!(i8);
sub_rhs_impl!(i16);
sub_rhs_impl!(i32);
sub_rhs_impl!(i64);
sub_rhs_impl!(i128);
sub_rhs_impl!(isize);
sub_rhs_impl!(u8);
sub_rhs_impl!(u16);
sub_rhs_impl!(u32);
sub_rhs_impl!(u64);
sub_rhs_impl!(u128);
sub_rhs_impl!(usize);

impl<F, T> std::ops::Mul<T> for Float<F>
where
    F: Copy + num_traits::NumCast,
    T: num_traits::NumCast,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: Self = num_traits::cast(rhs).unwrap();
        cfg_if! {
            if #[cfg(feature = "nightly")] {
                Self(unsafe { std::intrinsics::fmul_fast(self.0, rhs.0) })
            } else {
                Self(self.0 * rhs.0)
            }
        }
    }
}

macro_rules! mul_rhs_impl {
    ($ty:ty) => {
        impl<F> std::ops::Mul<Float<F>> for $ty
        where
            F: Copy,
            F: num_traits::NumCast,
        {
            type Output = Float<F>;

            fn mul(self, rhs: Float<F>) -> Self::Output {
                let lhs: Float<F> = num_traits::cast(self).unwrap();
                lhs * rhs
            }
        }
    };
}

mul_rhs_impl!(f32);
mul_rhs_impl!(f64);
mul_rhs_impl!(i8);
mul_rhs_impl!(i16);
mul_rhs_impl!(i32);
mul_rhs_impl!(i64);
mul_rhs_impl!(i128);
mul_rhs_impl!(isize);
mul_rhs_impl!(u8);
mul_rhs_impl!(u16);
mul_rhs_impl!(u32);
mul_rhs_impl!(u64);
mul_rhs_impl!(u128);
mul_rhs_impl!(usize);

impl<F, T> std::ops::Div<T> for Float<F>
where
    F: Copy + num_traits::NumCast,
    T: num_traits::NumCast,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: Self = num_traits::cast(rhs).unwrap();
        cfg_if! {
            if #[cfg(feature = "nightly")] {
                Self(unsafe { std::intrinsics::fdiv_fast(self.0, rhs.0) })
            } else {
                Self(self.0 / rhs.0)
            }
        }
    }
}

macro_rules! div_rhs_impl {
    ($ty:ty) => {
        impl<F> std::ops::Div<Float<F>> for $ty
        where
            F: Copy,
            F: num_traits::NumCast,
        {
            type Output = Float<F>;

            fn div(self, rhs: Float<F>) -> Self::Output {
                let lhs: Float<F> = num_traits::cast(self).unwrap();
                lhs / rhs
            }
        }
    };
}

div_rhs_impl!(f32);
div_rhs_impl!(f64);
div_rhs_impl!(i8);
div_rhs_impl!(i16);
div_rhs_impl!(i32);
div_rhs_impl!(i64);
div_rhs_impl!(i128);
div_rhs_impl!(isize);
div_rhs_impl!(u8);
div_rhs_impl!(u16);
div_rhs_impl!(u32);
div_rhs_impl!(u64);
div_rhs_impl!(u128);
div_rhs_impl!(usize);

impl<F, T> std::ops::Rem<T> for Float<F>
where
    F: Copy + num_traits::NumCast,
    T: num_traits::NumCast,
{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        let rhs: Self = num_traits::cast(rhs).unwrap();
        cfg_if! {
            if #[cfg(feature = "nightly")] {
                Self(unsafe { std::intrinsics::frem_fast(self.0, rhs.0) })
            } else {
                Self(self.0 % rhs.0)
            }
        }
    }
}

macro_rules! rem_rhs_impl {
    ($ty:ty) => {
        impl<F> std::ops::Rem<Float<F>> for $ty
        where
            F: Copy,
            F: num_traits::NumCast,
        {
            type Output = Float<F>;

            fn rem(self, rhs: Float<F>) -> Self::Output {
                let lhs: Float<F> = num_traits::cast(self).unwrap();
                lhs % rhs
            }
        }
    };
}

rem_rhs_impl!(f32);
rem_rhs_impl!(f64);
rem_rhs_impl!(i8);
rem_rhs_impl!(i16);
rem_rhs_impl!(i32);
rem_rhs_impl!(i64);
rem_rhs_impl!(i128);
rem_rhs_impl!(isize);
rem_rhs_impl!(u8);
rem_rhs_impl!(u16);
rem_rhs_impl!(u32);
rem_rhs_impl!(u64);
rem_rhs_impl!(u128);
rem_rhs_impl!(usize);

impl<F, T> std::ops::AddAssign<T> for Float<F>
where
    F: Copy + num_traits::NumCast,
    T: num_traits::NumCast,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<F, T> std::ops::SubAssign<T> for Float<F>
where
    F: Copy + num_traits::NumCast,
    T: num_traits::NumCast,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<F, T> std::ops::MulAssign<T> for Float<F>
where
    F: Copy + num_traits::NumCast,
    T: num_traits::NumCast,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<F, T> std::ops::DivAssign<T> for Float<F>
where
    F: Copy + num_traits::NumCast,
    T: num_traits::NumCast,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<F, T> std::ops::RemAssign<T> for Float<F>
where
    F: Copy + num_traits::NumCast,
    T: num_traits::NumCast,
{
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs;
    }
}

impl<F> std::ops::Neg for Float<F>
where
    F: Copy + num_traits::One + std::ops::Neg<Output = F> + num_traits::NumCast,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * Self(-num_traits::one::<F>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::default_numeric_fallback)]
    #[allow(clippy::cognitive_complexity)]
    fn test_type() {
        let mut a: Float<f64> = 1.into();
        let b: Float<f64> = 2.into();

        assert_eq!(a + b, 3);
        assert_eq!(a - b, -1);
        assert_eq!(a * b, 2);
        assert_eq!(a / b, 0.5);
        assert_eq!(a % b, 1);

        assert_eq!(a + 2i64, 3);
        assert_eq!(a - 2i64, -1);
        assert_eq!(a * 2i64, 2);
        assert_eq!(a / 2i64, 0.5);
        assert_eq!(a % 2i64, 1);

        assert_eq!(1i64 + b, 3);
        assert_eq!(1i64 - b, -1);
        assert_eq!(1i64 * b, 2);
        assert_eq!(1i64 / b, 0.5);
        assert_eq!(1i64 % b, 1);

        a += b;
        assert_eq!(a, 3);
        a -= b;
        assert_eq!(a, 1);
        a *= b;
        assert_eq!(a, 2);
        a /= b;
        assert_eq!(a, 1);
        a %= b;
        assert_eq!(a, 1);

        a += 2i64;
        assert_eq!(a, 3);
        a -= 2i64;
        assert_eq!(a, 1);
        a *= 2i64;
        assert_eq!(a, 2);
        a /= 2i64;
        assert_eq!(a, 1);
        a %= 2i64;
        assert_eq!(a, 1);
    }
}
