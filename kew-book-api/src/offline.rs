use super::Label;
use datafusion::execution::context::SessionContext;
use datafusion::prelude::SessionConfig;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use xshell::{cmd, Shell};

static FULL_RENDER: Lazy<bool> = Lazy::new(|| std::env::var("KEW_RENDER").is_ok());

fn sh() -> Shell {
    let sh = Shell::new().unwrap();
    sh.change_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../"));
    sh
}

impl super::Toc {
    pub fn render(&self) {
        let sh = sh();
        self.render_js(&sh);
        if *FULL_RENDER {
            self.render_pdf(&sh);
        }
    }

    fn render_js(&self, sh: &Shell) {
        let dir = book_dir().join("html");

        let json = emit(
            &sh,
            r#"
        {
            "type": "module",
            "private": true,
            "scripts": {
                "build": "observable build --debug",
                "dev": "observable preview --no-open"
            },
            "dependencies": {
                "@observablehq/framework": "^1.12.0"
            },
            "engines": {
                "node": ">=18"
            }
        }
        "#,
            Some("json"),
        );
        alias(&sh, json, dir.join("package.json"));

        let mut pages = String::new();
        self.render_sections(&mut pages, &mut vec![]);

        let config = emit(
            &sh,
            format!(
                "
            export default {{
                title: 'Kew',
                search: true,
                root: 'src',
                toc: true,
                footer: '',
                pages: {pages},
            }}
            "
            ),
            Some("js"),
        );
        alias(&sh, config, dir.join("observablehq.config.js"));
    }

    fn render_pdf(&self, sh: &Shell) {
        let dir = book_dir().join("pdf");

        let mut dependencies = String::new();
        self.render_dep(&mut dependencies);

        let make = emit(
            &sh,
            vec![
                "../../book/pdf/book.pdf: book.md",
                "\t@pandoc --toc --from markdown --to pdf --number-sections --standalone -o $@ $<",
                &format!("book.md:{dependencies}"),
                "\t@cat $^ > $@",
            ]
            .join("\n"),
            None,
        );
        alias(&sh, make, dir.join("Makefile"));
    }

    fn render_sections(&self, out: &mut String, path: &mut Vec<String>) {
        out.push('[');
        for (idx, section) in self.sections.iter().enumerate() {
            path.push((idx + 1).to_string());
            section.render_page(out, path);
            out.push(',');
            path.pop();
        }
        out.push(']');
    }

    fn render_page(&self, out: &mut String, section_path: &mut Vec<String>) {
        out.push('{');
        let id = section_path.join(".");
        let name = format!("{id}. {}", self.title);
        out.push_str(&format!("name: {name:?}, open: true, "));
        let path = if let Some(id) = self.id {
            format!("/{id}")
        } else {
            let id = slugify(self.title);
            format!("/{id}")
        };
        out.push_str(&format!("path: {path:?}, "));
        out.push_str("pages: ");
        self.render_sections(out, section_path);
        out.push('}');
    }

    fn render_dep(&self, out: &mut String) {
        if let Some(id) = self.id {
            out.push(' ');
            out.push_str(&format!("src/{id}.md"));
        } else if !self.title.is_empty() {
            let id = slugify(self.title);
            out.push(' ');
            out.push_str(&format!("src/{id}.md"));
        };

        for section in self.sections {
            section.render_dep(out);
        }
    }
}

fn slugify(name: &str) -> String {
    name.replace(&[' ', '_'], "-").to_lowercase()
}

pub struct Context {
    sh: Shell,
    df: Arc<SessionContext>,
    file: &'static str,
    name: String,
    inputs: HashMap<String, Input>,
    figures: HashMap<String, Figure>,
    full_render: bool,
}

impl Context {
    pub fn new(file: &'static str, name: &'static str) -> Self {
        let sh = sh();

        let config = SessionConfig::new();
        let df = Arc::new(SessionContext::new_with_config(config));

        let name = slugify(name);

        Self {
            sh,
            df,
            file,
            name,
            inputs: Default::default(),
            figures: Default::default(),
            full_render: *FULL_RENDER,
        }
    }

    pub fn input<L, I>(&mut self, label: L, input: I) -> I::Output
    where
        L: AsRef<Label>,
        I: super::Input,
    {
        // TODO register the input
        input.default_value()
    }

    pub fn sim<F: FnOnce()>(&mut self, f: F) -> super::Table {
        let batch = kew::sim::sim(f).unwrap();
        super::Table::new(batch, self.df.clone())
    }

