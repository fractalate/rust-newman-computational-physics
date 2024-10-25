pub fn integrate_trapezoidal_rule<F>(a: f64, b: f64, n: usize, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let h = (b - a) / (n as f64);
  let mut total = 0.0;

  total += f(a);

  for i in 1..n {
    let x = a + h*(i as f64);
    total += 2.0*f(x);
  }

  total += f(b);
  total *= h / 2.0;

  total
}