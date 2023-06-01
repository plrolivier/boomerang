/*
 * Decode syscall arguments.
 */
use std::rc::Rc;

use serde::{Serialize, Deserialize};

use crate::{
    arch::{ Architecture },
    syscall,
    syscall::{ Syscall },
    operation::Operation,
};



pub struct Decoder {
    arch: Rc<Architecture>,
}

impl Decoder {
    pub fn new(arch: Rc<Architecture>) -> Self {
        Self { 
            //arch: Architecture::new(arch),
            arch: arch,
        }
    }

    pub fn decode_entry(&self, syscall: &mut Syscall, pid: i32, operation: &Box<dyn Operation>) {

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

    fn decode_args(&self, syscall: &mut Syscall, pid: i32, operation: &Box<dyn Operation>) {
        if let Some(decoded_sc) = &mut syscall.decoded {
                decoded_sc.decode(pid, operation);
            }
    }

    fn parse_args(&self, syscall: &mut Syscall) {
        
        let raw = syscall.raw.clone();
        macro_rules! decode_syscall {
            ($name:ident, $category:ident) => {
                syscall.decoded = Some(DecodedSyscall::$name(syscall::$category::$name::new(raw)))
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

            "brk"       => { decode_syscall!(Brk, mmap) },
            "sbrk"      => { decode_syscall!(Sbrk, mmap) },
            "mmap"      => { decode_syscall!(Mmap, mmap) },
            "mremap"    => { decode_syscall!(Mremap, mmap) },
            "munmap"    => { decode_syscall!(Munmap, mmap) },
            "mprotect"  => { decode_syscall!(Mprotect, mmap) },
            "madvise"   => { decode_syscall!(Madvise, mmap) },

            //"close" => {
            //    // int close(int fd)
            //    decode_integer!(0, Fd);
            //},
            //"creat" => {
            //    // int creat(const char *pathname, mode_t mode)
            //    decode_string!(0, Direction::In);
            //    decode_integer!(1, Integer);
            //},
            //"open" => {
            //    // int open(const char *pathname, int flags)
            //    // int open(const char *pathname, int flags, mode_t mode)
            //    decode_integer!(0, NullBuffer);
            //    decode_integer!(1, Flag);
            //    decode_integer!(2, Integer);
            //},
            //"openat" => {
            //    // int openat(int dirfd, const char *pathname, int flags)
            //    // int openat(int dirfd, const char *pathname, int flags, mode_t mode)
            //    decode_integer!(0, Integer);
            //    decode_integer!(1, NullBuffer);
            //    decode_integer!(2, Flag);
            //    decode_integer!(3, Integer);
            //},
            //"openat2" => {
            //    // int openat2(int dirfd, const char *pathname, const struct open_how *how, size_t size)
            //    decode_integer!(0, Fd);
            //    decode_integer!(1, NullBuffer);
            //    decode_integer!(2, Struct);     // XXX: the size argument is needed to know the size of
            //                                // the struct!!!
            //    decode_integer!(3, Size);
            //},


            /* Groups come from https://linasm.sourceforge.net/docs/syscalls/index.php */
            //"mmap" => {
            //    define_arg!(0, Address);
            //    define_arg!(1, Size);
            //    define_arg!(2, Prot);
            //    define_arg!(3, Fd);
            //    define_arg!(4, Offset);
            //},

            // /* 
            //  * Filesystem 
            //  */
            // /* File operations */
            // "close" => {
            //     // int close(int fd)
            //     define_arg!(0, Fd);
            // },
            // "creat" => { 
            //     // int creat(const char *pathname, mode_t mode)
            //     define_arg!(0, NullBuf);
            //     define_arg!(1, Int);
            // },
            // "open" => {
            //     // int open(const char *pathname, int flags)
            //     // int open(const char *pathname, int flags, mode_t mode)
            //     define_arg!(0, NullBuf);
            //     define_arg!(1, Flag);
            //     define_arg!(2, Int);
            // },
            // "openat" => {
            //     // int openat(int dirfd, const char *pathname, int flags)
            //     // int openat(int dirfd, const char *pathname, int flags, mode_t mode)
            //     define_arg!(0, Int);
            //     define_arg!(1, NullBuf);
            //     define_arg!(2, Flag);
            //     define_arg!(3, Int);
            // },
            // "openat2" => {
            //     // int openat2(int dirfd, const char *pathname, const struct open_how *how, size_t size)
            //     define_arg!(0, Fd);
            //     define_arg!(1, NullBuf);
            //     define_arg!(2, Struct);     // XXX: the size argument is needed to know the size of
            //                                 // the struct!!!
            //     define_arg!(3, Size);
            // }
            //"name_to_handle_at" => {
            //    // int name_to_handle_at(int dirfd, const char *pathname, struct file_handle *handle, int *mount_id, int flags)
            //    define_arg!(0, Int);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Struct);
            //    define_arg!(3, Address);
            //    define_arg!(4, Flag);
            //},
            //"open_by_handle_at" => {
            //    // int open_by_handle_at(int mount_fd, struct file_handle *handle, int flags)
            //    define_arg!(0, Int);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Flag);
            //},
            //"memfd_create" => {
            //    // int memfd_create(const char *name, unsigned int flags)
            //    syscall.args[0] = Some(Box::new(NullBuf::new(syscall.raw.args[0])));
            //    syscall.args[1] = Some(Box::new(Flag::new(syscall.raw.args[1])));
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Flag);
            //},
            //"mknod" => {
            //    // int mknod(const char *pathname, mode_t mode, dev_t dev)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //},
            //"mknodat" => {
            //    // int mknodat(int dirfd, const char *pathname, mode_t mode, dev_t dev)
            //    define_arg!(0, Int);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Int);
            //    define_arg!(3, Int);
            //},
            //"rename" => {
            //    // int rename(const char *oldpath, const char *newpath)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, NullBuf);
            //},
            //"renameat" => {
            //    // int renameat(int olddirfd, const char *oldpath, int newdirfd, const char *newpath)
            //    define_arg!(0, Int);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Int);
            //    define_arg!(3, NullBuf);
            //},
            //"renameat2" => {
            //    // int renameat2(int olddirfd, const char *oldpath, int newdirfd, const char *newpath, unsigned int flags)
            //    define_arg!(0, Int);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Int);
            //    define_arg!(3, NullBuf);
            //    define_arg!(4, Int);
            //},
            //"truncate" => {
            //    // int truncate(const char *path, off_t length)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Offset);
            //},
            //"ftruncate" => {
            //    // int ftruncate(int fd, off_t length)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Offset);
            //},
            //"fallocate" => {
            //    // int fallocate(int fd, int mode, off_t offset, off_t len)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Int);
            //    define_arg!(2, Offset);
            //    define_arg!(3, Offset);
            //},

            // /* Directory operations */
            //"mkdir" => {
            //    // int mkdir(const char *pathname, mode_t mode)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Int);
            //},
            //"mkdirat" => {
            //    // int mkdirat(int dirfd, const char *pathname, mode_t mode)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Int);
            //},
            //"rmdir" => {
            //    // int rmdir(const char *pathname)
            //    define_arg!(0, NullBuf);
            //},
            //"getcwd" => {
            //    // char *getcwd(char *buf, size_t size)
            //    define_buffer!(0, 1);
            //    define_arg!(1, Size);
            //},
            //"chdir" => {
            //    // int chdir(const char *path)
            //    define_arg!(0, NullBuf);
            //},
            //"fchdir" => {
            //    // int fchdir(int fd)
            //    define_arg!(0, Fd);
            //},
            //"chroot" => {
            //    // int chroot(const char *path)
            //    define_arg!(0, NullBuf);
            //},
            //"getdents" => {
            //    // long syscall(SYS_getdents, unsigned int fd, struct linux_dirent *dirp, unsigned int count)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Size);
            //},
            //"getdents64" => {
            //    // ssize_t getdents64(int fd, void *dirp, size_t count)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Address);
            //    define_arg!(2, Size);
            //},
            //"lookup_dcookie" => {
            //    // int syscall(SYS_lookup_dcookie, uint64_t cookie, char *buffer, size_t len)
            //    define_arg!(0, Int);
            //    define_buffer!(1, 2);
            //    define_arg!(2, Size);
            //},

            // /* Link operations */
            //"link" => {
            //    // int link(const char *oldpath, const char *newpath)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, NullBuf);
            //},
            //"linkat" => {
            //    // int linkat(int olddirfd, const char *oldpath, int newdirfd, const char *newpath, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Fd);
            //    define_arg!(3, NullBuf);
            //    define_arg!(4, Int);
            //},
            //"symlink" => {
            //    // int symlink(const char *target, const char *linkpath)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, NullBuf);
            //},
            //"symlinkat" => {
            //    // int symlinkat(const char *target, int newdirfd, const char *linkpath)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Fd);
            //    define_arg!(2, NullBuf);
            //},
            //"unlink" => {
            //    // int unlink(const char *pathname)
            //    define_arg!(0, NullBuf);
            //},
            //"unlinkat" => {
            //    // int unlinkat(int dirfd, const char *pathname, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Flag);
            //},
            //"readlink" => {
            //    // ssize_t readlink(const char *restrict pathname, char *restrict buf, size_t bufsiz)
            //    define_arg!(0, NullBuf);
            //    define_buffer!(1, 2);
            //    define_arg!(2, Size);
            //},
            //"readlinkat" => {
            //    // ssize_t readlinkat(int dirfd, const char *restrict pathname, char *restrict buf, size_t bufsiz)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_buffer!(2, 3);
            //    define_arg!(4, Size);
            //},

            // /* Basic file attributes */
            //"umask" => {
            //    // mode_t umask(mode_t mask)
            //    define_arg!(0, Int);
            //},
            //"stat" => {
            //    // int stat(const char *restrict pathname, struct stat *restrict statbuf)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Struct);
            //},
            //"lstat" => {
            //    // int lstat(const char *restrict pathname, struct stat *restrict statbuf)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Struct);
            //},
            //"fstat" => {
            //    // int fstat(int fd, struct stat *statbuf)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //},
            //"fstatat" => {
            //    // int fstatat(int dirfd, const char *restrict pathname, struct stat *restrict statbuf, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Struct);
            //},
            //"chmod" => {
            //    // int chmod(const char *pathname, mode_t mode)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Int);
            //},
            //"fchmod" => {
            //    // int fchmod(int fd, mode_t mode)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Int);
            //},
            //"fchmodat" => {
            //    // int fchmodat(int dirfd, const char *pathname, mode_t mode, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Int);
            //    define_arg!(3, Flag);
            //},
            //"chown" => {
            //    // int chown(const char *pathname, uid_t owner, gid_t group)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //},
            //"lchown" => {
            //    // int lchown(const char *pathname, uid_t owner, gid_t group)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //},
            //"fchown" => {
            //    // int fchown(int fd, uid_t owner, gid_t group)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //},
            //"fchownat" => {
            //    // int fchownat(int dirfd, const char *pathname, uid_t owner, gid_t group, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Int);
            //    define_arg!(3, Int);
            //    define_arg!(4, Flag);
            //},
            //"utime" => {
            //    // int utime(const char *filename, const struct utimbuf *times)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Struct);
            //},
            //"utimes" => {
            //    // int utimes(const char *filename, const struct timeval times[2])
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Address);
            //},
            //"futimesat" => {
            //    // int futimesat(int dirfd, const char *pathname, const struct timeval times[2])
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Address);
            //},
            //"utimensat" => {
            //    //  int utimensat(int dirfd, const char *pathname, const struct timespec times[2], int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Struct);
            //    define_arg!(3, Flag);
            //},
            //"futimens" => {
            //    // mode_t umask(mode_t mask)
            //    define_arg!(0, Int);
            //}
            //"access" => {
            //    //  int access(const char *pathname, int mode)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Int);
            //},
            //"faccessat" => {
            //    // int faccessat(int dirfd, const char *pathname, int mode, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Int);
            //    define_arg!(3, Flag);
            //},
            //"faccessat2" => {
            //    // int syscall(SYS_faccessat2, int dirfd, const char *pathname, int mode, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Int);
            //    define_arg!(3, Flag);
            //},

            // /* Extended file attributes */
            //"setxattr" => {
            //    // int setxattr(const char *path, const char *name, const void *value, size_t size, int flags)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Address);
            //    define_arg!(3, Size);
            //    define_arg!(4, Flag);
            //},
            //"lsetxattr" => {
            //    // int lsetxattr(const char *path, const char *name, const void *value, size_t size, int flags)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Address);
            //    define_arg!(3, Size);
            //    define_arg!(4, Flag);
            //},
            //"fsetxattr" => {
            //    // int fsetxattr(int fd, const char *name, const void *value, size_t size, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Address);
            //    define_arg!(3, Size);
            //    define_arg!(4, Flag);
            //},
            //"getxattr" => {
            //    // ssize_t getxattr(const char *path, const char *name, void *value, size_t size)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Address);
            //    define_arg!(3, Size);
            //},
            //"lgetxattr" => {
            //    // ssize_t lgetxattr(const char *path, const char *name, void *value, size_t size)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Address);
            //    define_arg!(3, Size);
            //},
            //"fgetxattr" => {
            //    // ssize_t fgetxattr(int fd, const char *name, void *value, size_t size)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Address);
            //    define_arg!(3, Size);
            //},
            //"listxattr" => {        // XXX: should le char* be considered as a buffer?
            //    // ssize_t listxattr(const char *path, char *list, size_t size)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Address);
            //    define_arg!(2, Size);
            //},
            //"llistxattr" => {
            //    // ssize_t llistxattr(const char *path, char *list, size_t size)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Address);
            //    define_arg!(2, Size);
            //},
            //"flistxattr" => {
            //    // ssize_t flistxattr(int fd, char *list, size_t size)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Address);
            //    define_arg!(2, Size);
            //},
            //"removexattr" => {
            //    // int removexattr(const char *path, const char *name)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, NullBuf);
            //},
            //"lremovexattr" => {
            //    // int lremovexattr(const char *path, const char *name)
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, NullBuf);
            //},
            //"fremovexattr" => {
            //    // int fremovexattr(int fd, const char *name)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //},

