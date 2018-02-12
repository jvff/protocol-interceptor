use std::io;

#[derive(Debug, Fail)]
pub enum InterceptError<C, B, P> {
    #[fail(display = "failed to bind protocol to transport")]
    BindTransportError(#[cause] B),

    #[fail(display = "failure to create channel")]
    CreateChannelError(#[cause] C),

    #[fail(display = "IO error")]
    IoError(#[cause] io::Error),

    #[fail(display = "receive error in intercepted protocol")]
    ProtocolReceiveError(#[cause] P),

    #[fail(display = "send error in intercepted protocol")]
    ProtocolSendError(#[cause] P),
}

impl<C, B, P> From<io::Error> for InterceptError<C, B, P> {
    fn from(io_error: io::Error) -> Self {
        InterceptError::IoError(io_error)
    }
}
