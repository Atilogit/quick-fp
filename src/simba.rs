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

            impl simba::scalar::SubsetOf<Float<Self>> for $ty {
                fn to_superset(&self) -> Float<Self> {
                    Float(*self)
                }

                fn from_superset_unchecked(superset: &Float<Self>) -> Self {
                    superset.0
                }

                fn is_in_subset(_superset: &Float<Self>) -> bool {
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

simba_impl!(f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