            // /* File descriptor manipulations */
            //"ioctl" => {    // XXX
            //    // int ioctl(int fd, unsigned long request, ...)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //    define_arg!(3, Int);
            //    define_arg!(4, Int);
            //    define_arg!(5, Int);
            //    define_arg!(6, Int);
            //},
            //"fcntl" => {    // XXX
            //    // int fcntl(int fd, int cmd, ... /* arg */ )
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //    define_arg!(3, Int);
            //    define_arg!(4, Int);
            //    define_arg!(5, Int);
            //    define_arg!(6, Int);
            //},
            //"dup" => {
            //    // int dup(int oldfd)
            //    define_arg!(0, Fd);
            //},
            //"dup2" => {
            //    // int dup2(int oldfd, int newfd)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Fd);
            //},
            //"dup3" => {
            //    // int dup3(int oldfd, int newfd, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Fd);
            //    define_arg!(3, Flag);
            //},
            //"flock" => {
            //    // int flock(int fd, int operation)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Int);
            //},

            // /* Read / Write */
            //"read" => {
            //    // ssize_t read(int fd, void *buf, size_t count)
            //    define_arg!(0, Fd);
            //    define_buffer!(1, 2);
            //    define_arg!(2, Size);
            //},
            //"readv" => {
            //    // ssize_t readv(int fd, const struct iovec *iov, int iovcnt)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //},
            //"pread" => {
            //    // ssize_t pread(int fd, void *buf, size_t count, off_t offset)
            //    define_arg!(0, Fd);
            //    define_buffer!(1, 2);
            //    define_arg!(2, Size);
            //    define_arg!(3, Offset);
            //},
            //"preadv" => {
            //    // ssize_t preadv(int fd, const struct iovec *iov, int iovcnt, off_t offset);
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //    define_arg!(3, Offset);
            //},
            //"preadv2" => {
            //    // ssize_t preadv2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //    define_arg!(3, Offset);
            //    define_arg!(4, Flag);
            //},
            //"write" => {
            //    //
            //    define_arg!(0, Fd);
            //    define_buffer!(1, 2);
            //    define_arg!(2, Size);
            //},
            //"writev" => {
            //    // ssize_t writev(int fd, const struct iovec *iov, int iovcnt)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //},
            //"pwrite" => {
            //    // ssize_t pwrite(int fd, const void *buf, size_t count, off_t offset)
            //    define_arg!(0, Fd);
            //    define_buffer!(1, 2);
            //    define_arg!(2, Size);
            //    define_arg!(3, Offset);
            //},
            //"pwritev" => {
            //    // ssize_t pwritev(int fd, const struct iovec *iov, int iovcnt, off_t offset)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //    define_arg!(3, Offset);
            //},
            //"pwritev2" => {
            //    // ssize_t pwritev(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //    define_arg!(3, Offset);
            //    define_arg!(4, Flag);
            //},
            //"lseek" => {
            //    // off_t lseek(int fd, off_t offset, int whence)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Offset);
            //    define_arg!(2, Int);
            //},
            //"sendfile" => {     // XXX: *offset & count -> Buffer?
            //    // ssize_t sendfile(int out_fd, int in_fd, off_t *offset, size_t count)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Fd);
            //    define_arg!(2, Address);
            //    define_arg!(3, Size);
            //},

