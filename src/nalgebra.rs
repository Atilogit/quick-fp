use nalgebra::{ComplexField, Field, RealField, SimdValue};

use crate::Float;

impl<F> SimdValue for Float<F>
where
    F: Clone + Copy,
{
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

impl<F> Field for Float<F> where
    F: Copy + num_traits::Num + num_traits::NumCast + std::ops::Neg<Output = F>
{
}

impl<F> RealField for Float<F>
where
    F: 'static
        + Send
        + Sync
        + approx::UlpsEq<Epsilon = F>
        + approx::RelativeEq
        + num_traits::Signed
        + num_traits::Float
        + num_traits::FromPrimitive
        + std::fmt::Debug
        + std::fmt::Display,
    Self: simba::scalar::SubsetOf<Self>,
    f64: simba::scalar::SubsetOf<Self>,
{
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
        <Self as num_traits::NumCast>::from(f64::pi()).unwrap()
    }

    fn two_pi() -> Self {
        <Self as num_traits::NumCast>::from(f64::two_pi()).unwrap()
    }

    fn frac_pi_2() -> Self {
        <Self as num_traits::NumCast>::from(f64::frac_pi_2()).unwrap()
    }

    fn frac_pi_3() -> Self {
        <Self as num_traits::NumCast>::from(f64::frac_pi_3()).unwrap()
    }

    fn frac_pi_4() -> Self {
        <Self as num_traits::NumCast>::from(f64::frac_pi_4()).unwrap()
    }

    fn frac_pi_6() -> Self {
        <Self as num_traits::NumCast>::from(f64::frac_pi_6()).unwrap()
    }

    fn frac_pi_8() -> Self {
        <Self as num_traits::NumCast>::from(f64::frac_pi_8()).unwrap()
    }

    fn frac_1_pi() -> Self {
        <Self as num_traits::NumCast>::from(f64::frac_1_pi()).unwrap()
    }

    fn frac_2_pi() -> Self {
        <Self as num_traits::NumCast>::from(f64::frac_2_pi()).unwrap()
    }

    fn frac_2_sqrt_pi() -> Self {
        <Self as num_traits::NumCast>::from(f64::frac_2_sqrt_pi()).unwrap()
    }

    fn e() -> Self {
        <Self as num_traits::NumCast>::from(f64::e()).unwrap()
    }

    fn log2_e() -> Self {
        <Self as num_traits::NumCast>::from(f64::log2_e()).unwrap()
    }

    fn log10_e() -> Self {
        <Self as num_traits::NumCast>::from(f64::log10_e()).unwrap()
    }

    fn ln_2() -> Self {
        <Self as num_traits::NumCast>::from(f64::ln_2()).unwrap()
    }

    fn ln_10() -> Self {
        <Self as num_traits::NumCast>::from(f64::ln_10()).unwrap()
    }
}

