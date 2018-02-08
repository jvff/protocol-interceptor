use futures::{Async, Poll, Stream};
use futures::sync::BiLock;

use super::channel_factory::{ChannelFactory, ReadChannel};

pub struct ReadChannels {
    channel_factory: BiLock<ChannelFactory>,
}

impl ReadChannels {
    pub fn new(channel_factory: BiLock<ChannelFactory>) -> Self {
        ReadChannels {
            channel_factory,
        }
    }
}

impl Stream for ReadChannels {
    type Item = ReadChannel;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let lock_result = Ok(self.channel_factory.poll_lock());
        let mut channel_factory = try_ready!(lock_result);
        let read_channel = channel_factory.read_channel();

        Ok(Async::Ready(Some(read_channel)))
    }
}
