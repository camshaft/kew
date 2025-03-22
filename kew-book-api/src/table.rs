use super::Figure;
use arrow_ipc::writer as ipc;
use core::task;
use datafusion::{arrow::array::RecordBatch, prelude::*};
use std::pin::pin;
use std::sync::Arc;
use std::{future::Future, task::Poll};

pub struct Table {
    df: DataFrame,
    ctx: Arc<SessionContext>,
}

impl Table {
    pub fn new(batch: RecordBatch, ctx: Arc<SessionContext>) -> Self {
        let df = ctx.read_batch(batch).unwrap();
        Self { df, ctx }
    }

    pub fn figure<V>(&self, js: V) -> Figure<V>
    where
        V: core::fmt::Display,
    {
        Figure {
            js,
            table: Self {
                // TODO how cheap is cloning?
                df: self.df.clone(),
                ctx: self.ctx.clone(),
            },
        }
    }

    pub fn select_exprs(&self, exprs: &[&str]) -> Self {
        // TODO how cheap is cloning?
        let df = self.df.clone().select_exprs(exprs).unwrap();
        Self {
            df,
            ctx: self.ctx.clone(),
        }
    }

    pub fn filter(self, expr: Expr) -> Self {
        let df = self.df.filter(expr).unwrap();
        let ctx = self.ctx;
        Self { df, ctx }
    }

    pub fn collect(self) -> Vec<RecordBatch> {
        let f = self.df.collect();
        let mut f = pin!(f);

        let waker = waker::noop();
        let mut cx = task::Context::from_waker(&waker);

        loop {
            match f.as_mut().poll(&mut cx) {
                Poll::Pending => {
                    println!("pending");
                    continue;
                }
                Poll::Ready(res) => {
                    return res.unwrap();
                }
            }
        }
    }

    pub fn to_ipc(self) -> Option<Vec<u8>> {
        let batches = self.collect();
        let first = batches.first()?;
        let mut out = vec![];
        let mut writer = ipc::FileWriter::try_new(&mut out, first.schema_ref()).unwrap();
        for batch in batches {
            writer.write(&batch).unwrap();
        }
        writer.finish().unwrap();
        Some(out)
    }
}

mod waker {
    use core::ptr;
    use core::task::{RawWaker, RawWakerVTable, Waker};

    #[inline]
    pub fn noop() -> Waker {
        const VTABLE: RawWakerVTable = RawWakerVTable::new(
            // Cloning just returns a new no-op raw waker
            |_| RAW,
            // `wake` does nothing
            |_| {},
            // `wake_by_ref` does nothing
            |_| {},
            // Dropping does nothing as we don't allocate anything
            |_| {},
        );
        const RAW: RawWaker = RawWaker::new(ptr::null(), &VTABLE);

        unsafe { Waker::from_raw(RAW) }
    }
}
