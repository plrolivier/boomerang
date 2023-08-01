# Version 0.1.0 (August 1st, 2023)

* Features
    * a framework extending avatar2 for Linux syscall and process tracing
    * an instrumentation library `sysfwd` allowing syscall decoding and communication between targets
    * a tracer debugger target `ptracer`
    * a debugger target for remote syscall execution `pexecutor`
    * a library collecting examples of syscall filters `sysfwd_filter`
    * a basic kernel module `kbuf` implementing a buffer in kernel memory with open / close / read / write / lseek syscalls
