use gtk::prelude::*;

use crate::state::State;
use std::{cell::RefCell, rc::Rc};

pub struct App {
    ui: vte::Terminal,
    state: Rc<RefCell<State>>,
}

impl App {
    pub fn new() -> Self {
        let vte = vte::Terminal::new();
        Self {
            ui: vte,
            state: Rc::new(RefCell::new(State::new())),
        }
    }
}
