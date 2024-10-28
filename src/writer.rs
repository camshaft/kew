use crate::parser::Event;
use arrow_array::{
    builder::{StringDictionaryBuilder, TimestampNanosecondBuilder, UInt64Builder},
    types::{UInt16Type, UInt32Type, UInt8Type},
    ArrayRef, RecordBatch,
};
use arrow_schema::{DataType, Field, SchemaBuilder, TimeUnit};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Default)]
pub struct Writer {
    count: usize,
    values: Values,
    seen: HashSet<&'static str>,
}

impl Writer {
    pub fn append(&mut self, event: Event) {
        self.values.timestamp.append_value(event.timestamp as _);
        self.values.name.append_value(event.data.name);
        self.values.value.append_value(event.data.value);
        self.values.unit.append_value(event.data.unit);
        self.values.kind.append_value(event.kind.as_str());

        for (key, value) in event.attrs.iter() {
            self.seen
                .insert(unsafe { core::mem::transmute::<&str, &str>(key) });

            if let Some(builder) = self.values.attrs.get_mut(*key) {
                builder.append_value(value);
                continue;
            }

            let mut values = StringDictionaryBuilder::new();
            if self.count > 0 {
                values.append_nulls(self.count);
            }
            values.append_value(value);
            self.values.attrs.insert(key.to_string(), values);
        }

        for (name, builder) in self.values.attrs.iter_mut() {
            if self.seen.contains(name.as_str()) {
                continue;
            }

            builder.append_null();
        }

        self.seen.clear();

        self.count += 1;
    }

    pub fn finish(mut self) -> RecordBatch {
        let mut columns = self.values.columns();

        let mut attrs = self.values.attrs.into_iter().collect::<Vec<_>>();
        attrs.sort_by(|(a, _), (b, _)| a.cmp(b));

        let mut schema = SchemaBuilder::new();
        Values::fields(&mut schema);

        for (name, mut values) in attrs {
            let field = Field::new(format!("attr_{name}"), string_dict(DataType::UInt32), true);
            schema.push(field);
            columns.push(Arc::new(values.finish()));
        }

        let schema = Arc::new(schema.finish());

        RecordBatch::try_new(schema, columns).unwrap()
    }
}

fn string_dict(key: DataType) -> DataType {
    DataType::Dictionary(Box::new(key), Box::new(DataType::Utf8))
}

#[derive(Default)]
struct Values {
    timestamp: TimestampNanosecondBuilder,
    name: StringDictionaryBuilder<UInt32Type>,
    value: UInt64Builder,
    unit: StringDictionaryBuilder<UInt16Type>,
    kind: StringDictionaryBuilder<UInt8Type>,
    attrs: HashMap<String, StringDictionaryBuilder<UInt32Type>>,
}

impl Values {
    fn fields(schema: &mut SchemaBuilder) {
        schema.push(Field::new(
            "timestamp",
            DataType::Timestamp(TimeUnit::Nanosecond, None),
            false,
        ));
        schema.push(Field::new("name", string_dict(DataType::UInt32), false));
        schema.push(Field::new("value", DataType::UInt64, false));
        schema.push(Field::new("unit", string_dict(DataType::UInt16), false));
        schema.push(Field::new("kind", string_dict(DataType::UInt8), false));
    }

    fn columns(&mut self) -> Vec<ArrayRef> {
        vec![
            Arc::new(self.timestamp.finish()),
            Arc::new(self.name.finish()),
            Arc::new(self.value.finish()),
            Arc::new(self.unit.finish()),
            Arc::new(self.kind.finish()),
        ]
    }
}
