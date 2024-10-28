macro_rules! ensure {
    ($cond:expr) => {
        ensure!($cond, ());
    };
    ($cond:expr, $otherwise:expr) => {
        if !$cond {
            return $otherwise;
        }
    };
}

macro_rules! log {
    ($($tt:tt)*) => {
        crate::sim::write(format_args!($($tt)*));
    }
}

macro_rules! measure {
    ($name:expr, $value:expr) => {
        measure!($name, $value, "");
    };
    ($name:expr, $value:expr, $unit:expr $(, $attr:ident = $attr_v:expr)* $(,)?) => {
        let attrs = "";
        $(
            // TODO make this not allocate every time
            let attrs = format!("{attrs} {}={}", stringify!($attr), $attr_v);
        )*
        log!(
            "kew[{}]measure#{}={}{}{attrs}",
            bach::time::Instant::now().elapsed_since_start().as_nanos(),
            $name,
            $value,
            $unit,
        )
    };
}

macro_rules! count {
    ($name:expr, $value:expr) => {
        count!($name, $value, "");
    };
    ($name:expr, $value:expr, $unit:expr $(, $attr:ident = $attr_v:expr)* $(,)?) => {
        let attrs = "";
        $(
            // TODO make this not allocate every time
            let attrs = format!("{attrs} {}={}", stringify!($attr), $attr_v);
        )*
        log!(
            "kew[{}]count#{}={}{}{attrs}",
            bach::time::Instant::now().elapsed_since_start().as_nanos(),
            $name,
            $value,
            $unit,
        )
    };
}

pub mod channel;
pub mod parser;
pub mod ring_deque;
pub mod sim;
pub mod writer;

#[cfg(test)]
mod book;