            // /* Synchronized I/O */
            //"fdatasync" => {
            //    // int fdatasync(int fd)
            //    define_arg!(0, Fd);
            //},
            //"fsync" => {
            //    // int fsync(int fd)
            //    define_arg!(0, Fd);
            //},
            //"msync" => {
            //    // int msync(void *addr, size_t length, int flags)
            //    define_arg!(0, Address);
            //    define_arg!(1, Size);
            //    define_arg!(2, Flag);
            //},
            //"sync_file_range" => {
            //    // int sync_file_range(int fd, off64_t offset, off64_t nbytes, unsigned int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Offset);
            //    define_arg!(2, Offset);
            //    define_arg!(3, Flag);
            //},
            //"sync" => {
            //    // int syncfs(int fd)
            //    define_arg!(0, Fd);
            //},
            //"syncfs" => {
            //    // int syncfs(int fd)
            //    define_arg!(0, Fd);
            //},

            // /* Asynchronous I/O */
            //"io_setup" => {
            //    // long io_setup(unsigned int nr_events, aio_context_t *ctx_idp)
            //    define_arg!(0, Int);
            //    define_arg!(1, Address);
            //},
            //"io_destroy" => {
            //    //  int syscall(SYS_io_destroy, aio_context_t ctx_id)
            //    define_arg!(0, Int);
            //},
            //"io_submit" => {        // XXX: double pointer!
            //    // int io_submit(aio_context_t ctx_id, long nr, struct iocb **iocbpp)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Address);
            //},
            //"io_cancel" => {
            //    // int syscall(SYS_io_cancel, aio_context_t ctx_id, struct iocb *iocb, struct io_event *result)
            //    define_arg!(0, Int);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Struct);
            //},
            //"io_getevents" => {
            //    // int syscall(SYS_io_getevents, aio_context_t ctx_id, long min_nr, long nr, struct io_event *events, struct timespec *timeout)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //    define_arg!(2, Struct);
            //    define_arg!(2, Struct);
            //},

