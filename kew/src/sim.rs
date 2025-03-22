use crate::parser::Event;
use arrow_array::RecordBatch;
use bach::environment::default as env;
use std::io;

pub use bach::{ext::*, time::sleep};

#[derive(Default)]
pub struct Output {
    writer: crate::writer::Writer,
}

bach::scope::define!(output, Output);

pub fn sim<F: FnOnce()>(f: F) -> io::Result<RecordBatch> {
    let output = Output::default();
    let prev_output = output::set(Some(output));

    env::Runtime::default().run(f);

    let output = output::set(prev_output).unwrap();

    Ok(output.writer.finish())
}

pub fn write<T: core::fmt::Display>(out: T) {
    output::borrow_mut_with(|output| {
        let out = out.to_string();

        // #[cfg(not(target_family = "wasm"))]
        // eprintln!("{out}");

        if let Some(event) = Event::parse(&out) {
            output.writer.append(event);
        }
    })
}
