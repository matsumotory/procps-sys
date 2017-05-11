extern crate procps_sys;

use procps_sys::readproc::{openproc, readproc, closeproc, proc_t};
use std::ffi::CStr;
use std::ptr::null_mut;


fn main() {
    unsafe {
        // intialize query for process list
        let proctab = openproc(procps_sys::readproc::PROC_FILLSTAT | procps_sys::readproc::PROC_FILLSTATUS |
                               procps_sys::readproc::PROC_FILLCOM);

        // go through all processes
        let mut procinfo = proc_t::default();
        while readproc(proctab, &mut procinfo) != null_mut() {

            // read cmdline attribute
            let cmdline = if procinfo.cmdline == null_mut() {
                "".to_string()
            } else {
                CStr::from_ptr(*procinfo.cmdline)
                    .to_string_lossy()
                    .into_owned()
            };

            // print information
            println!("pid: {} cmdline: \"{}\"", procinfo.tid, cmdline);
        }
        closeproc(proctab);
    }
}
