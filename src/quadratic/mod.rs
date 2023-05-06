/// Returns a vector with the real roots of the polynomial
/// obtained with the quadratic formula.
/// Δ = b² - 4 a c
/// x = (-b ± √Δ) / (2 a)
/// The roots are in increasing order.
pub fn get_roots(
  polynomial: &crate::polynomials::Polynomial,
  has_zero: bool,
) -> Vec<f32> {
  let mut polynomial_iter = polynomial.iter();
  // The coefficients are in exponent increasing order, so
  // instead of a x² + b x + c we have c + b x + a x².
  if let (Some(c), Some(b), Some(a)) = (
    polynomial_iter.next(),
    polynomial_iter.next(),
    polynomial_iter.next(),
  ) {
    let delta = b * b - 4.0 * a * c;

    if crate::math::is_zero(delta) {
      let root = -b / (2.0 * a);
      if has_zero {
        if root > 0.0 {
          vec![0.0, root]
        } else {
          vec![root, 0.0]
        }
      } else {
        vec![root]
      }
    } else if delta > 0.0 {
      let root1 = (-b - delta.sqrt()) / (2.0 * a);
      let root2 = (-b + delta.sqrt()) / (2.0 * a);
      if has_zero {
        if root2 < 0.0 {
          // Both are less than 0.
          vec![root1, root2, 0.0]
        } else if root1 < 0.0 {
          // Just the 1st is less than 0.
          vec![root1, 0.0, root2]
        } else {
          // Both are greater than 0.
          vec![0.0, root1, root2]
        }
      } else {
        vec![root1, root2]
      }
    } else {
      if has_zero {
        vec![0.0]
      } else {
        vec![]
      }
    }
  } else {
    vec![]
  }
}
