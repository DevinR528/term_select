
mod selector;
use selector::{Color, Highlighter, SelectAction, Selector, Term, FuncBox};

pub mod prelude {
    pub use crate::selector::{Color, Highlighter, SelectAction, Selector, Term};
}

pub struct ActionBuilder<'a, T> {
    action: Option<SelectAction<'a, T>>,
    name: &'a str,
    func: Option<FuncBox<'a, T>>
}
impl<'a, T: Clone + Copy + 'static> ActionBuilder<'a, T> {

    pub fn new(name: &'a str) -> Self {
        ActionBuilder { action: None, name, func: None }
    }

    pub fn action(&mut self, func: FuncBox<'a, T>) -> &mut Self {
        self.func = Some(func);
        self
    }

    pub fn finish(self) -> Self {
        assert!(self.func.is_some());
        self
    }

    // TODO why is Selector taken by ref??? change to by value if possible
    pub fn sub_menu(&mut self, menu: &'a Selector<'a, T>) -> &mut Self {
        let f = self.func.take().expect("must call this last");
        self.action = Some(SelectAction::new(self.name, f, Some(menu)));
        self
    }
}

pub struct AppBuilder<'s, T> {
    app: Selector<'s, T>,
}
impl<'s, T: Clone + Copy + 'static> AppBuilder<'s, T> {

    pub fn new() -> Self {
        Self { app: Selector::default(), }
    }

    pub fn item_name(&mut self, name: &'s str) -> ActionBuilder<'s, T> {
        ActionBuilder::new(name)
    }

    pub fn select_color(&mut self, color: Color) -> &mut Self {
        self.app.sel_color = Some(color);
        self
    }

    pub fn select_char(&mut self, select_char: &'s str) -> &mut Self {
        self.app.sel_char = Some(select_char);
        self
    }

    // TODO rename
    pub fn finish(self) -> Selector<'s, T> {
        // we need one or the other in order to show selected menu item
        assert!(self.app.sel_char.is_some() || self.app.sel_color.is_some());

        self.app
    }
}
