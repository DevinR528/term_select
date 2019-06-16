#![feature(try_trait)]

use std::io;
use std::fmt::{ Display, Debug };
use std::io::Write;

use colored::Colorize;
use console::{ Term, TermFeatures, style, Emoji, Key };
pub use console::Color;

mod err;
use err::LBError;
use std::collections::HashMap;


pub struct Handler<'s> {
    item: &'s str,
    func: Box<dyn Fn()>,
}

impl<'s> Handler<'s> {
    pub fn new<F>(s: &'s str, f: F)  -> Self
        where F: Fn() + 'static,
    {
        Handler {
            item: s,
            func: Box::new(f),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Highlighter<'s> {
    Both(Color, &'s str),
    BgColor(Color),
    Character(&'s str),
}
/// Returns Selector for building arrow-able cli programmes.
///
/// opt_handle is a Vec of (str, Fn) the str is the name of the selectable item followed by
/// the function to run when that item is selected. Optional background color and/or
/// selection character.
///
/// # Examples
///
/// ```
/// use self::{
///     Selector, Color,
///     Highlighter, Handler
/// };
///
/// let hi = || println!("hello closure");
/// fn hello(p: u8) { println!("hello fn {}", p); }
///
/// let n = 10;
/// let bye = move || println!("move closure {}", n);
///
/// let v = vec![
///     Handler::new("hello", hi ),
///     Handler::new("hello fn", move || (hello)(n) ),
///     Handler::new("goodby capture", bye ),
/// ];
///
/// let sel = Selector::new(v, Highlighter::BgColor((Color::Green)));
///
/// ```
#[derive()]
pub struct Selector<'c> {
    item_handles: Vec<Handler<'c>>,
    items: Vec<&'c str>,
    sel_color: Option<Color>,
    sel_char: Option<&'c str>,
    index: usize,
}

impl<'c> Selector<'c> {
    /// Returns Selector for building arrow-able cli programmes.
    ///
    /// opt_handle is a Vec of (str, Fn) the str is the name of the selectable item followed by
    /// the function to run when that item is selected. Optional background color and/or
    /// selection character.
    ///
    /// # Examples
    ///
    /// ```
    /// use self::{
    ///     Selector, Color,
    ///     Highlighter, Handler
    /// };
    ///
    /// let hi = || println!("hello closure");
    /// let v = vec![ ("hello", hi) ];
    ///
    /// let sel = Selector::new(v, Highlighter::BgColor((Color::Green)));
    ///
    /// ```
    pub fn new(opt_handle: Vec<Handler<'c>>, high: Highlighter<'c>)
        -> Self
    {
        let mut i = Vec::new();

        for h in opt_handle.iter() {
            i.push(h.item.clone());
        }

        let (color, s_char) = match high {
            Highlighter::Both(c, s) => {
                (Some(c), Some(s))
            },
            Highlighter::BgColor(c) => {
                (Some(c), None)
            },
            Highlighter::Character(s) => {
                (None, Some(s))
            }
        };

        Selector {
            item_handles: opt_handle,
            items: i,
            sel_color: color,
            sel_char: s_char,
            index: 0,
        }
    }

    fn build_selected_str(&self, s: &str) -> String {
        let sel = self.sel_char
            .map(|c| String::from(format!("{} ", c))).or(Some("".into())).unwrap();

        if let Some(c) = self.sel_color {
            match c {
                Color::Green => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_green().to_string()
                },
                Color::Black => {
                    let res = format!("{}{}", sel, s);
                    res.white().on_black().to_string()
                },
                Color::Blue => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_blue().to_string()
                },
                Color::Yellow => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_yellow().to_string()
                },
                Color::Red => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_red().to_string()
                },
                Color::Magenta => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_magenta().to_string()
                },
                Color::Cyan => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_cyan().to_string()
                },
                Color::White => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_white().to_string()
                },
            }
        } else {
            format!("{}{}", sel, s)
        }
    }

    /// Drives the display of menues and selection.
    ///
    /// # Examples
    ///
    /// ```
    /// use self::{Selector, Highlighter, Color};
    /// use term_select::{Selector, Highlighter};
    ///
    /// let h = Highlighter::Character("$");
    /// let mut app = Selector::new(vec_of_select_items_and_handles, h);
    /// app.display_loop()?;
    /// ```
    pub fn display_loop(&mut self) -> Result<(), io::Error> {
        let mut run = true;
        let mut term = Term::stdout();

        while run {
            term.clear_screen()?;
            for (i, line) in self.iter().enumerate() {
                if i == self.index {
                    let c = self.sel_char
                        .map(|c| String::from(format!("{} ", c))).or(Some("".into()));

                    let color_line = self.build_selected_str(line);

                    term.write_line(&color_line)?;
                }

                term.write_line(&format!("{}", line))?;
            }

            match term.read_key()? {
                Key::ArrowDown => {
                    if self.index < self.items.len() {
                        self.index += 1;
                    }
                },
                Key::ArrowUp => {
                    if self.index != 0 {
                        self.index -= 1;
                    }
                },
                Key::Enter => {
                    term.clear_screen()?;
                    println!("WOWOWOWOWOWOw");
                },
                Key::Escape => {
                    std::process::exit(0);
                },
                _ => {}
            }

        }

        Ok(())
    }

    pub fn iter(&self) -> SelectIter {
        SelectIter {
            inner: &self.items,
            pos: 0,
        }
    }

}

