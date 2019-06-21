use std::io;

/// Trait to extend console::Term to include a cursor show/hide method
///
/// # Examples
///
/// ```
/// use term_select::Term;
/// use crate::term_select::CursorUtils;
///
/// let mut t = Term::stdout();
/// // cursor is now gone
/// t.cursor_hide();
/// // make visible again
/// t.cursor_show();
/// ```
pub trait CursorUtils {
    /// Hides the cursor
    fn cursor_hide(&self) -> Result<(), io::Error>;
    /// Shows the cursor
    fn cursor_show(&self) -> Result<(), io::Error>;
}
