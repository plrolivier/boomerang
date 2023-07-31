
## TODO

## First priority

- For decoding, create a struct for each syscall where the arguments are a vector with Int, Address, etc.. it's then going easier to store syscall related data there.
- Add a structure to represent memory ranges and memory blocks: MemoryRange; MemoryBlock
- Deserialize for protocol
- protocol module: json protocol to send syscall info
- MACRO to help definining the argument type during decoding
- executor module
- rework argument data structure classification: take inspiration from Syzkaller

## Second priority

- Serialize enum as number: https://serde.rs/enum-number.html
- ring buffer for logging syscall instead of just printing
- recover stackstrace for each syscall
- filtering: Filtering should be similar to what seccomp-bpf does with filter written in eBPF, therefore (JIT) compiling the eBPF when loading the filter or interpreting it.
- Architecture independent:
    1. Replace u64 with usize
- Add tracing for qemu user mode
- Process migration
    1. snapshot + dump in ELF file
    2. custom loader with qemu
- Decoding: use pattern matching on syscall definition to associate arguments type


## Eventually

- switch case on syscall to parse their arguments: how possible it is to reuse syzkaller syscall interface definition to be able to correctly parse them?
- Protocol: use serde for serializer or reimplement a custom one.
- replace json protocol with protobuf or custum binary protocol


## Notes on the protocol:

- composed of fixed length `header` and variable length `payload`
- JSON OR protobuf
- Should be able to send it over TCP / UDP / UART / Unix socket / etc.
- The length might not be needed to be sent within the `header` because it is redundant with the `command`.
- To send a message:
    1. Encode the message into the `payload`
    2. Get the length of the `payload` and add it in the `header` along the `command`.
    3. Send the `header` followed by the `payload`
- To receive a message:
    1. Receive the `header` 
    2. Decode the `payload` length
    3. Receive the `payload` as we know its length
    4. Decode the `payload`
- Example: https://krpc.github.io/krpc/communication-protocols/tcpip.html

