/*
 *
 */


/*
 * The control channel is used by the tracer to manage the whole executor process.
 */
pub mod control;

/*
 * The data channel is used between tracing and executor threads to exchange syscall data with UDP.
 */
pub mod data;

