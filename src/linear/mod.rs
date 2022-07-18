pub fn get_roots(
  polynomial: &crate::polynomials::Polynomial,
  has_zero: bool,
) -> Vec<f32> {
  if let (Some(a), Some(b)) =
    (polynomial.back(), polynomial.front())
  {
    let root = -b / a;

    if has_zero {
      // Ensure the roots are ordered.
      if root > 0.0 {
        return vec![0.0, root];
      } else {
        return vec![root, 0.0];
      }
    } else {
      return vec![root];
    }
  } else {
    return vec![];
  }
}
