/* Standard mathematical functions for ratlab.
 */

use num::*;
use std::f64::consts;

// Private helper methods

/* Converts a generic to an integer or throws an error. */
fn int_from_generic<T: cast::ToPrimitive>(n: T) -> i64 {
    match n.to_i64() {
        None => panic!("Generic is not an integer."),
        Some(val) => val,
    } 
}

/* Converts a generic to a float or throws an error. */
fn float_from_generic<T: cast::ToPrimitive>(n: T) -> f64 {
    match n.to_f64() {
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
pub fn absolute<T: cast::ToPrimitive>(n: T) -> f64 {
    let n_float: f64 = float_from_generic(n);
    if n_float >= 0f64 {n_float} else {-n_float}
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
        let val_num: f64 = float_from_generic(value);
        sum += val_num;
        count += 1;
    }
    sum / (count as f64)
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
    let i_power: i64 = int_from_generic(power);
    let num_base: f64 = match base {
        None => consts::E,
        Some(val) => float_from_generic(val),
    };
    let mut result: f64 = 1f64;
    for i in 1..i_power {
        result *= num_base;
    }
    result
}

pub fn factorial<T: PrimInt + Unsigned + cast::ToPrimitive>(n: T) -> u64 {
    let n_int: i64 = int_from_generic(n);
    if n_int < 0 {
        return 0;
    }
    let mut fact: u64 = 1;
    for i in 1..(n_int as u64) {
        fact *= i;
    }
    fact
}
