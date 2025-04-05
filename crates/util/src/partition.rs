use core::mem;
use core::pin::Pin;
use futures_core::future::{FusedFuture, Future};
use futures_core::ready;
use futures_core::stream::{FusedStream, Stream};
use futures_core::task::{Context, Poll};
use pin_project_lite::pin_project;

pin_project! {
    /// Future for the [`collect`](super::StreamExt::collect) method.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Partition<St, C, F> {
        #[pin]
        stream: St,
        left_collection: C,
        right_collection: C,
        f: F
    }
}

impl<St: Stream, C: Default, F> Partition<St, C, F> {
    fn finish(self: Pin<&mut Self>) -> (C, C) {
        let projected = self.project();
        (mem::take(projected.left_collection), mem::take(projected.right_collection))
    }

    pub(super) fn new(stream: St, f: F) -> Self {
        Self {
            stream,
            f,
            left_collection: Default::default(),
            right_collection:  Default::default(),
        }
    }
}

impl<St, C, F> FusedFuture for Partition<St, C, F>
where
    St: FusedStream,
    C: Default + Extend<St::Item>,
    F: FnMut(&St::Item) -> bool,
{
    fn is_terminated(&self) -> bool {
        self.stream.is_terminated()
    }
}

impl<St, C, F> Future for Partition<St, C, F>
where
    St: Stream,
    F: FnMut(&St::Item) -> bool,
    C: Default + Extend<St::Item>,
{
    type Output = (C, C);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<(C, C)> {
        let mut this = self.as_mut().project();
        loop {
            match ready!(this.stream.as_mut().poll_next(cx)) {
                Some(e) if (this.f)(&e) => this.left_collection.extend(Some(e)),
                Some(e) => this.right_collection.extend(Some(e)),
                None => return Poll::Ready(self.finish()),
            }
        }
    }
}