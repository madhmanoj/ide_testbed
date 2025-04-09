use std::future::Future;

use futures::Stream;

mod partition;
mod partition_map;

pub trait StreamTools: Stream {
    fn partition<C, F>(self, f: F) -> partition::Partition<Self, C, F>
    where
        Self: Sized,
        C: Default + Extend<Self::Item>,
        F: FnMut(&Self::Item) -> bool {
        assert_future::<(C, C), _>(partition::Partition::new(self, f))
    }

    fn partition_map<A, B, F, L, R>(self, predicate: F) -> partition_map::PartitionMap<Self, A, B, F, L, R>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> either::Either<L, R>,
        A: Default + Extend<L>,
        B: Default + Extend<R> {
        assert_future::<(A, B), _>(partition_map::PartitionMap::new(self, predicate))
    }
}

impl<T: Stream> StreamTools for T {}

pub(crate) fn assert_future<T, F>(future: F) -> F
where
    F: Future<Output = T>,
{
    future
}