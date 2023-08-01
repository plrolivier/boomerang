/*
 * Decode syscall arguments.
 */
use std::sync::Arc;

use serde::{Serialize, Deserialize};

use crate::{
    arch::Architecture,
    syscall::{ Syscall, syscalls },
    targets::operation::Operation,
};



pub struct Decoder {
    arch: Arc<Architecture>,
}

impl Decoder {
    pub fn new(arch: Arc<Architecture>) -> Self {
        Self { 
            //arch: Architecture::new(arch),
            arch: arch,
        }
    }

    pub fn decode_entry(&self, syscall: &mut Syscall, pid: i32, operation: &Box<Operation>) {

        // TODO: improve the match by using number instead of strings
        match self.arch.syscall_table.get_syscall_name(&syscall.raw.no) {
            Some(x) => syscall.name = x,
            None => println!("No name found for {}", syscall.raw.no),
        }

        /*
         * First, assign a type to each argument according to the syscall.
         */
        self.parse_args(syscall);

        /*
         * Second, iterate over the argument to decode them.
         */
        self.decode_args(syscall, pid, operation);
        //syscall.entry_decoded = true;
    }

    fn decode_args(&self, syscall: &mut Syscall, pid: i32, operation: &Box<Operation>) {
        if let Some(decoded_sc) = &mut syscall.decoded {
                decoded_sc.decode_entry(pid, operation);
            }
    }

