/* Standard mathematical functions for ratlab.
 */

use num::*;
use std::f64::consts;

// Private helper methods

/* Converts a generic to an integer or throws an error. */
fn int_from_generic<T: cast::ToPrimitive>(n: &T) -> i64 {
    match n.to_i64() {
        None => panic!("Generic is not an integer."),
        Some(val) => val,
    } 
}

/* Converts a generic to a float or throws an error. */
fn float_from_generic<T: cast::ToPrimitive>(x: &T) -> f64 {
    match x.to_f64() {
        None => panic!("Factorial is not a floating point number."),
        Some(val) => val,
    }
}

/* absolute()
 * -----
 * Takes the absolute value of the provided number.
 *
 * n: A number to find the absolute value of.
 *
 * Returns: A float representing the absolute value of n.
 */
pub fn absolute<T: cast::ToPrimitive>(x: T) -> f64 {
    let x_float: f64 = float_from_generic(&x);
    if x_float >= 0f64 {x_float} else {-x_float}
}

/* average()
 * -----
 * Calculates the average value of the vector provided.
 *
 * values: A vector of numbers to average.
 *
 * Returns: A float equal to the average of values.
 */
pub fn average<T: cast::ToPrimitive>(values: Vec<T>) -> f64 {
    let mut sum: f64 = 0f64;
    let mut count: i32 = 0;
    for value in values {
        let val_num: f64 = float_from_generic(&value);
        sum += val_num;
        count += 1;
    }
    sum / (count as f64)
}

/* ceil()
 * -----
 * Finds the ceiling of the provided value (rounds up to the next int)
 * 
 * x: The value to round up.
 *
 * Returns: The integer equal to x rounded up.
 */
pub fn ceil<T: cast::ToPrimitive>(x: T) -> i64 {
    let x_int: i64 = int_from_generic(&x);
    let fix_x: i64 = fix(x);
    if x_int < 0 || x_int == fix_x {
        return fix_x;
    } else {
        return fix_x + 1;
    }
}

/* cosine()
 * -----
 * Calculates the value of cos(x).
 *
 * x: The value to find the cosine of.
 *
 * Returns: The value cos(x).
 */
pub fn cosine<T: cast::ToPrimitive>(x: T) -> f64 {
    let x_float: f64 = float_from_generic(&x);
    x_float.cos()
}

/* exponential()
 * -----
 * Finds the value of the given power rased to the given base, or to Euler's
 * number if no base is provided.
 *
 * power: The power to raise the base to (currently must be an integer).
 * base: An Option<> to specify a base, if None provided, defaults to e.
 *
 * Returns: The value of base ^ power.
 */
pub fn exponential<P: cast::ToPrimitive + PrimInt, B: cast::ToPrimitive>(power: P, base: Option<B>) -> f64 {
    let i_power: i64 = int_from_generic(&power);
    let num_base: f64 = match base {
        None => consts::E,
        Some(val) => float_from_generic(&val),
    };
    let mut result: f64 = 1f64;
    for i in 1..i_power {
        result *= num_base;
    }
    result
}

/* factorial()
 * -----
 * Calculates the factorial of an unsigned integer.
 * 
 * n: The number to calculate the factorial of.
 *
 * Returns: An unsigned integer equal to the factorial of n.
 */
pub fn factorial<T: PrimInt + Unsigned + cast::ToPrimitive>(n: T) -> u64 {
    let n_int: i64 = int_from_generic(&n);
    if n_int < 0 {
        return 0;
    }
    let mut fact: u64 = 1;
    for i in 1..(n_int as u64) {
        fact *= i;
    }
    fact
}

/* fix()
 * -----
 * Finds the fix of the value provided (strips away any non-integer component).
 * 
 * x: The number to strip the decimal from.
 *
 * Returns: The integer component of x.
 */
pub fn fix<T: cast::ToPrimitive>(x: T) -> i64 {
    let x_float: f64 = float_from_generic(&x);
    int_from_generic(&(x_float % 1f64))
}

/* floor()
 * -----
 * Calculates the floor of the value provided (rounds down to the next int).
 * 
 * x: The number to round down.
 *
 * Returns: The resultant floor.
 */
