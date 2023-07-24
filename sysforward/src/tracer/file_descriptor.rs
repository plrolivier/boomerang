/*
 * To manage the file descriptor management and translation between local, remote, user and kernel.
 */

use std::{
    collections::HashSet,
};





/* 
 * Used to store the location where the FD is valid
 */
#[derive(Clone, Copy)]
pub enum FdLocation {
    Local(usize),
    Remote(usize),
}


/*
 * In order to differentiate local from remote file descriptor for the user program,
 * we add an offset to remote fd.
 */
static REMOTE_FD_OFFSET: usize = 4096;

/*
 * A wrapper structure around hashmap for managing file descriptor translation.
 * For now, it is only used to store remote FD.
 */
pub struct FdTable {
    fd_table: Vec<Option<FdLocation>>,
    available_fd: HashSet<usize>,
}

impl FdTable {

    pub fn new() -> Self
    {
        Self { 
            fd_table: Vec::new(),
            available_fd: HashSet::new(),
        }
    }

    /*
     * Create a new local-user / remote-kernel FD association.
     * Typically used during the exit of an open() system call.
     */
    fn insert(&mut self, kernel_fd: FdLocation) -> usize
    {
        if let Some(user_fd) = self.available_fd.iter().copied().min() {
            self.fd_table[user_fd] = Some(kernel_fd);
            self.available_fd.remove(&user_fd);
            user_fd + REMOTE_FD_OFFSET
        } else {
            let user_fd = self.fd_table.len();
            self.fd_table.push(Some(kernel_fd));
            user_fd + REMOTE_FD_OFFSET
        }
    }

    pub fn open_remote(&mut self, kernel_fd: usize) -> usize
    {
        let fd = FdLocation::Remote(kernel_fd);
        self.insert(fd)
    }

    /* The table is not used for local FD.
    pub fn open_local(&mut self, kernel_fd: usize) -> usize
    {
        let fd = FdLocation::Local(kernel_fd);
        self.insert(fd)
    }
    */

    /*
     * Close the local-user / remote-kernel FD association.
     * Typically used during the exit of an close() system call.
     */
    fn remove(&mut self, user_fd: usize) -> Option<FdLocation>
    {
        let user_fd = user_fd - REMOTE_FD_OFFSET;
        
        if user_fd < self.fd_table.len() {
            if let Some(kernel_fd) = self.fd_table[user_fd] {
                self.fd_table[user_fd] = None;
                self.available_fd.insert(user_fd);
                Some(kernel_fd)
            } else {
                // Element not present
                None
            }
        } else {
            // Index out of range
            None
        }
    }

    pub fn close_remote(&mut self, user_fd: usize) -> Option<usize>
    {
        if let Some(kernel_fd) = self.remove(user_fd) {
            if let (FdLocation::Remote(remote_fd)) = kernel_fd {
                Some(remote_fd)
            } else {
                panic!("FdTable is not supposed to store Local(fd) yet.");
            }
        } else {
            None
        }
    }

    //pub fn close_local(&mut self, user_fd: usize) -> Option<usize>

    /*
     * Translate a FD used in user space with the corresponding FD used by the remote kernel.
     * Typically used during the entry a read() or write() system call.
     */
    pub fn translate(&self, user_fd: usize) -> Option<usize>
    {
        let user_fd = user_fd - REMOTE_FD_OFFSET;

        if user_fd < self.fd_table.len() {
            if let Some(kernel_fd) = &self.fd_table[user_fd] {
                match kernel_fd {
                    FdLocation::Local(fd) => Some(*fd),
                    FdLocation::Remote(fd) => Some(*fd),
                }
            } else {
                None
            }
        } else {
            // Index out of range
            None
        }
    }


}
