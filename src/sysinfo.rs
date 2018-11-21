/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub type jiff = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct disk_stat {
    pub reads_sectors: ::std::os::raw::c_ulonglong,
    pub written_sectors: ::std::os::raw::c_ulonglong,
    pub disk_name: [::std::os::raw::c_char; 32usize],
    pub inprogress_IO: ::std::os::raw::c_uint,
    pub merged_reads: ::std::os::raw::c_uint,
    pub merged_writes: ::std::os::raw::c_uint,
    pub milli_reading: ::std::os::raw::c_uint,
    pub milli_spent_IO: ::std::os::raw::c_uint,
    pub milli_writing: ::std::os::raw::c_uint,
    pub partitions: ::std::os::raw::c_uint,
    pub reads: ::std::os::raw::c_uint,
    pub weighted_milli_spent_IO: ::std::os::raw::c_uint,
    pub writes: ::std::os::raw::c_uint,
}
impl ::std::default::Default for disk_stat {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct partition_stat {
    pub partition_name: [::std::os::raw::c_char; 35usize],
    pub reads_sectors: ::std::os::raw::c_ulonglong,
    pub parent_disk: ::std::os::raw::c_uint,
    pub reads: ::std::os::raw::c_uint,
    pub writes: ::std::os::raw::c_uint,
    pub requested_writes: ::std::os::raw::c_ulonglong,
}
impl ::std::clone::Clone for partition_stat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for partition_stat {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct slab_cache {
    pub name: [::std::os::raw::c_char; 48usize],
    pub active_objs: ::std::os::raw::c_uint,
    pub num_objs: ::std::os::raw::c_uint,
    pub objsize: ::std::os::raw::c_uint,
    pub objperslab: ::std::os::raw::c_uint,
}
impl ::std::clone::Clone for slab_cache {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for slab_cache {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub static mut Hertz: ::std::os::raw::c_ulonglong;
    pub static mut smp_num_cpus: ::std::os::raw::c_long;
    pub static mut have_privs: ::std::os::raw::c_int;
    pub static mut page_bytes: ::std::os::raw::c_long;
    pub static mut kb_main_shared: ::std::os::raw::c_ulong;
    pub static mut kb_main_buffers: ::std::os::raw::c_ulong;
    pub static mut kb_main_cached: ::std::os::raw::c_ulong;
    pub static mut kb_main_free: ::std::os::raw::c_ulong;
    pub static mut kb_main_total: ::std::os::raw::c_ulong;
    pub static mut kb_swap_free: ::std::os::raw::c_ulong;
    pub static mut kb_swap_total: ::std::os::raw::c_ulong;
    pub static mut kb_high_free: ::std::os::raw::c_ulong;
    pub static mut kb_high_total: ::std::os::raw::c_ulong;
    pub static mut kb_low_free: ::std::os::raw::c_ulong;
    pub static mut kb_low_total: ::std::os::raw::c_ulong;
    pub static mut kb_main_available: ::std::os::raw::c_ulong;
    pub static mut kb_active: ::std::os::raw::c_ulong;
    pub static mut kb_inact_laundry: ::std::os::raw::c_ulong;
    pub static mut kb_inact_dirty: ::std::os::raw::c_ulong;
    pub static mut kb_inact_clean: ::std::os::raw::c_ulong;
    pub static mut kb_inact_target: ::std::os::raw::c_ulong;
    pub static mut kb_swap_cached: ::std::os::raw::c_ulong;
    pub static mut kb_swap_used: ::std::os::raw::c_ulong;
    pub static mut kb_main_used: ::std::os::raw::c_ulong;
    pub static mut kb_writeback: ::std::os::raw::c_ulong;
    pub static mut kb_slab: ::std::os::raw::c_ulong;
    pub static mut nr_reversemaps: ::std::os::raw::c_ulong;
    pub static mut kb_committed_as: ::std::os::raw::c_ulong;
    pub static mut kb_dirty: ::std::os::raw::c_ulong;
    pub static mut kb_inactive: ::std::os::raw::c_ulong;
    pub static mut kb_mapped: ::std::os::raw::c_ulong;
    pub static mut kb_pagetables: ::std::os::raw::c_ulong;
    pub static mut vm_nr_dirty: ::std::os::raw::c_ulong;
    pub static mut vm_nr_writeback: ::std::os::raw::c_ulong;
    pub static mut vm_nr_pagecache: ::std::os::raw::c_ulong;
    pub static mut vm_nr_page_table_pages: ::std::os::raw::c_ulong;
    pub static mut vm_nr_reverse_maps: ::std::os::raw::c_ulong;
    pub static mut vm_nr_mapped: ::std::os::raw::c_ulong;
    pub static mut vm_nr_slab: ::std::os::raw::c_ulong;
    pub static mut vm_nr_slab_reclaimable: ::std::os::raw::c_ulong;
    pub static mut vm_nr_slab_unreclaimable: ::std::os::raw::c_ulong;
    pub static mut vm_nr_active_file: ::std::os::raw::c_ulong;
    pub static mut vm_nr_inactive_file: ::std::os::raw::c_ulong;
    pub static mut vm_nr_free_pages: ::std::os::raw::c_ulong;
    pub static mut vm_pgpgin: ::std::os::raw::c_ulong;
    pub static mut vm_pgpgout: ::std::os::raw::c_ulong;
    pub static mut vm_pswpin: ::std::os::raw::c_ulong;
    pub static mut vm_pswpout: ::std::os::raw::c_ulong;
    pub static mut vm_pgalloc: ::std::os::raw::c_ulong;
    pub static mut vm_pgfree: ::std::os::raw::c_ulong;
    pub static mut vm_pgactivate: ::std::os::raw::c_ulong;
    pub static mut vm_pgdeactivate: ::std::os::raw::c_ulong;
    pub static mut vm_pgfault: ::std::os::raw::c_ulong;
    pub static mut vm_pgmajfault: ::std::os::raw::c_ulong;
    pub static mut vm_pgscan: ::std::os::raw::c_ulong;
    pub static mut vm_pgrefill: ::std::os::raw::c_ulong;
    pub static mut vm_pgsteal: ::std::os::raw::c_ulong;
    pub static mut vm_kswapd_steal: ::std::os::raw::c_ulong;
    pub static mut vm_pageoutrun: ::std::os::raw::c_ulong;
    pub static mut vm_allocstall: ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn uptime(uptime_secs: *mut f64, idle_secs: *mut f64) -> ::std::os::raw::c_int;
    pub fn getbtime() -> ::std::os::raw::c_ulong;
    pub fn loadavg(av1: *mut f64, av5: *mut f64, av15: *mut f64);
    pub fn getstat(
        cuse: *mut jiff,
        cice: *mut jiff,
        csys: *mut jiff,
        cide: *mut jiff,
        ciow: *mut jiff,
        cxxx: *mut jiff,
        cyyy: *mut jiff,
        czzz: *mut jiff,
        pin: *mut ::std::os::raw::c_ulong,
        pout: *mut ::std::os::raw::c_ulong,
        s_in: *mut ::std::os::raw::c_ulong,
        sout: *mut ::std::os::raw::c_ulong,
        intr: *mut ::std::os::raw::c_uint,
        ctxt: *mut ::std::os::raw::c_uint,
        running: *mut ::std::os::raw::c_uint,
        blocked: *mut ::std::os::raw::c_uint,
        btime: *mut ::std::os::raw::c_uint,
        processes: *mut ::std::os::raw::c_uint,
    );
    pub fn meminfo();
    pub fn vminfo();
    pub fn getpartitions_num(
        disks: *mut disk_stat,
        ndisks: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
    pub fn getdiskstat(
        arg1: *mut *mut disk_stat,
        arg2: *mut *mut partition_stat,
    ) -> ::std::os::raw::c_uint;
    pub fn getslabinfo(arg1: *mut *mut slab_cache) -> ::std::os::raw::c_uint;
    pub fn get_pid_digits() -> ::std::os::raw::c_uint;
    pub fn cpuinfo();
}
static BUFFSIZE: ::std::os::raw::c_int = (64 * 1024);
