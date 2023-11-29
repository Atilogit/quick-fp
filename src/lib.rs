#![cfg_attr(feature = "nightly", feature(core_intrinsics))]

#[cfg(feature = "approx")]
mod approx;
#[cfg(feature = "nalgebra")]
mod nalgebra;
#[cfg(feature = "num_traits")]
mod num_traits;
mod ops;
#[cfg(feature = "simba")]
mod simba;

#[derive(Clone, Copy)]
#[cfg_attr(not(feature = "num_traits"), derive(PartialEq, PartialOrd))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "f64", into = "f64"))]
#[repr(transparent)]
pub struct Float(pub f64);

impl Float {
    pub const EPSILON: Self = Self(1e-12);
}

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

impl From<Float> for f32 {
    fn from(f: Float) -> Self {
        f.0 as Self
    }
}

#[cfg(feature = "num_traits")]
impl<T: ::num_traits::ToPrimitive> std::cmp::PartialEq<T> for Float {
    fn eq(&self, other: &T) -> bool {
        self.0.eq(&other.to_f64().unwrap())
    }
}

#[cfg(feature = "eq")]
impl Eq for Float {}

#[cfg(feature = "num_traits")]
impl<T: ::num_traits::ToPrimitive> std::cmp::PartialOrd<T> for Float {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.to_f64().unwrap())
    }
}

#[cfg(feature = "ord")]
#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
