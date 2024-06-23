use std::str::FromStr;
use bigdecimal::{num_traits::Float, BigDecimal, FromPrimitive, ParseBigDecimalError, Zero};

// Trait to convert to big decimals from primitive numeric type
pub trait ToBigDecimal {
  fn to_bigdecimal(&self) -> BigDecimal;
}

/// Cast floats to BigDecimal
pub fn float_to_big_decimal<T:Float + ToString>(fl_val: T) -> Result<BigDecimal, ParseBigDecimalError> {
  BigDecimal::from_str(&fl_val.to_string())
}

/// Implement for 32-bit Floats
/// NB: This will always succeeed
impl ToBigDecimal for f32 {
  fn to_bigdecimal(&self) -> BigDecimal {
      if let Ok(b_val) = float_to_big_decimal(*self) {
          b_val
      } else {
          BigDecimal::from_f32(*self).unwrap_or(BigDecimal::zero())
      }
  }
}

/// Implement for 64-bit Floats
/// NB: This will always succeeed
impl ToBigDecimal for f64 {
  fn to_bigdecimal(&self) -> BigDecimal {
      if let Ok(b_val) = float_to_big_decimal(*self) {
          b_val
      } else {
          BigDecimal::from_f64(*self).unwrap_or(BigDecimal::zero())
      }
  }
}

/// Implement for 32-bit integers, default when not specified
/// NB: This will always succeeed
impl ToBigDecimal for i32 {
  fn to_bigdecimal(&self) -> BigDecimal {
    BigDecimal::from(self)
  }
}

/// Implement for 64-bit integers
/// NB: This will always succeeed
impl ToBigDecimal for i64 {
  fn to_bigdecimal(&self) -> BigDecimal {
    BigDecimal::from(self)
  }
}

/// Implement for unsigned 64-bit integers
/// NB: This will always succeeed
impl ToBigDecimal for u64 {
  fn to_bigdecimal(&self) -> BigDecimal {
    BigDecimal::from(self)
  }
}

/// Implement for 128-bit integers
/// NB: This will always succeeed
impl ToBigDecimal for i128 {
  fn to_bigdecimal(&self) -> BigDecimal {
    BigDecimal::from(self)
  }
}

/// Implement for unsigned 128-bit integers
/// NB: This will always succeeed
impl ToBigDecimal for u128 {
  fn to_bigdecimal(&self) -> BigDecimal {
    BigDecimal::from(self)
  }
}