impl<F> ComplexField for Float<F>
where
    F: 'static
        + Send
        + Sync
        + Clone
        + approx::UlpsEq<Epsilon = F>
        + approx::RelativeEq
        + num_traits::Signed
        + num_traits::Float
        + num_traits::FromPrimitive
        + std::fmt::Debug
        + std::fmt::Display,
    Self: simba::scalar::SubsetOf<Self>,
    f64: simba::scalar::SubsetOf<Self>,
{
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::default_numeric_fallback)]
    fn test_type() {
        let mut a: Float<f64> = 1.into();

        Float::<f64>::lanes();
        Float::<f64>::splat(a);
        a.extract(0);
        unsafe { a.extract_unchecked(0) };
        a.replace(0, a);
        unsafe { a.replace_unchecked(0, a) };
        a.select(true, a);

        <Float<f64> as RealField>::is_sign_positive(&a);
        <Float<f64> as RealField>::is_sign_negative(&a);
        <Float<f64> as RealField>::copysign(a, a);
        <Float<f64> as RealField>::max(a, a);
        <Float<f64> as RealField>::min(a, a);
        <Float<f64> as RealField>::clamp(a, a, a);
        <Float<f64> as RealField>::atan2(a, a);
        <Float<f64> as RealField>::min_value();
        <Float<f64> as RealField>::max_value();
        <Float<f64> as RealField>::pi();
        <Float<f64> as RealField>::two_pi();
        <Float<f64> as RealField>::frac_pi_2();
        <Float<f64> as RealField>::frac_pi_3();
        <Float<f64> as RealField>::frac_pi_4();
        <Float<f64> as RealField>::frac_pi_6();
        <Float<f64> as RealField>::frac_pi_8();
        <Float<f64> as RealField>::frac_1_pi();
        <Float<f64> as RealField>::frac_2_pi();
        <Float<f64> as RealField>::frac_2_sqrt_pi();
        <Float<f64> as RealField>::e();
        <Float<f64> as RealField>::log2_e();
        <Float<f64> as RealField>::log10_e();
        <Float<f64> as RealField>::ln_2();
        <Float<f64> as RealField>::ln_10();

        <Float<f64> as ComplexField>::from_real(a);
        <Float<f64> as ComplexField>::real(a);
        <Float<f64> as ComplexField>::imaginary(a);
        <Float<f64> as ComplexField>::modulus(a);
        <Float<f64> as ComplexField>::modulus_squared(a);
        <Float<f64> as ComplexField>::argument(a);
        <Float<f64> as ComplexField>::norm1(a);
        <Float<f64> as ComplexField>::scale(a, a);
        <Float<f64> as ComplexField>::unscale(a, a);
        <Float<f64> as ComplexField>::floor(a);
        <Float<f64> as ComplexField>::ceil(a);
        <Float<f64> as ComplexField>::round(a);
        <Float<f64> as ComplexField>::trunc(a);
        <Float<f64> as ComplexField>::fract(a);
        <Float<f64> as ComplexField>::mul_add(a, a, a);
        <Float<f64> as ComplexField>::abs(a);
        <Float<f64> as ComplexField>::hypot(a, a);
        <Float<f64> as ComplexField>::recip(a);
        <Float<f64> as ComplexField>::conjugate(a);
        <Float<f64> as ComplexField>::sin(a);
        <Float<f64> as ComplexField>::cos(a);
        <Float<f64> as ComplexField>::sin_cos(a);
        <Float<f64> as ComplexField>::tan(a);
        <Float<f64> as ComplexField>::asin(a);
        <Float<f64> as ComplexField>::acos(a);
        <Float<f64> as ComplexField>::atan(a);
        <Float<f64> as ComplexField>::sinh(a);
        <Float<f64> as ComplexField>::cosh(a);
        <Float<f64> as ComplexField>::tanh(a);
        <Float<f64> as ComplexField>::asinh(a);
        <Float<f64> as ComplexField>::acosh(a);
        <Float<f64> as ComplexField>::atanh(a);
        <Float<f64> as ComplexField>::log(a, a);
        <Float<f64> as ComplexField>::log2(a);
        <Float<f64> as ComplexField>::log10(a);
        <Float<f64> as ComplexField>::ln(a);
        <Float<f64> as ComplexField>::ln_1p(a);
        <Float<f64> as ComplexField>::sqrt(a);
        <Float<f64> as ComplexField>::exp(a);
        <Float<f64> as ComplexField>::exp2(a);
        <Float<f64> as ComplexField>::exp_m1(a);
        <Float<f64> as ComplexField>::powi(a, 1);
        <Float<f64> as ComplexField>::powf(a, a);
        <Float<f64> as ComplexField>::powc(a, a);
        <Float<f64> as ComplexField>::cbrt(a);
        <Float<f64> as ComplexField>::is_finite(&a);
        <Float<f64> as ComplexField>::try_sqrt(a);
    }
}
