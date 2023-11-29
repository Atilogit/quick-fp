#![feature(core_intrinsics)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(from = "f64")]
#[serde(into = "f64")]
#[repr(transparent)]
pub struct Float(pub f64);

impl Float {
    pub const EPSILON: Self = Self(1e-12);
}

// Formatting

impl std::fmt::Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// Convert

impl From<f64> for Float {
    fn from(f: f64) -> Self {
        Self(f)
    }
}

impl From<Float> for f64 {
    fn from(f: Float) -> Self {
        f.0
    }
}

// std::ops

impl std::ops::Add for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Self(self.0 + rhs.0)
        Self(unsafe { std::intrinsics::fadd_fast(self.0, rhs.0) })
    }
}

impl std::ops::AddAssign for Float {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // Self(self.0 - rhs.0)
        Self(unsafe { std::intrinsics::fsub_fast(self.0, rhs.0) })
    }
}

impl std::ops::SubAssign for Float {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for Float {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // Self(self.0 * rhs.0)
        Self(unsafe { std::intrinsics::fmul_fast(self.0, rhs.0) })
    }
}

impl std::ops::MulAssign for Float {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for Float {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // Self(self.0 / rhs.0)
        Self(unsafe { std::intrinsics::fdiv_fast(self.0, rhs.0) })
    }
}

impl std::ops::DivAssign for Float {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::Rem for Float {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        // Self(self.0 % rhs.0)
        Self(unsafe { std::intrinsics::frem_fast(self.0, rhs.0) })
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

// Compare

impl<T: num_traits::ToPrimitive> std::cmp::PartialEq<T> for Float {
    fn eq(&self, other: &T) -> bool {
        self.0.eq(&other.to_f64().unwrap())
    }
}

impl Eq for Float {}

impl<T: num_traits::ToPrimitive> std::cmp::PartialOrd<T> for Float {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.to_f64().unwrap())
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// num_traits

impl num_traits::identities::Zero for Float {
    fn zero() -> Self {
        Self(0.0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0f64
    }
}

impl num_traits::identities::One for Float {
    fn one() -> Self {
        Self(1.0)
    }
}

impl num_traits::Signed for Float {
    fn abs(&self) -> Self {
        self.0.abs().into()
    }

    #[allow(deprecated)]
    fn abs_sub(&self, other: &Self) -> Self {
        self.0.abs_sub(other.0).into()
    }

    fn signum(&self) -> Self {
        self.0.signum().into()
    }

    #[allow(deprecated)]
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    #[allow(deprecated)]
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl num_traits::Float for Float {
    fn nan() -> Self {
        f64::nan().into()
    }

    fn infinity() -> Self {
        f64::infinity().into()
    }

    fn neg_infinity() -> Self {
        f64::neg_infinity().into()
    }

    fn neg_zero() -> Self {
        f64::neg_zero().into()
    }

    fn min_value() -> Self {
        f64::min_value().into()
    }

    fn min_positive_value() -> Self {
        f64::min_positive_value().into()
    }

    fn max_value() -> Self {
        f64::max_value().into()
    }

    fn is_nan(self) -> bool {
        f64::is_nan(self.0)
    }

    fn is_infinite(self) -> bool {
        f64::is_infinite(self.0)
    }

    fn is_finite(self) -> bool {
        f64::is_finite(self.0)
    }

    fn is_normal(self) -> bool {
        f64::is_normal(self.0)
    }

    fn classify(self) -> std::num::FpCategory {
        f64::classify(self.0)
    }

    fn floor(self) -> Self {
        f64::floor(self.0).into()
    }

    fn ceil(self) -> Self {
        f64::ceil(self.0).into()
    }

    fn round(self) -> Self {
        f64::round(self.0).into()
    }

    fn trunc(self) -> Self {
        f64::trunc(self.0).into()
    }

    fn fract(self) -> Self {
        f64::fract(self.0).into()
    }

    fn abs(self) -> Self {
        f64::abs(self.0).into()
    }

    fn signum(self) -> Self {
        f64::signum(self.0).into()
    }

    fn is_sign_positive(self) -> bool {
        f64::is_sign_positive(self.0)
    }

    fn is_sign_negative(self) -> bool {
        f64::is_sign_negative(self.0)
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }

    fn recip(self) -> Self {
        Self(1.) / self
    }

    fn powi(self, n: i32) -> Self {
        f64::powi(self.0, n).into()
    }

    fn powf(self, n: Self) -> Self {
        f64::powf(self.0, n.0).into()
    }

    fn sqrt(self) -> Self {
        f64::sqrt(self.0).into()
    }

    fn exp(self) -> Self {
        f64::exp(self.0).into()
    }

    fn exp2(self) -> Self {
        f64::exp2(self.0).into()
    }

    fn ln(self) -> Self {
        f64::ln(self.0).into()
    }

    fn log(self, base: Self) -> Self {
        f64::log(self.0, base.0).into()
    }

    fn log2(self) -> Self {
        f64::log2(self.0).into()
    }

    fn log10(self) -> Self {
        f64::log10(self.0).into()
    }

    fn max(self, other: Self) -> Self {
        f64::max(self.0, other.0).into()
    }

    fn min(self, other: Self) -> Self {
        f64::min(self.0, other.0).into()
    }

    fn abs_sub(self, other: Self) -> Self {
        #[allow(deprecated)]
        f64::abs_sub(self.0, other.0).into()
    }

    fn cbrt(self) -> Self {
        f64::cbrt(self.0).into()
    }

    fn hypot(self, other: Self) -> Self {
        f64::hypot(self.0, other.0).into()
    }

    fn sin(self) -> Self {
        f64::sin(self.0).into()
    }

    fn cos(self) -> Self {
        f64::cos(self.0).into()
    }

    fn tan(self) -> Self {
        f64::tan(self.0).into()
    }

    fn asin(self) -> Self {
        f64::asin(self.0).into()
    }

    fn acos(self) -> Self {
        f64::acos(self.0).into()
    }

    fn atan(self) -> Self {
        f64::atan(self.0).into()
    }

    fn atan2(self, other: Self) -> Self {
        f64::atan2(self.0, other.0).into()
    }

    fn sin_cos(self) -> (Self, Self) {
        let (a, b) = f64::sin_cos(self.0);
        (a.into(), b.into())
    }

    fn exp_m1(self) -> Self {
        f64::exp_m1(self.0).into()
    }

    fn ln_1p(self) -> Self {
        f64::ln_1p(self.0).into()
    }

    fn sinh(self) -> Self {
        f64::sinh(self.0).into()
    }

    fn cosh(self) -> Self {
        f64::cosh(self.0).into()
    }

    fn tanh(self) -> Self {
        f64::tanh(self.0).into()
    }

    fn asinh(self) -> Self {
        f64::asinh(self.0).into()
    }

    fn acosh(self) -> Self {
        f64::acosh(self.0).into()
    }

    fn atanh(self) -> Self {
        f64::atanh(self.0).into()
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        f64::integer_decode(self.0)
    }
}

impl num_traits::Num for Float {
    type FromStrRadixErr = <f64 as num_traits::Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        f64::from_str_radix(str, radix).map(Self)
    }
}

impl num_traits::NumCast for Float {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        num_traits::NumCast::from(n).map(Self)
    }
}

impl num_traits::cast::ToPrimitive for Float {
    fn to_i64(&self) -> Option<i64> {
        f64::to_i64(&self.0)
    }

