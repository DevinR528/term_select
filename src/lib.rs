//! Term Select is used to build up a menu. Each menu item when selected runs
//! the user provided closure. The closure has access to the `Terminal` and if it
//! is a sub menu an `Option<T>` is passed from the previous menu. Each closure returns
//! a `Result<Option<T>>` passed to the next as `Option<T>`.
//!
//! An "App" is built from [`AppBuilder`] given a background [`Color`]
//! [`AppBuilder::select_color`] and/or selecting character [`AppBuilder::select_char`]
//! the menu item title and an action: `Fn(Term, Option<T>) -> Result<Option<T>>`.
//! Each [`Selector`] menu item can also have a sub menu built from [`SubBuilder`].
//! Each action has access to the [`Term`] and the previous menu items result as `Option<T>`.
//! 
//! [`AppBuilder`]: struct.AppBuilder.html
//! [`Selector`]: struct.Selector.html
//! [`ActionBuilder`]: struct.ActionBuilder.html
//! [`SubBuilder`]: struct.SubBuilder.html
//! [`Term`]: https://docs.rs/console/0.9.0/console/struct.Term.html
//! [`Color`]: https://docs.rs/console/0.9.0/console/enum.Color.html
//! 
//! ## Example
//! 
//! ```rust
//! use std::io;
//! use term_select::{Term, Color, AppBuilder};
//! 
//! fn main() -> io::Result<()> {
//!     let term = Term::stdout();
//!
//!     AppBuilder::new()
//!         .select_color(Color::Green)
//!         .item_name("Hit enter to tell us your name")
//!             .action(|t: Term, _res: Option<String>| -> io::Result<Option<String>> {
//!                 t.clear_screen()?;
//!                 t.show_cursor()?;
//!                 t.write_str("what's your name: ")?;
//!                 let name = t.read_line().ok();
//!                 Ok(name)
//!             })
//!             .sub_menu()
//!                 .select_color(Color::Red)
//!                 .item_name("Hit enter to print")
//!                     .action(|t: Term, res: Option<String>| -> io::Result<Option<String>> {
//!                         if let Some(name) = res {
//!                             t.clear_screen()?;
//!                             t.write_str(&format!("Hello {}", name))?;
//!                             t.write_str("\n\nHit enter to continue")?;
//!                             t.read_line()?;
//!                         }
//!                         Ok(None)
//!                     })
//!                 .push_sub_menu()
//!         .push_menu_item()
//!     .display(&term, None)
//! }
//! ```

mod selector;
use selector::FuncBox;

pub use crate::selector::{Color, Term, Selector, SelectAction};

/// Builder for the sub menu items action. This closure is passed a
/// 'Term' and the previous menu items result from the action
/// `Option<T>`. 
pub struct SubActionBuilder<'a, T> {
    sub: Option<Selector<'a, T>>,
    name: &'a str,
    func: Option<FuncBox<'a, T>>,
    prev_builder: &'a mut SubBuilder<'a, T>,
}
impl<'a, T: Clone + 'static> SubActionBuilder<'a, T> {

    /// The Option<T> is passed in via the [`AppBuilder::display`] and 
    /// when a menu items action returns a Result it is passed
    /// to sub menu actions.
    pub fn action<F>(&mut self, f: F) -> &mut SubActionBuilder<'a, T> 
    where
        F: Fn(Term, Option<T>) -> std::io::Result<Option<T>> + 'static,
    {
        self.func = Some(Box::new(f));
        self
    }

    /// Sub menu for your sub menu anyone!
    pub fn sub_menu(&'a mut self) -> SubBuilder<'a, T> {
        SubBuilder::new(self.prev_builder.action)
    }

    /// Adds the sub menu to the `Selector`.
    pub fn push_sub_menu(&'a mut self) -> &'a mut ActionBuilder<'a, T> {
        assert!(self.func.is_some());
        let sel_action = SelectAction::new(self.name, self.func.take().unwrap(), None);
        
        if let Some(add_to_sub) = &mut self.sub {
            add_to_sub.item_handles.push(sel_action);
            add_to_sub.items.push(self.name);
            add_to_sub.sel_char = self.prev_builder.sel_char;
            add_to_sub.sel_color = self.prev_builder.color;
        }

        self.prev_builder.action.sub = self.sub.take();
        self.prev_builder.action
    }
}

