use std::marker::PhantomData;

use futures::{Poll, Sink, StartSend, Stream};

use super::error::InterceptError;

pub struct InterceptedTransport<C, B, T> {
    transport: T,
    _bind_error: PhantomData<B>,
    _channel_error: PhantomData<C>,
}

impl<C, B, T> From<T> for InterceptedTransport<C, B, T> {
    fn from(transport: T) -> Self {
        InterceptedTransport {
            transport,
            _bind_error: PhantomData,
            _channel_error: PhantomData,
        }
    }
}

impl<C, B, T> Stream for InterceptedTransport<C, B, T>
where
    T: Stream,
{
    type Item = T::Item;
    type Error = InterceptError<C, B, T::Error>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.transport.poll().map_err(InterceptError::ProtocolReceiveError)
    }
}

impl<C, B, T> Sink for InterceptedTransport<C, B, T>
where
    T: Sink,
{
    type SinkItem = T::SinkItem;
    type SinkError = InterceptError<C, B, T::SinkError>;

    fn start_send(
        &mut self,
        item: Self::SinkItem,
    ) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.transport
            .start_send(item)
            .map_err(InterceptError::ProtocolSendError)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.transport
            .poll_complete()
            .map_err(InterceptError::ProtocolSendError)
    }
}
