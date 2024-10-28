use crate::parser::Event;
use bach::environment::default as env;
use blake3::Hasher;
use parquet::{arrow::ArrowWriter, file::properties::WriterProperties};
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Output {
    writer: crate::writer::Writer,
    hash: Hasher,
}

bach::scope::define!(output, Output);

pub fn sim<F: FnOnce()>(f: F) -> PathBuf {
    let output = Output::default();
    let prev_output = output::set(Some(output));

    env::Runtime::default().run(f);

    let output = output::set(prev_output).unwrap();

    let hash = output.hash.finalize();
    let batch = output.writer.finish();
    let dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/target/sim"));

    std::fs::create_dir_all(dir).unwrap();

    let path = dir.join(hash.to_hex()).with_extension("arrow");

    let out = std::fs::File::create(&path).unwrap();

    let props = WriterProperties::builder().build();

    let mut writer = ArrowWriter::try_new(out, batch.schema(), Some(props)).unwrap();

    writer.write(&batch).unwrap();

    writer.close().unwrap();

    path
}

pub use bach::{ext::*, time::sleep};

pub fn write<T: core::fmt::Display>(out: T) {
    output::borrow_mut_with(|output| {
        let out = out.to_string();

        eprintln!("{out}");

        output.hash.update(out.as_bytes());

        if let Some(event) = Event::parse(&out) {
            output.writer.append(event);
        }
    })
}
