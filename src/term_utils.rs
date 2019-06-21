use std::io;

pub trait CursorUtils {
    /// Hides the cursor
    fn cursor_hide(&self) -> Result<(), io::Error>;
    /// Shows the cursor
    fn cursor_show(&self) -> Result<(), io::Error>;
}
