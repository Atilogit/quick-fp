use nalgebra::{ComplexField, Field, RealField, SimdValue};

use crate::Float;

impl SimdValue for Float {
    type Element = Self;

    type SimdBool = bool;

    fn lanes() -> usize {
        1
    }

    fn splat(val: Self::Element) -> Self {
        val
    }

    fn extract(&self, _: usize) -> Self::Element {
        *self
    }

    unsafe fn extract_unchecked(&self, _: usize) -> Self::Element {
        *self
    }

    fn replace(&mut self, _: usize, val: Self::Element) {
        *self = val;
    }

    unsafe fn replace_unchecked(&mut self, _: usize, val: Self::Element) {
        *self = val;
    }

    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        if cond {
            self
        } else {
            other
        }
    }
}

impl Field for Float {}

impl RealField for Float {
    fn is_sign_positive(&self) -> bool {
        <Self as num_traits::Float>::is_sign_positive(*self)
    }

    fn is_sign_negative(&self) -> bool {
        <Self as num_traits::Float>::is_sign_negative(*self)
    }

    fn copysign(self, sign: Self) -> Self {
        <Self as num_traits::Float>::copysign(self, sign)
    }

    fn max(self, other: Self) -> Self {
        <Self as num_traits::Float>::max(self, other)
    }

    fn min(self, other: Self) -> Self {
        <Self as num_traits::Float>::min(self, other)
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        <Self as num_traits::Float>::max(<Self as num_traits::Float>::min(self, max), min)
    }

    fn atan2(self, other: Self) -> Self {
        <Self as num_traits::Float>::atan2(self, other)
    }

    fn min_value() -> Option<Self> {
        Some(<Self as num_traits::Float>::min_value())
    }

    fn max_value() -> Option<Self> {
        Some(<Self as num_traits::Float>::max_value())
    }

    fn pi() -> Self {
        Self(f64::pi())
    }

    fn two_pi() -> Self {
        Self(f64::two_pi())
    }

    fn frac_pi_2() -> Self {
        Self(f64::frac_pi_2())
    }

    fn frac_pi_3() -> Self {
        Self(f64::frac_pi_3())
    }

    fn frac_pi_4() -> Self {
        Self(f64::frac_pi_4())
    }

    fn frac_pi_6() -> Self {
        Self(f64::frac_pi_6())
    }

    fn frac_pi_8() -> Self {
        Self(f64::frac_pi_8())
    }

    fn frac_1_pi() -> Self {
        Self(f64::frac_1_pi())
    }

    fn frac_2_pi() -> Self {
        Self(f64::frac_2_pi())
    }

    fn frac_2_sqrt_pi() -> Self {
        Self(f64::frac_2_sqrt_pi())
    }

    fn e() -> Self {
        Self(f64::e())
    }

    fn log2_e() -> Self {
        Self(f64::log2_e())
    }

    fn log10_e() -> Self {
        Self(f64::log10_e())
    }

    fn ln_2() -> Self {
        Self(f64::ln_2())
    }

    fn ln_10() -> Self {
        Self(f64::ln_10())
    }
}

impl ComplexField for Float {
    type RealField = Self;

    fn from_real(re: Self::RealField) -> Self {
        re
    }

    fn real(self) -> Self::RealField {
        self
    }

    fn imaginary(self) -> Self::RealField {
        num_traits::zero()
    }

    fn modulus(self) -> Self::RealField {
        self.abs()
    }

    fn modulus_squared(self) -> Self::RealField {
        self * self
    }

    fn argument(self) -> Self::RealField {
        if self >= num_traits::zero::<Self>() {
            num_traits::zero()
        } else {
            Self::pi()
        }
    }

    fn norm1(self) -> Self::RealField {
        self.abs()
    }

    fn scale(self, factor: Self::RealField) -> Self {
        self * factor
    }

    fn unscale(self, factor: Self::RealField) -> Self {
        self / factor
    }

    fn floor(self) -> Self {
        <Self as num_traits::Float>::floor(self)
    }

    fn ceil(self) -> Self {
        <Self as num_traits::Float>::ceil(self)
    }

    fn round(self) -> Self {
        <Self as num_traits::Float>::round(self)
    }

    fn trunc(self) -> Self {
        <Self as num_traits::Float>::trunc(self)
    }

    fn fract(self) -> Self {
        <Self as num_traits::Float>::fract(self)
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        <Self as num_traits::Float>::mul_add(self, a, b)
    }

    fn abs(self) -> Self::RealField {
        <Self as num_traits::Float>::abs(self)
    }

    fn hypot(self, other: Self) -> Self::RealField {
        <Self as num_traits::Float>::hypot(self, other)
    }

    fn recip(self) -> Self {
        <Self as num_traits::Float>::recip(self)
    }

    fn conjugate(self) -> Self {
        self
    }

    fn sin(self) -> Self {
        <Self as num_traits::Float>::sin(self)
    }

    fn cos(self) -> Self {
        <Self as num_traits::Float>::cos(self)
    }

    fn sin_cos(self) -> (Self, Self) {
        <Self as num_traits::Float>::sin_cos(self)
    }

    fn tan(self) -> Self {
        <Self as num_traits::Float>::tan(self)
    }

    fn asin(self) -> Self {
        <Self as num_traits::Float>::asin(self)
    }

    fn acos(self) -> Self {
        <Self as num_traits::Float>::acos(self)
    }

    fn atan(self) -> Self {
        <Self as num_traits::Float>::atan(self)
    }

    fn sinh(self) -> Self {
        <Self as num_traits::Float>::sinh(self)
    }

    fn cosh(self) -> Self {
        <Self as num_traits::Float>::cosh(self)
    }

    fn tanh(self) -> Self {
        <Self as num_traits::Float>::tanh(self)
    }

    fn asinh(self) -> Self {
        <Self as num_traits::Float>::asinh(self)
    }

    fn acosh(self) -> Self {
        <Self as num_traits::Float>::acosh(self)
    }

    fn atanh(self) -> Self {
        <Self as num_traits::Float>::atanh(self)
    }

    fn log(self, base: Self::RealField) -> Self {
        <Self as num_traits::Float>::log(self, base)
    }

    fn log2(self) -> Self {
        <Self as num_traits::Float>::log2(self)
    }

    fn log10(self) -> Self {
        <Self as num_traits::Float>::log10(self)
    }

    fn ln(self) -> Self {
        <Self as num_traits::Float>::ln(self)
    }

    fn ln_1p(self) -> Self {
        <Self as num_traits::Float>::ln_1p(self)
    }

    fn sqrt(self) -> Self {
        <Self as num_traits::Float>::sqrt(self)
    }

    fn exp(self) -> Self {
        <Self as num_traits::Float>::exp(self)
    }

    fn exp2(self) -> Self {
        <Self as num_traits::Float>::exp2(self)
    }

    fn exp_m1(self) -> Self {
        <Self as num_traits::Float>::exp_m1(self)
    }

    fn powi(self, n: i32) -> Self {
        <Self as num_traits::Float>::powi(self, n)
    }

    fn powf(self, n: Self::RealField) -> Self {
        <Self as num_traits::Float>::powf(self, n)
    }

    fn powc(self, n: Self) -> Self {
        <Self as num_traits::Float>::powf(self, n)
    }

    fn cbrt(self) -> Self {
        <Self as num_traits::Float>::cbrt(self)
    }

    fn is_finite(&self) -> bool {
        <Self as num_traits::Float>::is_finite(*self)
    }

    fn try_sqrt(self) -> Option<Self> {
        Some(<Self as num_traits::Float>::sqrt(self))
    }
}
