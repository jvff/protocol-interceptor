use std::io;
use std::io::{Read, Write};

use bytes::BytesMut;
use futures::Poll;
use tokio_io::{AsyncRead, AsyncWrite};

pub struct IoInterceptor<T, S> {
    intercepted: T,
    interceptor: S,
    queue_buffer: BytesMut,
}

impl<T, S> IoInterceptor<T, S>
where
    S: Write,
{
    pub fn new(intercepted: T, interceptor: S) -> Self {
        Self {
            intercepted,
            interceptor,
            queue_buffer: BytesMut::new(),
        }
    }

    fn forward_data(&mut self, data: &[u8]) -> io::Result<usize> {
        let length = data.len();

        let (bytes_written, error) = match self.interceptor.write(data) {
            Ok(bytes_written) => (bytes_written, None),
            Err(error) => (0, Some(error)),
        };

        if bytes_written != data.len() {
            self.queue_buffer.extend(&data[bytes_written..length]);
        }

        match error {
            Some(error) => Err(error),
            None => Ok(length),
        }
    }

    fn write_queued_data(&mut self) -> io::Result<()> {
        while self.queue_buffer.len() > 0 {
            let bytes_written = self.interceptor.write(&self.queue_buffer)?;

            self.queue_buffer.split_to(bytes_written);
        }

        self.queue_buffer.truncate(0);

        Ok(())
    }
}

impl<T, S> Read for IoInterceptor<T, S>
where
    T: Read,
    S: Write,
{
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.write_queued_data()?;

        let bytes_read = self.intercepted.read(buffer)?;

        self.forward_data(&buffer[0..bytes_read])
    }
}

impl<T, S> AsyncRead for IoInterceptor<T, S>
where
    T: AsyncRead,
    S: Write,
{}

impl<T, S> Write for IoInterceptor<T, S>
where
    T: Write,
    S: Write,
{
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.write_queued_data()?;

        let bytes_written = self.intercepted.write(buffer)?;

        self.forward_data(&buffer[0..bytes_written])
    }

    fn flush(&mut self) -> io::Result<()> {
        self.write_queued_data()
    }
}

impl<T, S> AsyncWrite for IoInterceptor<T, S>
where
    T: AsyncWrite,
    S: Write,
{
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.intercepted.shutdown()
    }
}
