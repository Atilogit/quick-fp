use crate::Float;

impl simba::scalar::SubsetOf<f64> for Float<f64> {
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

impl simba::scalar::SubsetOf<Float<Self>> for f64 {
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

impl simba::scalar::SubsetOf<Self> for Float<f64> {
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
