
mod selector;
use selector::{Color, SelectAction, Selector, Term, FuncBox};

pub mod prelude {
    pub use super::{ActionBuilder, AppBuilder};
    pub use crate::selector::{Color, Term};
}

pub struct ActionBuilder<'a, T> {
    sub: Option<&'a Selector<'a, T>>,
    name: &'a str,
    func: Option<FuncBox<'a, T>>,
    app: &'a mut AppBuilder<'a, T>
}
impl<'a, T: Clone + Copy + 'static> ActionBuilder<'a, T> {

    fn new(name: &'a str, app: &'a mut AppBuilder<'a, T>) -> Self {
        ActionBuilder { sub: None, name, func: None, app }
    }

    pub fn action<F>(&mut self, f: F) -> &mut Self 
    where
        F: Fn(Term, Option<T>) -> std::io::Result<Option<T>> + 'static,
    {
        self.func = Some(Box::new(f));
        self
    }

    // TODO why is Selector taken by ref??? change to by value if possible
    pub fn sub_menu(&mut self, menu: &'a Selector<'a, T>) -> &mut Self {
        self.sub = Some(menu);
        self
    }

    pub fn push_menu_item(&'a mut self) -> &'a mut AppBuilder<'a, T> {
        assert!(self.func.is_some());
        let sel_action = SelectAction::new(self.name, self.func.take().unwrap(), self.sub);
        self.app.menu.item_handles.push(sel_action);
        self.app
    }
}

pub struct AppBuilder<'s, T> {
    menu: Selector<'s, T>,
}
impl<'s, T: Clone + Copy + 'static> AppBuilder<'s, T> {

    pub fn new() -> Self {
        AppBuilder { menu: Selector::default(), }
    }

    pub fn item_name(&'s mut self, name: &'s str) -> ActionBuilder<'s, T> {
        // call select_color or select_char first
        assert!(self.menu.sel_char.is_some() || self.menu.sel_color.is_some());
        ActionBuilder::new(name, self)
    }

    pub fn select_color(&mut self, color: Color) -> &mut Self {
        self.menu.sel_color = Some(color);
        self
    }

    pub fn select_char(&mut self, select_char: &'s str) -> &mut Self {
        self.menu.sel_char = Some(select_char);
        self
    }

    pub fn menu_item(&mut self) -> &mut Self {
        // we need one or the other in order to show selected menu item
        assert!(self.menu.sel_char.is_some() || self.menu.sel_color.is_some());
        self
    }

    pub fn display(&mut self, term: &Term, res: Option<T>) -> Result<(), std::io::Error> {
        // we need one or the other in order to show selected menu item
        assert!(self.menu.sel_char.is_some() || self.menu.sel_color.is_some());
        self.menu.display_loop(term, res)
    }
}
