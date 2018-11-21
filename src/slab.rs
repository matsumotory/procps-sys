/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
#[repr(C)]
#[derive(Copy)]
pub struct slab_info {
    pub name: [::std::os::raw::c_char; 128usize],
    pub next: *mut slab_info,
    pub cache_size: ::std::os::raw::c_ulong,
    pub nr_objs: ::std::os::raw::c_uint,
    pub nr_active_objs: ::std::os::raw::c_uint,
    pub obj_size: ::std::os::raw::c_uint,
    pub objs_per_slab: ::std::os::raw::c_uint,
    pub pages_per_slab: ::std::os::raw::c_uint,
    pub nr_slabs: ::std::os::raw::c_uint,
    pub nr_active_slabs: ::std::os::raw::c_uint,
    pub use_: ::std::os::raw::c_uint,
}
impl ::std::clone::Clone for slab_info {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for slab_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct slab_stat {
    pub total_size: ::std::os::raw::c_ulong,
    pub active_size: ::std::os::raw::c_ulong,
    pub nr_objs: ::std::os::raw::c_uint,
    pub nr_active_objs: ::std::os::raw::c_uint,
    pub nr_pages: ::std::os::raw::c_uint,
    pub nr_slabs: ::std::os::raw::c_uint,
    pub nr_active_slabs: ::std::os::raw::c_uint,
    pub nr_caches: ::std::os::raw::c_uint,
    pub nr_active_caches: ::std::os::raw::c_uint,
    pub avg_obj_size: ::std::os::raw::c_uint,
    pub min_obj_size: ::std::os::raw::c_uint,
    pub max_obj_size: ::std::os::raw::c_uint,
}
impl ::std::default::Default for slab_stat {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn put_slabinfo(arg1: *mut slab_info);
    pub fn free_slabinfo(arg1: *mut slab_info);
    pub fn get_slabinfo(arg1: *mut *mut slab_info, arg2: *mut slab_stat) -> ::std::os::raw::c_int;
}

static SLAB_INFO_NAME_LEN: ::std::os::raw::c_int = 128;
