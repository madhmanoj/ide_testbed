use core::mem;
use core::pin::Pin;
use std::marker::PhantomData;
use futures_core::future::{FusedFuture, Future};
use futures_core::ready;
use futures_core::stream::{FusedStream, Stream};
use futures_core::task::{Context, Poll};
use pin_project_lite::pin_project;
use either::Either;

pin_project! {
    /// Future for the [`partition_map`](super::StreamTools::partition_map) method.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct PartitionMap<St, A, B, F, L, R> {
        #[pin]
        stream: St,
        left_collection: A,
        right_collection: B,
        predicate: F,
        left_type: PhantomData<L>,
        right_type: PhantomData<R>
    }
}

impl<St: Stream, A: Default, B: Default, F, L, R> PartitionMap<St, A, B, F, L, R> {
    fn finish(self: Pin<&mut Self>) -> (A, B) {
        let projected = self.project();
        (mem::take(projected.left_collection), mem::take(projected.right_collection))
    }

    pub(super) fn new(stream: St, predicate: F) -> Self {
        Self {
            stream,
            predicate,
            left_collection: Default::default(),
            right_collection:  Default::default(),
            left_type: Default::default(),
            right_type: Default::default()
        }
    }
}

impl<St, A, B, F, L, R> FusedFuture for PartitionMap<St, A, B, F, L, R>
where
    St: FusedStream,
    F: FnMut(St::Item) -> Either<L, R>,
    A: Default + Extend<L>,
    B: Default + Extend<R>,
{
    fn is_terminated(&self) -> bool {
        self.stream.is_terminated()
    }
}

impl<St, A, B, F, L, R> Future for PartitionMap<St, A, B, F, L, R>
where
    St: Stream,
    F: FnMut(St::Item) -> Either<L, R>,
    A: Default + Extend<L>,
    B: Default + Extend<R>,
{
    type Output = (A, B);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<(A, B)> {
        let mut this = self.as_mut().project();
        loop {
            match ready!(this.stream.as_mut().poll_next(cx)) {
                Some(e) => match (this.predicate)(e) {
                    Either::Left(e) => this.left_collection.extend(Some(e)),
                    Either::Right(e) => this.right_collection.extend(Some(e)),
                }
                None => return Poll::Ready(self.finish()),
            }
        }
    }
}