            // /* Multiplexed I/O */
            //"select" => {       // XXX: redo the argument parsing
            //    // int select(int nfds, fd_set *restrict readfds, fd_set *restrict writefds, fd_set *restrict exceptfds, struct timeval *restrict timeout)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //    define_arg!(3, Int);
            //    define_arg!(4, Int);
            //    define_arg!(5, Int);
            //},
            //"pselect" => {      // XXX: redo the argument parsing
            //    // int pselect(int nfds, fd_set *restrict readfds, fd_set *restrict writefds, fd_set *restrict exceptfds, const struct timespec *restrict timeout, const sigset_t *restrict sigmask)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //    define_arg!(3, Int);
            //    define_arg!(4, Int);
            //    define_arg!(5, Int);
            //    define_arg!(6, Int);
            //},
            //"poll" => {     // XXX list of FD
            //    // int poll(struct pollfd *fds, nfds_t nfds, int timeout)
            //    define_arg!(0, Address);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //},
            //"ppoll" => {    // XXX list of FD
            //    // int ppoll(struct pollfd *fds, nfds_t nfds, const struct timespec *tmo_p, const sigset_t *sigmask)
            //    define_arg!(0, Address);
            //    define_arg!(1, Int);
            //    define_arg!(2, Struct);
            //    define_arg!(3, Address);
            //},
            //"epoll_create" => {
            //    // int epoll_create(int size)
            //    define_arg!(0, Size);
            //},
            //"epoll_create1" => {
            //    // int epoll_create1(int flags)
            //    define_arg!(0, Flag);
            //},
            //"epoll_ctl" => {
            //    // int epoll_ctl(int epfd, int op, int fd, struct epoll_event *event)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Int);
            //    define_arg!(2, Fd);
            //    define_arg!(3, Struct);
            //},
            //"epoll_wait" => {
            //    // int epoll_wait(int epfd, struct epoll_event *events, int maxevents, int timeout)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //    define_arg!(3, Int);
            //},
            //"epoll_pwait" => {
            //    // int epoll_pwait(int epfd, struct epoll_event *events, int maxevents, int timeout, const sigset_t *sigmask)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //    define_arg!(3, Int);
            //    define_arg!(4, Struct);
            //},
            //"epoll_pwait2" => {
            //    // int epoll_pwait2(int epfd, struct epoll_event *events, int maxevents, const struct timespec *timeout, const sigset_t *sigmask)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //    define_arg!(3, Struct);
            //    define_arg!(4, Struct);
            //},

