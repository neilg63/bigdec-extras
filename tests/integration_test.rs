use bigdec_extras::*;

#[cfg(test)]

#[test]
pub fn should_calculate_cosine() {
  let angle = 30.to_bigdecimal().to_radians();

  let cosine_60 = angle.cos();
  // If cosine is squared it should equal 3/4, but is still affected by rounding errors in decimal
  // with an arbitrary high-precesion PI value
  // smart_round with a max of 50 decimal places should resolve this inconsistency
  let cosime_60_squared = cosine_60.square().smart_round(50);

  assert_eq!(cosime_60_squared, 0.75.to_bigdecimal());

}

#[test]
pub fn should_calculate_large_decimal_power() {
  let exponent = 98.75.to_bigdecimal();

  let base_value = 98.42.to_bigdecimal();
  let result = base_value.powf(&exponent);
  let billion = 1_000_000_000.to_bigdecimal();
  let quintillion = &billion * &billion; // 10^18
  assert!(result > quintillion);

}

#[test]
pub fn should_calculate_root_correctly() {
  let base_value = 7.to_bigdecimal();

  let result = base_value.root(4);
  
  assert_eq!(result.round(8), 1.62657656.to_bigdecimal());


  let base_value = 32.to_bigdecimal();

  let result = base_value.root(5);
  print!("{} powf: {}", result, 9f64.powf(1.2));
  assert_eq!(result, 2.to_bigdecimal());
  
}

/* #[test]
pub fn should_calculate_tangent_and_cotangent() {
  let angle = 45.to_bigdecimal().to_radians();

  let tan_75 = angle.tan();
  // If cosine is squared it should equal 3/4, but is still affected by rounding errors in decimal
  // with an arbitrary high-precesion PI value
  // smart_round with a max of 50 decimal places should resolve this inconsistency
  let tan_75_cotan = tan_75.cotan();
  println!("t {} {}", tan_75, &angle);
  assert_eq!(tan_75_cotan, angle);

} */