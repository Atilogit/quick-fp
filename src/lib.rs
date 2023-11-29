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
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct Float<F>(pub F)
where
    F: Clone;

// impl<F> Float<F>
// where
//     F: Clone,
// {
//     pub const EPSILON: Self = Self(1e-12);
// }

impl<F> std::fmt::Debug for Float<F>
where
    F: Clone + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<F> std::fmt::Display for Float<F>
where
    F: Clone + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// impl<F> From<f64> for Float<F>
// where
//     F: Clone,
// {
//     fn from(f: f64) -> Self {
//         Self(f)
//     }
// }

// impl<F> From<Float<F>> for f64
// where
//     F: Clone,
// {
//     fn from(f: Float<F>) -> Self {
//         f.0
//     }
// }

// impl<F> From<Float<F>> for f32
// where
//     F: Clone,
// {
//     fn from(f: Float<F>) -> Self {
//         f.0 as Self
//     }
// }

#[cfg(feature = "num_traits")]
impl<F, T> std::cmp::PartialEq<T> for Float<F>
where
    F: Clone + PartialEq + ::num_traits::NumCast,
    T: Clone + ::num_traits::NumCast,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(&::num_traits::cast(other.clone()).unwrap())
    }
}

#[cfg(feature = "eq")]
impl<F> Eq for Float<F> where F: ::num_traits::NumCast + Clone + PartialEq {}

#[cfg(feature = "num_traits")]
impl<F, T> std::cmp::PartialOrd<T> for Float<F>
where
    F: Clone + PartialOrd + ::num_traits::NumCast,
    T: Clone + ::num_traits::NumCast,
{
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0
            .partial_cmp(&::num_traits::cast(other.clone()).unwrap())
    }
}

#[cfg(feature = "ord")]
#[allow(clippy::derive_ord_xor_partial_ord)]
impl<F> Ord for Float<F>
where
    F: ::num_traits::NumCast + Clone + PartialOrd,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
