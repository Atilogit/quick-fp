use crate::Float;

impl<F> approx::UlpsEq for Float<F>
where
    F: approx::UlpsEq<Epsilon = F> + num_traits::ToPrimitive + Clone + num_traits::NumCast,
{
    fn default_max_ulps() -> u32 {
        F::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        F::ulps_eq(&self.0, &other.0, epsilon.0, max_ulps)
    }
}

impl<F> approx::AbsDiffEq for Float<F>
where
    F: approx::AbsDiffEq<Epsilon = F> + num_traits::ToPrimitive + Clone + num_traits::NumCast,
{
    type Epsilon = Self;

    fn default_epsilon() -> Self::Epsilon {
        Self(F::default_epsilon())
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        F::abs_diff_eq(&self.0, &other.0, epsilon.0)
    }
}

impl<F> approx::RelativeEq for Float<F>
where
    F: approx::RelativeEq<Epsilon = F> + num_traits::ToPrimitive + Clone + num_traits::NumCast,
{
    fn default_max_relative() -> Self::Epsilon {
        Self(F::default_max_relative())
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        F::relative_eq(&self.0, &other.0, epsilon.0, max_relative.0)
    }
}
