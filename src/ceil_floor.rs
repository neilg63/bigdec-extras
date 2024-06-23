pub use bigdecimal::BigDecimal;

// Implement ceil() and floor() methods
trait CeilFloor {
  // Will alway round down to the integer below
  fn floor(&self) -> Self;

  // Will alway round up to the integer above
  fn ceil(&self) -> Self;
}

/// Implement for BigDecimal
impl CeilFloor for BigDecimal {
  fn floor(&self) -> Self {
      self.with_scale_round(0, bigdecimal::RoundingMode::Floor)
  }

  fn ceil(&self) -> Self {
      self.with_scale_round(0, bigdecimal::RoundingMode::Ceiling)
  }
}