            // /* Monitorng file events */
            //"inotify_init" => {
            //    // int inotify_init(void)
            //},
            //"inotify_init1" => {
            //    //  int inotify_init1(int flags)
            //    define_arg!(0, Flag);
            //},
            //"inotify_add_watch" => {
            //    // int inotify_add_watch(int fd, const char *pathname, uint32_t mask)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Int);
            //},
            //"inotify_rm_watch" => {
            //    //  int inotify_rm_watch(int fd, int wd)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Int);
            //},
            //"fanotify_init" => {
            //    // int fanotify_init(unsigned int flags, unsigned int event_f_flags)
            //    define_arg!(0, Flag);
            //    define_arg!(1, Int);
            //},
            //"fanotify_mark" => {
            //    // int fanotify_mark(int fanotify_fd, unsigned int flags, uint64_t mask, int dirfd, const char *pathname)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Flag);
            //    define_arg!(2, Int);
            //    define_arg!(3, Fd);
            //    define_arg!(4, NullBuf);
            //},

            // /* Miscellaneous */
            //"fadvise64" => {
            //    //  int posix_fadvise(int fd, off_t offset, off_t len, int advice)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Offset);
            //    define_arg!(2, Offset);
            //    define_arg!(3, Int);
            //},
            //"readahead" => {
            //    //  ssize_t readahead(int fd, off64_t offset, size_t count)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Offset);
            //    define_arg!(2, Size);
            //},
            //"getrandom" => {
            //    //  ssize_t getrandom(void *buf, size_t buflen, unsigned int flags)
            //    define_buffer!(0, 1);
            //    define_arg!(1, Size);
            //    define_arg!(2, Flag);
            //},

            // /*  
            //"" => {
            //    // 
            //    define_arg!(0, );
            //    define_arg!(1, );
            //    define_arg!(2, );
            //},
            //*/

            // /* 
            //  * Network
            //  */

            // /* Socket operations */
            //"socket" => {
            //    // int socket(int domain, int type, int protocol)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //},
            //"socketpair" => {
            //    // int socketpair(int domain, int type, int protocol, int sv[2])
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //    define_arg!(3, Address);
            //},
            //"getsockopt" => {
            //    // int getsockopt(int sockfd, int level, int optname, void *restrict optval, socklen_t *restrict optlen)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //    define_arg!(3, Address);
            //    define_arg!(4, Address);
            //},
            //"setsockopt" => {
            //    // int setsockopt(int sockfd, int level, int optname, const void *optval, socklen_t optlen)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //    define_arg!(3, Address);
            //    define_arg!(4, Int);
            //},
            //"getsockname" => {
            //    // int getsockname(int sockfd, struct sockaddr *restrict addr, socklen_t *restrict addrlen)
            //    define_arg!(0, Int);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //},
            //"getpeername" => {
            //    // int getpeername(int sockfd, struct sockaddr *restrict addr, socklen_t *restrict addrlen)
            //    define_arg!(0, Int);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //},
            //"bind" => {
            //    //  int bind(int sockfd, const struct sockaddr *addr, socklen_t addrlen)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //},
            //"listen" => {
            //    // int listen(int sockfd, int backlog)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //},
            //"accept" => {
            //    //  int accept(int sockfd, struct sockaddr *restrict addr, socklen_t *restrict addrlen)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //},
            //"accept4" => {
            //    //  int accept4(int sockfd, struct sockaddr *restrict addr, socklen_t *restrict addrlen, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //    define_arg!(3, Int);
            //},
            //"connect" => {
            //    // int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //},
            //"shutdown" => {
            //    // int shutdown(int sockfd, int how)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Int);
            //},
            //
            // /* Send / Receive */
            //"recv" => {
            //    // ssize_t recv(int sockfd, void *buf, size_t len, int flags)
            //    define_arg!(0, Fd);
            //    define_buffer!(1, 2);
            //    define_arg!(2, Size);
            //    define_arg!(3, Flag);
            //},
            //"recvfrom" => {
            //    //  ssize_t recvfrom(int sockfd, void *restrict buf, size_t len, int flags, struct sockaddr *restrict src_addr, socklen_t *restrict addrlen)
            //    define_arg!(0, Fd);
            //    define_buffer!(1, 2);
            //    define_arg!(2, Size);
            //    define_arg!(3, Flag);
            //    define_arg!(4, Struct);
            //    define_arg!(5, Address);
            //},
            //"recvmsg" => {
            //    // ssize_t recvmsg(int sockfd, struct msghdr *msg, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Flag);
            //},
            //"recvmmsg" => {
            //    //  int recvmmsg(int sockfd, struct mmsghdr *msgvec, unsigned int vlen, int flags, struct timespec *timeout)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Size);
            //    define_arg!(3, Flag);
            //    define_arg!(4, Struct);
            //},
            //"send" => {
            //    // ssize_t send(int sockfd, const void *buf, size_t len, int flags)
            //    define_arg!(0, Fd);
            //    define_buffer!(1, 2);
            //    define_arg!(2, Size);
            //    define_arg!(3, Flag);
            //},
            //"sendto" => {
            //    //  ssize_t sendto(int sockfd, const void *buf, size_t len, int flags, const struct sockaddr *dest_addr, socklen_t addrlen)
            //    define_arg!(0, Fd);
            //    define_buffer!(1, 2);
            //    define_arg!(2, Size);
            //    define_arg!(3, Flag);
            //    define_arg!(4, Struct);
            //    define_arg!(5, Size);
            //},
            //"sendmsg" => {
            //    //  ssize_t sendmsg(int sockfd, const struct msghdr *msg, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Flag);
            //},
            //"sendmmsg" => {
            //    //  int sendmmsg(int sockfd, struct mmsghdr *msgvec, unsigned int vlen, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Int);
            //    define_arg!(3, Flag);
            //},

