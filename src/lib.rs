
mod selector;
use selector::{SelectAction, Selector, FuncBox};

pub use crate::selector::{Color, Term};

pub struct SubActionBuilder<'a, T> {
    sub: Option<Selector<'a, T>>,
    name: &'a str,
    func: Option<FuncBox<'a, T>>,
    prev_builder: &'a mut SubBuilder<'a, T>,
}
impl<'a, T: Clone + 'static> SubActionBuilder<'a, T> {

    ///
    /// The Option<T> is passed in via the AppBuilder::display() and 
    /// when a menu item action returns a Result it is always passed
    /// to that menu items submenu actions.
    pub fn action<F>(&mut self, f: F) -> &mut Self 
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

    /// Adds the sub menu to the ``
    pub fn push_sub_menu(&'a mut self) -> &'a mut ActionBuilder<'a, T> {
        assert!(self.func.is_some());
        let sel_action = SelectAction::new(self.name, self.func.take().unwrap(), self.sub.take());
        
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
    menu: Option<Selector<'s, T>>,
    color: Option<Color>,
    sel_char: Option<&'s str>,
    action: &'s mut ActionBuilder<'s, T>
}
impl<'s, T: Clone + 'static> SubBuilder<'s, T> {

    fn new(action: &'s mut ActionBuilder<'s, T>) -> Self {
        SubBuilder { menu: None, color: None, sel_char: None, action, }
    }

    pub fn item_name(&'s mut self, name: &'s str) -> SubActionBuilder<'s, T> {
        let menu = Selector::default();
        SubActionBuilder { sub: Some(menu), name, func: None, prev_builder: self }
    }

    pub fn select_color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn select_char(&mut self, select_char: &'s str) -> &mut Self {
        self.sel_char = Some(select_char);
        self
    }

    
}

pub struct ActionBuilder<'a, T> {
    sub: Option<Selector<'a, T>>,
    name: &'a str,
    func: Option<FuncBox<'a, T>>,
    app: &'a mut AppBuilder<'a, T>
}
impl<'a, T: Clone + 'static> ActionBuilder<'a, T> {

    fn new(name: &'a str, app: &'a mut AppBuilder<'a, T>) -> Self {
        ActionBuilder { sub: None, name, func: None, app }
    }

    ///
    /// The Option<T> is passed in via the AppBuilder::display() and 
    /// when a menu item's action returns a Result it is always passed
    /// to that menu items submenu actions, if there is one.
    pub fn action<F>(&mut self, f: F) -> &mut Self 
    where
        F: Fn(Term, Option<T>) -> std::io::Result<Option<T>> + 'static,
    {
        self.func = Some(Box::new(f));
        self
    }

    /// Adds a sub_menu to the current menu item
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// ```
    pub fn sub_menu(&'a mut self) -> SubBuilder<'a, T> {
        SubBuilder::new(self)
    }

    /// Starts building a sub_menu for the current menu item.
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// ```
    pub fn push_menu_item(&'a mut self) -> &'a mut AppBuilder<'a, T> {
        assert!(self.func.is_some());
        let sel_action = SelectAction::new(self.name, self.func.take().unwrap(), self.sub.take());
        self.app.menu.item_handles.push(sel_action);
        self.app.menu.items.push(self.name);
        self.app
    }
}

pub struct AppBuilder<'s, T> {
    menu: Selector<'s, T>,
    color: Option<Color>,
    sel_char: Option<&'s str>,
}
impl<'s, T: Clone + 'static> AppBuilder<'s, T> {

    pub fn new() -> Self {
        AppBuilder { menu: Selector::default(), color: None, sel_char: None }
    }

    pub fn item_name(&'s mut self, name: &'s str) -> ActionBuilder<'s, T> {
        ActionBuilder::new(name, self)
    }

    /// Sets the main menu's highlight color
    pub fn select_color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }
    /// Sets the main menu's highlight character.
    pub fn select_char(&mut self, select_char: &'s str) -> &mut Self {
        self.sel_char = Some(select_char);
        self
    }
    /// A marker to separate menu items visually. 
    /// 
    pub fn new_menu_item(&mut self) -> &mut Self {
        self
    }
    /// Starts the display loop, this needs to be the last called.
    /// 
    /// 
    pub fn display(&mut self, term: &Term, res: Option<T>) -> Result<(), std::io::Error> {
        // we need one or the other in order to show selected menu item
        assert!(self.sel_char.is_some() || self.color.is_some());
        self.menu.sel_char = self.sel_char;
        self.menu.sel_color = self.color;
        self.menu.display_loop(term, res)
    }
}
