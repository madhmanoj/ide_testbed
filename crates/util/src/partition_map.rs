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
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct PartitionMap<St, A, B, F, L, R> {
        #[pin]
        stream: St,
        left_iterator: Option<Box<dyn Iterator<Item = L>>>,
        right_iterator: Option<Box<dyn Iterator<Item = R>>>,
        predicate: F,
        left_collection_type: PhantomData<A>,
        right_collection_type: PhantomData<B>
    }
}

impl<St: Stream, A: FromIterator<L>, B: FromIterator<R>, F, L: 'static, R: 'static> PartitionMap<St, A, B, F, L, R> {
    fn finish(self: Pin<&mut Self>) -> (A, B) {
        let projected = self.project();
        
        let left_collection =
            A::from_iter(mem::take(projected.left_iterator).into_iter().flatten());
        let right_collection =
            B::from_iter(mem::take(projected.right_iterator).into_iter().flatten());

        (left_collection, right_collection)
    }

    pub(super) fn new(stream: St, predicate: F) -> Self {
        Self {
            stream,
            predicate,
            left_iterator: None,
            right_iterator:  None,
            left_collection_type: Default::default(),
            right_collection_type: Default::default()
        }
    }
}

impl<St, A, B, F, L, R> FusedFuture for PartitionMap<St, A, B, F, L, R>
where
    St: FusedStream,
    F: FnMut(St::Item) -> Either<L, R>,
    A: FromIterator<L>,
    B: FromIterator<R>,
    L: 'static,
    R: 'static
{
    fn is_terminated(&self) -> bool {
        self.stream.is_terminated()
    }
}

impl<St, A, B, F, L, R> Future for PartitionMap<St, A, B, F, L, R>
where
    St: Stream,
    F: FnMut(St::Item) -> Either<L, R>,
    A: FromIterator<L>,
    B: FromIterator<R>,
    L: 'static,
    R: 'static
{
    type Output = (A, B);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<(A, B)> {
        let mut this = self.as_mut().project();
        loop {
            match ready!(this.stream.as_mut().poll_next(cx)) {
                Some(e) => match (this.predicate)(e) {
                    Either::Left(e) => {
                        *this.left_iterator = match mem::take(this.left_iterator) {
                            Some(iterator) => Some(Box::new(iterator.chain(Some(e)))),
                            _ => Some(Box::new(std::iter::once(e)))
                        };
                    },
                    Either::Right(e) => {
                        *this.right_iterator = match mem::take(this.right_iterator) {
                            Some(iterator) => Some(Box::new(iterator.chain(Some(e)))),
                            _ => Some(Box::new(std::iter::once(e)))
                        };
                    }
                }
                None => return Poll::Ready(self.finish()),
            }
        }
    }
}