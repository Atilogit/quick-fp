use cfg_if::cfg_if;

use crate::Float;

impl std::ops::Add for Float {
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

impl std::ops::Sub for Float {
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

impl std::ops::Mul for Float {
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

impl std::ops::Div for Float {
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

impl std::ops::Rem for Float {
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

impl std::ops::AddAssign for Float {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for Float {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::MulAssign for Float {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign for Float {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::RemAssign for Float {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl std::ops::Neg for Float {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * Self(-1.)
    }
}