            // /* Naming */
            //"sethostname" => {
            //    //  int sethostname(const char *name, size_t len)
            //    define_buffer!(0, 1);
            //    define_arg!(1, Size);
            //},
            //"gethostname" => {
            //    //  int gethostname(char *name, size_t len)
            //    define_buffer!(0, 1);
            //    define_arg!(1, Size);
            //},
            //"getdomainname" => {
            //    // int getdomainname(char *name, size_t len)
            //    define_buffer!(0, 1);
            //    define_arg!(1, Size);
            //},
            //"setdomainname" => {
            //    //  int setdomainname(const char *name, size_t len)
            //    define_buffer!(0, 1);
            //    define_arg!(1, Size);
            //},

            // /* Packet filtering */
            //"bpf" => {
            //    //  int bpf(int cmd, union bpf_attr *attr, unsigned int size)
            //    define_arg!(0, Int);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Size);
            //},

            // /* 
            //  * TODO Time
            //  */
            //
            // /* 
            //  * Process management
            //  */
            //
            // /* Creation and termination */
            //"fork" => {
            //    // pid_t fork(void)
            //},
            //"vfork" => {
            //    // pid_t vfork(void)
            //},
            //"execve" => {
            //    // int execve(const char *pathname, char *const argv[], char *const envp[])
            //    define_arg!(0, NullBuf);
            //    define_arg!(1, Address);
            //    define_arg!(2, Address);
            //},
            //"execveat" => {
            //    // int execveat(int dirfd, const char *pathname, const char *const argv[], const char *const envp[], int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, NullBuf);
            //    define_arg!(2, Address);
            //    define_arg!(3, Address);
            //    define_arg!(3, Int);
            //},
            //"exit" => {
            //    // noreturn void _exit(int status)
            //    define_arg!(0, Int);
            //},
            //"exit_group" => {
            //    // noreturn void syscall(SYS_exit_group, int status)
            //    define_arg!(0, Int);
            //},
            //"wait4" => {
            //    // pid_t wait4(pid_t pid, int *wstatus, int options, struct rusage *rusage)
            //    define_arg!(0, Int);
            //    define_arg!(1, Address);
            //    define_arg!(2, Int);
            //    define_arg!(3, Struct);
            //},
            //"waitid" => {
            //    // int waitid(idtype_t idtype, id_t id, siginfo_t *infop, int options)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Address);
            //    define_arg!(3, Int);
            //},

            // /* Process id */
            //"getpid" => {
            //    // pid_t getpid(void)
            //},
            //"getppid" => {
            //    // pid_t getppid(void)
            //},
            //"gettid" => {
            //    // pid_t gettid(void)
            //},

            // /* Session id */
            //"setsid" => {
            //    // pid_t setsid(void)
            //},
            //"getsid" => {
            //    // pid_t getsid(pid_t pid)
            //    define_arg!(0, Int);
            //},

            // /* Process group id */
            //"setpgid" => {
            //    // int setpgid(pid_t pid, pid_t pgid)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //},
            //"getpgid" => {
            //    // pid_t getpgid(pid_t pid)
            //    define_arg!(0, Int);
            //},
            //"getpgrp" => {
            //    // pid_t getpgrp(void)
            //},

            // /* TODO ... */
            //
            // /* 
            //  * Signals
            //  */
            //
            // /* Standard signals */
            //"kill" => {
            //    //  int kill(pid_t pid, int sig)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //},
            //"tkill" => {
            //    //  int syscall(SYS_tkill, pid_t tid, int sig)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //},
            //"tgkill" => {
            //    //  int tgkill(pid_t tgid, pid_t tid, int sig)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //},
            //"pause" => {
            //    //  int pause(void)
            //},

            // /* Real-time signals */
            //"rt_sigaction" => {
            //    //  int sigaction(int signum, const struct sigaction *restrict act, struct sigaction *restrict oldact)
            //    define_arg!(0, Int);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Struct);
            //},
            //"rt_sigprocmask" => {
            //    //  int syscall(SYS_rt_sigprocmask, int how, const kernel_sigset_t *set, kernel_sigset_t *oldset, size_t sigsetsize)
            //    define_arg!(0, Int);
            //    define_arg!(1, Address);
            //    define_arg!(2, Address);
            //    define_arg!(2, Size);
            //},
            //"rt_sigpending" => {
            //    //  int sigpending(sigset_t *set)
            //    define_arg!(0, Address);
            //},
            //"rt_sigqueueinfo" => {
            //    //  int syscall(SYS_rt_sigqueueinfo, pid_t tgid, int sig, siginfo_t *info)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Address);
            //},
            //"rt_tgsigqueueinfo" => {
            //    //  int syscall(SYS_rt_tgsigqueueinfo, pid_t tgid, pid_t tid, int sig, siginfo_t *info)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //    define_arg!(3, Address);
            //},
            //"rt_sigtimewait" => {
            //    //  int sigtimedwait(const sigset_t *restrict set, siginfo_t *restrict info, const struct timespec *restrict timeout)
            //    define_arg!(0, Address);
            //    define_arg!(1, Address);
            //    define_arg!(2, Struct);
            //},
            //"rt_sigsuspend" => {
            //    //  int sigsuspend(const sigset_t *mask)
            //    define_arg!(0, Address);
            //},
            //"rt_sigreturn" => {
            //    //  int sigreturn(...)
            //},
            //"signalstack" => {
            //    //  int sigaltstack(const stack_t *restrict ss, stack_t *restrict old_ss)
            //    define_arg!(0, Address);
            //    define_arg!(1, Address);
            //},

