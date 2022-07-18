/// Checks if |number| <= EPSILON.
#[inline]
pub fn is_zero(number: f32) -> bool {
  return -crate::constants::EPSILON < number
    && number < crate::constants::EPSILON;
}