pub fn floor<T: cast::ToPrimitive>(x: T) -> i64 {
    let x_int: i64 = int_from_generic(&x);
    let fix_x: i64 = fix(x);
    if x_int > 0 || x_int == fix_x {
        return fix_x;
    } else {
        return fix_x - 1;
    }
}

/* log_e()
 * -----
 * Takes the natural logarithm.
 *
 * x: The value to log.
 *
 * Returns: The natural logarithm of x, ln(x).
 */
pub fn log_e<T: cast::ToPrimitive>(x: T) -> f64 {
    let x_float: f64 = float_from_generic(&x);
    x_float.ln()
}

/* log_2()
 * -----
 * Takes the base 2 logarithm.
 *
 * x: The value to log.
 *
 * Returns: The base two logarithm of x, log2(x).
 */
pub fn log_2<T: cast::ToPrimitive>(x: T) -> f64 {
    let x_float: f64 = float_from_generic(&x);
    x_float.log2()
}

/* log_10()
 * -----
 * Takes the base 10 logarithm.
 *
 * x: The value to log.
 *
 * Returns: The base 10 logarithm of x, ln(x).
 */
pub fn log_10<T: cast::ToPrimitive>(x: T) -> f64 {
    let x_float: f64 = float_from_generic(&x);
    x_float.log10()
}

/* logarithm()
 * -----
 * The general logarithm.
 *
 * x: The value to log.
 * b: The base of the logarithm.
 *
 * Returns: The base n logarithm of x, log_n(x).
 */
pub fn logarithm<T: cast::ToPrimitive>(x: T, n: T) -> f64 {
    let x_float: f64 = float_from_generic(&x);
    let n_float: f64 = float_from_generic(&n);
    x_float.log(n_float)
}

/* modulus()
 * -----
 * Calculates the value of x (mod div).
 *
 * x: The value to take the modulus of.
 * div: The divisor to take the remainder of.
 *
 * Returns: The value of x (mod div).
 */
pub fn modulus<T: cast::ToPrimitive>(x: T, div: T) -> f64 {
    let x_float = float_from_generic(&x);
    let div_float = float_from_generic(&div);
    let rem: f64 = x_float % div_float;
    if rem < 0f64 {
        return rem + div_float;
    } else {
        return rem;
    }
}

/* round()
 * -----
 * Rounds the provided value to the nearest integer.
 *
 * x: The value to round.
 *
 * Returns: The closest integer to x.
 */
pub fn round<T: cast::ToPrimitive>(x: T) -> i64 {
    let x_float = float_from_generic(&x);
    if x_float - (x_float % 1f64) < 0.5 {
        return floor(x);
    } else {
        return ceil(x);
    }
}

/* sign()
 * -----
 * Calculates the sign of the given number.
 *
 * n: The number to find the sign of.
 *
 * Returns: 1 if positive, -1 if negative, 0 if zero
 */
pub fn sign<T: cast::ToPrimitive>(n: T) -> i8 {
    let n_int: f64 = float_from_generic(&n);
    if n_int == 0f64 {
        return 0;
    } else if n_int / absolute(n_int) > 0f64 {
        return 1;
    } else {
        return -1;
    }
}

/* sine()
 * -----
 * Calculates the value of sin(x).
 *
 * x: The value to find the sine of.
 *
 * Returns: The value sin(x).
 */
pub fn sine<T: cast::ToPrimitive>(x: T) -> f64 {
    let x_float: f64 = float_from_generic(&x);
    x_float.sin()
}

/* square_root()
 * -----
 * Calculates the square root of the given value.
 * 
 * x: The value to find the root of.
 *
 * Returns: The square root of x.
 */
pub fn numerical_root<T: cast::ToPrimitive>(x: T) -> f64 {
    let float_x: f64 = float_from_generic(&x);
    float_x.sqrt()
}

/* tangent()
 * -----
 * Calculates the value of tan(x).
 *
 * x: The value to find the tangent of.
 *
 * Returns: The value tan(x).
 */
pub fn tangent<T: cast::ToPrimitive>(x: T) -> f64 {
    let x_float: f64 = float_from_generic(&x);
    if (x_float - consts::PI/2f64) % (2f64 * consts::PI) == 0f64 {
        return f64::INFINITY;
    } else if (x_float - consts::PI/2f64) % (consts::PI) == 0f64 {
        return f64::NEG_INFINITY;
    }
    x_float.tan()
}