pub struct SubBuilder<'s, T> {
    #[allow(dead_code)]
    menu: Option<Selector<'s, T>>,
    color: Option<Color>,
    sel_char: Option<&'s str>,
    action: &'s mut ActionBuilder<'s, T>
}
impl<'s, T: Clone + 'static> SubBuilder<'s, T> {

    fn new(action: &'s mut ActionBuilder<'s, T>) -> SubBuilder<'s, T> {
        SubBuilder { menu: None, color: None, sel_char: None, action, }
    }
    /// Sets the title of the sub menu item. Returns `ActionBuilder` 
    /// to build action closure.
    pub fn item_name(&'s mut self, name: &'s str) -> SubActionBuilder<'s, T> {
        // we need one or the other in order to show selected menu item
        assert!(self.sel_char.is_some() || self.color.is_some());
        let menu = Selector::default();
        SubActionBuilder { sub: Some(menu), name, func: None, prev_builder: self }
    }
    /// Sets the sub menu's highlight color.
    pub fn select_color(&mut self, color: Color) -> &mut SubBuilder<'s, T> {
        self.color = Some(color);
        self
    }
    /// Sets the sub menu's highlight character.
    pub fn select_char(&mut self, select_char: &'s str) -> &mut SubBuilder<'s, T> {
        self.sel_char = Some(select_char);
        self
    }

    /// Marker to separate sub menu items visually. 
    pub fn new_sub_menu_item(&mut self) -> &mut SubBuilder<'s, T> {
        self
    }
}

/// Builder for the items action. This closure is passed a
/// 'Term' and the previous menu items result from the action
/// `Option<T>`. 
pub struct ActionBuilder<'a, T> {
    sub: Option<Selector<'a, T>>,
    name: &'a str,
    func: Option<FuncBox<'a, T>>,
    app: &'a mut AppBuilder<'a, T>
}
impl<'a, T: Clone + 'static> ActionBuilder<'a, T> {

    fn new(name: &'a str, app: &'a mut AppBuilder<'a, T>) -> ActionBuilder<'a, T> {
        ActionBuilder { sub: None, name, func: None, app }
    }

    /// The Option<T> is passed in via the AppBuilder::display() and 
    /// when a menu item's action returns a Result it is always passed
    /// to that menu items submenu actions, if there is one.
    pub fn action<F>(&mut self, f: F) -> &mut ActionBuilder<'a, T> 
    where
        F: Fn(Term, Option<T>) -> std::io::Result<Option<T>> + 'static,
    {
        self.func = Some(Box::new(f));
        self
    }

    /// Adds a sub_menu to the current menu item
    pub fn sub_menu(&'a mut self) -> SubBuilder<'a, T> {
        SubBuilder::new(self)
    }

    /// Adds the sub_menu to the current menu item.
    pub fn push_menu_item(&'a mut self) -> &'a mut AppBuilder<'a, T> {
        assert!(self.func.is_some());
        let sel_action = SelectAction::new(self.name, self.func.take().unwrap(), self.sub.take());
        self.app.menu.item_handles.push(sel_action);
        self.app.menu.items.push(self.name);
        self.app
    }
}

/// Builds a selectable menu.
pub struct AppBuilder<'s, T> {
    menu: Selector<'s, T>,
    color: Option<Color>,
    sel_char: Option<&'s str>,
}
impl<'s, T> Default for AppBuilder<'s, T> {
    fn default() -> Self {
        Self { menu: Selector::default(), color: None, sel_char: None }
    }
}
impl<'s, T: Clone + 'static> AppBuilder<'s, T> {

    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the title of the menu item. Returns `ActionBuilder` 
    /// to build action closure.
    pub fn item_name(&'s mut self, name: &'s str) -> ActionBuilder<'s, T> {
        ActionBuilder::new(name, self)
    }

    /// Sets the main menu's highlight color
    pub fn select_color(&mut self, color: Color) -> &mut AppBuilder<'s, T> {
        self.color = Some(color);
        self
    }
    /// Sets the main menu's highlight character.
    pub fn select_char(&mut self, select_char: &'s str) -> &mut AppBuilder<'s, T> {
        self.sel_char = Some(select_char);
        self
    }
    /// Marker to separate menu items visually. 
    pub fn new_menu_item(&mut self) -> &mut AppBuilder<'s, T> {
        self
    }
    /// Starts the display loop, this needs to be the last called.
    pub fn display(&mut self, term: &Term, res: Option<T>) -> Result<(), std::io::Error> {
        // we need one or the other in order to show selected menu item
        assert!(self.sel_char.is_some() || self.color.is_some());
        self.menu.sel_char = self.sel_char;
        self.menu.sel_color = self.color;
        self.menu.display_loop(term, res)
    }
}

impl<'d, T: Clone + 'static> std::fmt::Debug for AppBuilder<'d, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.menu.item_handles[0].sub_menu)
    }
}
