# RoadMap

## 0.1.0

Goal:
Having a first demo which works with a typical character device:
The kernel module `kbuf` is an example to use.
Syscalls to support are `open/read/write/lseek/close`.

- [x] Filtering lib for open / read / write / close syscalls
- [x] Invoke syscall on executor
- [x] `EncodeEntry` and `EncodeExit` traits for open / read / write / close syscalls
- [x] Reorganize project structure
- [x] Clean all (most) compiler warnings

## 0.2.0

Goal: support for multiple architecture (mips)

- [ ] Rework how arch are selected
- [ ] Add mips architecture
- [ ] New example with lima dev board

## 0.3.0

Goal: add new tracer based on qemu user mode

- [ ] Qemu emulator target
- [ ] Support for mmap syscall family
- [ ] Process migration (snapshot + dump file)


## 1.0.0

- [ ] Process migration between targets
- [ ] Generate all syscall instrumentation code with proc macro
- [ ] Improve syscall encoding to automatically place arguments into correct RawSyscall
- [ ] Rework the syscall encoding before its target execution to allow sync of modified decoded syscall
- [ ] Define clear interfaces (modules, pub, etc.)
- [ ] Reorganize what should be in the library, and what should be outside in debuggers/targets
- [ ] Merge encode / decode traits into a single interface for better clarity for syscalls
- [ ] Improve error handling: Result<(), >
- [ ] Find a better way to switch case on syscall name & unwrap() the DecodedSyscall


## Misc.

- [ ] Recover the stack trace on each syscall entry
- [ ] ring buffer for logging syscalls
- [ ] New structure to represent memory ranges and blocks
- [ ] Syscall filtering with ebpf
