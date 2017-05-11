/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use ffi::dev_t;
extern "C" {
    pub fn dev_to_tty(ret: *mut ::std::os::raw::c_char,
                      chop: ::std::os::raw::c_uint,
                      dev_t_dev: dev_t,
                      pid: ::std::os::raw::c_int,
                      flags: ::std::os::raw::c_uint)
                      -> ::std::os::raw::c_uint;
    pub fn tty_to_dev(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}

pub static ABBREV_DEV: ::std::os::raw::c_int = 1; /* remove /dev/         */
pub static ABBREV_TTY: ::std::os::raw::c_int = 2; /* remove tty           */
pub static ABBREV_PTS: ::std::os::raw::c_int = 4; /* remove pts/          */
