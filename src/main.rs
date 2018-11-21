extern crate procps_sys;

use procps_sys::readproc::{closeproc, openproc, proc_t, readproc};
use std::ffi::CStr;
use std::ptr::null_mut;

fn main() {
    unsafe {
        // intialize query for process list
        let proctab = openproc(
            procps_sys::readproc::PROC_FILLSTAT
                | procps_sys::readproc::PROC_FILLSTATUS
                | procps_sys::readproc::PROC_FILLCOM,
        );

        // go through all processes
        let mut procinfo = proc_t::default();
        while readproc(proctab, &mut procinfo) != null_mut() {
            let cmd_cstr = CStr::from_ptr(procinfo.cmd.as_ptr()).to_owned();
            println!("pid: {} cmdline: {:?}", procinfo.tid, cmd_cstr);
        }
        closeproc(proctab);
    }
}
