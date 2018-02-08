use std::collections::VecDeque;
use std::io;
use std::io::{Read, Write};

use futures::{Async, Poll};
use tokio_io::{AsyncRead, AsyncWrite};

pub struct IoQueue {
    buffer: VecDeque<u8>,
}

impl IoQueue {
    pub fn new() -> Self {
        IoQueue {
            buffer: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        IoQueue {
            buffer: VecDeque::with_capacity(capacity),
        }
    }
}

impl Read for IoQueue {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut bytes = Vec::with_capacity(buffer.len());
        let count = buffer.len().min(self.buffer.len());

        bytes.append(&mut self.buffer.drain(0..count).collect());
        bytes.resize(buffer.len(), 0);

        buffer.copy_from_slice(&bytes);

        Ok(count)
    }
}

impl AsyncRead for IoQueue {}

impl Write for IoQueue {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.buffer.extend(bytes);

        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl AsyncWrite for IoQueue {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        Ok(Async::Ready(()))
    }
}
