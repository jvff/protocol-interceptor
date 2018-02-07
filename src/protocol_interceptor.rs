use std::io;
use std::io::Write;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use futures::{Future, Stream};
use futures::future::Flatten;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::ServerProto;

use super::intercept_io::InterceptIo;
use super::possibly_intercepted_io::PossiblyInterceptedIo;

pub struct ProtocolInterceptor<C, P, E> {
    channel_factory: Arc<Mutex<C>>,
    protocol: Arc<P>,
    _error: PhantomData<E>,
}

impl<C, P, E> ProtocolInterceptor<C, P, E> {
    pub fn new(channel_factory: C, protocol: P) -> Self {
        ProtocolInterceptor {
            channel_factory: Arc::new(Mutex::new(channel_factory)),
            protocol: Arc::new(protocol),
            _error: PhantomData,
        }
    }
}

impl<C, P, T, I, O, E> ServerProto<T> for ProtocolInterceptor<C, P, E>
where
    C: 'static + Stream<Item = (I, O)>,
    P: ServerProto<PossiblyInterceptedIo<T, I, O>>,
    P::BindTransport: Future,
    T: 'static + AsyncRead + AsyncWrite,
    I: 'static + Write,
    O: 'static + Write,
    E: 'static + From<C::Error> + From<<P::BindTransport as Future>::Error>,
    io::Error: From<E>,
{
    type Request = P::Request;
    type Response = P::Response;
    type Transport = P::Transport;
    type BindTransport = Flatten<InterceptIo<C, P, T, E>>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let channel_factory = self.channel_factory.clone();
        let protocol = self.protocol.clone();

        InterceptIo::new(channel_factory, protocol, io).flatten()
    }
}
