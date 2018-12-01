#include <string.h>
#include <stdio.h>
#include <proc/readproc.h>

int main(int argc, char **argv) {

  PROCTAB *proc = openproc(PROC_FILLCOM | PROC_FILLSTAT);
  proc_t *proc_info;

  while ((proc_info = readproc(proc, NULL)) != NULL) {
	  if (proc_info->cmdline != NULL) {
    printf("%20s:\t%5ld\t%5lld\t%5lld\t%20s\t%10s\n", proc_info->cmd, proc_info->resident,
           proc_info->utime, proc_info->stime, proc_info->cmdline[0], proc_info->cmd);
    freeproc(proc_info);
	  }
  }

  closeproc(proc);
}

