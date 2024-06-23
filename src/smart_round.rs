use std::str::FromStr;
use bigdecimal::BigDecimal;

// Simply BigDecimal to last significant decimal digit within the specified precision
pub trait SmartRound {
  fn smart_round(&self, prec: u8) -> BigDecimal;
}

/// Implement for BigDecimal
impl SmartRound for BigDecimal {
  fn smart_round(&self, prec: u8) -> BigDecimal {
      let rounded_string = self.round(prec as i64).to_string();
      let rounded_trimmed = rounded_string.trim_end_matches("0");
      let smart_trimed = if rounded_trimmed.ends_with(".") {
          rounded_trimmed.trim_end_matches(".")
      } else {
          rounded_trimmed
      };
      BigDecimal::from_str(smart_trimed).unwrap_or(BigDecimal::from(0))
  }
}

