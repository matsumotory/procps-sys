# procps-sys

This library lets you a fully interface `procps`, which provides an API for Linux' `/proc` filesystem. 

You can find it on [crates.io](https://crates.io/crates/procps-sys/).

## Dependency

- ubuntu 16.04

```
sudo apt-get install libprocps4-dev
```

## Example

```rust
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
