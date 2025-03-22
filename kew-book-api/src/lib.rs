#[cfg(not(target_family = "wasm"))]
#[path = "offline.rs"]
mod mode;

#[cfg(target_family = "wasm")]
#[path = "online.rs"]
mod mode;

pub mod figure;
pub mod input;
pub mod table;

pub use bach::{ext::*, rand, time::sleep};
pub use datafusion::prelude::{col, lit};
pub use figure::Figure;
pub use input::Input;
pub use kew::channel::{self as channel, new as channel, Behavior};
pub use kew_book_macros::*;
pub use mode::*;
pub use table::Table;

#[derive(Clone, Copy, Debug)]
pub struct Label {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

impl Label {
    pub const fn new(title: &'static str) -> Self {
        Self {
            id: "",
            title,
            description: "",
        }
    }

    pub const fn id(mut self, id: &'static str) -> Self {
        self.id = id;
        self
    }

    pub const fn description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }

    pub fn generate_id(&self) -> String {
        if self.id.is_empty() {
            self.title.replace([' ', '-'], "_").to_lowercase()
        } else {
            self.id.to_string()
        }
    }
}

impl AsRef<Label> for Label {
    fn as_ref(&self) -> &Label {
        self
    }
}

#[derive(Debug)]
pub struct Toc {
    pub id: Option<&'static str>,
    pub title: &'static str,
    pub sections: &'static [Toc],
}

#[macro_export]
macro_rules! toc {
    ($($tt:tt)*) => {
        $crate::Toc {
            id: None,
            title: "",
            sections: &$crate::toc_builder!([$($tt)*], []),
        }
    };
}

#[macro_export]
macro_rules! toc_builder {
    ([], [$($acc:tt)*]) => {
        [$($acc)*]
    };
    ([, $($tt:tt)*], [$($acc:tt)*]) => {
        $crate::toc_builder!([$($tt)*], [$($acc)*])
    };
    ([$title:literal as $id:ident $($tt:tt)*], [$($acc:tt)*]) => {
        $crate::toc_builder!([$($tt)*], [$($acc)* $crate::Toc {
            id: Some(stringify!($id)),
            title: $title,
            sections: &[],
        },])
    };
    ([$title:literal $($tt:tt)*], [$($acc:tt)*]) => {
        $crate::toc_builder!([$($tt)*], [$($acc)* $crate::Toc {
            id: None,
            title: $title,
            sections: &[],
        },])
    };
    ([($title:literal as $id:ident, [$($sub:tt)*]) $($tt:tt)*], [$($acc:tt)*]) => {
        $crate::toc_builder!([$($tt)*], [$($acc)* $crate::Toc {
            id: Some(stringify!($id)),
            title: $title,
            sections: &$crate::toc_builder!([$($sub)*], []),
        },])
    };
    ([($title:literal, [$($sub:tt)*]) $($tt:tt)*], [$($acc:tt)*]) => {
        $crate::toc_builder!([$($tt)*], [$($acc)* $crate::Toc {
            id: None,
            title: $title,
            sections: &$crate::toc_builder!([$($sub)*], []),
        },])
    };
}