    fn parse_args(&self, syscall: &mut Syscall) {
        
        let raw = syscall.raw.clone();
        macro_rules! decode_syscall {
            ($name:ident, $category:ident) => {
                syscall.decoded = Some(DecodedSyscall::$name(syscalls::$category::$name::new(raw)))
            };
        }

        /*
         * TODO:
         * 1. Produce a list of system calls with their definition
         *    Use something similar to https://github.com/panda-re/panda/blob/dev/panda/plugins/syscalls2/scripts/syscall_parser.py
         * 2. Craft a macro which parse each argument and associate it to a type
         *    e.g., integer, address, fd, size_t, offset_t, enum, struct, array, string
         */

        match syscall.name.as_str() {

            "close"     => { decode_syscall!(Close, open) },
            "creat"     => { decode_syscall!(Creat, open) },
            "open"      => { decode_syscall!(Open, open) },
            "openat"    => { decode_syscall!(Openat, open) },
            "openat2"   => { decode_syscall!(Openat2, open) },

            "read"      => { decode_syscall!(Read, io) },
            "write"     => { decode_syscall!(Write, io) },
            "readv"     => { decode_syscall!(Readv, io) },
            "writev"    => { decode_syscall!(Writev, io) },
            "pread"     => { decode_syscall!(Pread, io) },
            "pread64"   => { decode_syscall!(Pread, io) },
            "pwrite"    => { decode_syscall!(Pwrite, io) },
            "preadv"    => { decode_syscall!(Preadv, io) },
            "pwritev"   => { decode_syscall!(Pwritev, io) },
            "preadv2"   => { decode_syscall!(Preadv2, io) },
            "pwritev2"  => { decode_syscall!(Pwritev2, io) },

            "ioctl"     => { decode_syscall!(Ioctl, ioctl) },

            "lseek"     => { decode_syscall!(Lseek, lseek) },
            "llseek"     => { decode_syscall!(Llseek, lseek) },

            "access"    => { decode_syscall!(Access, access) },
            "faccessat" => { decode_syscall!(Faccessat, access) },
            "faccessat2"=> { decode_syscall!(Faccessat2, access) },

            "fallocate" => { decode_syscall!(Fallocate, fallocate) },
            "name_to_handle_at" => { decode_syscall!(NameToHandleAt, file_handle) },
            "open_by_handle_at" => { decode_syscall!(OpenByHandleAt, file_handle) },
            "memfd_create" => { decode_syscall!(MemfdCreate, memfd) },
            "mknod" => { decode_syscall!(Mknod, mknod) },
            "mknodat" => { decode_syscall!(Mknodat, mknod) },
            "rename" => { decode_syscall!(Rename, renameat) },
            "renameat" => { decode_syscall!(Renameat, renameat) },
            "renameat2" => { decode_syscall!(Renameat2, renameat) },
            "truncate" => { decode_syscall!(Truncate, truncate) },
            "ftruncate" => { decode_syscall!(Ftruncate, truncate) },

            "execve"    => { decode_syscall!(Execve, execve) },
            "execveat"  => { decode_syscall!(Execveat, execve) },

            "prctl"     => { decode_syscall!(Prctl, prctl) },
            "arch_prctl"=> { decode_syscall!(ArchPrctl, prctl) },

            "brk"       => { decode_syscall!(Brk, mmap) },
            "sbrk"      => { decode_syscall!(Sbrk, mmap) },
            "mmap"      => { decode_syscall!(Mmap, mmap) },
            "mremap"    => { decode_syscall!(Mremap, mmap) },
            "munmap"    => { decode_syscall!(Munmap, mmap) },
            "mprotect"  => { decode_syscall!(Mprotect, mmap) },
            "madvise"   => { decode_syscall!(Madvise, mmap) },

            "getdents"  => { decode_syscall!(Getdents, dirent) },
            "getdents64"=> { decode_syscall!(Getdents64, dirent) },
            "readdir"   => { decode_syscall!(Readdir, dirent) },

            "stat"      => { decode_syscall!(Stat, stat) },
            "fstat"     => { decode_syscall!(Fstat, stat) },
            "lstat"     => { decode_syscall!(Lstat, stat) },
            "fstatat"   => { decode_syscall!(Fstatat, stat) },
            "newfstatat"=> { decode_syscall!(Fstatat, stat) },
            
            "statx"     => { decode_syscall!(Statx, statx) },

            "getrlimit" => { decode_syscall!(Getrlimit, resource) },
            "setrlimit" => { decode_syscall!(Setrlimit, resource) },
            "prlimit"   => { decode_syscall!(Prlimit, resource) },
            "prlimit64" => { decode_syscall!(Prlimit, resource) },
            "getrusage" => { decode_syscall!(Getrusage, resource) },

            "rseq"      => { decode_syscall!(Rseq, rseq) },

            "getrandom" => { decode_syscall!(Getrandom, getrandom) },

            "epollcreate"   => { decode_syscall!(EpollCreate, epoll) },
            "epollcreate1"  => { decode_syscall!(EpollCreate1, epoll) },
            "epollctl"      => { decode_syscall!(EpollCtl, epoll) },
            "epollwait"     => { decode_syscall!(EpollWait, epoll) },
            "epollpwait"    => { decode_syscall!(EpollPwait, epoll) },
            "epollpwait2"   => { decode_syscall!(EpollPwait2, epoll) },

            "get_robust_list" => { decode_syscall!(GetRobustList, robust_list) },
            "set_robust_list" => { decode_syscall!(SetRobustList, robust_list) },

            "set_tid_address" => { decode_syscall!(SetTidAddress, tid) },

            "exit_group"    => { decode_syscall!(ExitGroup, exit) },

            _ => (),
        }
    }


    pub fn decode_exit(&self, syscall: &mut Syscall, pid: i32, operation: &Box<Operation>) {

        /* Decode return value */
        if let Some(decoded_sc) = &mut syscall.decoded {
            decoded_sc.decode_exit(syscall.raw.retval, pid, operation).unwrap();
        }

    }
}



pub trait DecodeArg {
    fn decode(&mut self, _pid: i32, _operation: &Box<Operation>) -> Result<(), std::io::Error> { 
        Ok(())
    }
    fn print(&self) { }
}

pub trait DecodeEntry {
    //fn as_any(&self) -> &dyn Any;
    #[allow(unused_variables)]
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) { }
    fn print(&self) { }
}

pub trait DecodeExit {
    fn decode_exit(&mut self, _value: usize, _pid: i32, _operation: &Box<Operation>) -> Result<(), std::io::Error> { 
        Ok(())
    }
}