    fn to_u64(&self) -> Option<u64> {
        f64::to_u64(&self.0)
    }

    fn to_isize(&self) -> Option<isize> {
        f64::to_isize(&self.0)
    }

    fn to_i8(&self) -> Option<i8> {
        f64::to_i8(&self.0)
    }

    fn to_i16(&self) -> Option<i16> {
        f64::to_i16(&self.0)
    }

    fn to_i32(&self) -> Option<i32> {
        f64::to_i32(&self.0)
    }

    fn to_i128(&self) -> Option<i128> {
        f64::to_i128(&self.0)
    }

    fn to_usize(&self) -> Option<usize> {
        f64::to_usize(&self.0)
    }

    fn to_u8(&self) -> Option<u8> {
        f64::to_u8(&self.0)
    }

    fn to_u16(&self) -> Option<u16> {
        f64::to_u16(&self.0)
    }

    fn to_u32(&self) -> Option<u32> {
        f64::to_u32(&self.0)
    }

    fn to_u128(&self) -> Option<u128> {
        f64::to_u128(&self.0)
    }

    fn to_f32(&self) -> Option<f32> {
        f64::to_f32(&self.0)
    }

    fn to_f64(&self) -> Option<f64> {
        f64::to_f64(&self.0)
    }
}

impl num_traits::cast::FromPrimitive for Float {
    fn from_i64(n: i64) -> Option<Self> {
        f64::from_i64(n).map(Self)
    }

    fn from_u64(n: u64) -> Option<Self> {
        f64::from_u64(n).map(Self)
    }

    fn from_isize(n: isize) -> Option<Self> {
        f64::from_isize(n).map(Self)
    }

    fn from_i8(n: i8) -> Option<Self> {
        f64::from_i8(n).map(Self)
    }

    fn from_i16(n: i16) -> Option<Self> {
        f64::from_i16(n).map(Self)
    }

    fn from_i32(n: i32) -> Option<Self> {
        f64::from_i32(n).map(Self)
    }

    fn from_i128(n: i128) -> Option<Self> {
        f64::from_i128(n).map(Self)
    }

    fn from_usize(n: usize) -> Option<Self> {
        f64::from_usize(n).map(Self)
    }

    fn from_u8(n: u8) -> Option<Self> {
        f64::from_u8(n).map(Self)
    }

    fn from_u16(n: u16) -> Option<Self> {
        f64::from_u16(n).map(Self)
    }

    fn from_u32(n: u32) -> Option<Self> {
        f64::from_u32(n).map(Self)
    }

    fn from_u128(n: u128) -> Option<Self> {
        f64::from_u128(n).map(Self)
    }

