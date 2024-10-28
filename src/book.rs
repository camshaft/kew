mod fifo;
mod lifo;

mod api {
    use std::{
        cell::RefCell,
        fs::File,
        io::{BufRead, Write},
        panic::Location,
        path::{Path, PathBuf},
    };

    thread_local! {
        static CONTEXT: RefCell<Context> = RefCell::new(Context::default());
    }

    pub use crate::{channel::new as channel, sim::*};

    #[derive(Default)]
    struct Context {
        title: String,
        out: Vec<u8>,
        capture: Option<&'static Location<'static>>,
    }

    impl Context {
        fn enable_capture(&mut self, location: &'static Location<'static>) {
            if self.capture.is_none() {
                self.capture = Some(location);
            }
        }

        fn disable_capture(&mut self, finish: &'static Location<'static>) {
            let start = self.capture.take().expect("missing opening capture");

            assert_eq!(start.file(), finish.file());
            assert!(start.line() < finish.line());

            let file = std::fs::File::open(start.file()).unwrap();
            let file = std::io::BufReader::new(file);

            let skip = start.line() as usize;
            let total = (finish.line() - start.line() - 1) as usize;

            let mut out = vec![];

            let mut has_content = false;
            let mut dedent = 0;

            for line in file.lines().skip(skip).take(total) {
                let line = line.unwrap();

                if !has_content && line.trim().is_empty() {
                    continue;
                }

                if !has_content {
                    dedent = line.len() - line.trim_start().len();
                }

                let line = if line.is_empty() {
                    &line
                } else {
                    &line[dedent..]
                };

                has_content = true;

                writeln!(out, "{line}").unwrap();
            }

            writeln!(self.out).unwrap();
            writeln!(self.out, "```rust").unwrap();
            self.out.extend(out);
            writeln!(self.out, "```").unwrap();
            writeln!(self.out).unwrap();
        }
    }

    fn with<F: FnOnce(&mut Context) -> R, R>(f: F) -> R {
        CONTEXT.with(|c| {
            let mut ctx = c.borrow_mut();
            f(&mut ctx)
        })
    }

    pub fn title<T: core::fmt::Display>(v: T) {
        with(|ctx| ctx.title = v.to_string());
    }

    pub fn md<T: core::fmt::Display>(v: T) {
        with(|ctx| write!(ctx.out, "{v}").unwrap());
    }

    #[track_caller]
    pub fn capture(enabled: bool) {
        let location = Location::caller();
        with(|ctx| {
            if enabled {
                ctx.enable_capture(location);
            } else {
                ctx.disable_capture(location);
            }
        })
    }

    fn dir() -> &'static Path {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/target/book"))
    }

    pub fn finish() {
        with(|ctx| {
            let dir = dir();
            std::fs::create_dir_all(dir).unwrap();
            let mut path = dir.join(&ctx.title);
            path.set_extension("md");
            let mut file = File::create(path).unwrap();

            file.write_all(&ctx.out).unwrap();
        })
    }

    pub fn emit_file<P: AsRef<Path>>(path: P) -> PathBuf {
        let path = path.as_ref();
        let contents = std::fs::read(path).unwrap();
        let hash = blake3::hash(&contents);
        let mut out = dir().join(hash.to_hex());
        if let Some(ext) = path.extension() {
            out.set_extension(ext);
        }

        std::fs::create_dir_all(out.parent().unwrap()).unwrap();

        if !out.exists() {
            std::fs::write(&out, contents).unwrap();
        }

        out
    }
}
