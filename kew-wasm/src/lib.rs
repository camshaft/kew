use core::time::Duration;
use kew::{
    channel,
    ring_deque::Behavior,
    sim::{sim_with_out, *},
};
use serde::Deserialize;
use serde_with::{serde_as, DurationSecondsWithFrac};
use std::{collections::HashMap, sync::Arc};
use wasm_bindgen::prelude::*;

type DurationSerde = DurationSecondsWithFrac<f64>;

#[wasm_bindgen(start)]
fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Kew {
    #[serde(default)]
    actors: Vec<Actor>,
    #[serde(default)]
    queues: Vec<Queue>,
    #[serde_as(as = "DurationSerde")]
    #[serde(default = "one_s")]
    run_time: Duration,
}

fn one_s() -> Duration {
    Duration::from_secs(1)
}

impl Default for Kew {
    fn default() -> Self {
        Self {
            actors: Default::default(),
            queues: Default::default(),
            run_time: one_s(),
        }
    }
}

#[wasm_bindgen]
impl Kew {
    #[wasm_bindgen(constructor)]
    pub fn new(v: JsValue) -> Result<Self, JsValue> {
        let mut kew = Self::default();
        if v.is_null() || v.is_undefined() {
            return Ok(kew);
        }

        kew.update(v)?;

        Ok(kew)
    }

    #[wasm_bindgen]
    pub fn update(&mut self, v: JsValue) -> Result<bool, JsValue> {
        let v = if let Some(v) = wasm_bindgen::JsValue::as_string(&v) {
            serde_json::from_str(&v).map_err(|err| err.to_string())?
        } else {
            serde_wasm_bindgen::from_value(v)?
        };
        // TODO only return true if anything changed
        *self = v;
        Ok(true)
    }

    #[wasm_bindgen(getter)]
    pub fn run_time(&self) -> f32 {
        self.run_time.as_secs_f32()
    }

    #[wasm_bindgen(setter)]
    pub fn set_run_time(&mut self, run_time: f32) {
        self.run_time = Duration::from_secs_f32(run_time);
    }

    #[wasm_bindgen]
    pub fn to_arrow(&self) -> Vec<u8> {
        self.run(Format::Arrow)
    }

    #[wasm_bindgen]
    #[cfg(feature = "parquet")]
    pub fn to_parquet(&self) -> Vec<u8> {
        self.run(Format::Parquet)
    }

    fn run(&self, format: Format) -> Vec<u8> {
        let config = unsafe {
            // SAFETY: self outlives the simulator runtime
            core::mem::transmute::<&Self, &'static Self>(self)
        };

        let mut out = vec![];

        sim_with_out(format, &mut std::io::Cursor::new(&mut out), || {
            let queues: HashMap<&'static str, Pair<u64>> = config
                .queues
                .iter()
                .map(|q| (q.id.as_str(), q.create()))
                .collect();
            let queues = std::sync::Arc::new(queues);

            for actor in &config.actors {
                actor.start(queues.clone());
            }

            config.run_time.sleep().primary().spawn();
        })
        .unwrap();

        out
    }
}

type Queues<T> = Arc<HashMap<&'static str, Pair<T>>>;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Actor {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    operations: Vec<Operation>,
}

impl Actor {
    fn start(&'static self, queues: Queues<u64>) {
        async move {
            let mut id = Id::default();
            Operation::run_all(&self.operations, &mut id, &queues).await;
        }
        .group(&self.id)
        .spawn();
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Operation {
    Sleep(#[serde_as(as = "DurationSerde")] Duration),
    PushBack {
        queue: String,
        #[serde(default = "default_count")]
        count: u32,
    },
    PushFront {
        queue: String,
        #[serde(default = "default_count")]
        count: u32,
    },
    PopBack {
        queue: String,
    },
    PopFront {
        queue: String,
    },
    Loop(Vec<Operation>),
}

fn default_count() -> u32 {
    1
}

impl Operation {
    async fn run_all(ops: &[Operation], id: &mut Id, queues: &Queues<u64>) -> bool {
        let mut did_sleep = false;
        for op in ops {
            did_sleep |= op.run(id, queues).await;
        }
        did_sleep
    }

    async fn run(&self, id: &mut Id, queues: &Queues<u64>) -> bool {
        match self {
            Operation::Sleep(amount) => {
                // a zero sleep is invalid so default to 1ms
                if !amount.is_zero() {
                    amount.sleep().await;
                    return true;
                }
            }
            Operation::PushBack { queue, count } => {
                for _ in 0..*count {
                    let queue = queues
                        .get(queue.as_str())
                        .unwrap_or_else(|| panic!("invalid queue {queue:?}"));
                    queue.send.send_back(id.next().unwrap()).await.unwrap();
                }
            }
            Operation::PushFront { queue, count } => {
                for _ in 0..*count {
                    let queue = queues
                        .get(queue.as_str())
                        .unwrap_or_else(|| panic!("invalid queue {queue:?}"));
                    queue.send.send_front(id.next().unwrap()).await.unwrap();
                }
            }
            Operation::PopBack { queue } => {
                let queue = queues
                    .get(queue.as_str())
                    .unwrap_or_else(|| panic!("invalid queue {queue:?}"));
                queue.recv.recv_back().await.unwrap();
            }
            Operation::PopFront { queue } => {
                let queue = queues
                    .get(queue.as_str())
                    .unwrap_or_else(|| panic!("invalid queue {queue:?}"));
                queue.recv.recv_front().await.unwrap();
            }
            Operation::Loop(ops) => loop {
                let did_sleep = Box::pin(Self::run_all(ops, id, queues)).await;

                // if the loop doesn't sleep at all then we will hang
                if !did_sleep {
                    1.ms().sleep().await;
                }
            },
        }

        false
    }
}

#[derive(Debug, Default)]
struct Id(u64);

impl Iterator for Id {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let v = self.0;
        self.0 += 1;
        Some(v)
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Queue {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default, rename = "type")]
    ty: QueueType,
}

impl Queue {
    fn create<T>(&self) -> Pair<T> {
        let (send, recv) = channel::new(&self.id, self.ty.into());
        Pair { send, recv }
    }
}

struct Pair<T> {
    send: channel::Sender<T>,
    recv: channel::Receiver<T>,
}

#[derive(Copy, Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum QueueType {
    #[default]
    Unbounded,
    PreferNew(u32),
    PreferOld(u32),
}

impl From<QueueType> for Behavior {
    fn from(value: QueueType) -> Self {
        match value {
            QueueType::Unbounded => Behavior::Unbounded,
            QueueType::PreferNew(cap) => Behavior::Wrap(cap as _),
            QueueType::PreferOld(cap) => Behavior::Reject(cap as _),
        }
    }
}
