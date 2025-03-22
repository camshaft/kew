use pulldown_cmark::{CodeBlockKind, CowStr, Event, Parser, Tag, TagEnd};

pub struct Tokenizer<'a> {
    events: Parser<'a>,
    state: State<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            events: Parser::new(input),
            state: State::default(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(event) = self.events.next() else {
                return self.state.flush();
            };

            if let Some(out) = self.state.on_event(event) {
                return Some(out);
            }
        }
    }
}

#[derive(Default)]
enum State<'a> {
    #[default]
    Scanning,
    CodeBlock {
        info: CowStr<'a>,
        content: Vec<CowStr<'a>>,
    },
}

impl<'a> State<'a> {
    fn on_event(&mut self, event: Event<'a>) -> Option<Token<'a>> {
        match (event, self) {
            (Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))), v @ Self::Scanning) => {
                *v = Self::CodeBlock {
                    info,
                    content: vec![],
                };
                None
            }
            (
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))),
                Self::CodeBlock { content, .. },
            ) => {
                content.push(info);
                None
            }
            (Event::Text(text), Self::CodeBlock { content, .. }) => {
                content.push(text);
                None
            }
            (Event::End(TagEnd::CodeBlock), v @ Self::CodeBlock { .. }) => v.flush(),
            (other, Self::Scanning) => Some(Token::Other(other)),
            (other, Self::CodeBlock { .. }) => {
                panic!("event inside of code block: {other:?}");
            }
        }
    }

    fn flush(&mut self) -> Option<Token<'a>> {
        match core::mem::take(self) {
            State::Scanning => None,
            State::CodeBlock { info, content } => Some(Token::Code { info, content }),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Token<'a> {
    Code {
        info: CowStr<'a>,
        content: Vec<CowStr<'a>>,
    },
    Splice(Vec<Event<'a>>),
    Other(Event<'a>),
}

impl<'a> Token<'a> {
    pub fn iter(&'a self) -> Box<dyn Iterator<Item = Event<'a>> + '_> {
        match self {
            Token::Code { info, content } => {
                let start = core::iter::once(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(
                    CowStr::Borrowed(info),
                ))));
                let content = content
                    .iter()
                    .map(|content| Event::Text(CowStr::Borrowed(content)));
                let end = core::iter::once(Event::End(TagEnd::CodeBlock));
                Box::new(start.chain(content).chain(end))
            }
            Token::Splice(events) => {
                let events = events.iter().cloned();
                Box::new(events)
            }
            Token::Other(event) => Box::new(core::iter::once(event.clone())),
        }
    }
}
