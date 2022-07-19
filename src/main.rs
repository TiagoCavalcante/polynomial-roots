//! Blazingly fast polynomial root finder

mod constants;
mod false_position;
mod linear;
mod math;
mod polynomials;
mod quadratic;

/// Get the coefficients of the polynomial in exponent
/// increasing order and print the roots of the polynomial,
/// for example:
/// ```sh
/// sh$ ./target/release/polynomial-roots
/// 0 1 2 1
/// { -1; 0; }
/// ```
/// This function will write an error and exit with status
/// code 1 if it can't read from stdin.
fn main() {
  let mut buffer = String::new();
  match std::io::stdin().read_line(&mut buffer) {
    Ok(_) => {}
    Err(error) => {
      println!("error: {error}");
      std::process::exit(1);
    }
  }

  let mut polynomial = polynomials::Polynomial::new(
    buffer
      .split_whitespace()
      .filter_map(|s| s.parse().ok())
      .collect(),
  );

  // We may "loose" the root 0 when simplifying the
  // polynomial, e.g.:
  // 0 is a root of x² - x = 0, but isn't a root of the
  // equivalent polynomial x - 1 = 0.
  // If the last term is 0, 0 certainly is a root, as proved
  // bellow:
  //      cₙ xⁿ + ··· + c₁ x¹ + c₀ = 0
  //   => cₙ xⁿ + ··· + c₁ x¹ = -c₀
  //   => x¹ (cₙ xⁿ⁻¹ + ··· + c₁ x⁰) = -c₀
  //   => 0 = -c₀
  //   => c₀ = 0
  let has_zero = match polynomial.first() {
    Some(first) => math::is_zero(*first),
    _ => false,
  };

  polynomial.simplify();

  let roots = match polynomial.len() {
    0 | 1 if has_zero => vec![0.0],
    0 | 1 => vec![],
    2 => linear::get_roots(&polynomial, has_zero),
    3 => quadratic::get_roots(&polynomial, has_zero),
    _ => false_position::get_roots(&polynomial, has_zero),
  };

  // Print the roots.
  print!("{{ ");
  for root in &roots {
    print!("{root}; ");
  }
  print!("}}\n");
}
