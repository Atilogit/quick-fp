use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::Float;

impl<F> UlpsEq for Float<F>
where
    F: UlpsEq<Epsilon = F> + Clone + num_traits::NumCast,
{
    fn default_max_ulps() -> u32 {
        F::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        F::ulps_eq(&self.0, &other.0, epsilon.0, max_ulps)
    }
}

impl<F> AbsDiffEq for Float<F>
where
    F: AbsDiffEq<Epsilon = F> + Clone + num_traits::NumCast,
{
    type Epsilon = Self;

    fn default_epsilon() -> Self::Epsilon {
        Self(F::default_epsilon())
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        F::abs_diff_eq(&self.0, &other.0, epsilon.0)
    }
}

impl<F> RelativeEq for Float<F>
where
    F: RelativeEq<Epsilon = F> + Clone + num_traits::NumCast,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::default_numeric_fallback)]
    fn test_type() {
        let a: Float<f64> = 1.into();
        let b: Float<f64> = 1.into();

        Float::<f64>::default_max_ulps();
        Float::<f64>::ulps_eq(&a, &b, 1.into(), 1);

        Float::<f64>::default_epsilon();
        Float::<f64>::abs_diff_eq(&a, &b, 1.into());

        Float::<f64>::default_max_relative();
        Float::<f64>::relative_eq(&a, &b, 1.into(), 1.into());
    }
}
