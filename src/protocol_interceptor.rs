use std::io::Write;
use std::sync::{Arc, Mutex};

use futures::{Future, IntoFuture, Stream};
use futures::future::Flatten;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::ServerProto;

use super::error::InterceptError;
use super::intercept_io::InterceptIo;
use super::intercepted_transport::InterceptedTransport;
use super::possibly_intercepted_io::PossiblyInterceptedIo;

pub struct ProtocolInterceptor<C, P> {
    channel_factory: Arc<Mutex<C>>,
    protocol: Arc<P>,
}

impl<C, P> ProtocolInterceptor<C, P> {
    pub fn new(channel_factory: C, protocol: P) -> Self {
        ProtocolInterceptor {
            channel_factory: Arc::new(Mutex::new(channel_factory)),
            protocol: Arc::new(protocol),
        }
    }
}

impl<C, P, T, I, O> ServerProto<T> for ProtocolInterceptor<C, P>
where
    C: 'static + Stream<Item = (I, O)>,
    P: ServerProto<PossiblyInterceptedIo<T, I, O>>,
    T: 'static + AsyncRead + AsyncWrite,
    I: 'static + Write,
    O: 'static + Write,
{
    type Request = P::Request;
    type Response = P::Response;
    type Error =
        InterceptError<
            C::Error,
            <P::BindTransport as IntoFuture>::Error,
            P::Error,
        >;
    type Transport = InterceptedTransport<
        C::Error,
        <P::BindTransport as IntoFuture>::Error,
        <P::BindTransport as IntoFuture>::Item,
    >;
    type BindTransport = Flatten<InterceptIo<C, P, T>>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let channel_factory = self.channel_factory.clone();
        let protocol = self.protocol.clone();

        InterceptIo::new(channel_factory, protocol, io).flatten()
    }
}
