use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll, Stream};

pub struct NextItem<S> {
    stream: Option<Arc<Mutex<S>>>,
}

impl<S> NextItem<S> {
    pub fn new(stream: Arc<Mutex<S>>) -> Self {
        NextItem {
            stream: Some(stream),
        }
    }
}

impl<S> Future for NextItem<S>
where
    S: Stream,
{
    type Item = Option<S::Item>;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Some(stream) = self.stream.take() {
            match lock_and_poll_stream(&stream)? {
                Async::Ready(item) => Ok(Async::Ready(item)),
                Async::NotReady => {
                    self.stream = Some(stream);

                    Ok(Async::NotReady)
                }
            }
        } else {
            panic!(
                "attempt to retrieve two stream items through a single future"
            );
        }
    }
}

fn lock_and_poll_stream<S>(
    stream: &Arc<Mutex<S>>,
) -> Poll<Option<S::Item>, S::Error>
where
    S: Stream,
{
    let mut locked_stream = stream.lock()
        .expect(
            concat!(
                "failed to fetch new element from stream because an other ",
                "thread panicked while using it",
            )
        );

    locked_stream.poll()
}
