use std::str::FromStr;

use bigdecimal::{BigDecimal, One, Zero};
use crate::{bernoulli_number, factorial, pi::*, FromBigFraction};
use crate::power::*;

pub trait BigTrigonometry {
  fn sin(&self) -> BigDecimal;

  fn cos(&self) -> BigDecimal;

  fn tan(&self) -> BigDecimal;

  fn cotan(&self) -> BigDecimal;
}

impl BigTrigonometry for BigDecimal {
  fn sin(&self) -> BigDecimal {
      let mut result = self.clone();
      let mut term = self.clone();
      let x_squared = self * self;
      
      for i in 1..20 { // Adjust the number of iterations for desired precision
          term = -term * &x_squared / BigDecimal::from((2 * i) * (2 * i + 1));
          result += &term;
      }
      
      result.round(max_pi_round())
  }

  fn cos(&self) -> BigDecimal {
      let mut result = BigDecimal::one();
      let mut term = BigDecimal::one();
      let x_squared = self * self;
      
      for i in 1..20 { // Adjust the number of iterations for desired precision
          term = -term * &x_squared / BigDecimal::from((2 * i - 1) * (2 * i));
          result += &term;
      }
      
      result.round(max_pi_round())
  }

  fn tan(&self) -> BigDecimal {
    let pi = Self::calc_pi();
    let half_pi = &pi / BigDecimal::from(2);
    // let two_pi = &pi * 2;
    
    // Normalize input to [-π/2, π/2]
    let mut x = self % &pi;
    if x > half_pi {
        x = &pi - &x;
    } else if x < -&half_pi {
        x = -&pi - &x;
    }

    // Check for undefined points
    if x.abs() > BigDecimal::from_str("1.570796").unwrap() { // slightly less than π/2
      return BigDecimal::infinity(); // Represent infinity
    }

    // For small angles, return x for better accuracy
    if x.abs() < BigDecimal::from_str("1e-10").unwrap() {
      return x;
    }

    let x_squared = &x * &x;
    let mut result = x.clone();
    let mut term = x.clone();
    let mut n: u64 = 1;
    
    let precision = BigDecimal::from_str("1e-50").unwrap();
    loop {
      let bernoulli = match n {
        1 => BigDecimal::from_fraction(1, 6),
        2 => BigDecimal::from_fraction(1, 15),
        3 => BigDecimal::from_fraction(2, 45),
        4 => BigDecimal::from_fraction(1, 35),
        5 => BigDecimal::from_fraction(2, 105),
        _ => break, // Stop after a few terms for efficiency
      };

      term = &term * &x_squared * &bernoulli * BigDecimal::from(2 * n * (2 * n - 1));
      
      if term.abs() < precision {
          break;
      }
      
      result += &term;
      n += 1;
    }
    result.round(max_pi_round())
  }

  fn cotan(&self) -> BigDecimal {
      let small = BigDecimal::epsilon();
  
      // Define pi
      let pi = Self::calc_pi();
      
      // Normalize x to be within [0, 2π)
      let two_pi = &pi * BigDecimal::from(2);
      let x_normalized = self % &two_pi;
      
      // Check for undefined points (multiples of π)
      if (&x_normalized % &pi).abs() < small {
          // Return a very large number to represent "infinity"
          return BigDecimal::infinity();
      }
      
      let t = x_normalized.tan();
      
      // Handle very small values of tan(x)
      if t.abs() < small {
          if t >= BigDecimal::zero() {
              BigDecimal::infinity()
          } else {
              BigDecimal::minus_infinity()
          }
      } else {
          BigDecimal::one() / t
      }
  }
}

