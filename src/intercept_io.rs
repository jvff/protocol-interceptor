use std::io::Write;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll, Stream};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::ServerProto;

use super::next_item::NextItem;
use super::possibly_intercepted_io::PossiblyInterceptedIo;

pub struct InterceptIo<C, P, T, E> {
    channel: NextItem<C>,
    protocol: Arc<P>,
    io: Option<T>,
    _error: PhantomData<E>,
}

impl<C, P, T, E> InterceptIo<C, P, T, E> {
    pub fn new(channel_factory: Arc<Mutex<C>>, protocol: Arc<P>, io: T) -> Self {
        InterceptIo {
            channel: NextItem::new(channel_factory),
            io: Some(io),
            protocol,
            _error: PhantomData,
        }
    }
}

impl<C, P, T, E, I, O> Future for InterceptIo<C, P, T, E>
where
    C: Stream<Item = (I, O)>,
    P: ServerProto<PossiblyInterceptedIo<T, I, O>>,
    P::BindTransport: Future,
    E: From<C::Error> + From<<P::BindTransport as Future>::Error>,
    T: 'static + AsyncRead + AsyncWrite,
    I: 'static + Write,
    O: 'static + Write,
{
    type Item = P::BindTransport;
    type Error = E;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let maybe_channel = try_ready!(self.channel.poll());
        let io = self.io.take().expect("NextItem can't be polled twice");

        let possibly_intercepted_io = if let Some(channel) = maybe_channel {
            let (read_interceptor, write_interceptor) = channel;

            PossiblyInterceptedIo::intercept(
                io,
                read_interceptor,
                write_interceptor,
            )
        } else {
            PossiblyInterceptedIo::dont_intercept(io)
        };

        Ok(Async::Ready(self.protocol.bind_transport(possibly_intercepted_io)))
    }
}
