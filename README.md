# Term Select
Easily build arrow based selectable menu's in the terminal.

[![Build Status](https://travis-ci.com/DevinR528/term_select.svg?branch=master)](https://travis-ci.com/DevinR528/trie-try-again)
[![Latest Version](https://img.shields.io/crates/v/term-select.svg)](https://crates.io/crates/toml)

# Description
Term Select is used to build up a menu. Each menu item when selected runs
the user provided closure. The closure has access to the `Terminal` and if it
is a sub menu an `Option<T>` is passed from the previous menu. Each closure returns
a `Result<Option<T>>` passed to the next as `Option<T>`.

# Clone
`git clone https://github.com/DevinR528/term_select'

# Examples
```rust
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

```