#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub enum DecodedSyscall {
    /* Filesystem */
    //#[serde(rename = "close")]
    Close(syscalls::open::Close),
    Creat(syscalls::open::Creat),
    Open(syscalls::open::Open),
    Openat(syscalls::open::Openat),
    Openat2(syscalls::open::Openat2),

    /* io */
    Read(syscalls::io::Read),
    Write(syscalls::io::Write),
    Readv(syscalls::io::Readv),
    Writev(syscalls::io::Writev),
    Pread(syscalls::io::Pread),
    Pwrite(syscalls::io::Pwrite),
    Preadv(syscalls::io::Preadv),
    Pwritev(syscalls::io::Pwritev),
    Preadv2(syscalls::io::Preadv2),
    Pwritev2(syscalls::io::Pwritev2),

    Ioctl(syscalls::ioctl::Ioctl),

    Fallocate(syscalls::fallocate::Fallocate),

    NameToHandleAt(syscalls::file_handle::NameToHandleAt),
    OpenByHandleAt(syscalls::file_handle::OpenByHandleAt),

    MemfdCreate(syscalls::memfd::MemfdCreate),

    Mknod(syscalls::mknod::Mknod),
    Mknodat(syscalls::mknod::Mknodat),

    Rename(syscalls::renameat::Rename),
    Renameat(syscalls::renameat::Renameat),
    Renameat2(syscalls::renameat::Renameat2),

    Truncate(syscalls::truncate::Truncate),
    Ftruncate(syscalls::truncate::Ftruncate),

    Access(syscalls::access::Access),
    Faccessat(syscalls::access::Faccessat),
    Faccessat2(syscalls::access::Faccessat2),

    /* mmap */
    Brk(syscalls::mmap::Brk),
    Sbrk(syscalls::mmap::Sbrk),
    Mmap(syscalls::mmap::Mmap),
    Mremap(syscalls::mmap::Mremap),
    Munmap(syscalls::mmap::Munmap),
    Mprotect(syscalls::mmap::Mprotect),
    Madvise(syscalls::mmap::Madvise),

    Execve(syscalls::execve::Execve),
    Execveat(syscalls::execve::Execveat),

    Prctl(syscalls::prctl::Prctl),
    ArchPrctl(syscalls::prctl::ArchPrctl),

    Getdents(syscalls::dirent::Getdents),
    Getdents64(syscalls::dirent::Getdents64),
    Readdir(syscalls::dirent::Readdir),

    Stat(syscalls::stat::Stat),
    Fstat(syscalls::stat::Fstat),
    Lstat(syscalls::stat::Lstat),
    Fstatat(syscalls::stat::Fstatat),
    
    Statx(syscalls::statx::Statx),

    Getrlimit(syscalls::resource::Getrlimit),
    Setrlimit(syscalls::resource::Setrlimit),
    Prlimit(syscalls::resource::Prlimit),
    Prlimit64(syscalls::resource::Prlimit),
    Getrusage(syscalls::resource::Getrusage),

    Rseq(syscalls::rseq::Rseq),

    Getrandom(syscalls::getrandom::Getrandom),

    EpollCreate(syscalls::epoll::EpollCreate),
    EpollCreate1(syscalls::epoll::EpollCreate1),
    EpollCtl(syscalls::epoll::EpollCtl),
    EpollWait(syscalls::epoll::EpollWait),
    EpollPwait(syscalls::epoll::EpollPwait),
    EpollPwait2(syscalls::epoll::EpollPwait2),

    GetRobustList(syscalls::robust_list::GetRobustList),
    SetRobustList(syscalls::robust_list::SetRobustList),

    SetTidAddress(syscalls::tid::SetTidAddress),

    Lseek(syscalls::lseek::Lseek),
    Llseek(syscalls::lseek::Llseek),

    ExitGroup(syscalls::exit::ExitGroup),

    /* ... */
}

