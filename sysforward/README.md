
## TODO

- add structure for SyscallArgument and MemoryRange / MemoryBuffer
- ring buffer for logging syscall
- recover stackstrace for each syscall
- switch case on syscall to parse their arguments
    - how possible it is to reuse syzkaller syscall interface definition to be able to correctly parse them?
- filtering implemented via a eBPF VM
- protocol module: json protocol to send syscall info
- executor module
- Replace u64 with usize where it is architecture dependent
