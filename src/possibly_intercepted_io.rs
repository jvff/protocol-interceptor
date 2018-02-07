use std::io;
use std::io::{Read, Write};

use futures::Poll;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::io::{ReadHalf, WriteHalf};

use super::io_channel::IoChannel;
use super::io_interceptor::IoInterceptor;

pub enum PossiblyInterceptedIo<T, I, O>
where
    I: Write,
    O: Write,
{
    NotIntercepted(T),
    Intercepted(
        IoChannel<
            IoInterceptor<ReadHalf<T>, I>,
            IoInterceptor<WriteHalf<T>, O>,
        >,
    )
}

impl<T, I, O> PossiblyInterceptedIo<T, I, O>
where
    T: AsyncRead + AsyncWrite,
    I: Write,
    O: Write,
{
    pub fn intercept(io: T, read_interceptor: I, write_interceptor: O) -> Self {
        let (read, write) = io.split();

        let input = IoInterceptor::new(read, read_interceptor);
        let output = IoInterceptor::new(write, write_interceptor);
        let intercepted_io = IoChannel::new(input, output);

        PossiblyInterceptedIo::Intercepted(intercepted_io)
    }

    pub fn dont_intercept(io: T) -> Self {
        PossiblyInterceptedIo::NotIntercepted(io)
    }
}

impl<T, I, O> Read for PossiblyInterceptedIo<T, I, O>
where
    T: AsyncRead,
    I: Write,
    O: Write,
{
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        use self::PossiblyInterceptedIo::*;

        match *self {
            Intercepted(ref mut io) => io.read(buffer),
            NotIntercepted(ref mut io) => io.read(buffer),
        }
    }
}

impl<T, I, O> AsyncRead for PossiblyInterceptedIo<T, I, O>
where
    T: AsyncRead,
    I: Write,
    O: Write,
{}

impl<T, I, O> Write for PossiblyInterceptedIo<T, I, O>
where
    T: AsyncWrite,
    I: Write,
    O: Write,
{
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        use self::PossiblyInterceptedIo::*;

        match *self {
            Intercepted(ref mut io) => io.write(buffer),
            NotIntercepted(ref mut io) => io.write(buffer),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        use self::PossiblyInterceptedIo::*;

        match *self {
            Intercepted(ref mut io) => io.flush(),
            NotIntercepted(ref mut io) => io.flush(),
        }
    }
}

impl<T, I, O> AsyncWrite for PossiblyInterceptedIo<T, I, O>
where
    T: AsyncWrite,
    I: Write,
    O: Write,
{
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        use self::PossiblyInterceptedIo::*;

        match *self {
            Intercepted(ref mut io) => io.shutdown(),
            NotIntercepted(ref mut io) => io.shutdown(),
        }
    }
}
