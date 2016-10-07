#![allow(non_camel_case_types)]

pub type size_t = usize;
pub type dev_t = u32;
pub type gid_t = u32;
pub type uid_t = u32;
pub type pid_t = i32;
pub type DIR = ::std::os::raw::c_void; // opaque type
