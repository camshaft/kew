use super::*;

pub fn render<T: core::fmt::Display>(value: T) {
    let path = emit(value, Some("json"));
    md(format_args!("\n!VEGA{{\"path\":{:?}}}\n", path.display()));
}
