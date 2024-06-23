use bigdecimal::{BigDecimal, One, Zero};

pub fn bernoulli_number(n: usize) -> BigDecimal {
  // This is a simplified implementation of Bernoulli numbers.
  // For a more complete implementation, you'd need to calculate more terms.
  let one = BigDecimal::one();
  let minus_one = BigDecimal::from(-1);
  let bd_30 = BigDecimal::from(30);
  match n {
      0 => one,
      1 => minus_one / BigDecimal::from(2),
      2 => one / BigDecimal::from(6),
      4 => minus_one / bd_30,
      6 => one / BigDecimal::from(42),
      8 => minus_one / bd_30,
      _ => BigDecimal::zero(),
  }
}