#![cfg_attr(feature = "nightly", feature(core_intrinsics))]

#[cfg(feature = "approx")]
mod approx;
#[cfg(feature = "nalgebra")]
mod nalgebra;
mod num;
mod ops;
#[cfg(feature = "simba")]
mod simba;

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct Float<F>(pub F)
where
    F: Clone;

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

impl<F, T> std::cmp::PartialEq<T> for Float<F>
where
    F: Clone + PartialEq + num_traits::NumCast,
    T: Clone + num_traits::NumCast,
{
    fn eq(&self, other: &T) -> bool {
        if let Some(other) = num_traits::cast(other.clone()) {
            self.0.eq(&other)
        } else {
            false
        }
    }
}

#[cfg(feature = "eq")]
impl<F> Eq for Float<F> where F: num_traits::NumCast + Clone + PartialEq {}

impl<F, T> std::cmp::PartialOrd<T> for Float<F>
where
    F: Clone + PartialOrd + num_traits::NumCast,
    T: Clone + num_traits::NumCast,
{
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&num_traits::cast(other.clone())?)
    }
}

#[cfg(feature = "ord")]
impl<F> Ord for Float<F>
where
    F: num_traits::NumCast + Clone + PartialOrd,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// num_traits::Bounded is intentionally not implemented for Float<F>
// to avoid the From<Float<T>> for Float<F> implementation which would conflict
// with the standard library.
//
// This implementation is infallible because num_traits::cast never errors when casting to a float.
#[allow(clippy::fallible_impl_from)]
impl<T, F> From<T> for Float<F>
where
    T: num_traits::NumCast + num_traits::Bounded,
    F: num_traits::NumCast + Clone,
{
    fn from(value: T) -> Self {
        num_traits::cast(value).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::default_numeric_fallback)]
    fn test_type() {
        let a: Float<f64> = 1.into();
        let b: Float<f32> = 1f32.into();
        let c: Float<i64> = 1i32.into();

        println!("{a:?} {a}");
        println!("{b:?} {b}");
        println!("{c:?} {c}");

        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(b, c);

        assert!(a < Float(2f64));
        assert!(b < Float(2f64));
        assert!(c < Float(2f64));

        assert!(a < Float(2i64));
        assert!(b < Float(2i64));
        assert!(c < Float(2i64));
    }
}
