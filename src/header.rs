/* Library file to set defaults for the language.
 */

pub mod glitter {
    pub const INDENT: &str = "\t  ";
    pub const INDEX_OFFSET: i8 = 2;
}

pub mod types {
    pub const CHAR: &str = "ch_ter";
    pub const BOOL: &str = "_log-boo_";
    pub const INT_8: &str = "8b-int";
    pub const UINT_8: &str = "unt_8b";
    pub const INT_16: &str = "2B-int";
    pub const UINT_16: &str = "unt_2B";
    pub const INT_32: &str = "8nybl-int";
    pub const UINT_32: &str = "unt_8nybl";
    pub const INT_64: &str = "8*8b-int";
    pub const UINT_64: &str = "unt_8*8b";
    pub const FLOAT: &str = "si_flo";
    pub const DOUBLE: &str = "do-flo";
    pub const STRING: &str = "&ret_hc";
}
