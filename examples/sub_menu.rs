use std::io;
use term_select::{Term, Color, AppBuilder};

fn main() -> io::Result<()> {
    let term = Term::stdout();

    AppBuilder::new()
        .select_color(Color::Green)
        .item_name("Hit enter to tell us your name")
            .action(|t: Term, _res: Option<String>| -> io::Result<Option<String>> {
                t.clear_screen()?;
                t.show_cursor()?;
                t.write_str("what's your name: ")?;
                let name = t.read_line().ok();
                Ok(name)
            })
            .sub_menu()
                .select_color(Color::Red)
                .item_name("Hit enter to print")
                    .action(|t: Term, res: Option<String>| -> io::Result<Option<String>> {
                        if let Some(name) = res {
                            t.clear_screen()?;
                            t.write_str(&format!("Hello {}", name))?;
                            t.write_str("\n\nHit enter to continue")?;
                            t.read_line()?;
                        }
                        Ok(None)
                    })
                .push_sub_menu()
        .push_menu_item()
    .display(&term, None)
}
