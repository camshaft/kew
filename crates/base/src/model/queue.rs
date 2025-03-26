use super::{item::Item, scope, Id};
use bach::queue::{CloseError, PopError, PushError, Pushable, Queue as Q};
use std::task::Context;

struct PushableItem<'a> {
    queue: Id,
    item: &'a mut dyn Pushable<Item>,
}

impl Pushable<Item> for PushableItem<'_> {
    fn produce(&mut self) -> Item {
        let item = self.item.produce();
        // TODO how do we know if it's push back/front?
        item.on_push(self.queue);
        item
    }
}

pub struct Queue<T: Q<Item>> {
    id: Id,
    inner: T,
}

impl<T: Q<Item>> Q<Item> for Queue<T> {
    fn push_lazy(&mut self, value: &mut dyn Pushable<Item>) -> Result<Option<Item>, PushError> {
        let mut item = PushableItem {
            queue: self.id,
            item: value,
        };
        let prev = self.inner.push_lazy(&mut item)?;

        if let Some(item) = prev.as_ref() {
            item.on_pop(self.id);
        }

        Ok(prev)
    }

    fn push_with_notify(
        &mut self,
        value: &mut dyn Pushable<Item>,
        cx: &mut Context,
    ) -> Result<Option<Item>, PushError> {
        let mut item = PushableItem {
            queue: self.id,
            item: value,
        };
        let prev = self.inner.push_with_notify(&mut item, cx)?;

        if let Some(item) = prev.as_ref() {
            item.on_pop(self.id);
        }

        Ok(prev)
    }

    fn pop(&mut self) -> Result<Item, PopError> {
        let item = self.inner.pop()?;
        item.on_pop(self.id);
        Ok(item)
    }

    fn pop_with_notify(&mut self, cx: &mut Context) -> Result<Item, PopError> {
        let item = self.inner.pop_with_notify(cx)?;
        item.on_pop(self.id);
        Ok(item)
    }

    fn close(&mut self) -> Result<(), CloseError> {
        self.inner.close()
    }

    fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn is_full(&self) -> bool {
        self.inner.is_full()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn capacity(&self) -> Option<usize> {
        self.inner.capacity()
    }
}

pub trait Ext: Q<Item> + Sized {
    fn items<N: AsRef<str>>(self, name: N) -> Queue<Self>;
}

impl<T: Q<Item>> Ext for T {
    fn items<N: AsRef<str>>(self, name: N) -> Queue<Self> {
        let id = scope::borrow_mut_with(|scope| scope.create_queue(name.as_ref()));
        Queue { id, inner: self }
    }
}
