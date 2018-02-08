use futures::{Async, Poll, Stream};

use super::channel_factory::new_channel;
use super::channel_factory::ReadChannel;
use super::io::IoQueue;
use super::shared_write_half::SharedWriteHalf;

type SharedWriteChannel = (SharedWriteHalf<IoQueue>, SharedWriteHalf<IoQueue>);

pub struct SharedChannelFactory {
    requests: SharedWriteHalf<IoQueue>,
    responses: SharedWriteHalf<IoQueue>,
}

impl SharedChannelFactory {
    pub fn new() -> (Self, ReadChannel) {
        let (read_channel, write_channel) = new_channel();
        let (requests, responses) = write_channel;

        let channel_factory = SharedChannelFactory {
            requests: requests.into(),
            responses: responses.into(),
        };

        (channel_factory, read_channel)
    }
}

impl Stream for SharedChannelFactory {
    type Item = SharedWriteChannel;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let requests = self.requests.clone();
        let responses = self.responses.clone();
        let write_channel = (requests, responses);

        Ok(Async::Ready(Some(write_channel)))
    }
}
