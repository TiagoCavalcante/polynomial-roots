/// Checks if |number| <= EPSILON.
#[inline]
pub fn is_zero(number: f32) -> bool {
  -crate::constants::EPSILON < number
    && number < crate::constants::EPSILON
}

#[cfg(test)]
mod tests {
  #[test]
  fn zero() {
    assert_eq!(crate::math::is_zero(0.0), true);
  }

  #[test]
  fn negative_number() {
    assert_eq!(crate::math::is_zero(-10.0), false);
  }

  #[test]
  fn positive_number() {
    assert_eq!(crate::math::is_zero(10.0), false);
  }
}
