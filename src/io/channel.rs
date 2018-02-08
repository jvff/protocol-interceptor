use std::io;
use std::io::{Read, Write};

use futures::Poll;
use tokio_io::{AsyncRead, AsyncWrite};

pub struct IoChannel<I, O> {
    input: I,
    output: O,
}

impl<I, O> IoChannel<I, O> {
    pub fn new(input: I, output: O) -> Self {
        IoChannel {
            input,
            output,
        }
    }
}

impl<I, O> Read for IoChannel<I, O>
where
    I: Read,
{
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.input.read(buffer)
    }
}

impl<I, O> AsyncRead for IoChannel<I, O>
where
    I: AsyncRead,
{}

impl<I, O> Write for IoChannel<I, O>
where
    O: Write,
{
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.output.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

impl<I, O> AsyncWrite for IoChannel<I, O>
where
    O: AsyncWrite,
{
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.output.shutdown()
    }
}
