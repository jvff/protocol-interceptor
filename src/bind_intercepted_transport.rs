use std::marker::PhantomData;

use futures::{Async, Future, Poll};

use super::error::InterceptError;
use super::intercepted_transport::InterceptedTransport;

pub struct BindInterceptedTransport<C, B, P> {
    bind_transport: B,
    _channel_error: PhantomData<C>,
    _protocol_error: PhantomData<P>,
}

impl<C, B, P> From<B> for BindInterceptedTransport<C, B, P> {
    fn from(bind_transport: B) -> Self {
        BindInterceptedTransport {
            bind_transport,
            _channel_error: PhantomData,
            _protocol_error: PhantomData,
        }
    }
}

impl<C, B, P> Future for BindInterceptedTransport<C, B, P>
where
    B: Future,
{
    type Item = InterceptedTransport<C, B::Error, B::Item>;
    type Error = InterceptError<C, B::Error, P>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.bind_transport.poll() {
            Ok(Async::Ready(transport)) => Ok(Async::Ready(transport.into())),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(error) => Err(InterceptError::BindTransportError(error)),
        }
    }
}
