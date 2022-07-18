/// Variations of signs of the polynomial p(x), it is the
/// maximum number of positive roots.
/// This is the maximum number of positive roots of p(x).
pub fn variations(
  polynomial: &crate::polynomials::Polynomial,
) -> i32 {
  let mut v: i32 = 0;
  if let Some(mut old) = polynomial.first() {
    for coefficient in polynomial.iter().skip(1) {
      // We don't compare if it is 0 because 0 doesn't count
      // as a change in sign, but 0 between 2 opposite signs
      // still counts as a change in sign (+a, 0, -a), so we
      // shouldn't assign to old  when the coefficient is 0,
      // so none of the code bellow should be executed.
      if !crate::math::is_zero(*coefficient) {
        if (*coefficient < 0.0 && *old > 0.0)
          || (*coefficient > 0.0 && *old < 0.0)
        {
          v += 1;
        }
        old = coefficient;
      }
    }
  }
  return v;
}

/// Variations of signs of the polynomial p(-x), it is the
/// maximum number of negative roots.
/// This is the maximum number of negative roots of p(x).
pub fn negative_variations(
  polynomial: &crate::polynomials::Polynomial,
) -> i32 {
  let mut v: i32 = 0;
  if let Some(first) = polynomial.first() {
    // The first coefficient doesn't not multiplies an x
    // power, and so should be negated, otherwise we should
    // negate all x powers, and that'd be slower.
    let mut old = -first;
    for coefficient in polynomial.iter().skip(1) {
      if !crate::math::is_zero(*coefficient) {
        // If i is odd.
        // (-k)^n, where k is a positive number is -k, so
        // instead of checking if the signs are opposite we
        // are going to check if the signs are the same as
        // either the sign of the current coefficient is
        // negated or the sign of old is negated.
        if (*coefficient < 0.0 && old < 0.0)
          || (*coefficient > 0.0 && old > 0.0)
        {
          v += 1;
        }
        old = *coefficient;
      }
    }
  }
  return v;
}

/// Returns the biggest (in terms of modulo) non-last
/// coefficient (the last coefficient is the one that
/// accompanies the term a^n).
fn biggest_non_last_coefficient(
  polynomial: &crate::polynomials::Polynomial,
) -> f32 {
  if let Some(first) = polynomial.first() {
    let mut biggest = first.abs();

    // Iterate over all but the last coefficients.
    for coefficient in polynomial.iter().rev().skip(1) {
      if coefficient.abs() > biggest {
        biggest = coefficient.abs();
      }
    }
    return biggest;
  } else {
    return 0.0;
  }
}

/// Returns the modulo of the Cauchy's bound.
/// All the negative roots are between -bound and 0.
/// All the positive roots are between 0 and bound.
pub fn bound(
  polynomial: &crate::polynomials::Polynomial,
) -> Option<f32> {
  if let Some(last) = polynomial.last() {
    return Some(
      (1.0
        + biggest_non_last_coefficient(polynomial) / last)
        .abs(),
    );
  } else {
    return None;
  }
}
