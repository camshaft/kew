use super::{Id, Pop, PushBack, PushFront, scope};
use bach::time::Instant;
use core::fmt;
use std::time::Duration;

#[derive(Debug)]
pub struct Item {
    id: Id,
    push_time: Instant,
    push_queue_id: i32,
    pop_time: Instant,
    already_dropped: bool,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Default for Item {
    fn default() -> Self {
        let (push_queue_id, id) = scope::borrow_mut_with(|scope| {
            let id = scope.next_item;
            scope.next_item += 1;
            let queue_id = scope.queue_id_for_current_group();
            let event = PushBack::new(None, queue_id, id);
            scope.current_step().on_push_back(event);
            (queue_id, id)
        });
        Self {
            id,
            push_time: Instant::now(),
            push_queue_id,
            pop_time: Instant::now(),
            already_dropped: false,
        }
    }
}

impl Item {
    pub fn sojourn_time(&self) -> Duration {
        self.pop_time.saturating_duration_since(self.push_time)
    }

    pub fn on_pop(&mut self, queue_id: Id) {
        self.pop_time = Instant::now();
        scope::borrow_mut_with(|scope| {
            let group_id = scope.queue_id_for_current_group();
            self.push_queue_id = queue_id;
            let event = PushBack::new(Some(queue_id), group_id, self.id);
            scope.current_step().on_push_back(event);
        });
    }

    pub fn on_push_back(&mut self, queue_id: Id) {
        self.push_time = Instant::now();
        scope::borrow_mut_with(|scope| {
            let group_id = scope.queue_id_for_current_group();
            let event = PushBack::new(Some(group_id), queue_id, self.id);
            scope.current_step().on_push_back(event);
        });
    }

    pub fn on_push_front(&mut self, queue_id: Id) {
        self.push_time = Instant::now();
        scope::borrow_mut_with(|scope| {
            let group_id = scope.queue_id_for_current_group();
            let event = PushFront::new(Some(group_id), queue_id, self.id);
            scope.current_step().on_push_front(event);
        });
    }

    pub fn error(mut self) {
        // self.already_dropped = true;
        // scope::borrow_mut_with(|scope| {
        //     let event = Pop::new(self.push_queue_id, self.id);
        //     scope.current_step().on_pop(event);
        // });
    }
}

impl Drop for Item {
    fn drop(&mut self) {
        if self.already_dropped {
            return;
        }

        scope::borrow_mut_with(|scope| {
            let queue_id = scope.queue_id_for_current_group();
            let event = Pop::new(queue_id, self.id);
            scope.current_step().on_pop(event);
        });
    }
}
