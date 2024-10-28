use std::path::Path;

static LINE: &str = include_str!("./charts/line.json");

pub fn line<P: AsRef<Path>>(path: P) -> String {
    let file = path.as_ref().file_name().unwrap().to_str().unwrap();
    LINE.replace("URL", file)
}