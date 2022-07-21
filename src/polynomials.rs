pub struct Polynomial {
  coefficients: std::collections::LinkedList<f32>,
}

impl Polynomial {
  /// Create a new polynomial.
  pub fn new(
    coefficients: std::collections::LinkedList<f32>,
  ) -> Polynomial {
    return Polynomial { coefficients };
  }

  /// Remove the leading zeros of the polynomial, both from
  /// the right and the left.
  /// For example: 0 + 0 x¹ + 1 x² + 0 x³ becomes 1.
  pub fn simplify(&mut self) {
    while let Some(coefficient) = self.first() {
      // If it isn't 0 we shoudn't remove it.
      if !crate::math::is_zero(*coefficient) {
        break;
      }
      self.coefficients.pop_front();
    }
    while let Some(coefficient) = self.last() {
      if !crate::math::is_zero(*coefficient) {
        break;
      }
      self.coefficients.pop_back();
    }
  }

  /// Evaluate p(x).
  pub fn get_value_at(&self, x: f32) -> f32 {
    let mut result: f32 = 0.0;
    let mut x_pow: f32 = 1.0;

    for coefficient in &self.coefficients {
      result += coefficient * x_pow;
      x_pow *= x;
    }

    return result;
  }

  // Expose functions of self.linked_list.
  pub fn len(&self) -> usize {
    return self.coefficients.len();
  }
  pub fn iter(
    &self,
  ) -> std::collections::linked_list::Iter<'_, f32> {
    return self.coefficients.iter();
  }
  pub fn first(&self) -> Option<&f32> {
    return self.coefficients.front();
  }
  pub fn last(&self) -> Option<&f32> {
    return self.coefficients.back();
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn create_polynomial() {
    let linked_list =
      std::collections::LinkedList::<f32>::new();

    let polynomial_1 = crate::polynomials::Polynomial {
      coefficients: linked_list.clone(),
    };
    let polynomial_2 = crate::polynomials::Polynomial::new(
      linked_list.clone(),
    );

    assert_eq!(
      polynomial_1.coefficients,
      polynomial_2.coefficients
    );
  }

  #[test]
  fn evaluate_polynomial() {
    let linked_list =
      // c = 1, b = 2, a = -3
      // a x² + b x + c
      std::collections::LinkedList::from([1.0, 2.0, -3.0]);

    let polynomial = crate::polynomials::Polynomial {
      coefficients: linked_list,
    };

    assert!(crate::math::is_zero(
      //    p(x) - y = 0
      // => p(x) = y
      // We check if this is 0 instead of checking if both
      // are equal because of floating point errors.
      polynomial.get_value_at(-1.0) + 4.0
    ))
  }
}
