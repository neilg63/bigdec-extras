use bigdecimal::BigDecimal;

pub trait FromBigFraction {

  fn from_fraction(multiple: u64, divisor: u64) -> BigDecimal {
      BigDecimal::from(multiple) / BigDecimal::from(divisor) 
  }
}

impl FromBigFraction for BigDecimal {}