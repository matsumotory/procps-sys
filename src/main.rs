extern crate procps_sys;

use procps_sys::readproc::*;
use std::ffi::CStr;
use std::ptr::null_mut;

fn main() {
    unsafe {
        // intialize query for process list
        let proctab = openproc(
            /* fills cmdline line attribute */
            PROC_FILLSTAT | PROC_FILLSTATUS | PROC_FILLCOM,
        );

        // go through all processes
        let mut optional = Some(readproc(proctab, null_mut()));

        while let Some(p) = optional {
            if p.is_null() {
                optional = None;
            } else {
                let env_str = if !(*p).cmdline.is_null() && !(*(*p).cmdline).is_null() {
                    CStr::from_ptr(*(*p).cmdline).to_string_lossy().into_owned()
                } else {
                    "null".to_string()
                };
                println!(
                    "pid: {} vm_size: {} cmdline: {}",
                    (*p).tid,
                    (*p).vm_size,
                    env_str
                );
                optional = Some(readproc(proctab, null_mut()));
            }
            freeproc(p);
        }
        closeproc(proctab);
    }
}