impl DecodedSyscall {
    pub fn get_syscall(&self) -> &dyn std::any::Any
    {
        match self {
            DecodedSyscall::Close(sc) => return sc,
            DecodedSyscall::Creat(sc) => return sc,
            DecodedSyscall::Open(sc) => return sc,
            DecodedSyscall::Openat(sc) => return sc,
            DecodedSyscall::Openat2(sc) => return sc,
            DecodedSyscall::Read(sc) => return sc,
            DecodedSyscall::Write(sc) => return sc,
            DecodedSyscall::Readv(sc) => return sc,
            DecodedSyscall::Writev(sc) => return sc,
            DecodedSyscall::Pread(sc) => return sc,
            DecodedSyscall::Pwrite(sc) => return sc,
            DecodedSyscall::Preadv(sc) => return sc,
            DecodedSyscall::Pwritev(sc) => return sc,
            DecodedSyscall::Preadv2(sc) => return sc,
            DecodedSyscall::Pwritev2(sc) => return sc,
            DecodedSyscall::Ioctl(sc) => return sc,
            DecodedSyscall::Brk(sc) => return sc,
            DecodedSyscall::Sbrk(sc) => return sc,
            DecodedSyscall::Mmap(sc) => return sc,
            DecodedSyscall::Mremap(sc) => return sc,
            DecodedSyscall::Munmap(sc) => return sc,
            DecodedSyscall::Mprotect(sc) => return sc,
            DecodedSyscall::Madvise(sc) => return sc,
            DecodedSyscall::Execve(sc) => return sc,
            DecodedSyscall::Execveat(sc) => return sc,
            DecodedSyscall::Fallocate(sc) => return sc,
            DecodedSyscall::NameToHandleAt(sc) => return sc,
            DecodedSyscall::OpenByHandleAt(sc) => return sc,
            DecodedSyscall::MemfdCreate(sc) => return sc,
            DecodedSyscall::Mknod(sc) => return sc,
            DecodedSyscall::Mknodat(sc) => return sc,
            DecodedSyscall::Rename(sc) => return sc,
            DecodedSyscall::Renameat(sc) => return sc,
            DecodedSyscall::Renameat2(sc) => return sc,
            DecodedSyscall::Truncate(sc) => return sc,
            DecodedSyscall::Ftruncate(sc) => return sc,
            DecodedSyscall::Access(sc) => return sc,
            DecodedSyscall::Faccessat(sc) => return sc,
            DecodedSyscall::Faccessat2(sc) => return sc,
            DecodedSyscall::Prctl(sc) => return sc,
            DecodedSyscall::ArchPrctl(sc) => return sc,
            DecodedSyscall::Getdents(sc) => return sc,
            DecodedSyscall::Getdents64(sc) => return sc,
            DecodedSyscall::Readdir(sc) => return sc,
            DecodedSyscall::Stat(sc) => return sc,
            DecodedSyscall::Fstat(sc) => return sc,
            DecodedSyscall::Lstat(sc) => return sc,
            DecodedSyscall::Fstatat(sc) => return sc,
            DecodedSyscall::Statx(sc) => return sc,
            DecodedSyscall::Getrlimit(sc) => return sc,
            DecodedSyscall::Setrlimit(sc) => return sc,
            DecodedSyscall::Prlimit(sc) => return sc,
            DecodedSyscall::Prlimit64(sc) => return sc,
            DecodedSyscall::Getrusage(sc) => return sc,
            DecodedSyscall::Rseq(sc) => return sc,
            DecodedSyscall::Getrandom(sc) => return sc,
            DecodedSyscall::EpollCreate(sc) => return sc,
            DecodedSyscall::EpollCreate1(sc) => return sc,
            DecodedSyscall::EpollCtl(sc) => return sc,
            DecodedSyscall::EpollWait(sc) => return sc,
            DecodedSyscall::EpollPwait(sc) => return sc,
            DecodedSyscall::EpollPwait2(sc) => return sc,
            DecodedSyscall::SetTidAddress(sc) => return sc,
            DecodedSyscall::GetRobustList(sc) => return sc,
            DecodedSyscall::SetRobustList(sc) => return sc,
            DecodedSyscall::Lseek(sc) => return sc,
            DecodedSyscall::Llseek(sc) => return sc,
            DecodedSyscall::ExitGroup(sc) => return sc,
        }

    }
}