#[derive(Debug, Clone)]
pub struct SelectIter<'i> {
    inner: &'i Vec<&'i str>,
    pos: usize,
}

impl<'i> Iterator for SelectIter<'i> {
    type Item= &'i str;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            None
        } else {
            self.pos += 1;
            if let Some(el) = self.inner.get(self.pos - 1) {
                Some(el)
            } else {
                None
            }
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct SelectIterMut<'i, V> {
//     inner: &'i mut Vec<V>,
//     pos: usize,
// }

// impl<'i, V> Iterator for SelectIterMut<'i, V> {
//     type Item= &'i mut V;
    
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.inner.is_empty() {
//             None
//         } else {
//             self.pos += 1;
//             self.inner.get_mut(self.pos - 1)
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let hi = || println!("hello");

        let v = vec![ Handler::new("hello", || println!("hello") ) ];
        let sel = Selector::new(v, Highlighter::BgColor(Color::Green));

        assert_eq!(sel.sel_color, Some(Color::Green));
    }

    #[test]
    fn test_selector() {
        let hi = || println!("hello");

        let v = vec![ Handler::new("hello", hi ) ];
        let sel = Selector::new(v, Highlighter::Character("*"));

        assert_eq!(sel.sel_char, Some("*"));
    }

    #[test]
    fn test_fns() {
        let hi = || println!("hello closure");

        fn hello(p: u8) {
            println!("hello fn {}", p);
        }

        let n = 10;
        let bye = move || println!("move closure {}", n);

        let v = vec![
            Handler::new("hello", hi ),
            Handler::new("hello", move || (hello)(n) ),
            Handler::new("hello", bye ),
        ];
        let sel = Selector::new(v, Highlighter::Character("*"));

        for i in 0..3 {
            (sel.item_handles[i].func)()
        }
    }

    #[test]
    fn test_iter() {
        let hi = || println!("hello");
        let bye = || println!("goodbye");

        let v = vec![ Handler::new("hello", hi), Handler::new("goodbye", bye) ];
        let sel = Selector::new(v, Highlighter::BgColor(Color::Green));

        let v_test = vec![ "hello", "goodbye" ];
        for (i, line) in sel.iter().enumerate() {
            println!("{} {}", line, v_test[i]);
            assert_eq!(line, v_test[i]);
        }
    }

    #[test]
    fn test_term_stuff() -> Result<(), io::Error> {
        let mut t = Term::stdout();


        let e = "\u{001B}[?25l".as_bytes();
        t.write(e)?;
        Ok(())
    }

    #[test]
    fn tests_display() -> Result<(), io::Error> {
        let hi = || println!("hello");
        let bye = || println!("goodbye");

        let v = vec![ Handler::new("hello", hi), Handler::new("goodbye", bye) ];
        let mut selector = Selector::new(v, Highlighter::BgColor(Color::Green));

        selector.display_loop()?;

        Ok(())
    }
}
