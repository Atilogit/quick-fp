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

#[allow(clippy::same_name_method)]
impl<F> Float<F>
where
    F: From<f32> + Clone + num_traits::NumCast,
{
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn atan(self) -> Self {
        cfg_if! {
            if #[cfg(feature = "fast_math")] {
                Self(fast_math::atan(num_traits::cast(self).unwrap()).into())
            } else {
                Self(self.0.atan())
            }
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn atan2(self, rhs: Self) -> Self {
        cfg_if! {
            if #[cfg(feature = "fast_math")] {
                Self(fast_math::atan2(num_traits::cast(self).unwrap(), num_traits::cast(rhs).unwrap()).into())
            } else {
                Self(self.0.atan2(rhs.0))
            }
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub fn exp(self) -> Self {
        cfg_if! {
            if #[cfg(feature = "fast_math")] {
                Self(fast_math::exp(num_traits::cast(self).unwrap()).into())
            } else {
                Self(self.0.exp())
            }
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn exp2(self) -> Self {
        cfg_if! {
            if #[cfg(feature = "fast_math")] {
                Self(fast_math::exp2(num_traits::cast(self).unwrap()).into())
            } else {
                Self(self.0.exp2())
            }
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn log2(self) -> Self {
        cfg_if! {
            if #[cfg(feature = "fast_math")] {
                Self(fast_math::log2(num_traits::cast(self).unwrap()).into())
            } else {
                Self(self.0.log2())
            }
        }
    }
}
