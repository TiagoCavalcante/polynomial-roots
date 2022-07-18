# polynomial-roots

Find the real roots of huge polynomials in milliseconds

## How to compile?

```sh
cargo build --release
```

## How to use?

The coefficients are in degree-ascending order, that is: $x^0 + x^1 + x^2 + x^3 + \cdots$  
The coefficients are passed space-separated, therefore to solve the polynomial $3 - 9x + x^3$ you need to run the program the following way:
```sh
$ ./target/release/polynomial-roots
3 -9 0 1
{ -3.154523; 0.33760893; 2.816914; }
```

(note: don't forget the zero coefficients)

## How it works?

The roots are found using the linear formula ($\dfrac{-b}{a}$), the quadratic formula ($\dfrac{-b \pm \sqrt{b^2 - 4 a c}}{2 a}$), or the [false position method](https://en.wikipedia.org/wiki/Regula_falsi) over the [Cauchy's bound](https://en.wikipedia.org/wiki/Geometrical_properties_of_polynomial_roots#Lagrange's_and_Cauchy's_bounds). It can solve polynomials of any degree.

## I'm not getting all the roots, what should I do?

If you aren't getting all the roots you should modify the constants from the file `src/constants.rs`. You can DECREASE `PARTITION_SIZE` and INCREASE `ITERATIONS`. That will make the program slower, but with the right tweaks it can solve any polynomial.
