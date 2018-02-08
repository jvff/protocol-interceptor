use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex, MutexGuard};

use futures::Poll;
use tokio_io::AsyncWrite;
use tokio_io::io::WriteHalf;

pub struct SharedWriteHalf<T> {
    write_half: Arc<Mutex<WriteHalf<T>>>,
}

impl<T> SharedWriteHalf<T> {
    fn lock_write_half(&mut self) -> io::Result<MutexGuard<WriteHalf<T>>> {
        self.write_half.lock().map_err(|_| {
            let message = concat!(
                "a thread panicked while holding a lock to the shared write ",
                "half",
            );

            io::Error::new(io::ErrorKind::Other, message.to_string())
        })
    }
}

impl<T> From<WriteHalf<T>> for SharedWriteHalf<T> {
    fn from(write_half: WriteHalf<T>) -> Self {
        SharedWriteHalf {
            write_half: Arc::new(Mutex::new(write_half)),
        }
    }
}

impl<T> Write for SharedWriteHalf<T>
where
    T: AsyncWrite,
{
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.lock_write_half()?.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.lock_write_half()?.flush()
    }
}

impl<T> AsyncWrite for SharedWriteHalf<T>
where
    T: AsyncWrite,
{
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.lock_write_half()?.shutdown()
    }
}

impl<T> Clone for SharedWriteHalf<T> {
    fn clone(&self) -> Self {
        SharedWriteHalf {
            write_half: self.write_half.clone(),
        }
    }
}
