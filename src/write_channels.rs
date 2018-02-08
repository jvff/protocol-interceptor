use futures::{Async, Poll, Stream};
use futures::sync::BiLock;

use super::channel_factory::{ChannelFactory, WriteChannel};

pub struct WriteChannels {
    channel_factory: BiLock<ChannelFactory>,
}

impl WriteChannels {
    pub fn new(channel_factory: BiLock<ChannelFactory>) -> Self {
        WriteChannels {
            channel_factory,
        }
    }
}

impl Stream for WriteChannels {
    type Item = WriteChannel;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let lock_result = Ok(self.channel_factory.poll_lock());
        let mut channel_factory = try_ready!(lock_result);
        let write_channel = channel_factory.write_channel();

        Ok(Async::Ready(Some(write_channel)))
    }
}
