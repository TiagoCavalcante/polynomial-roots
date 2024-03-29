mod couchy;

/// Get x₀ in false position method.
/// x₀ = [a f(b) - b f(a)] / [f(a) - f(b)]
/// This is also know as regula falsi method.
fn false_position(
  polynomial: &crate::polynomials::Polynomial,
  a: f32,
  b: f32,
) -> f32 {
  let fa = polynomial.get_value_at(a);
  let fb = polynomial.get_value_at(b);

  (a * fb - b * fa) / (fb - fa)
}

/// Get a root in the interval [a, b] using the false
/// position method.
fn get_root(
  polynomial: &crate::polynomials::Polynomial,
  a: f32,
  b: f32,
) -> f32 {
  let mut a = a;
  let mut b = b;
  let mut x = false_position(polynomial, a, b);

  for _ in 0..crate::constants::ITERATIONS {
    if polynomial.get_value_at(x) < 0.0
      && polynomial.get_value_at(b) > 0.0
    {
      a = x;
    } else {
      b = x;
    }

    x = false_position(polynomial, a, b);
  }

  x
}

/// Gets all the roots in the interval [a, b].
/// The function returns when the interval was traveled or
/// the maximum number of roots is reached.
fn get_roots_in_interval(
  roots: &mut Vec<f32>,
  polynomial: &crate::polynomials::Polynomial,
  a: f32,
  b: f32,
  remaining_roots: i32,
) {
  let mut current = a;
  let mut remaining_roots = remaining_roots;
  while current < b {
    let old = current;
    current += crate::constants::PARTITION_SIZE;

    // If one value is bellow the y=0 line and the other
    // value is over it, certainly it crossed the y=0 line.
    if polynomial.get_value_at(old)
      * polynomial.get_value_at(current)
      < 0.0
    {
      let x = get_root(polynomial, old, current);

      // Check if x really is a root.
      if crate::math::is_zero(polynomial.get_value_at(x)) {
        roots.push(x);
        remaining_roots -= 1;

        if remaining_roots == 0 {
          break;
        }
      }
    }
  }
}

/// Returns a vector with the real roots of the polynomial
/// obtained with the false position method.
/// The roots are in increasing order.
pub fn get_roots(
  polynomial: &crate::polynomials::Polynomial,
  has_zero: bool,
) -> Vec<f32> {
  let mut roots = vec![];

  if let Some(upper_bound) = couchy::bound(polynomial) {
    // Negatives first so it is ordered.
    let negative_roots =
      couchy::negative_variations(polynomial);
    if negative_roots > 0 {
      get_roots_in_interval(
        &mut roots,
        polynomial,
        -upper_bound,
        crate::constants::PARTITION_SIZE,
        negative_roots,
      );
    }

    if has_zero {
      roots.push(0.0);
    }

    let positive_roots = couchy::variations(polynomial);
    if positive_roots > 0 {
      get_roots_in_interval(
        &mut roots,
        polynomial,
        0.0,
        upper_bound + crate::constants::EPSILON,
        positive_roots,
      );
    }
  }

  roots
}
