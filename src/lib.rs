use std::sync::Arc;

use arc_swap::ArcSwap;

#[derive(Debug)]
pub struct Swap<T> {
    inner: Arc<ArcSwap<T>>,
}
impl<T> Swap<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(Arc::new(inner).into()),
        }
    }

    pub fn replaced_by(&self, new: T) {
        self.inner.store(Arc::new(new));
    }

    pub fn inner(&self) -> arc_swap::Guard<Arc<T>> {
        self.inner.load()
    }
}
impl<T> Clone for Swap<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
