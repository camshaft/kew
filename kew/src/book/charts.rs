use std::path::Path;

macro_rules! chart {
    ($name:ident) => {
        pub fn $name<P: AsRef<Path>>(path: P) -> String {
            let file = path.as_ref().file_name().unwrap().to_str().unwrap();
            include_str!(concat!("charts/", stringify!($name), ".json")).replace("URL", file)
        }
    };
}

chart!(latency);
chart!(count);