    fn from_f32(n: f32) -> Option<Self> {
        f64::from_f32(n).map(Self)
    }

    fn from_f64(n: f64) -> Option<Self> {
        f64::from_f64(n).map(Self)
    }
}

// nalgebra

impl nalgebra::SimdValue for Float {
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

impl nalgebra::Field for Float {}

impl nalgebra::RealField for Float {
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

impl nalgebra::ComplexField for Float {
    type RealField = Self;

    fn from_real(re: Self::RealField) -> Self {
        todo!()
    }

    fn real(self) -> Self::RealField {
        todo!()
    }

    fn imaginary(self) -> Self::RealField {
        todo!()
    }

    fn modulus(self) -> Self::RealField {
        todo!()
    }

    fn modulus_squared(self) -> Self::RealField {
        todo!()
    }

    fn argument(self) -> Self::RealField {
        todo!()
    }

    fn norm1(self) -> Self::RealField {
        todo!()
    }

    fn scale(self, factor: Self::RealField) -> Self {
        todo!()
    }

    fn unscale(self, factor: Self::RealField) -> Self {
        todo!()
    }

    fn floor(self) -> Self {
        todo!()
    }

    fn ceil(self) -> Self {
        todo!()
    }

    fn round(self) -> Self {
        todo!()
    }

    fn trunc(self) -> Self {
        todo!()
    }

    fn fract(self) -> Self {
        todo!()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        todo!()
    }

    fn abs(self) -> Self::RealField {
        todo!()
    }

    fn hypot(self, other: Self) -> Self::RealField {
        todo!()
    }

    fn recip(self) -> Self {
        todo!()
    }

    fn conjugate(self) -> Self {
        todo!()
    }

    fn sin(self) -> Self {
        todo!()
    }

    fn cos(self) -> Self {
        todo!()
    }

    fn sin_cos(self) -> (Self, Self) {
        todo!()
    }

    fn tan(self) -> Self {
        todo!()
    }

    fn asin(self) -> Self {
        todo!()
    }

    fn acos(self) -> Self {
        todo!()
    }

    fn atan(self) -> Self {
        todo!()
    }

    fn sinh(self) -> Self {
        todo!()
    }

    fn cosh(self) -> Self {
        todo!()
    }

    fn tanh(self) -> Self {
        todo!()
    }

    fn asinh(self) -> Self {
        todo!()
    }

    fn acosh(self) -> Self {
        todo!()
    }

    fn atanh(self) -> Self {
        todo!()
    }

    fn log(self, base: Self::RealField) -> Self {
        todo!()
    }

    fn log2(self) -> Self {
        todo!()
    }

    fn log10(self) -> Self {
        todo!()
    }

    fn ln(self) -> Self {
        todo!()
    }

    fn ln_1p(self) -> Self {
        todo!()
    }

    fn sqrt(self) -> Self {
        todo!()
    }

    fn exp(self) -> Self {
        todo!()
    }

    fn exp2(self) -> Self {
        todo!()
    }

    fn exp_m1(self) -> Self {
        todo!()
    }

    fn powi(self, n: i32) -> Self {
        todo!()
    }

    fn powf(self, n: Self::RealField) -> Self {
        todo!()
    }

    fn powc(self, n: Self) -> Self {
        todo!()
    }

    fn cbrt(self) -> Self {
        todo!()
    }

    fn is_finite(&self) -> bool {
        todo!()
    }

    fn try_sqrt(self) -> Option<Self> {
        todo!()
    }
}

// Approx

impl approx::UlpsEq for Float {
    fn default_max_ulps() -> u32 {
        f64::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        f64::ulps_eq(&self.0, &other.0, epsilon.0, max_ulps)
    }
}

impl approx::AbsDiffEq for Float {
    type Epsilon = Self;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon().into()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f64::abs_diff_eq(&self.0, &other.0, epsilon.0)
    }
}

impl approx::RelativeEq for Float {
    fn default_max_relative() -> Self::Epsilon {
        f64::default_max_relative().into()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        f64::relative_eq(&self.0, &other.0, epsilon.0, max_relative.0)
    }
}

impl simba::scalar::SubsetOf<f64> for Float {
    fn to_superset(&self) -> f64 {
        self.0
    }

    fn from_superset_unchecked(superset: &f64) -> Self {
        Self(*superset)
    }

    fn is_in_subset(_superset: &f64) -> bool {
        true
    }
}

impl simba::scalar::SubsetOf<Float> for f64 {
    fn to_superset(&self) -> Float {
        Float(*self)
    }

    fn from_superset_unchecked(superset: &Float) -> Self {
        superset.0
    }

    fn is_in_subset(_superset: &Float) -> bool {
        true
    }
}

impl simba::scalar::SubsetOf<Self> for Float {
    fn to_superset(&self) -> Self {
        *self
    }

    fn from_superset_unchecked(superset: &Self) -> Self {
        *superset
    }

    fn is_in_subset(_superset: &Self) -> bool {
        true
    }
}
