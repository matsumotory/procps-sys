extern crate procps_sys;
use procps_sys::readproc::{closeproc, freeproc, openproc, proc_t, readproc};
use std::ffi::CStr;
use std::ptr::null_mut;

fn main() {
    unsafe {
        // intialize query for process list
        let proctab = openproc(
            /* fills cmdline line attribute */
            procps_sys::readproc::PROC_FILLCOM,
        );

        // go through all processes
        let mut procinfo = proc_t::default();
        while (procinfo = readproc(proctab, null_mut())) != null_mut() {
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
            freeproc(procinfo);
        }
        closeproc(proctab);
    }
}