            // /* File descriptor based signals */
            //"signalfd" => {
            //    //  int signalfd(int fd, const sigset_t *mask, int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Address);
            //    define_arg!(2, Int);
            //},
            //"signalfd4" => {
            //},
            //"eventfd" => {
            //    //  int eventfd(unsigned int initval, int flags)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //},
            //"eventfd2" => {
            //},

            // /* TODO Miscellaneous */

            // /* 
            //  * IPC
            //  */
            //
            // /* Pipe */
            //"pipe" => {
            //    //  int pipe(int pipefd[2])
            //    define_arg!(0, Address);
            //},
            //"pipe2" => {
            //    //  int pipe2(int pipefd[2], int flags)
            //    define_arg!(0, Address);
            //},
            //"tee" => {
            //    //  ssize_t tee(int fd_in, int fd_out, size_t len, unsigned int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Int);
            //    define_arg!(2, Size);
            //    define_arg!(3, Int);
            //},
            //"splice" => {
            //    //  ssize_t splice(int fd_in, off64_t *off_in, int fd_out, off64_t *off_out, size_t len, unsigned int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Address);
            //    define_arg!(2, Fd);
            //    define_arg!(3, Address);
            //    define_arg!(4, Size);
            //    define_arg!(5, Flag);
            //},
            //"vmsplice" => {
            //    //  ssize_t vmsplice(int fd, const struct iovec *iov, size_t nr_segs, unsigned int flags)
            //    define_arg!(0, Fd);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Size);
            //    define_arg!(3, Int);
            //},


            // /* Shared memory */
            //"shmget" => {
            //    //  int shmget(key_t key, size_t size, int shmflg)
            //    define_arg!(0, Int);
            //    define_arg!(1, Size);
            //    define_arg!(2, Int);
            //},
            //"shmctl" => {
            //    //  int shmctl(int shmid, int cmd, struct shmid_ds *buf)
            //    define_arg!(0, Int);
            //    define_arg!(1, Int);
            //    define_arg!(2, Struct);
            //},
            //"shmat" => {
            //    //  void *shmat(int shmid, const void *shmaddr, int shmflg)
            //    define_arg!(0, Int);
            //    define_arg!(1, Address);
            //    define_arg!(2, Int);
            //},
            //"shmdt" => {
            //    //  int shmdt(const void *shmaddr)
            //    define_arg!(0, Address);
            //},

            // /* TODO: Semaphores */

            // /* Futexes */
            //"futex" => {
            //    //  long syscall(SYS_futex, uint32_t *uaddr, int futex_op, uint32_t val, const struct timespec *timeout,   /* or: uint32_t val2 */ uint32_t *uaddr2, uint32_t val3)
            //    define_arg!(0, Address);
            //    define_arg!(1, Int);
            //    define_arg!(2, Int);
            //    define_arg!(3, Struct);
            //    define_arg!(4, Address);
            //    define_arg!(5, Int);
            //},
            //"set_robust_list" => {
            //    //  long syscall(SYS_set_robust_list, struct robust_list_head *head, size_t len)
            //    define_arg!(0, Struct);
            //    define_arg!(1, Size);
            //},
            //"get_robust_list" => {
            //    //  long syscall(SYS_get_robust_list, int pid, struct robust_list_head **head_ptr, size_t *len_ptr)
            //    define_arg!(0, Int);
            //    define_arg!(1, Struct);
            //    define_arg!(2, Address);
            //    define_arg!(2, Size);
            //},

            // /* TODO Message queues */
            //
            // /* 
            //  * TODO Non-uniform memory access
            //  */
            //
            // /* 
            //  * TODO Linux key-management 
            //  */

            // /* 
            //  * TODO System-wide
            //  */
            _ => (),
        }
    }


    pub fn decode_exit(&self) { }
}



pub trait Decode: {
    //fn as_any(&self) -> &dyn Any;
    #[allow(unused_variables)]
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) { }
    fn print(&self) { }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub enum DecodedSyscall {
    /* Filesystem */
    //#[serde(rename = "close")]
    Close(syscall::open::Close),
    Creat(syscall::open::Creat),
    Open(syscall::open::Open),
    Openat(syscall::open::Openat),
    Openat2(syscall::open::Openat2),

    /* io */
    Read(syscall::io::Read),
    Write(syscall::io::Write),
    Readv(syscall::io::Readv),
    Writev(syscall::io::Writev),
    Pread(syscall::io::Pread),
    Pwrite(syscall::io::Pwrite),
    Preadv(syscall::io::Preadv),
    Pwritev(syscall::io::Pwritev),
    Preadv2(syscall::io::Preadv2),
    Pwritev2(syscall::io::Pwritev2),

    Ioctl(syscall::ioctl::Ioctl),

    Fallocate(syscall::fallocate::Fallocate),

    NameToHandleAt(syscall::file_handle::NameToHandleAt),
    OpenByHandleAt(syscall::file_handle::OpenByHandleAt),

    MemfdCreate(syscall::memfd::MemfdCreate),

    Mknod(syscall::mknod::Mknod),
    Mknodat(syscall::mknod::Mknodat),

    Rename(syscall::renameat::Rename),
    Renameat(syscall::renameat::Renameat),
    Renameat2(syscall::renameat::Renameat2),

    Truncate(syscall::truncate::Truncate),
    Ftruncate(syscall::truncate::Ftruncate),

    Access(syscall::access::Access),
    Faccessat(syscall::access::Faccessat),
    Faccessat2(syscall::access::Faccessat2),

    /* mmap */
    Brk(syscall::mmap::Brk),
    Sbrk(syscall::mmap::Sbrk),
    Mmap(syscall::mmap::Mmap),
    Mremap(syscall::mmap::Mremap),
    Munmap(syscall::mmap::Munmap),
    Mprotect(syscall::mmap::Mprotect),
    Madvise(syscall::mmap::Madvise),

    Execve(syscall::execve::Execve),
    Execveat(syscall::execve::Execveat),

    Prctl(syscall::prctl::Prctl),
    ArchPrctl(syscall::prctl::ArchPrctl),

    Getdents(syscall::dirent::Getdents),
    Getdents64(syscall::dirent::Getdents64),
    Readdir(syscall::dirent::Readdir),

    Stat(syscall::stat::Stat),
    Fstat(syscall::stat::Fstat),
    Lstat(syscall::stat::Lstat),
    Fstatat(syscall::stat::Fstatat),

    Getrlimit(syscall::resource::Getrlimit),
    Setrlimit(syscall::resource::Setrlimit),
    Prlimit(syscall::resource::Prlimit),
    Getrusage(syscall::resource::Getrusage),


    /* ... */
}

