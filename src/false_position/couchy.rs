/// Variations of signs of the polynomial p(x), it is the
/// maximum number of positive roots.
/// This is the maximum number of positive roots of p(x).
pub fn variations(
  polynomial: &crate::polynomials::Polynomial,
) -> i32 {
  let mut v = 0;

  if let Some(mut old) = polynomial.first() {
    polynomial
      .iter()
      // We already assigned the first to old.
      .skip(1)
      // We don't compare if it is 0 because 0 doesn't count
      // as a change in sign, but 0 between 2 opposite signs
      // still counts as a change in sign (+a, 0, -a), so we
      // shouldn't assign to old  when the coefficient is 0,
      // so none of the code bellow should be executed.
      .filter(|&&coefficient| {
        !crate::math::is_zero(coefficient)
      })
      .for_each(|coefficient| {
        // If coefficient and old have opposite signs then
        // coefficient * old < 0.0.
        v += (coefficient * old < 0.0) as i32;
        old = coefficient;
      });
  }

  v
}

/// Variations of signs of the polynomial p(-x), it is the
/// maximum number of negative roots.
/// This is the maximum number of negative roots of p(x).
pub fn negative_variations(
  polynomial: &crate::polynomials::Polynomial,
) -> i32 {
  let mut v = 0;
  if let Some(first) = polynomial.first() {
    // The first coefficient doesn't not multiplies an x
    // power, and so should be negated, otherwise we should
    // negate all x powers, and that'd be slower.
    let mut old = -first;

    polynomial
      .iter()
      .skip(1)
      .filter(|&&coefficient| {
        !crate::math::is_zero(coefficient)
      })
      .for_each(|&coefficient| {
        // If i is odd.
        // (-k)^n, where k is a positive number is -k, so
        // instead of checking if the signs are opposite we
        // are going to check if the signs are the same as
        // either the sign of the current coefficient is
        // negated or the sign of old is negated.
        // If coefficient and old have the same sign then
        // coefficient * old > 0.0.
        v += (coefficient * old > 0.0) as i32;
        old = coefficient;
      });
  }

  v
}

/// Returns the biggest (in terms of modulo) non-last
/// coefficient (the last coefficient is the one that
/// accompanies the term a^n).
fn biggest_non_last_coefficient(
  polynomial: &crate::polynomials::Polynomial,
) -> f32 {
  polynomial
    // Get the coefficients.
    .iter()
    // Reverse the order of the coefficients.
    .rev()
    // Skip the first (last if it wasn't reversed) coefficient.
    .skip(1)
    // Takes the modulo of each coefficient.
    .map(|c| c.abs())
    // Gets the maximum value.
    .reduce(f32::max)
    // If everything is ok return the result, otherwise return 0.0.
    .unwrap_or(0.0)
}

/// Returns the modulo of the Cauchy's bound.
/// All the negative roots are between -bound and 0.
/// All the positive roots are between 0 and bound.
pub fn bound(
  polynomial: &crate::polynomials::Polynomial,
) -> Option<f32> {
  polynomial.last().map(|last| {
    (1.0 + biggest_non_last_coefficient(polynomial) / last)
      .abs()
  })
}