    pub fn figure<L, V>(&mut self, label: L, figure: super::Figure<V>)
    where
        L: AsRef<Label>,
        V: core::fmt::Display,
    {
        let label = label.as_ref();
        let title = label.title;

        let arrow = {
            let out = figure.table.to_ipc().expect("empty batch");
            self.emit(out, Some("arrow"))
        };

        let js = {
            let js = &figure.js;
            let js = format!(
                "
                import * as Plot from '@observablehq/plot';
                export default {js};
                ",
            );
            self.emit(js, Some("js"))
        };

        let offline = {
            let js = js.display().to_string();
            let arrow = arrow.display().to_string();
            let width: u32 = 800; // TODO make this configurable
            let js = format!(
                "
                import {{ JSDOM }} from 'jsdom';
                import * as Plot from '@observablehq/plot';
                import {{ tableFromIPC }} from 'apache-arrow';
                import {{readFile, writeFile}} from 'node:fs/promises';
                import {{ argv }} from 'node:process';
                import renderPlot from '{js}';

                const title = {title:?};
                const data = tableFromIPC(await readFile({arrow:?}));
                const window = new JSDOM('').window;
                const document = window.document;

                // patch the globals to make Plot happy
                global.Event = window.Event;

                const width = {width};
                const config = renderPlot({{ data, width, title, document }});
                let plot = {{ width, title, document }};
                Object.assign(plot, config);
                plot = Plot.plot(plot).querySelector('svg');

                plot.setAttributeNS('http://www.w3.org/2000/xmlns/', 'xmlns', 'http://www.w3.org/2000/svg');
                plot.setAttributeNS('http://www.w3.org/2000/xmlns/', 'xmlns:xlink', 'http://www.w3.org/1999/xlink');

                if (argv[2]) await writeFile(argv[2], plot.outerHTML);
                else process.stdout.write(plot.outerHTML);
                ",
            );

            self.emit(js, Some("js"))
        };

        let img = if self.full_render {
            let img = offline.with_extension("svg");
            if !img.exists() {
                cmd!(self.sh, "node {offline} {img}").run().unwrap();
            }
            Some(img)
        } else {
            None
        };

        let figure = Figure {
            title: title.to_string(),
            arrow,
            js,
            offline,
            img,
        };
        self.figures.insert(label.generate_id(), figure);
    }

    pub fn finish(&mut self) {
        let markup = Path::new(self.file).with_extension("md");

        if !self.sh.path_exists(&markup) {
            self.sh
                .write_file(&markup, format!("## {}", self.name))
                .unwrap();
        }

        let markup = self.sh.read_file(&markup).unwrap();

        let html_src = book_dir().join("html/src");

        let src = self.finish_html(&markup, &html_src);

        self.alias(src, html_src.join(&self.name).with_extension("md"));

        if self.full_render {
            let pdf_src = book_dir().join("pdf/src");

            let src = self.finish_pdf(&markup, &pdf_src);

            self.alias(src, pdf_src.join(&self.name).with_extension("md"));
        }
    }

