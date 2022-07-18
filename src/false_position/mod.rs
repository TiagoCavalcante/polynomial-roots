mod couchy;

/// Returns -1 if x < 0, 1 if x > 0, 0 otherwise.
fn sign(x: f32) -> i32 {
  if x < 0.0 {
    return -1;
  } else if x > 0.0 {
    return 1;
  } else {
    return 0;
  }
}

/// Get x₀ in false position method.
/// x₀ = [a f(b) - b f(a)] / [f(a) - f(b)]
/// This is also know as regula falsi method.
fn false_position(
  polynomial: &crate::polynomials::Polynomial,
  a: f32,
  b: f32,
) -> f32 {
  let fa = polynomial.get_value_at_x(a);
  let fb = polynomial.get_value_at_x(b);
  return (a * fb - b * fa) / (fb - fa);
}

/// Get a root in the interval [a, b] using the false
/// postion method.
fn get_root(
  polynomial: &crate::polynomials::Polynomial,
  a: f32,
  b: f32,
) -> f32 {
  let mut a = a;
  let mut b = b;
  let mut x = false_position(polynomial, a, b);

  for _ in 0..crate::constants::ITERATIONS {
    if polynomial.get_value_at_x(x) < 0.0
      && polynomial.get_value_at_x(b) > 0.0
    {
      a = x;
    } else {
      b = x;
    }

    x = false_position(polynomial, a, b);
  }

  return x;
}

/// Gets all the roots in the iterval [a, b].
/// The function returns when the interval was traveled or
/// the maximum number of roots is reached.
fn get_roots_in_interval(
  roots: &mut Vec<f32>,
  polynomial: &crate::polynomials::Polynomial,
  a: f32,
  b: f32,
  remaining_roots: i32,
) {
  let mut i = a;
  let mut remaining_roots = remaining_roots;
  while i < b {
    let current = i;
    i += crate::constants::PARTITION_SIZE;

    // If one value is bellow the y=0 line and the other
    // value is over it, certainly it crossed the y=0 line.
    if sign(polynomial.get_value_at_x(current))
      != sign(polynomial.get_value_at_x(i))
    {
      let x = get_root(polynomial, current, i);

      // Check if x really is a root.
      if crate::math::is_zero(polynomial.get_value_at_x(x))
      {
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
  let mut roots = Vec::<f32>::new();

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

  return roots;
}
