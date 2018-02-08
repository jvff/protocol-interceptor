use std::collections::VecDeque;

use futures::{Async, AsyncSink, Poll, Sink, StartSend, Stream};

pub struct AsyncQueue<T> {
    queue: VecDeque<T>,
}

impl<T> AsyncQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
        }
    }
}

impl<T> Stream for AsyncQueue<T> {
    type Item = T;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(self.queue.pop_front()))
    }
}

impl<T> Sink for AsyncQueue<T> {
    type SinkItem = T;
    type SinkError = ();

    fn start_send(
        &mut self,
        item: Self::SinkItem,
    ) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.queue.push_back(item);

        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}
