/// Returns a vector with the real roots of the polynomial
/// obtained with the linear formula.
/// x = -b / a
/// The roots are in increasing order.
pub fn get_roots(
  polynomial: &crate::polynomials::Polynomial,
  has_zero: bool,
) -> Vec<f32> {
  if let (Some(a), Some(b)) =
    (polynomial.last(), polynomial.first())
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

#[cfg(test)]
mod tests {
  #[test]
  fn polynomial_with_one_root() {
    let polynomial = crate::polynomials::Polynomial::new(
      std::collections::LinkedList::from([1.0, 2.0]),
    );
    assert_eq!(
      crate::linear::get_roots(&polynomial, false),
      vec![-0.5],
    );
  }

  #[test]
  fn polynomial_with_two_roots() {
    let polynomial = crate::polynomials::Polynomial::new(
      std::collections::LinkedList::from([-2.0, 1.0]),
    );
    assert_eq!(
      crate::linear::get_roots(&polynomial, true),
      vec![0.0, 2.0],
    );
  }
}
