/*
 *
 */
//mod arm;
//mod aarch64;
//mod mipsn32;
//mod mipsn64;
mod mipso32;
//mod powerpc;
//mod riscv;
mod x86_64;
mod x86_64_x32;


use std::collections::HashMap;



#[derive(Clone, Copy)]
pub enum TargetArch {
    //Aarch32,
    //Aarch64,
    //Arm_eabi,
    //Arm_oabi,
    //Mipsn32,
    //Mipsn64,
    Mipso32,
    //Powerpc32,
    //Powerpc64,
    //Riscv32,
    //Riscv64,
    X86,
    X86_64,
    X86_64X32,
}


pub struct Architecture {
    pub name: TargetArch,
    //register_table: Register,
    pub(crate) syscall_table: SyscallTable,
}

impl Architecture {
    pub fn new(name: TargetArch) -> Self {
        Self {
            name: name,
            //register_table: Register::new(&name),
            syscall_table: SyscallTable::new(&name),
        }
    }
}


/*
pub(crate) struct Register {
    map: HashMap<&'static str, u8>,
}

impl Register {
    pub fn new(arch: &TargetArch) -> Self {
        Self {
            map: match arch {
                TargetArch::X86_64  => x86_64::create_register_table(),
                //TargetArch::X86     => x86::create_register_table(),
                //TargetArch::Arm     => arm::create_register_table(),
                //TargetArch::Aarch64 => aarch64::create_register_table(),
                //TargetArch::Mipso32 => mipso32::create_register_table(),
                //TargetArch::Mipsn32 => mipsn32::create_register_table(),
                //TargetArch::Mipsn64 => mipsn64::create_register_table(),
                //TargetArch::Powerpc => powerpc::create_register_table(),
                //TargetArch::Riscv32 => riscv32::create_register_table(),
                //TargetArch::Riscv64 => riscv64::create_register_table(),
                _ => panic!("Architecture not implemented"),
            },
        }
    }

    pub fn get_no(&self, name: &str) -> Option<u8> {
        self.map.get(name).copied()
    }

    pub fn get_name(&self, no: &u64) -> Option<str> {
        self.map.iter().find_map(|(key, &val)| if val == no { Some(no) } else { None })
    }
}
*/


pub(crate) struct SyscallTable {
    map: HashMap<&'static str, usize>,
}

impl SyscallTable {
    pub fn new(arch: &TargetArch) -> Self {
        Self {
            map: match arch {
                /*
                TargetArch::Aarch32     => aarch32::create_syscall_table(),
                TargetArch::Aarch64     => aarch64::create_syscall_table(),
                TargetArch::Arm_eabi    => arm_eabi::create_syscall_table(),
                TargetArch::Arm_oabi    => arm_oabi::create_syscall_table(),
                */
                TargetArch::Mipso32 => mipso32::syscall_table::create_syscall_table(),
                /*
                TargetArch::Mipsn32 => mipsn32::create_syscall_table(),
                TargetArch::Mipsn64 => mipsn64::create_syscall_table(),
                TargetArch::Powerpc32 => powerpc32::create_syscall_table(),
                TargetArch::Powerpc64 => powerpc64::create_syscall_table(),
                TargetArch::Riscv32 => riscv32::create_syscall_table(),
                TargetArch::Riscv64 => riscv64::create_syscall_table(),
                */
                TargetArch::X86_64      => x86_64::syscall_table::create_syscall_table(),
                TargetArch::X86_64X32   => x86_64_x32::syscall_table::create_syscall_table(),
                /*
                TargetArch::X86_64_ia32 => x86_64_ia32::syscall_table::create_syscall_table(),
                TargetArch::X86_32      => x86_32::syscall_table::create_syscall_table(),
                */
                _ => panic!("Architecture not implemented"),
            },
        }
    }

    pub fn _get_syscall_no(&self, name: &str) -> Option<usize> {
        self.map.get(name).copied()
    }

    pub fn get_syscall_name(&self, no: &usize) -> Option<String> {
        self.map.iter().find_map(|(&key, &val)| if val == *no { Some(String::from(key)) } else { None })
    }
}
