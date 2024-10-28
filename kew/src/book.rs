pub mod charts;
mod fifo;
mod lifo;

#[test]
fn summary() {
    use api::*;

    title("SUMMARY.md");

    md("# Summary");
    md("- [Chapter 1 - Intoduction]()");
    md("- [Chapter 2 - FIFO](./fifo.md)");

    finish();
}

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

    pub use crate::{book::charts, channel::new as channel, sim::*};

    struct Context {
        db: duckdb::Connection,
        title: String,
        out: Vec<u8>,
        capture: Option<&'static Location<'static>>,
    }

    impl Default for Context {
        fn default() -> Self {
            let db = duckdb::Connection::open_in_memory().unwrap();
            Self {
                db,
                title: Default::default(),
                out: vec![],
                capture: None,
            }
        }
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

            dbg!(start.file());
            let file = Path::new(start.file());
            let file = if file.is_relative() {
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join(file)
            } else {
                file.to_path_buf()
            };
            let file = std::fs::File::open(file).unwrap();
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

            // trim trailing ws
            while out.last().filter(|v| **v == b'\n').is_some() {
                out.pop();
            }
            out.push(b'\n');

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
        with(|ctx| writeln!(ctx.out, "{v}").unwrap());
    }

    pub fn sql<T: core::fmt::Display>(sql: T) -> PathBuf {
        let sql = sql.to_string();
        let hash = blake3::hash(sql.as_bytes());
        let hash = hash.to_hex().to_string();
        let out = dir().join(hash).with_extension("tsv");

        let sql = format!(
            "COPY ({sql}) TO '{}' (FORMAT CSV, DELIM '\t');",
            out.display()
        );

        with(|ctx| {
            ctx.db.execute(&sql, []).unwrap();
        });

        out
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
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/book"))
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

    pub fn vega<T: core::fmt::Display>(value: T) {
        let path = emit(value, Some("json"));
        md(format_args!("#VEGA({})", path.display()));
    }

    pub fn emit<T: core::fmt::Display>(value: T, ext: Option<&str>) -> PathBuf {
        let contents = value.to_string();
        let hash = blake3::hash(contents.as_bytes());
        let mut out = dir().join(hash.to_hex());
        if let Some(ext) = ext {
            out.set_extension(ext);
        }

        std::fs::create_dir_all(out.parent().unwrap()).unwrap();

        if !out.exists() {
            std::fs::write(&out, contents).unwrap();
        }

        out
    }
}