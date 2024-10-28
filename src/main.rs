use parquet::{arrow::arrow_writer::ArrowWriter, file::properties::WriterProperties};

fn main() {
    let stdin = std::io::stdin();
    let stdin = std::io::BufReader::new(stdin);

    let mut writer = kew::writer::Writer::default();

    kew::parser::parse(stdin, |event| {
        writer.append(event);
    })
    .unwrap();

    let batch = writer.finish();

    let out = std::fs::File::create("metrics.arrow").unwrap();

    let props = WriterProperties::builder()
        //.set_compression(Compression::SNAPPY)
        .build();

    let mut writer = ArrowWriter::try_new(out, batch.schema(), Some(props)).unwrap();

    writer.write(&batch).unwrap();

    writer.close().unwrap();
}
