use super::{scope, Id, Pop, PushBack, PushFront};
use core::fmt;

#[derive(Debug)]
pub struct Item {
    id: Id,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Default for Item {
    fn default() -> Self {
        let id = scope::borrow_mut_with(|scope| {
            let id = scope.next_item;
            scope.next_item += 1;
            let queue_id = scope.queue_id_for_current_group();
            let event = PushBack::new(None, queue_id, id);
            scope.current_step().on_push_back(event);
            id
        });
        Self { id }
    }
}

impl Item {
    pub fn on_pop(&self, queue_id: Id) {
        scope::borrow_mut_with(|scope| {
            let group_id = scope.queue_id_for_current_group();
            let event = PushBack::new(Some(queue_id), group_id, self.id);
            scope.current_step().on_push_back(event);
        });
    }

    pub fn on_push_back(&self, queue_id: Id) {
        scope::borrow_mut_with(|scope| {
            let group_id = scope.queue_id_for_current_group();
            let event = PushBack::new(Some(group_id), queue_id, self.id);
            scope.current_step().on_push_back(event);
        });
    }

    pub fn on_push_front(&self, queue_id: Id) {
        scope::borrow_mut_with(|scope| {
            let group_id = scope.queue_id_for_current_group();
            let event = PushFront::new(Some(group_id), queue_id, self.id);
            scope.current_step().on_push_front(event);
        });
    }
}

impl Drop for Item {
    fn drop(&mut self) {
        scope::borrow_mut_with(|scope| {
            let queue_id = scope.queue_id_for_current_group();
            let event = Pop::new(queue_id, self.id);
            scope.current_step().on_pop(event);
        });
    }
}
