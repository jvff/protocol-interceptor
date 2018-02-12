extern crate bytes;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;

pub mod io;

mod async_queue;
mod bind_intercepted_transport;
mod error;
mod intercept_io;
mod intercepted_transport;
mod next_item;
mod possibly_intercepted_io;
mod protocol_interceptor;

mod channel_factory;
mod read_channels;
mod write_channels;

mod shared_channel_factory;
mod shared_write_half;

pub use async_queue::AsyncQueue;
pub use channel_factory::ChannelFactory;
pub use error::InterceptError;
pub use next_item::NextItem;
pub use protocol_interceptor::ProtocolInterceptor;
pub use shared_channel_factory::SharedChannelFactory;
