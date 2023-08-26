/* Standard IO functions for the ratlab library
 */

use std::fmt;

/* disp()
 * -----
 * Display function, prints the value given with a trailing newline.
 *
 * print_val: A generic with a display trait that can be printed.
 */
pub fn disp<T: fmt::Display>(print_val: T) {
    println!("{}", print_val);
}
