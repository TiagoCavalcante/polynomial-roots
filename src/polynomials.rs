pub struct Polynomial {
  pub linked_list: std::collections::LinkedList<f32>,
}

impl Polynomial {
  /// Remove the leading zeros of the polynomial, both from
  /// the right and the left.
  /// For example: 0 + 0 x¹ + 1 x² + 0 x³ becomes 1.
  pub fn simplify(&mut self) {
    while let Some(coefficient) = self.front() {
      // If it isn't 0 we shoudn't remove it.
      if !crate::math::is_zero(*coefficient) {
        break;
      }
      self.linked_list.pop_front();
    }
    while let Some(coefficient) = self.back() {
      if !crate::math::is_zero(*coefficient) {
        break;
      }

      self.linked_list.pop_back();
    }
  }

  /// Evaluate p(x).
  pub fn get_value_at(&self, x: f32) -> f32 {
    let mut result: f32 = 0.0;
    let mut x_pow: f32 = 1.0;

    for coefficient in &self.linked_list {
      result += coefficient * x_pow;
      x_pow *= x;
    }

    return result;
  }

  // Expose functions of self.linked_list.
  pub fn len(&self) -> usize {
    return self.linked_list.len();
  }
  pub fn iter(
    &self,
  ) -> std::collections::linked_list::Iter<'_, f32> {
    return self.linked_list.iter();
  }
  pub fn front(&self) -> Option<&f32> {
    return self.linked_list.front();
  }
  pub fn back(&self) -> Option<&f32> {
    return self.linked_list.back();
  }
}
