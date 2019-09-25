use std::fmt::Debug;
use std::io;

use colored::Colorize;
pub use console::{Color, Key, Term};

pub type FuncBox<'s, T> = Box<dyn Fn(Term, Option<T>) -> io::Result<Option<T>> + 'static>;

pub struct SelectAction<'s, T> {
    pub(crate) item: &'s str,
    pub(crate) sub_menu: Option<Selector<'s, T>>,
    func: FuncBox<'s, T>,
}

impl<'s, T> SelectAction<'s, T>
where
    T: Clone
{
    pub fn new(
        item: &'s str,
        func: FuncBox<'s, T>,
        sub_menu: Option<Selector<'s, T>>
    ) -> Self {
        SelectAction { item, sub_menu, func, }
    }
}

/// Highlight applied to selected menu item.
#[derive(Debug, PartialEq)]
pub enum Highlighter<'s> {
    Both(Color, &'s str),
    BgColor(Color),
    Character(&'s str),
}

/// Selector for building arrow-able cli programmes.
///
/// Selector consists of a Vec of (str, Fn) the str is the name of the selectable item followed by
/// the function to run when that item is selected. Optional background color and/or
/// selection character.
pub struct Selector<'c, T> {
    pub(crate) item_handles: Vec<SelectAction<'c, T>>,
    pub(crate) items: Vec<&'c str>,
    pub(crate) sel_color: Option<Color>,
    pub(crate) sel_char: Option<&'c str>,
}

impl<'c, T> Default for Selector<'c, T> {
    fn default() -> Self {
        Self {
            item_handles: vec![],
            items: vec![],
            sel_color: None,
            sel_char: None,
        }
    }
}

/// Selector for building arrow-able cli programmes.
impl<'c, T> Selector<'c, T>
where
    T: Clone,
{
    /// Another way to create a Selector is to construct each instance directly.
    ///
    /// opt_handle is a Vec of (str, Fn) the str is the name of the selectable item followed by
    /// the function to run when that item is selected. Optional background color and/or
    /// selection character.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Error;
    /// use term_select::{
    ///     Selector, Color, Term,
    ///     Highlighter, SelectHandler
    /// };
    ///
    /// let hi = |t: Term, r: Option<String>| -> Result<Option<String>, Error> {
    ///     t.write_str("hello {}", r)?;
    ///     t.read_line()?;
    ///     Ok(None)
    /// };
    ///
    /// let v = vec![ SelectHandler::new("hello", hi, None) ];
    ///
    /// let sel = Selector::new(v, Highlighter::BgColor((Color::Green)));
    /// 
    /// let term = Term::stdio;
    /// sel.display_loop(term, Some("me".to_string()));
    /// // once selected will print `hello me`.
    /// ```
    pub fn new(opt_handle: Vec<SelectAction<'c, T>>, high: Highlighter<'c>) -> Self {
        let mut i = Vec::new();

        for h in opt_handle.iter() {
            i.push(h.item);
        }

        let (color, s_char) = match high {
            Highlighter::Both(c, s) => (Some(c), Some(s)),
            Highlighter::BgColor(c) => (Some(c), None),
            Highlighter::Character(s) => (None, Some(s)),
        };

        Selector {
            item_handles: opt_handle,
            items: i,
            sel_color: color,
            sel_char: s_char,
        }
    }
    fn build_selected_str(&self, s: &str) -> String {
        let sel = self
            .sel_char
            .map(|c| format!("{} ", c))
            .or_else(|| Some("".into()))
            .unwrap();

        if let Some(c) = self.sel_color {
            match c {
                Color::Green => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_green().to_string()
                }
                Color::Black => {
                    let res = format!("{}{}", sel, s);
                    res.white().on_black().to_string()
                }
                Color::Blue => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_blue().to_string()
                }
                Color::Yellow => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_yellow().to_string()
                }
                Color::Red => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_red().to_string()
                }
                Color::Magenta => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_magenta().to_string()
                }
                Color::Cyan => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_cyan().to_string()
                }
                Color::White => {
                    let res = format!("{}{}", sel, s);
                    res.black().on_white().to_string()
                }
            }
        } else {
            format!("{}{}", sel, s)
        }
    }

    /// Drives the display of menus and selection.
    pub fn display_loop(&self, term: &Term, result: Option<T>) -> Result<(), io::Error> {
        let mut index = 0;
        loop {
            // TODO until term.hide_cursor() works
            let esc = "\u{001B}";
            term.write_str(&format!("{}[?25l", esc))?;
            // term.hide_cursor()?;
            term.clear_screen()?;
            for (i, line) in self.iter().enumerate() {
                if i == index {
                    // build color and selected char into string
                    let color_line = self.build_selected_str(line);

                    term.write_line(&color_line)?;
                } else {
                    term.write_line(line)?;
                }
            }
            term.write_str("\r\nEsc to quit Left arrow to go back one menu.")?;

            match term.read_key()? {
                Key::ArrowDown => {
                    if index < self.items.len() - 1 {
                        index += 1;
                    } else {
                        index = 0;
                    }
                }
                Key::ArrowUp => {
                    if index != 0 {
                        index -= 1;
                    } else {
                        index = self.items.len() - 1;
                    }
                }
                Key::Enter => {
                    let handle = &self.item_handles[index];
                    // calls the function provided for the selection
                    let res = (*handle.func)(term.clone(), result.clone())?;

                    if let Some(sub) = &self.item_handles[index].sub_menu {
                        sub.display_loop(term, res)?;
                    }
                }
                Key::ArrowLeft => {
                    // this will allow back button
                    // how to check if we are at top level
                    term.clear_screen()?;
                    return Ok(());
                }
                Key::Escape => {
                    term.show_cursor()?;
                    std::process::exit(0);
                }
                _ => {}
            }
        }
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
    type Item = &'i str;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        self.inner.get(self.pos - 1).copied()
    }
}

impl<'d, T> Debug for Selector<'d, T>
where
    T: Clone,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for handle in self.item_handles.iter() {
            let handle: &SelectAction<T> = handle;
            writeln!(fmt, "{}  {:#?}", handle.item, handle.sub_menu)?;
        }
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_stuff() -> Result<(), io::Error> {
        let t = Term::buffered_stdout();

        t.hide_cursor()?;

        Ok(t.show_cursor()?)
    }
}
