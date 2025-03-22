use super::*;
use core::fmt;
use pulldown_cmark::{CowStr, Event, Parser};
use std::path::PathBuf;

mod tokenizer;
use tokenizer::{Token, Tokenizer};

#[derive(Debug, Parser)]
pub struct Compile {
    path: PathBuf,
    output: Option<PathBuf>,
}

impl Compile {
    pub fn run(&self, sh: &Shell) {
        let file = sh.read_file(&self.path).unwrap();
        let events: Vec<_> = Tokenizer::new(&file).collect();
        dbg!(&events);
        let mut compiler = Compiler { events };
        compiler.compile();

        println!("{compiler}");
    }
}

struct Compiler<'a> {
    events: Vec<Token<'a>>,
}

impl fmt::Display for Compiler<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use pulldown_cmark_to_cmark::cmark;
        cmark(self.events.iter().flat_map(|event| event.iter()), f)?;
        Ok(())
    }
}

impl Compiler<'_> {
    fn compile(&mut self) {
        self.json_sim();
    }

    fn json_sim(&mut self) {
        for event in self.events.iter_mut() {
            match event {
                Token::Code { info, content } => {
                    dbg!(info, content);
                }
                _ => {}
            }
        }
    }
}
