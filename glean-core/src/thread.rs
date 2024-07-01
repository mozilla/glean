use std::io;
use std::thread::{self, JoinHandle};

/// Spawns a new thread, returning a [`JoinHandle`] for it.
///
/// Wrapper around [`std::thread::spawn`], but automatically naming the thread.
pub fn spawn<F, T>(name: &'static str, f: F) -> Result<JoinHandle<T>, io::Error>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    thread::Builder::new().name(name.to_string()).spawn(f)
}
