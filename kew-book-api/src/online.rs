use super::Label;
use datafusion::execution::context::SessionContext;
use datafusion::prelude::SessionConfig;
use std::sync::Arc;

pub use wasm_bindgen::prelude::*;

impl super::Toc {
    pub fn render(&self) {
        unimplemented!()
    }
}

pub struct Context {
    df: Arc<SessionContext>,
    file: &'static str,
    name: &'static str,
}

impl Context {
    pub fn new(input: JsValue, file: &'static str, name: &'static str) -> Self {
        let config = SessionConfig::new();
        let df = Arc::new(SessionContext::new_with_config(config));

        Self { df, file, name }
    }

    pub fn input<L, I>(&mut self, label: L, input: I) -> I::Output
    where
        L: AsRef<Label>,
        I: super::Input,
    {
        // TODO read the actual value
        input.default_value()
    }

    pub fn sim<F: FnOnce()>(&mut self, f: F) -> super::Table {
        let batch = kew::sim::sim(f).unwrap();
        super::Table::new(batch, self.df.clone())
    }

    pub fn figure<L, V>(&mut self, label: L, figure: super::Figure<V>)
    where
        L: AsRef<Label>,
        V: core::fmt::Display,
    {
        let label = label.as_ref();
        let title = label.title;

        // TODO render the figure
    }

    pub fn finish(&mut self) {
        // TODO
    }
}
