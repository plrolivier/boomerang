/*
 * The lib API
 */

pub mod sync;
pub mod arch;
pub mod protocol; // Should be private?
pub mod memory;

pub mod syscall;
pub mod tracer;
pub mod executor;
//pub mod migration;
pub mod targets;


/*
 * Exposed interface
 */
pub use crate::tracer::TracerEngine;
pub use crate::executor::ExecutorEngine;

