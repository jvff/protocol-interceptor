use std::collections::VecDeque;

use futures::Async;
use futures::task;
use futures::task::Task;
use futures::sync::BiLock;
use tokio_io::AsyncRead;
use tokio_io::io::{ReadHalf, WriteHalf};

use super::io::IoQueue;
use super::read_channels::ReadChannels;
use super::write_channels::WriteChannels;

pub(crate) type ReadChannel = (ReadHalf<IoQueue>, ReadHalf<IoQueue>);
pub(crate) type WriteChannel = (WriteHalf<IoQueue>, WriteHalf<IoQueue>);

pub struct ChannelFactory {
    read_task: Option<Task>,
    read_channels: VecDeque<ReadChannel>,
    write_channels: VecDeque<WriteChannel>,
}

impl ChannelFactory {
    pub fn new() -> Self {
        ChannelFactory {
            read_task: None,
            read_channels: VecDeque::new(),
            write_channels: VecDeque::new(),
        }
    }

    pub fn split(self) -> (ReadChannels, WriteChannels) {
        let (read_lock, write_lock) = BiLock::new(self);

        (ReadChannels::new(read_lock), WriteChannels::new(write_lock))
    }

    pub(crate) fn read_channel(&mut self) -> Async<ReadChannel> {
        if let Some(read_channel) = self.read_channels.pop_front() {
            Async::Ready(read_channel)
        } else {
            self.park_read_task();

            Async::NotReady
        }
    }


    pub(crate) fn write_channel(&mut self) -> WriteChannel {
        self.notify_read_task();

        self.write_channels.pop_front().unwrap_or_else(|| {
            let (read_channel, write_channel) = new_channel();

            self.read_channels.push_back(read_channel);

            write_channel
        })
    }

    fn notify_read_task(&mut self) {
        if let Some(task) = self.read_task.take() {
            task.notify();
        }
    }

    fn park_read_task(&mut self) {
        if self.read_task.is_some() {
            panic!(
                concat!(
                    "two WriteChannels are attempting to use the same ",
                    "ChannelFactory",
                )
            )
        }

        self.read_task = Some(task::current());
    }
}

pub(crate) fn new_channel() -> (ReadChannel, WriteChannel) {
    let request_queue = IoQueue::new();
    let response_queue = IoQueue::new();

    let (read_request, write_request) = request_queue.split();
    let (read_response, write_response) = response_queue.split();

    let read_channel = (read_request, read_response);
    let write_channel = (write_request, write_response);

    (read_channel, write_channel)
}
