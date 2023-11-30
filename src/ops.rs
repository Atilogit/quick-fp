use cfg_if::cfg_if;

use crate::Float;

impl<F> std::ops::Add for Float<F>
where
    F: Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        cfg_if! {
            if #[cfg(feature = "nightly")] {
                Self(unsafe { std::intrinsics::fadd_fast(self.0, rhs.0) })
            } else {
                Self(self.0 + rhs.0)
            }
        }
    }
}

impl<F> std::ops::Sub for Float<F>
where
    F: Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        cfg_if! {
            if #[cfg(feature = "nightly")] {
                Self(unsafe { std::intrinsics::fsub_fast(self.0, rhs.0) })
            } else {
                Self(self.0 - rhs.0)
            }
        }
    }
}

impl<F> std::ops::Mul for Float<F>
where
    F: Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        cfg_if! {
            if #[cfg(feature = "nightly")] {
                Self(unsafe { std::intrinsics::fmul_fast(self.0, rhs.0) })
            } else {
                Self(self.0 * rhs.0)
            }
        }
    }
}

impl<F> std::ops::Div for Float<F>
where
    F: Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        cfg_if! {
            if #[cfg(feature = "nightly")] {
                Self(unsafe { std::intrinsics::fdiv_fast(self.0, rhs.0) })
            } else {
                Self(self.0 / rhs.0)
            }
        }
    }
}

impl<F> std::ops::Rem for Float<F>
where
    F: Copy,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        cfg_if! {
            if #[cfg(feature = "nightly")] {
                Self(unsafe { std::intrinsics::frem_fast(self.0, rhs.0) })
            } else {
                Self(self.0 % rhs.0)
            }
        }
    }
}

impl<F> std::ops::AddAssign for Float<F>
where
    F: Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<F> std::ops::SubAssign for Float<F>
where
    F: Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<F> std::ops::MulAssign for Float<F>
where
    F: Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<F> std::ops::DivAssign for Float<F>
where
    F: Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<F> std::ops::RemAssign for Float<F>
where
    F: Copy,
{
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl<F> std::ops::Neg for Float<F>
where
    F: Copy + num_traits::One + std::ops::Neg<Output = F>,
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
        let a: Float<f64> = 1.into();
        let b: Float<f64> = 2.into();

        assert_eq!(a + b, 3);
        assert_eq!(a - b, -1);
        assert_eq!(a * b, 2);
        assert_eq!(a / b, 0.5);
        assert_eq!(a % b, 1);

        // assert_eq!(a + 2i64, 3);
        // assert_eq!(a - 2i64, -1);
        // assert_eq!(a * 2i64, 2);
        // assert_eq!(a / 2i64, 0.5);
        // assert_eq!(a % 2i64, 1);

        // assert_eq!(1i64 + b, 3);
        // assert_eq!(1i64 - b, -1);
        // assert_eq!(1i64 * b, 2);
        // assert_eq!(1i64 / b, 0.5);
        // assert_eq!(1i64 % b, 1);
    }
}
