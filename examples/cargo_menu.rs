use std::io;

use term_select::{Color, Highlighter, SelectHandler, Selector, Term};

fn main() -> io::Result<()> {
    let hello = |t: Term, _res: Option<()>| -> io::Result<Option<()>> {
        t.clear_screen()?;
        t.write_str("hello")?;
        t.read_line()?;
        Ok(None)
    };

    let goodbye = |t: Term, _res: Option<()>| -> io::Result<Option<()>> {
        t.clear_screen()?;
        t.write_str("goodbye")?;
        t.read_line()?;
        Ok(None)
    };

    let items = vec![
        SelectHandler::new("hello", hello, None),
        SelectHandler::new("goodbye", goodbye, None),
    ];

    let selctor = Selector::new(items, Highlighter::BgColor(Color::Green));

    let term = Term::stdout();
    selctor.display_loop(&term, None)?;

    Ok(())
}
