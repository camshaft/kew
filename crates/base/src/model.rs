use crate::macro_support::*;
use bach::group::Group as BachGroup;
use std::collections::HashMap;

::bach::scope::define!(scope, Scope);

pub fn register_group<N: AsRef<str>>(name: N) {
    scope::borrow_mut_with(|scope| scope.queue_id_for_group(BachGroup::new(name.as_ref())));
}

pub struct Scope {
    pub sim: Sim,
    pub current_step: Option<(Instant, Step)>,
    pub next_item: Id,
    pub next_queue: Id,
    pub group_to_queue: HashMap<BachGroup, Id>,
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            sim: Sim::new(),
            current_step: None,
            next_item: 0,
            next_queue: 0,
            group_to_queue: HashMap::new(),
        }
    }
}

impl Scope {
    pub fn create_queue(&mut self, name: &str) -> Id {
        let id = self.next_queue;
        self.next_queue += 1;

        let queue = Queue::new(name);
        self.sim.push_queue(queue);
        id
    }

    pub fn queue_id_for_current_group(&mut self) -> Id {
        self.queue_id_for_group(bach::group::current())
    }

    pub fn queue_id_for_group(&mut self, group: BachGroup) -> Id {
        *self.group_to_queue.entry(group).or_insert_with(|| {
            let id = self.next_queue;
            self.next_queue += 1;

            let queue = Group::new(&group.name());
            self.sim.push_group(queue);
            id
        })
    }

    pub fn current_step(&mut self) -> &mut Step {
        let now = Instant::now();

        if self.current_step.is_none() {
            // create a start step if we haven't already
            if !now.elapsed_since_start().is_zero() {
                let step = Step::new(0.0);
                self.sim.push_step(step);
            }

            let step = Step::new(now.elapsed_since_start().as_secs_f32());
            self.current_step = Some((now, step));
        }

        let (start, step) = self.current_step.as_mut().unwrap();

        if *start == now {
            return step;
        }

        *start = now;
        let prev = std::mem::replace(step, Step::new(now.elapsed_since_start().as_secs_f32()));
        self.sim.push_step(prev);
        step
    }

    pub fn finish(mut self) -> Sim {
        if let Some((_, step)) = self.current_step.take() {
            self.sim.push_step(step);
        }
        self.sim
    }
}

pub mod item;
pub mod queue;

#[wasm_bindgen(raw_module = "../../data/sim.ts")]
extern "C" {
    pub type Sim;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Sim;

    #[wasm_bindgen(method, final, js_name = "pushQueue")]
    pub fn push_queue(this: &Sim, queue: Queue) -> Id;

    #[wasm_bindgen(method, final, js_name = "pushQueue")]
    pub fn push_group(this: &Sim, group: Group) -> Id;

    #[wasm_bindgen(method, final, js_name = "pushStep")]
    pub fn push_step(this: &Sim, step: Step);
}

#[wasm_bindgen(raw_module = "../../data/sim.ts")]
extern "C" {
    pub type Queue;

    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Queue;
}

#[wasm_bindgen(raw_module = "../../data/sim.ts")]
extern "C" {
    #[wasm_bindgen(extends = Queue)]
    pub type Group;

    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Group;
}

#[wasm_bindgen(raw_module = "../../data/sim.ts")]
extern "C" {
    pub type Step;

    #[wasm_bindgen(constructor)]
    pub fn new(seconds: f32) -> Step;

    #[wasm_bindgen(method, final, js_name = "pushEvent")]
    pub fn on_push_back(this: &Step, event: PushBack);

    #[wasm_bindgen(method, final, js_name = "pushEvent")]
    pub fn on_push_front(this: &Step, event: PushFront);

    #[wasm_bindgen(method, final, js_name = "pushEvent")]
    pub fn on_pop(this: &Step, event: Pop);
}

type Id = i32;

#[wasm_bindgen(raw_module = "../../data/sim.ts")]
extern "C" {
    pub type PushBack;

    #[wasm_bindgen(constructor)]
    pub fn new(source: Option<Id>, destination: Id, value: Id) -> PushBack;
}

#[wasm_bindgen(raw_module = "../../data/sim.ts")]
extern "C" {
    pub type PushFront;

    #[wasm_bindgen(constructor)]
    pub fn new(source: Option<Id>, destination: Id, value: Id) -> PushFront;
}

#[wasm_bindgen(raw_module = "../../data/sim.ts")]
extern "C" {
    pub type Pop;

    #[wasm_bindgen(constructor)]
    pub fn new(queue: Id, value: Id) -> Pop;
}
