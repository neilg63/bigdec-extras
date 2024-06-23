use bigdecimal::{BigDecimal, One};

pub fn factorial(n: u32) -> BigDecimal {
  (1..=n).fold(BigDecimal::one(), |acc, x| acc * BigDecimal::from(x))
}

