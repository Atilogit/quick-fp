use crate::Float;

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
