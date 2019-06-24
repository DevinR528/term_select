mod term_utils;
pub use term_utils::CursorUtils;

mod selector;
pub use selector::{Color, Highlighter, SelectHandler, Selector, Term};

pub mod prelude {
    pub use crate::selector::{Color, Highlighter, SelectHandler, Selector, Term};
}