    fn finish_html(&mut self, markup: &str, src_dir: &Path) -> PathBuf {
        let mut out = String::with_capacity(markup.len());
        let mut used_inputs = HashSet::with_capacity(self.inputs.len());
        let mut used_figures = HashSet::with_capacity(self.figures.len());

        let mut errors = vec![];

        for (lineno, line) in markup.lines().enumerate() {
            let lineno = lineno + 1;

            if let Some(name) = line.strip_prefix("!FIGURE(") {
                let name = name.trim_end_matches(')');
                if let Some(fig) = self.figures.get(name) {
                    let title = &fig.title;

                    // TODO check if we have inputs. if so, we need to load the js

                    let arrow = &fig.arrow;
                    let arrow_path = format!("{}/{name}.arrow", self.name);
                    self.alias(arrow, src_dir.join(&arrow_path));
                    let relative_arrow = format!("./{name}.arrow");

                    let js = &fig.js;
                    let js_path = format!("{}/{name}_plot.js", self.name);
                    self.alias(js, src_dir.join(&js_path));

                    let dynamic_js = self.emit(
                        format!(
                            "
                            import {{ FileAttachment }} from 'observablehq:stdlib';
                            import * as Plot from '@observablehq/plot';
                            import renderPlot from './{name}_plot.js';

                            const title = {title:?};
                            let pending;

                            export default async () => {{
                                if (!pending) pending = FileAttachment({relative_arrow:?}).arrow();
                                const data = await pending;
                                return (width) => {{
                                    const config = renderPlot({{ data, width, title }});
                                    let plot = {{ width, title }};
                                    Object.assign(plot, config);
                                    return Plot.plot(plot);
                                }};
                            }};
                            "
                        ),
                        Some("js"),
                    );

                    let path = format!("{}/{name}.js", self.name);
                    self.alias(dynamic_js, src_dir.join(&path));

                    let plot_name = format!("__{}_plot", name.replace('-', "_"));

                    if !used_figures.contains(name) {
                        out.push_str(&"\n```js\n");
                        out.push_str(&format!("import load from {:?};\n", format!("./{path}")));
                        out.push_str(&format!("const {plot_name} = load();\n"));
                        out.push_str(&"```\n\n");
                    }

                    out.push_str(&format!("${{ resize({plot_name}) }}\n"));

                    used_figures.insert(name);
                } else {
                    let error = format!("#{lineno}: Missing figure {name:?}");

                    if !self.full_render {
                        out.push_str(&format!("```js\nthrow new Error({error:?});\n```"));
                    }

                    errors.push(error);
                }
                continue;
            }

            if let Some(name) = line.strip_prefix("!INPUT(") {
                let name = name.trim_end_matches(')');
                if let Some(input) = self.inputs.get(name) {
                    let js = &input.js;
                    out.push_str(&format!("```js\n{js}\n```"));
                    used_inputs.insert(name);
                } else {
                    let error = format!("#{lineno}: Missing input {name:?}");

                    if !self.full_render {
                        out.push_str(&format!("```js\nthrow new Error({error:?});\n```"));
                    }

                    errors.push(error);
                }
                continue;
            }

            out.push_str(line);
            out.push('\n');
        }

        for name in self.inputs.keys() {
            if !used_inputs.contains(name.as_str()) {
                errors.push(format!(": Unused input {name:?}"));
            }
        }

        for name in self.figures.keys() {
            if !used_figures.contains(name.as_str()) {
                errors.push(format!(": Unused figure {name:?}"));
            }
        }

        if !errors.is_empty() {
            for error in errors {
                eprintln!("{}{error}", self.file);
            }
            assert!(!self.full_render);
        }

        self.emit(out, Some("md"))
    }

    fn finish_pdf(&mut self, markup: &str, _src_dir: &Path) -> PathBuf {
        let mut out = String::with_capacity(markup.len());

        for line in markup.lines() {
            if let Some(name) = line.strip_prefix("!FIGURE(") {
                let name = name.trim_end_matches(')');
                if let Some(fig) = self.figures.get(name) {
                    let title = &fig.title;
                    let img = fig.img.as_ref().unwrap();
                    let img = img.display();
                    out.push_str(&format!("![{title}]({img})\n"));
                }
                continue;
            }

            // skip inputs with pandoc
            if line.starts_with("!INPUT(") {
                continue;
            }

            out.push_str(line);
            out.push('\n');
        }

        out.push('\n');

        self.emit(out, Some("md"))
    }

    fn emit<T: AsRef<[u8]>>(&self, value: T, ext: Option<&str>) -> PathBuf {
        emit(&self.sh, value, ext)
    }

    fn alias<S, D>(&self, source: S, destination: D)
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
    {
        alias(&self.sh, source, destination)
    }
}

struct Input {
    name: String,
    js: String,
}

struct Figure {
    title: String,
    js: PathBuf,
    arrow: PathBuf,
    offline: PathBuf,
    img: Option<PathBuf>,
}

fn book_dir() -> &'static Path {
    Path::new("target/book-src")
}

fn emit<T: AsRef<[u8]>>(sh: &Shell, value: T, ext: Option<&str>) -> PathBuf {
    let value = value.as_ref();
    let hash = blake3::hash(value);
    let mut out = book_dir().join("contents").join(hash.to_hex().to_string());

    if let Some(ext) = ext {
        out.set_extension(ext);
    }

    sh.create_dir(out.parent().unwrap()).unwrap();

    if !sh.path_exists(&out) {
        sh.write_file(&out, value).unwrap();
    }

    sh.current_dir().join(out).canonicalize().unwrap()
}

fn alias<S, D>(sh: &Shell, source: S, destination: D)
where
    S: AsRef<Path>,
    D: AsRef<Path>,
{
    let source = source.as_ref();
    let destination = sh.current_dir().join(destination.as_ref());

    sh.create_dir(destination.parent().unwrap()).unwrap();

    sh.copy_file(source, destination).unwrap()
}
