use crate::Float;

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
