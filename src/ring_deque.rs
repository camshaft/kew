use bach::time::Instant;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Closed;

#[derive(Clone, Copy, Debug, Default)]
pub enum Priority {
    #[default]
    Required,
    Optional,
}

pub struct RingDeque<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

impl<T> Clone for RingDeque<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> RingDeque<T> {
    #[inline]
    pub fn new<N: core::fmt::Display>(capacity: Option<usize>, name: N) -> Self {
        let queue = VecDeque::with_capacity(capacity.unwrap_or(64));
        let inner = Inner {
            open: true,
            has_cap: capacity.is_some(),
            queue,
            name: name.to_string(),
        };
        let inner = Arc::new(Mutex::new(inner));
        RingDeque { inner }
    }

    #[inline]
    pub fn push_back(&self, value: T) -> Result<Option<T>, Closed> {
        let mut inner = self.lock()?;

        let prev = if inner.queue.capacity() == inner.queue.len() {
            inner.queue.pop_front().map(|(value, time)| {
                measure!("sojourn_time", time.elapsed().as_nanos(), "ns");
                value
            })
        } else {
            None
        };

        inner.queue.push_back((value, Instant::now()));

        Ok(prev)
    }

    #[inline]
    pub fn push_front(&self, value: T) -> Result<Option<T>, Closed> {
        let mut inner = self.lock()?;

        let prev = if inner.queue.capacity() == inner.queue.len() {
            inner.queue.pop_back().map(|(value, time)| {
                measure!("sojourn_time", time.elapsed().as_nanos(), "ns");
                value
            })
        } else {
            None
        };

        inner.queue.push_front((value, Instant::now()));

        Ok(prev)
    }

    #[inline]
    pub fn pop_back(&self) -> Result<Option<T>, Closed> {
        let mut inner = self.lock()?;
        Ok(inner.queue.pop_back().map(|(value, time)| {
            measure!("sojourn_time", time.elapsed().as_nanos(), "ns");
            value
        }))
    }

    #[inline]
    pub fn pop_back_if<F>(&self, priority: Priority, check: F) -> Result<Option<T>, Closed>
    where
        F: FnOnce(&T) -> bool,
    {
        let inner = match priority {
            Priority::Required => Some(self.lock()?),
            Priority::Optional => self.try_lock()?,
        };

        let Some(mut inner) = inner else {
            return Ok(None);
        };

        let Some(back) = inner.queue.back() else {
            return Ok(None);
        };

        if check(&back.0) {
            Ok(inner.queue.pop_back().map(|(value, time)| {
                measure!("sojourn_time", time.elapsed().as_nanos(), "ns");
                value
            }))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn pop_front(&self) -> Result<Option<T>, Closed> {
        let mut inner = self.lock()?;
        Ok(inner.queue.pop_front().map(|(value, time)| {
            measure!("sojourn_time", time.elapsed().as_nanos(), "ns");
            value
        }))
    }

    #[inline]
    pub fn pop_front_if<F>(&self, priority: Priority, check: F) -> Result<Option<T>, Closed>
    where
        F: FnOnce(&T) -> bool,
    {
        let inner = match priority {
            Priority::Required => Some(self.lock()?),
            Priority::Optional => self.try_lock()?,
        };

        let Some(mut inner) = inner else {
            return Ok(None);
        };

        let Some(back) = inner.queue.front() else {
            return Ok(None);
        };

        if check(&back.0) {
            Ok(inner.queue.pop_front().map(|(value, time)| {
                measure!("sojourn_time", time.elapsed().as_nanos(), "ns");
                value
            }))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn close(&self) -> Result<(), Closed> {
        let mut inner = self.lock()?;
        inner.open = false;
        Ok(())
    }

    #[inline]
    fn lock(&self) -> Result<std::sync::MutexGuard<Inner<T>>, Closed> {
        let inner = self.inner.lock().unwrap();
        ensure!(inner.open, Err(Closed));
        Ok(inner)
    }

    #[inline]
    fn try_lock(&self) -> Result<Option<std::sync::MutexGuard<Inner<T>>>, Closed> {
        use std::sync::TryLockError;
        let inner = match self.inner.try_lock() {
            Ok(inner) => inner,
            Err(TryLockError::WouldBlock) => return Ok(None),
            Err(TryLockError::Poisoned(_)) => return Err(Closed),
        };
        ensure!(inner.open, Err(Closed));
        Ok(Some(inner))
    }
}

struct Inner<T> {
    open: bool,
    has_cap: bool,
    queue: VecDeque<(T, Instant)>,
    name: String,
}
