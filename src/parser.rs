use std::io;

#[inline]
pub fn parse<R: io::BufRead>(r: R) -> Parser<R> {
    Parser {
        line: String::new(),
        lines: r.lines(),
    }
}

#[derive(Clone, Debug)]
pub struct Event<'a> {
    pub timestamp: u64,
    pub kind: Kind,
    pub data: Data<'a>,
    pub attrs: Attrs<'a>,
}

impl<'a> Event<'a> {
    fn parse(v: &'a str) -> Option<Self> {
        let v = v.expect("kew[")?;
        let (timestamp, v) = v.split_once_ws(']')?;
        let timestamp = timestamp.parse().ok()?;
        let (kind, v) = v.one_of(&[
            ("count#", || Kind::Count),
            ("gauge#", || Kind::Gauge),
            ("measure#", || Kind::Measure),
        ])?;
        let (data, v) = Data::parse(v)?;
        let attrs = Attrs::parse(v)?;

        Some(Self {
            timestamp,
            kind,
            data,
            attrs,
        })
    }
}

trait StrExt {
    fn expect<'a>(&'a self, lit: &str) -> Option<&'a str>;
    fn one_of<'a, T>(&'a self, matches: &[(&str, fn() -> T)]) -> Option<(T, &'a str)>;
    fn split_once_ws<'a>(&'a self, pat: char) -> Option<(&'a str, &'a str)>;
}

impl StrExt for str {
    fn expect<'a>(&'a self, lit: &str) -> Option<&'a str> {
        if self.starts_with(lit) {
            let (_, v) = self.split_at(lit.len());
            Some(v)
        } else {
            None
        }
    }

    fn one_of<'a, T>(&'a self, matches: &[(&str, fn() -> T)]) -> Option<(T, &'a str)> {
        for (ty, v) in matches {
            if let Some(remaining) = self.expect(ty) {
                return Some((v(), remaining));
            }
        }

        None
    }

    fn split_once_ws<'a>(&'a self, pat: char) -> Option<(&'a str, &'a str)> {
        let (a, b) = self.split_once(pat)?;
        let a = a.trim_end();
        let b = b.trim_start();
        Some((a, b))
    }
}

#[derive(Clone, Debug)]
pub struct Data<'a> {
    name: &'a str,
    value: u64,
    unit: &'a str,
}

impl<'a> Data<'a> {
    fn parse(v: &'a str) -> Option<(Self, &'a str)> {
        let (name, v) = v.split_once_ws('=')?;
        let (value, v) = if let Some(v) = v.split_once(' ') {
            v
        } else {
            (v, "")
        };
        let (value, unit) = value.split_once(char::is_numeric)?;
        let value = value.parse().ok()?;

        let data = Data { name, value, unit };
        Some((data, v))
    }
}

#[derive(Clone, Debug)]
pub struct Attrs<'a> {
    attrs: Vec<(&'a str, &'a str)>,
}

impl<'a> Attrs<'a> {
    fn parse(v: &'a str) -> Option<Self> {
        let mut attrs = vec![];
        for kv in v.split_whitespace() {
            if let Some((key, value)) = kv.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                attrs.push((key, value));
            }
        }
        Some(Self { attrs })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Count,
    Gauge,
    Measure,
}

pub struct Parser<R: io::BufRead> {
    line: String,
    lines: io::Lines<R>,
}

impl<R: io::BufRead> Parser<R> {
    fn event(&mut self) -> Option<io::Result<Event>> {
        loop {
            match self.lines.next() {
                None => return None,
                Some(Err(err)) => return Some(Err(err)),
                Some(Ok(v)) if v.is_empty() => continue,
                Some(Ok(v)) => {
                    self.line = v;
                }
            };

            let res = Event::parse(&self.line);

            if res.is_none() {
                drop(res);
                continue;
            }

            return res.map(Ok);
        }
    }
}
