use bach::time::Instant;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Closed;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Behavior {
    #[default]
    Unbounded,
    Backpressure(usize),
    Wrap(usize),
    Reject(usize),
}

impl Behavior {
    pub fn capacity(&self) -> Option<usize> {
        match self {
            Behavior::Unbounded => None,
            Behavior::Backpressure(cap) => Some(*cap),
            Behavior::Wrap(cap) => Some(*cap),
            Behavior::Reject(cap) => Some(*cap),
        }
    }
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
    pub fn new<N: core::fmt::Display>(behavior: Behavior, name: N) -> Self {
        let queue = VecDeque::with_capacity(behavior.capacity().unwrap_or(64));
        let inner = Inner {
            open: true,
            behavior,
            queue,
            name: name.to_string(),
        };
        let inner = Arc::new(Mutex::new(inner));
        RingDeque { inner }
    }

    #[inline]
    pub fn push_back(&self, value: T) -> Result<Option<T>, Closed> {
        let mut inner = self.lock()?;

        let prev = if !inner.has_capacity() {
            if matches!(inner.behavior, Behavior::Reject(_)) {
                count!(
                    "push",
                    1,
                    "",
                    queue_name = inner.name,
                    reason = "reject",
                    actor = actor_name(),
                );
                count!(
                    "push_back",
                    1,
                    "",
                    queue_name = inner.name,
                    reason = "reject",
                    actor = actor_name(),
                );
                return Ok(Some(value));
            }
            inner.pop_front("wrap")
        } else {
            None
        };

        inner.push_back(value);

        if prev.is_none() {
            inner.measure_len();
        }

        Ok(prev)
    }

    #[inline]
    pub fn push_front(&self, value: T) -> Result<Option<T>, Closed> {
        let mut inner = self.lock()?;

        let prev = if !inner.has_capacity() {
            if matches!(inner.behavior, Behavior::Reject(_)) {
                count!(
                    "push",
                    1,
                    "",
                    queue_name = inner.name,
                    reason = "reject",
                    actor = actor_name(),
                );
                count!(
                    "push_front",
                    1,
                    "",
                    queue_name = inner.name,
                    reason = "reject",
                    actor = actor_name(),
                );
                return Ok(Some(value));
            }
            inner.pop_back("wrap")
        } else {
            None
        };

        inner.push_front(value);

        if prev.is_none() {
            inner.measure_len();
        }

        Ok(prev)
    }

    #[inline]
    pub fn pop_back(&self) -> Result<Option<T>, Closed> {
        let mut inner = self.lock()?;
        Ok(inner.pop_back("explicit"))
    }

    #[inline]
    pub fn pop_back_if<F>(&self, reason: &str, check: F) -> Result<Option<T>, Closed>
    where
        F: FnOnce(&T) -> bool,
    {
        let mut inner = self.lock()?;

        let Some(back) = inner.queue.back() else {
            return Ok(None);
        };

        if !check(&back.0) {
            return Ok(None);
        }

        Ok(inner.pop_back(reason))
    }

    #[inline]
    pub fn pop_front(&self) -> Result<Option<T>, Closed> {
        let mut inner = self.lock()?;
        Ok(inner.pop_front("explicit"))
    }

    #[inline]
    pub fn pop_front_if<F>(&self, reason: &str, check: F) -> Result<Option<T>, Closed>
    where
        F: FnOnce(&T) -> bool,
    {
        let mut inner = self.lock()?;

        let Some(back) = inner.queue.front() else {
            return Ok(None);
        };

        if !check(&back.0) {
            return Ok(None);
        }

        Ok(inner.pop_front(reason))
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
}

struct Inner<T> {
    open: bool,
    behavior: Behavior,
    queue: VecDeque<(T, Instant)>,
    name: String,
}

impl<T> Inner<T> {
    fn has_capacity(&self) -> bool {
        if let Some(max_cap) = self.behavior.capacity() {
            max_cap > self.queue.len()
        } else {
            true
        }
    }

    fn measure_len(&self) {
        measure!("queue_len", self.queue.len(), "", queue_name = self.name);
    }

    fn push_front(&mut self, value: T) {
        count!("push", 1, "", queue_name = self.name, actor = actor_name(),);
        count!(
            "push_front",
            1,
            "",
            queue_name = self.name,
            actor = actor_name(),
        );

        self.queue.push_front((value, Instant::now()));
    }

    fn push_back(&mut self, value: T) {
        count!("push", 1, "", queue_name = self.name, actor = actor_name(),);
        count!(
            "push_back",
            1,
            "",
            queue_name = self.name,
            actor = actor_name(),
        );

        self.queue.push_back((value, Instant::now()));
    }

    fn pop_front(&mut self, reason: &str) -> Option<T> {
        let item = self.queue.pop_front().map(|(value, time)| {
            measure!(
                "sojourn_time",
                time.elapsed().as_nanos(),
                "ns",
                reason = reason,
                queue_name = self.name,
                actor = actor_name(),
            );
            value
        });

        if item.is_some() {
            self.measure_len();
            count!(
                "pop",
                1,
                "",
                reason = reason,
                queue_name = self.name,
                actor = actor_name(),
            );
            count!(
                "pop_front",
                1,
                "",
                reason = reason,
                queue_name = self.name,
                actor = actor_name(),
            );
        }

        item
    }

    fn pop_back(&mut self, reason: &str) -> Option<T> {
        let item = self.queue.pop_back().map(|(value, time)| {
            measure!(
                "sojourn_time",
                time.elapsed().as_nanos(),
                "ns",
                reason = reason,
                queue_name = self.name,
                actor = actor_name(),
            );
            value
        });

        if item.is_some() {
            self.measure_len();
            count!(
                "pop",
                1,
                "",
                reason = reason,
                queue_name = self.name,
                actor = actor_name(),
            );
            count!(
                "pop_back",
                1,
                "",
                reason = reason,
                queue_name = self.name,
                actor = actor_name(),
            );
        }

        item
    }
}

fn actor_name() -> String {
    bach::group::current().name()
}
