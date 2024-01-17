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
pub mod x86_64;
mod x86_64_x32;



use std::collections::HashMap;


use crate::syscall::Syscall;



#[derive(Debug, Clone, Copy)]
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
    //X86,
    X86_64,
    X86_64X32,
}


pub struct Architecture {
    pub name: TargetArch,
    //register_table: Register,
    pub syscall_table: SyscallTable,
}

impl Architecture {
    pub fn new(name: TargetArch) -> Self {
        Self {
            name: name,
            syscall_table: SyscallTable::new(&name),
        }
    }

    pub fn create_user_register(&self) -> Box<dyn UserRegister> {
        match self.name {
            //TargetArch::Aarch32 => Box::new(aarch32::Aarch32Register::new()),
            //TargetArch::Aarch64 => Box::new(aarch64::Aarch64Register::new()),
            //TargetArch::Arm_eabi => Box::new(arm_eabi::ArmEabiRegister::new()),
            //TargetArch::Arm_oabi => Box::new(arm_oabi::ArmOabiRegister::new()),
            //TargetArch::Mipsn32 => Box::new(mipsn32::Mipsn32Register::new()),
            //TargetArch::Mipsn64 => Box::new(mipsn64::Mipsn64Register::new()),
            TargetArch::Mipso32 => Box::new(mipso32::Mipso32Register::new()),
            //TargetArch::Powerpc32 => Box::new(powerpc32::Powerpc32Register::new()),
            //TargetArch::Powerpc64 => Box::new(powerpc64::Powerpc64Register::new()),
            //TargetArch::Riscv32 => Box::new(riscv32::Riscv32Register::new()),
            //TargetArch::Riscv64 => Box::new(riscv64::Riscv64Register::new()),
            //TargetArch::X86 => Box::new(x86::X86::new()),
            TargetArch::X86_64 => Box::new(x86_64::X86Register::default()),
            TargetArch::X86_64X32 => Box::new(x86_64::X86Register::default()),
            _ => panic!("Architecture not implemented"),
        }
}
}



pub trait UserRegister {
    fn set_syscall_entry(&self, syscall: &mut Syscall);
    fn set_syscall_exit(&self, syscall: &mut Syscall);
}



#[derive(Clone, Debug)]
pub struct SyscallTable {
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
                TargetArch::X86_32      => x86_32::syscall_table::create_syscall_table(),
                */
                TargetArch::X86_64      => x86_64::syscall_table::create_syscall_table(),
                TargetArch::X86_64X32   => x86_64_x32::syscall_table::create_syscall_table(),
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
