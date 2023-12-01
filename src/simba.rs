use crate::Float;

macro_rules! simba_impl {
    ($($ty:ty),*) => {
        $(
            impl simba::scalar::SubsetOf<$ty> for Float<$ty> {
                fn to_superset(&self) -> $ty {
                    self.0
                }

                fn from_superset_unchecked(superset: &$ty) -> Self {
                    Self(*superset)
                }

                fn is_in_subset(_superset: &$ty) -> bool {
                    true
                }
            }

            impl simba::scalar::SubsetOf<Float<$ty>> for f64 {
                fn to_superset(&self) -> Float<$ty> {
                    (*self).into()
                }

                fn from_superset_unchecked(superset: &Float<$ty>) -> f64 {
                    superset.0 as f64
                }

                fn is_in_subset(_superset: &Float<$ty>) -> bool {
                    true
                }
            }

            impl simba::scalar::SubsetOf<Self> for Float<$ty> {
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

        )*
    };
}

simba_impl!(f32, f64);
