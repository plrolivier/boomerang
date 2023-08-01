/*
 *
 */
use sysfwd::{
    syscall::{
        Syscall,
        decoder::DecodedSyscall,
    },
    tracer::filtering::{ Decision, Rule },
};



pub struct ForwardFileRule {
    pub filename: String,
    fd: Option<usize>,
    trigger_on_entry: bool,     // to avoid having to recheck during the callback
}

impl ForwardFileRule {

    pub fn new(filename: String) -> Self 
    {
        ForwardFileRule { filename: filename, fd: None, trigger_on_entry: false }
    }

}

impl Rule for ForwardFileRule {

    fn filter_entry(&mut self, syscall: Syscall) -> Result<Decision, std::io::Error>
    {
        let mut decision = Decision::Pass;
        self.trigger_on_entry = false;

        //println!("Syscall: {:?}", syscall);
        if let Some(decoded_syscall) = syscall.decoded {

            match decoded_syscall {

                /* Open class syscalls */
                DecodedSyscall::Open(sc) => {
                    let bytes_filename: Vec<u8> = self.filename.bytes().collect::<Vec<_>>();
                    /* Note: the filename should exactly match the pathname and not a substring of it */
                    /* It could be nice to have some sort of regex pattern matching */
                    if sc.pathname.content ==  bytes_filename {
                        self.trigger_on_entry = true;
                        decision = Decision::Forward;
                    }
                    // TODO: recuperer FD on syscall returns
                }
                DecodedSyscall::Creat(sc) => {
                    let bytes_filename: Vec<u8> = self.filename.bytes().collect::<Vec<_>>();
                    /* Note: the filename should exactly match the pathname and not a substring of it */
                    /* It could be nice to have some sort of regex pattern matching */
                    if sc.pathname.content ==  bytes_filename {
                        self.trigger_on_entry = true;
                        decision = Decision::Forward;
                    }
                    // TODO: recuperer FD on syscall returns
                }
                DecodedSyscall::Openat(sc) => {
                    let bytes_filename: Vec<u8> = self.filename.bytes().collect::<Vec<_>>();
                    /* Note: the filename should exactly match the pathname and not a substring of it */
                    /* It could be nice to have some sort of regex pattern matching */
                    if sc.pathname.content ==  bytes_filename {
                        self.trigger_on_entry = true;
                        decision = Decision::Forward;
                    }
                    // TODO: recuperer FD on syscall returns
                }
                DecodedSyscall::Openat2(sc) => {
                    let bytes_filename: Vec<u8> = self.filename.bytes().collect::<Vec<_>>();
                    /* Note: the filename should exactly match the pathname and not a substring of it */
                    /* It could be nice to have some sort of regex pattern matching */
                    if sc.pathname.content ==  bytes_filename {
                        self.trigger_on_entry = true;
                        decision = Decision::Forward;
                    }
                    // TODO: recuperer FD on syscall returns
                }
                DecodedSyscall::Close(sc) => {
                    match self.fd {
                        Some(fd) => {
                            if sc.fd.value == fd {
                                self.trigger_on_entry = true;
                                decision = Decision::Forward;
                            }
                        },
                        None => (),
                    }
                }

                /* Read class syscall */
                DecodedSyscall::Read(sc) => {
                    match self.fd {
                        Some(fd) => {
                            if sc.fd.value == fd {
                                self.trigger_on_entry = true;
                                decision = Decision::Forward;
                            }
                        },
                        None => (),
                    }
                }

                /* Write class syscall */
                DecodedSyscall::Write(sc) => {
                    match self.fd {
                        Some(fd) => {
                            if sc.fd.value == fd {
                                self.trigger_on_entry = true;
                                decision = Decision::Forward;
                            }
                        },
                        None => (),
                    }
                }

                DecodedSyscall::Lseek(sc) => {
                    match self.fd {
                        Some(fd) => {
                            if sc.fd.value == fd {
                                self.trigger_on_entry = true;
                                decision = Decision::Forward;
                            }
                        },
                        None => (),
                    }
                }

                /* Others */
                _ => (),
            }   // match

        }   // if let

        Ok(decision)
    }

    fn filter_exit(&mut self, syscall: Syscall) -> Result<Decision, std::io::Error>
    {
        Ok(syscall.decision.unwrap())
    }

    fn on_syscall_exit(&mut self, syscall: Syscall)
    {
        if syscall.decision.unwrap() != Decision::Forward {
            return;
        }

        if self.trigger_on_entry {
            let decoded_syscall = syscall.decoded.as_ref().unwrap();

            match decoded_syscall {

                DecodedSyscall::Close(_sc) => {
                    self.fd = None;
                }

                DecodedSyscall::Creat(sc) => {
                    match sc.retval.as_ref() {
                        Some(fd) => self.fd = Some(fd.value),
                        None => (),
                    }
                }
                DecodedSyscall::Open(sc) => {
                    match sc.retval.as_ref() {
                        Some(fd) => self.fd = Some(fd.value),
                        None => (),
                    }
                }
                DecodedSyscall::Openat(sc) => {
                    match sc.retval.as_ref() {
                        Some(fd) => self.fd = Some(fd.value),
                        None => (),
                    }
                }
                DecodedSyscall::Openat2(sc) => {
                    match sc.retval.as_ref() {
                        Some(fd) => self.fd = Some(fd.value),
                        None => (),
                    }
                }

                _ => (),
            }
        }

        self.trigger_on_entry = false;
    }
}