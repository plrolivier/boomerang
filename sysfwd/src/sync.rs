/*
 *
 */
use std::sync::{ Arc, Condvar, Mutex };



#[derive(Clone, Debug)]
pub struct Event {
    mutex: Arc<(Mutex<bool>, Condvar)>,
}

impl Event {

    pub fn new() -> Self
    {
        Self {
            mutex: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    pub fn set(&self) 
    {
        let &(ref lock, ref cvar) = &*self.mutex;
        let mut triggered = lock.lock().unwrap();
        *triggered = true;
        cvar.notify_all();
    }

    pub fn is_set(&self) -> bool 
    {
        let &(ref lock, _) = &*self.mutex;
        *lock.lock().unwrap()
    }

    pub fn wait(&self) 
    {
        let &(ref lock, ref cvar) = &*self.mutex;
        let mut triggered = lock.lock().unwrap();
        while !*triggered {
            triggered = cvar.wait(triggered).unwrap();
        }
    }
}