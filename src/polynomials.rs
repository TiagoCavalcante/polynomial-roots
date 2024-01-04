pub struct Polynomial {
  coefficients: std::collections::LinkedList<f32>,
}

impl Polynomial {
  /// Create a new polynomial.
  pub const fn new(
    coefficients: std::collections::LinkedList<f32>,
  ) -> Polynomial {
    Polynomial { coefficients }
  }

  /// Remove the leading zeros of the polynomial, both from
  /// the right and the left.
  /// For example: 0 + 0 x¹ + 1 x² + 0 x³ becomes 1.
  pub fn simplify(&mut self) {
    while let Some(&coefficient) = self.first() {
      // If it isn't 0 we shouldn't remove it.
      if !crate::math::is_zero(coefficient) {
        break;
      }
      self.coefficients.pop_front();
    }
    while let Some(&coefficient) = self.last() {
      if !crate::math::is_zero(coefficient) {
        break;
      }
      self.coefficients.pop_back();
    }
  }

  /// Evaluate p(x).
  pub fn get_value_at(&self, x: f32) -> f32 {
    self
      .coefficients
      .iter()
      .fold((0.0, 1.0), |acc, e| {
        (acc.0 + e * acc.1, acc.1 * x)
      })
      .1
  }

  // Expose functions of self.coefficients.
  pub fn len(&self) -> usize {
    self.coefficients.len()
  }
  pub fn iter(
    &self,
  ) -> std::collections::linked_list::Iter<f32> {
    self.coefficients.iter()
  }
  pub fn first(&self) -> Option<&f32> {
    self.coefficients.front()
  }
  pub fn last(&self) -> Option<&f32> {
    self.coefficients.back()
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn create_polynomial() {
    let coefficients = std::collections::LinkedList::new();

    let polynomial_1 = crate::polynomials::Polynomial {
      coefficients: coefficients.clone(),
    };
    let polynomial_2 = crate::polynomials::Polynomial::new(
      coefficients.clone(),
    );

    assert_eq!(
      polynomial_1.coefficients,
      polynomial_2.coefficients
    );
  }

  #[test]
  fn evaluate_polynomial() {
    let coefficients =
      // c = 1, b = 2, a = -3
      // a x² + b x + c
      std::collections::LinkedList::from([1.0, 2.0, -3.0]);

    let polynomial =
      crate::polynomials::Polynomial { coefficients };

    assert!(crate::math::is_zero(
      //    p(x) - y = 0
      // => p(x) = y
      // We check if this is 0 instead of checking if both
      // are equal because of floating point errors.
      polynomial.get_value_at(-1.0) + 4.0
    ));
  }
}
