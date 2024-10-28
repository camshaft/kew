use core::ops::Deref;
use std::io;

#[inline]
pub fn parse<R, E>(r: R, mut on_event: E) -> io::Result<()>
where
    R: io::BufRead,
    E: FnMut(Event),
{
    for line in r.lines() {
        let line = line?;
        if let Some(event) = Event::parse(&line) {
            on_event(event);
        }
    }

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Event<'a> {
    pub timestamp: u64,
    pub kind: Kind,
    pub data: Data<'a>,
    pub attrs: Attrs<'a>,
}

impl<'a> Event<'a> {
    pub fn parse(v: &'a str) -> Option<Self> {
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
    fn split_once_ws(&self, pat: char) -> Option<(&str, &str)>;
    fn take_numeric<T: core::str::FromStr>(&self) -> Option<(T, &str)>;
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

    fn split_once_ws(&self, pat: char) -> Option<(&str, &str)> {
        let (a, b) = self.split_once(pat)?;
        let a = a.trim_end();
        let b = b.trim_start();
        Some((a, b))
    }

    fn take_numeric<T: core::str::FromStr>(&self) -> Option<(T, &str)> {
        let index = self
            .char_indices()
            .find_map(|(idx, c)| if !c.is_numeric() { Some(idx) } else { None })
            .unwrap_or(self.len());
        let (a, b) = self.split_at(index);
        let a = a.parse().ok()?;
        Some((a, b))
    }
}

#[derive(Clone, Debug)]
pub struct Data<'a> {
    pub name: &'a str,
    pub value: u64,
    pub unit: &'a str,
}

impl<'a> Data<'a> {
    fn parse(v: &'a str) -> Option<(Self, &'a str)> {
        let (name, v) = v.split_once_ws('=')?;
        let (value, v) = if let Some(v) = v.split_once(' ') {
            v
        } else {
            (v, "")
        };
        let (value, unit) = value.take_numeric()?;

        let data = Data { name, value, unit };
        Some((data, v))
    }
}

pub type Attr<'a> = (&'a str, &'a str);

#[derive(Clone, Debug)]
pub struct Attrs<'a> {
    attrs: Vec<Attr<'a>>,
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
        attrs.sort_by(|(a, _), (b, _)| a.cmp(b));
        let mut prev: Option<&str> = None;
        let prev = &mut prev;
        attrs.retain(move |(key, _)| {
            // extend the lifetime of the str
            let key = unsafe { core::mem::transmute::<&str, &str>(*key) };
            if let Some(prev) = core::mem::replace(prev, Some(key)) {
                key != prev
            } else {
                true
            }
        });
        Some(Self { attrs })
    }
}

impl<'a> Deref for Attrs<'a> {
    type Target = [Attr<'a>];

    fn deref(&self) -> &Self::Target {
        &self.attrs
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Count,
    Gauge,
    Measure,
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::Count => "count",
            Kind::Gauge => "gauge",
            Kind::Measure => "measure",
        }
    }
}
