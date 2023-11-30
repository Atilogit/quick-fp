use crate::Float;

impl<F> num_traits::Zero for Float<F>
where
    F: Copy + PartialEq + num_traits::Zero,
{
    fn zero() -> Self {
        Self(F::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<F> num_traits::One for Float<F>
where
    F: Copy + num_traits::One,
{
    fn one() -> Self {
        Self(F::one())
    }
}

impl<F> num_traits::Signed for Float<F>
where
    F: Copy + num_traits::Signed + num_traits::NumCast,
{
    fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    #[allow(deprecated)]
    fn abs_sub(&self, other: &Self) -> Self {
        Self(self.0.abs_sub(&other.0))
    }

    fn signum(&self) -> Self {
        Self(self.0.signum())
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

impl<F> num_traits::Float for Float<F>
where
    F: num_traits::Zero + num_traits::Float,
{
    fn nan() -> Self {
        Self(F::nan())
    }

    fn infinity() -> Self {
        Self(F::infinity())
    }

    fn neg_infinity() -> Self {
        Self(F::neg_infinity())
    }

    fn neg_zero() -> Self {
        Self(F::neg_zero())
    }

    fn min_value() -> Self {
        Self(F::min_value())
    }

    fn min_positive_value() -> Self {
        Self(F::min_positive_value())
    }

    fn max_value() -> Self {
        Self(F::max_value())
    }

    fn is_nan(self) -> bool {
        F::is_nan(self.0)
    }

    fn is_infinite(self) -> bool {
        F::is_infinite(self.0)
    }

    fn is_finite(self) -> bool {
        F::is_finite(self.0)
    }

    fn is_normal(self) -> bool {
        F::is_normal(self.0)
    }

    fn classify(self) -> std::num::FpCategory {
        F::classify(self.0)
    }

    fn floor(self) -> Self {
        Self(F::floor(self.0))
    }

    fn ceil(self) -> Self {
        Self(F::ceil(self.0))
    }

    fn round(self) -> Self {
        Self(F::round(self.0))
    }

    fn trunc(self) -> Self {
        Self(F::trunc(self.0))
    }

    fn fract(self) -> Self {
        Self(F::fract(self.0))
    }

    fn abs(self) -> Self {
        Self(F::abs(self.0))
    }

    fn signum(self) -> Self {
        Self(F::signum(self.0))
    }

    fn is_sign_positive(self) -> bool {
        F::is_sign_positive(self.0)
    }

    fn is_sign_negative(self) -> bool {
        F::is_sign_negative(self.0)
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }

    fn recip(self) -> Self {
        Self(F::one()) / self
    }

    fn powi(self, n: i32) -> Self {
        Self(F::powi(self.0, n))
    }

    fn powf(self, n: Self) -> Self {
        Self(F::powf(self.0, n.0))
    }

    fn sqrt(self) -> Self {
        Self(F::sqrt(self.0))
    }

    #[allow(unconditional_recursion)]
    fn exp(self) -> Self {
        self.exp()
    }

    #[allow(unconditional_recursion)]
    fn exp2(self) -> Self {
        self.exp2()
    }

    fn ln(self) -> Self {
        Self(F::ln(self.0))
    }

    fn log(self, base: Self) -> Self {
        Self(F::log(self.0, base.0))
    }

    #[allow(unconditional_recursion)]
    fn log2(self) -> Self {
        self.log2()
    }

    fn log10(self) -> Self {
        Self(F::log10(self.0))
    }

    fn max(self, other: Self) -> Self {
        Self(F::max(self.0, other.0))
    }

    fn min(self, other: Self) -> Self {
        Self(F::min(self.0, other.0))
    }

    fn abs_sub(self, other: Self) -> Self {
        #[allow(deprecated)]
        Self(F::abs_sub(self.0, other.0))
    }

    fn cbrt(self) -> Self {
        Self(F::cbrt(self.0))
    }

    fn hypot(self, other: Self) -> Self {
        Self(F::hypot(self.0, other.0))
    }

    fn sin(self) -> Self {
        Self(F::sin(self.0))
    }

    fn cos(self) -> Self {
        Self(F::cos(self.0))
    }

    fn tan(self) -> Self {
        Self(F::tan(self.0))
    }

    fn asin(self) -> Self {
        Self(F::asin(self.0))
    }

    fn acos(self) -> Self {
        Self(F::acos(self.0))
    }

    #[allow(unconditional_recursion)]
    fn atan(self) -> Self {
        self.atan()
    }

    #[allow(unconditional_recursion)]
    #[allow(clippy::only_used_in_recursion)]
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    fn sin_cos(self) -> (Self, Self) {
        let (a, b) = F::sin_cos(self.0);
        (Self(a), Self(b))
    }

    fn exp_m1(self) -> Self {
        Self(F::exp_m1(self.0))
    }

    fn ln_1p(self) -> Self {
        Self(F::ln_1p(self.0))
    }

    fn sinh(self) -> Self {
        Self(F::sinh(self.0))
    }

    fn cosh(self) -> Self {
        Self(F::cosh(self.0))
    }

    fn tanh(self) -> Self {
        Self(F::tanh(self.0))
    }

    fn asinh(self) -> Self {
        Self(F::asinh(self.0))
    }

    fn acosh(self) -> Self {
        Self(F::acosh(self.0))
    }

    fn atanh(self) -> Self {
        Self(F::atanh(self.0))
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        F::integer_decode(self.0)
    }
}

impl<F> num_traits::Num for Float<F>
where
    F: Copy + PartialEq + num_traits::Num + num_traits::NumCast,
{
    type FromStrRadixErr = <F as num_traits::Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        F::from_str_radix(str, radix).map(Self)
    }
}

impl<F> num_traits::NumCast for Float<F>
where
    F: num_traits::NumCast + Clone,
{
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        num_traits::NumCast::from(n).map(Self)
    }
}

impl<F> num_traits::ToPrimitive for Float<F>
where
    F: num_traits::ToPrimitive + Clone,
{
    fn to_i64(&self) -> Option<i64> {
        F::to_i64(&self.0)
    }

    fn to_u64(&self) -> Option<u64> {
        F::to_u64(&self.0)
    }

    fn to_isize(&self) -> Option<isize> {
        F::to_isize(&self.0)
    }

    fn to_i8(&self) -> Option<i8> {
        F::to_i8(&self.0)
    }

    fn to_i16(&self) -> Option<i16> {
        F::to_i16(&self.0)
    }

    fn to_i32(&self) -> Option<i32> {
        F::to_i32(&self.0)
    }

    fn to_i128(&self) -> Option<i128> {
        F::to_i128(&self.0)
    }

    fn to_usize(&self) -> Option<usize> {
        F::to_usize(&self.0)
    }

    fn to_u8(&self) -> Option<u8> {
        F::to_u8(&self.0)
    }

    fn to_u16(&self) -> Option<u16> {
        F::to_u16(&self.0)
    }

    fn to_u32(&self) -> Option<u32> {
        F::to_u32(&self.0)
    }

    fn to_u128(&self) -> Option<u128> {
        F::to_u128(&self.0)
    }

    fn to_f32(&self) -> Option<f32> {
        F::to_f32(&self.0)
    }

    fn to_f64(&self) -> Option<f64> {
        F::to_f64(&self.0)
    }
}

impl<F> num_traits::FromPrimitive for Float<F>
where
    F: num_traits::FromPrimitive + Clone,
{
    fn from_i64(n: i64) -> Option<Self> {
        F::from_i64(n).map(Self)
    }

    fn from_u64(n: u64) -> Option<Self> {
        F::from_u64(n).map(Self)
    }

    fn from_isize(n: isize) -> Option<Self> {
        F::from_isize(n).map(Self)
    }

    fn from_i8(n: i8) -> Option<Self> {
        F::from_i8(n).map(Self)
    }

    fn from_i16(n: i16) -> Option<Self> {
        F::from_i16(n).map(Self)
    }

    fn from_i32(n: i32) -> Option<Self> {
        F::from_i32(n).map(Self)
    }

    fn from_i128(n: i128) -> Option<Self> {
        F::from_i128(n).map(Self)
    }

    fn from_usize(n: usize) -> Option<Self> {
        F::from_usize(n).map(Self)
    }

    fn from_u8(n: u8) -> Option<Self> {
        F::from_u8(n).map(Self)
    }

    fn from_u16(n: u16) -> Option<Self> {
        F::from_u16(n).map(Self)
    }

    fn from_u32(n: u32) -> Option<Self> {
        F::from_u32(n).map(Self)
    }

    fn from_u128(n: u128) -> Option<Self> {
        F::from_u128(n).map(Self)
    }

    fn from_f32(n: f32) -> Option<Self> {
        F::from_f32(n).map(Self)
    }

    fn from_f64(n: f64) -> Option<Self> {
        F::from_f64(n).map(Self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::default_numeric_fallback)]
    fn test_type() {
        let a: Float<f64> = 1.into();

        <Float<f64> as num_traits::Zero>::zero();
        <Float<f64> as num_traits::Zero>::is_zero(&a);

        <Float<f64> as num_traits::One>::one();

        <Float<f64> as num_traits::Signed>::abs(&a);
        <Float<f64> as num_traits::Signed>::abs_sub(&a, &a);
        <Float<f64> as num_traits::Signed>::signum(&a);
        <Float<f64> as num_traits::Signed>::is_positive(&a);
        <Float<f64> as num_traits::Signed>::is_negative(&a);

        <Float<f64> as num_traits::Float>::nan();
        <Float<f64> as num_traits::Float>::infinity();
        <Float<f64> as num_traits::Float>::neg_infinity();
        <Float<f64> as num_traits::Float>::neg_zero();
        <Float<f64> as num_traits::Float>::min_value();
        <Float<f64> as num_traits::Float>::min_positive_value();
        <Float<f64> as num_traits::Float>::max_value();
        <Float<f64> as num_traits::Float>::is_nan(a);
        <Float<f64> as num_traits::Float>::is_infinite(a);
        <Float<f64> as num_traits::Float>::is_finite(a);
        <Float<f64> as num_traits::Float>::is_normal(a);
        <Float<f64> as num_traits::Float>::classify(a);
        <Float<f64> as num_traits::Float>::floor(a);
        <Float<f64> as num_traits::Float>::ceil(a);
        <Float<f64> as num_traits::Float>::round(a);
        <Float<f64> as num_traits::Float>::trunc(a);
        <Float<f64> as num_traits::Float>::fract(a);
        <Float<f64> as num_traits::Float>::abs(a);
        <Float<f64> as num_traits::Float>::signum(a);
        <Float<f64> as num_traits::Float>::is_sign_positive(a);
        <Float<f64> as num_traits::Float>::is_sign_negative(a);
        <Float<f64> as num_traits::Float>::mul_add(a, a, a);
        <Float<f64> as num_traits::Float>::recip(a);
        <Float<f64> as num_traits::Float>::powi(a, 1);
        <Float<f64> as num_traits::Float>::powf(a, a);
        <Float<f64> as num_traits::Float>::sqrt(a);
        <Float<f64> as num_traits::Float>::exp(a);
        <Float<f64> as num_traits::Float>::exp2(a);
        <Float<f64> as num_traits::Float>::ln(a);
        <Float<f64> as num_traits::Float>::log(a, a);
        <Float<f64> as num_traits::Float>::log2(a);
        <Float<f64> as num_traits::Float>::log10(a);
        <Float<f64> as num_traits::Float>::max(a, a);
        <Float<f64> as num_traits::Float>::min(a, a);
        <Float<f64> as num_traits::Float>::abs_sub(a, a);
        <Float<f64> as num_traits::Float>::cbrt(a);
        <Float<f64> as num_traits::Float>::hypot(a, a);
        <Float<f64> as num_traits::Float>::sin(a);
        <Float<f64> as num_traits::Float>::cos(a);
        <Float<f64> as num_traits::Float>::tan(a);
        <Float<f64> as num_traits::Float>::asin(a);
        <Float<f64> as num_traits::Float>::acos(a);
        <Float<f64> as num_traits::Float>::atan(a);
        <Float<f64> as num_traits::Float>::atan2(a, a);
        <Float<f64> as num_traits::Float>::sin_cos(a);
        <Float<f64> as num_traits::Float>::exp_m1(a);
        <Float<f64> as num_traits::Float>::ln_1p(a);
        <Float<f64> as num_traits::Float>::sinh(a);
        <Float<f64> as num_traits::Float>::cosh(a);
        <Float<f64> as num_traits::Float>::tanh(a);
        <Float<f64> as num_traits::Float>::asinh(a);
        <Float<f64> as num_traits::Float>::acosh(a);
        <Float<f64> as num_traits::Float>::atanh(a);
        <Float<f64> as num_traits::Float>::integer_decode(a);

        <Float<f64> as num_traits::Num>::from_str_radix("1", 10).unwrap();

        <Float<f64> as num_traits::NumCast>::from(1);

        <Float<f64> as num_traits::ToPrimitive>::to_i64(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_u64(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_isize(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_i8(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_i16(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_i32(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_i128(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_usize(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_u8(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_u16(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_u32(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_u128(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_f32(&a);
        <Float<f64> as num_traits::ToPrimitive>::to_f64(&a);

        <Float<f64> as num_traits::FromPrimitive>::from_i64(1);
        <Float<f64> as num_traits::FromPrimitive>::from_u64(1);
        <Float<f64> as num_traits::FromPrimitive>::from_isize(1);
        <Float<f64> as num_traits::FromPrimitive>::from_i8(1);
        <Float<f64> as num_traits::FromPrimitive>::from_i16(1);
        <Float<f64> as num_traits::FromPrimitive>::from_i32(1);
        <Float<f64> as num_traits::FromPrimitive>::from_i128(1);
        <Float<f64> as num_traits::FromPrimitive>::from_usize(1);
        <Float<f64> as num_traits::FromPrimitive>::from_u8(1);
        <Float<f64> as num_traits::FromPrimitive>::from_u16(1);
        <Float<f64> as num_traits::FromPrimitive>::from_u32(1);
        <Float<f64> as num_traits::FromPrimitive>::from_u128(1);
        <Float<f64> as num_traits::FromPrimitive>::from_f32(1.);
        <Float<f64> as num_traits::FromPrimitive>::from_f64(1.);
    }
}
