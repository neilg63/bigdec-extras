
use std::str::FromStr;
use bigdecimal::BigDecimal;


pub const BIG_PI_STR: &str = "3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067";

pub fn max_pi_round() -> i64 {
  (BIG_PI_STR.len() - 1) as i64
}

pub trait BigDecimalPI {
    fn calc_pi() -> BigDecimal {
        BigDecimal::from_str(BIG_PI_STR).unwrap()
    }

    fn calc_pi_fraction(multiple: u32, divisor: u32) -> BigDecimal {
        Self::calc_pi() * BigDecimal::from(multiple) / BigDecimal::from(divisor)
    }

    fn to_pi_multiple(&self) -> BigDecimal;

    fn to_radians(&self) -> BigDecimal;
}

impl BigDecimalPI for BigDecimal  {

    fn to_pi_multiple(&self) -> BigDecimal {
        self * Self::calc_pi()
    }
    
    fn to_radians(&self) -> BigDecimal {
        let pi = BigDecimal::calc_pi();
        self * &pi / BigDecimal::from(180)
    }
}