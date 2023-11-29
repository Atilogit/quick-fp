use crate::Float;

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
