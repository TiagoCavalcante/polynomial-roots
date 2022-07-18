pub fn get_roots(
  polynomial: &crate::polynomials::Polynomial,
  has_zero: bool,
) -> Vec<f32> {
  if let (Some(a), Some(b), Some(c)) = (
    polynomial.back(),
    polynomial.iter().nth(1),
    polynomial.front(),
  ) {
    let delta = b * b - 4.0 * a * c;

    if crate::math::is_zero(delta) {
      let root = -b / (2.0 * a);
      if has_zero {
        if root > 0.0 {
          return vec![0.0, root];
        } else {
          return vec![root, 0.0];
        }
      } else {
        return vec![root];
      }
    } else if delta > 0.0 {
      let root1 = (-b - delta.sqrt()) / (2.0 * a);
      let root2 = (-b + delta.sqrt()) / (2.0 * a);
      if has_zero {
        if root2 < 0.0 {
          // Both are less than 0.
          return vec![root1, root2, 0.0];
        } else if root1 < 0.0 {
          // Just the 1st is less than 0.
          return vec![root1, 0.0, root2];
        } else {
          // Both are greater than 0.
          return vec![0.0, root1, root2];
        }
      } else {
        return vec![root1, root2];
      }
    } else {
      if has_zero {
        return vec![0.0];
      } else {
        return vec![];
      }
    }
  } else {
    return vec![];
  }
}
