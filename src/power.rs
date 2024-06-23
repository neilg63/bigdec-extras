use std::str::FromStr;
use bigdecimal::{BigDecimal, ToPrimitive, One, Zero};

use crate::{FromBigFraction, SmartRound};

pub trait BigDecimalPower {
  fn pow(&self, exponent: i32) -> BigDecimal;
  fn root(&self, exponent: i32) -> BigDecimal;
  fn powf(&self, exponent: &BigDecimal) -> BigDecimal;
  fn ln(&self) -> BigDecimal;
  fn has_fractional_part(&self) -> bool;
}

impl BigDecimalPower for BigDecimal {
  fn powf(&self, exponent: &BigDecimal) -> BigDecimal {
      if exponent.is_zero() {
          return BigDecimal::one();
      }
      if self.is_zero() {
          return BigDecimal::zero();
      }
      if *exponent == BigDecimal::one() {
          return self.clone();
      }

      // For integer exponents, use the more efficient integer power method
      if !exponent.has_fractional_part() {
          return self.pow(exponent.to_i32().unwrap());
      }

      let result = (self.ln() * exponent).smart_round(50);
      
      result.fast_exp()
  }

  fn ln(&self) -> BigDecimal {
      if *self <= BigDecimal::zero() {
          panic!("Logarithm is undefined for non-positive numbers");
      }

      let precision = 100; // Adjust this for desired precision
      let one = BigDecimal::one();
      let two = BigDecimal::from(2);

      // Use the formula: ln(x) = 2 * artanh((x-1)/(x+1))
      let x = (self - &one) / (self + &one);
      let mut sum = BigDecimal::zero();
      let mut term = x.clone();
      let x_squared = &x * &x;
      
      

      for n in 0..precision {
          sum += &term / BigDecimal::from(2 * n + 1);
          term *= &x_squared;
      }
      
      &sum * &two
  }

  // faster power of integer exponents only
  fn pow(&self, exponent: i32) -> BigDecimal {
      if exponent == 0 {
          return BigDecimal::one();
      }
      if exponent < 0 {
          return BigDecimal::one() / self.pow(-exponent);
      }
      let mut result = BigDecimal::one();
      let mut base = self.clone();
      let mut exp = exponent;
      while exp > 0 {
          if exp % 2 == 1 {
              result *= &base;
          }
          base = base.square();
          exp /= 2;
      }
      result
  }

  // faster power of integer exponents only
  fn root(&self, exponent: i32) -> BigDecimal {
    if exponent == 0 {
        return BigDecimal::one();
    }
    if exponent < 0 {
        return BigDecimal::one() / self.root(-exponent);
    }
    let mut result = self.clone();
    let divide_self = exponent % 2 == 1;
    let num_sqrts = exponent / 2;
    let last_index = num_sqrts - 1;
    for i in 0..num_sqrts {
        if let Some(sqrt_val) = result.sqrt() {
            result = sqrt_val;
            if divide_self && i == last_index {
                result = result.powf(&BigDecimal::from_fraction((i * 2) as u64, exponent as u64));
            }
        }
    }
    result
}

  // check if the BigDecimal has decimal places
  fn has_fractional_part(&self) -> bool {
      let (_, exp) = self.clone().into_bigint_and_exponent();
      exp < 0
  }
}

pub trait BigDecimalInfinity {
  // very large value considered infinity
  fn infinity() -> BigDecimal;
  /// very large negative considted negative infinity
  fn minus_infinity() -> BigDecimal;
  /// very small value near zero
  fn epsilon() -> BigDecimal;
   /// very small negative value near zero
   fn minus_epsilon() -> BigDecimal;
}

impl BigDecimalInfinity for BigDecimal {
  fn infinity() -> BigDecimal {
      BigDecimal::from_str("1e1000").unwrap()
  }

  fn minus_infinity() -> BigDecimal {
      BigDecimal::from_str("-1e1000").unwrap()
  }

  fn epsilon() -> BigDecimal {
      BigDecimal::from_str("1e-1000").unwrap()
  }

  fn minus_epsilon() -> BigDecimal {
      BigDecimal::from_str("-1e-1000").unwrap()
  }
}


pub trait FastExponent {
    fn fast_exp(&self) -> BigDecimal;
}

impl FastExponent for BigDecimal {
    fn fast_exp(&self) -> BigDecimal {
        let one = BigDecimal::one();
        let zero = BigDecimal::zero();

        if *self == zero {
            return one;
        }

        // Define a precision for the calculation
        let precision = BigDecimal::from_str("1e-50").unwrap();

        // Use scaling and squaring method
        let mut x = self.clone();
        let mut n = 0;
        while x.abs() > one {
            x = x / 2;
            n += 1;
        }

        let mut result = one.clone();
        let mut term = one.clone();
        let mut i = 1;

        loop {
            term = term * &x / BigDecimal::from(i);
            let next_result = &result + &term;
            if (&next_result - &result).abs() < precision {
                result = next_result;
                break;
            }
            result = next_result;
            i += 1;
        }

        // Square the result n times
        for _ in 0..n {
            result = &result * &result;
        }

        result
    }
}