impl DecodeEntry for DecodedSyscall {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        /* Why not match on the syscall name? */
        match self {
            DecodedSyscall::Close(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Creat(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Open(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Openat(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Openat2(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Read(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Write(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Readv(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Writev(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Pread(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Pwrite(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Preadv(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Pwritev(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Preadv2(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Pwritev2(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Ioctl(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Brk(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Sbrk(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Mmap(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Mremap(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Munmap(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Mprotect(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Madvise(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Execve(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Execveat(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Fallocate(x) => x.decode_entry(pid, operation),
            DecodedSyscall::NameToHandleAt(x) => x.decode_entry(pid, operation),
            DecodedSyscall::OpenByHandleAt(x) => x.decode_entry(pid, operation),
            DecodedSyscall::MemfdCreate(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Mknod(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Mknodat(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Rename(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Renameat(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Renameat2(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Truncate(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Ftruncate(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Access(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Faccessat(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Faccessat2(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Prctl(x) => x.decode_entry(pid, operation),
            DecodedSyscall::ArchPrctl(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Getdents(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Getdents64(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Readdir(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Stat(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Fstat(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Lstat(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Fstatat(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Statx(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Getrlimit(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Setrlimit(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Prlimit(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Prlimit64(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Getrusage(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Rseq(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Getrandom(x) => x.decode_entry(pid, operation),
            DecodedSyscall::EpollCreate(x) => x.decode_entry(pid, operation),
            DecodedSyscall::EpollCreate1(x) => x.decode_entry(pid, operation),
            DecodedSyscall::EpollCtl(x) => x.decode_entry(pid, operation),
            DecodedSyscall::EpollWait(x) => x.decode_entry(pid, operation),
            DecodedSyscall::EpollPwait(x) => x.decode_entry(pid, operation),
            DecodedSyscall::EpollPwait2(x) => x.decode_entry(pid, operation),
            DecodedSyscall::SetTidAddress(x) => x.decode_entry(pid, operation),
            DecodedSyscall::GetRobustList(x) => x.decode_entry(pid, operation),
            DecodedSyscall::SetRobustList(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Lseek(x) => x.decode_entry(pid, operation),
            DecodedSyscall::Llseek(x) => x.decode_entry(pid, operation),
            DecodedSyscall::ExitGroup(x) => x.decode_entry(pid, operation),
            //DecodedSyscall::(x) => x.decode_entry(pid, operation),
        }
    }
}

impl DecodeExit for DecodedSyscall {
    fn decode_exit(&mut self, value: usize, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> { 
        match self {
            DecodedSyscall::Open(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Close(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Creat(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Openat(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Openat2(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Read(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Write(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Readv(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Writev(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Pread(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Pwrite(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Preadv(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Pwritev(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Preadv2(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Pwritev2(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Ioctl(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Brk(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Sbrk(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Mmap(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Mremap(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Munmap(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Mprotect(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Madvise(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Execve(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Execveat(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Fallocate(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::NameToHandleAt(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::OpenByHandleAt(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::MemfdCreate(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Mknod(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Mknodat(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Rename(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Renameat(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Renameat2(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Truncate(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Ftruncate(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Access(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Faccessat(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Faccessat2(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Prctl(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::ArchPrctl(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Getdents(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Getdents64(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Readdir(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Stat(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Fstat(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Lstat(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Fstatat(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Statx(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Getrlimit(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Setrlimit(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Prlimit(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Prlimit64(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Getrusage(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Rseq(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Getrandom(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::EpollCreate(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::EpollCreate1(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::EpollCtl(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::EpollWait(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::EpollPwait(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::EpollPwait2(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::SetTidAddress(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::GetRobustList(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::SetRobustList(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Lseek(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::Llseek(x) => x.decode_exit(value, pid, operation),
            DecodedSyscall::ExitGroup(_) => Ok(()),
            //_ => panic!("oops"),
            //DecodedSyscall::(x) => x.decode_exit(value, pid, operation),
        }
    }
}