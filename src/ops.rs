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
