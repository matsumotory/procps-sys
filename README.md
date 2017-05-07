# procps-sys

This library lets you a fully interface `procps`, which provides an API for Linux' `/proc` filesystem. 

You can find it on [crates.io](https://crates.io/crates/procps-sys/).

## Example

```rus
extern crate procps_sys;
use procps_sys::readproc::{openproc, readproc, closeproc, proc_t};
use std::ffi::CStr;
use std::ptr::null_mut;


fn main() {
    unsafe {
        // intialize query for process list
        let proctab = openproc(/* fills cmdline line attribute */
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
```

```c
#include <string.h>
#include <stdio.h>
#include <proc/readproc.h>

int main(int argc, char **argv) {

  PROCTAB *proc = openproc(PROC_FILLMEM | PROC_FILLSTAT | PROC_FILLSTATUS | PROC_FILLCOM);
  proc_t *proc_info;

  while ((proc_info = readproc(proc, NULL)) != NULL) {
          if (proc_info->cmdline != NULL) {
    printf("%20s:\t%5ld\t%5lld\t%5lld\t%20s\n", proc_info->cmd, proc_info->resident,
           proc_info->utime, proc_info->stime, proc_info->cmdline[0]);
    freeproc(proc_info);
          }
  }

  closeproc(proc);
}
```
