use std::io;

use term_select::{Term, Color, AppBuilder};

fn main() -> io::Result<()> {
    let term = Term::stdout();

    AppBuilder::new()
        .new_menu_item()
        .select_color(Color::Green)
        .item_name("hello")
            .action(|t: Term, _res: Option<()>| -> io::Result<Option<()>> {
                t.clear_screen()?;
                t.write_str("hello")?;
                t.read_line()?;
                Ok(None)
            })
            .push_menu_item()

        .new_menu_item()
        .select_color(Color::Green)
        .item_name("goodbye")
            .action(|t: Term, _res: Option<()>| -> io::Result<Option<()>> {
                t.clear_screen()?;
                t.write_str("goodbye")?;
                t.read_line()?;
                Ok(None)
            })
            .push_menu_item()
            
        .display(&term, None)
}
