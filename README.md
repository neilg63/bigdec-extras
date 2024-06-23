# BigDecimal Extras

This crate adds the core power and trigonomtric methods to the BigDecimal crate using a pre-calculated PI value accurate to 100 decimal places.

* ```pow(n: i32)``` acts only integers and is always faster than the equivalent with a BigDecimal exponent
* ```powf(big: BigDecimal)``` works with any BigDecimal exponent, but may be slow
* ```root(n: i32)``` works only with integers, but faster than powf() with a BigDecimal exponent
* ```smart_round(n: u8)``` Round the nearest n digits and then remove all trailing zeroes after the last signifying decimal digit.
* ```calc_pi()``` Calculates PI to high precision from a constant string
* ```sin()``` sine of angle in radians
* ```cos()``` cosine of angle in radians
* ```tan()``` tangent of angle in radians