impl Decode for DecodedSyscall {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        match self {
            DecodedSyscall::Close(x) => x.decode(pid, operation),
            DecodedSyscall::Creat(x) => x.decode(pid, operation),
            DecodedSyscall::Open(x) => x.decode(pid, operation),
            DecodedSyscall::Openat(x) => x.decode(pid, operation),
            DecodedSyscall::Openat2(x) => x.decode(pid, operation),
            DecodedSyscall::Read(x) => x.decode(pid, operation),
            DecodedSyscall::Write(x) => x.decode(pid, operation),
            DecodedSyscall::Readv(x) => x.decode(pid, operation),
            DecodedSyscall::Writev(x) => x.decode(pid, operation),
            DecodedSyscall::Pread(x) => x.decode(pid, operation),
            DecodedSyscall::Pwrite(x) => x.decode(pid, operation),
            DecodedSyscall::Preadv(x) => x.decode(pid, operation),
            DecodedSyscall::Pwritev(x) => x.decode(pid, operation),
            DecodedSyscall::Preadv2(x) => x.decode(pid, operation),
            DecodedSyscall::Pwritev2(x) => x.decode(pid, operation),
            DecodedSyscall::Ioctl(x) => x.decode(pid, operation),
            DecodedSyscall::Brk(x) => x.decode(pid, operation),
            DecodedSyscall::Sbrk(x) => x.decode(pid, operation),
            DecodedSyscall::Mmap(x) => x.decode(pid, operation),
            DecodedSyscall::Mremap(x) => x.decode(pid, operation),
            DecodedSyscall::Munmap(x) => x.decode(pid, operation),
            DecodedSyscall::Mprotect(x) => x.decode(pid, operation),
            DecodedSyscall::Madvise(x) => x.decode(pid, operation),
            DecodedSyscall::Execve(x) => x.decode(pid, operation),
            DecodedSyscall::Execveat(x) => x.decode(pid, operation),
            DecodedSyscall::Fallocate(x) => x.decode(pid, operation),
            DecodedSyscall::NameToHandleAt(x) => x.decode(pid, operation),
            DecodedSyscall::OpenByHandleAt(x) => x.decode(pid, operation),
            DecodedSyscall::MemfdCreate(x) => x.decode(pid, operation),
            DecodedSyscall::Mknod(x) => x.decode(pid, operation),
            DecodedSyscall::Mknodat(x) => x.decode(pid, operation),
            DecodedSyscall::Rename(x) => x.decode(pid, operation),
            DecodedSyscall::Renameat(x) => x.decode(pid, operation),
            DecodedSyscall::Renameat2(x) => x.decode(pid, operation),
            DecodedSyscall::Truncate(x) => x.decode(pid, operation),
            DecodedSyscall::Ftruncate(x) => x.decode(pid, operation),
            DecodedSyscall::Access(x) => x.decode(pid, operation),
            DecodedSyscall::Faccessat(x) => x.decode(pid, operation),
            DecodedSyscall::Faccessat2(x) => x.decode(pid, operation),
            DecodedSyscall::Prctl(x) => x.decode(pid, operation),
            DecodedSyscall::ArchPrctl(x) => x.decode(pid, operation),
            DecodedSyscall::Getdents(x) => x.decode(pid, operation),
            DecodedSyscall::Getdents64(x) => x.decode(pid, operation),
            DecodedSyscall::Readdir(x) => x.decode(pid, operation),
            DecodedSyscall::Stat(x) => x.decode(pid, operation),
            DecodedSyscall::Fstat(x) => x.decode(pid, operation),
            DecodedSyscall::Lstat(x) => x.decode(pid, operation),
            DecodedSyscall::Fstatat(x) => x.decode(pid, operation),
            DecodedSyscall::Getrlimit(x) => x.decode(pid, operation),
            DecodedSyscall::Setrlimit(x) => x.decode(pid, operation),
            DecodedSyscall::Prlimit(x) => x.decode(pid, operation),
            DecodedSyscall::Getrusage(x) => x.decode(pid, operation),
            //DecodedSyscall::(x) => x.decode(pid, operation),
        }
    }
}
