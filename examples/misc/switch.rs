#![allow(unused)]

use std::ops::Deref;

use frui::prelude::InheritedState;

#[derive(Default)]
pub struct Switch(bool);

impl Switch {
    pub fn switch(&mut self) {
        self.0 = !self.0;
    }

    pub fn value(&self) -> bool {
        self.0
    }
}

pub struct SwitchGuard<'a> {
    pub state: InheritedState<'a, bool>,
    pub value: bool,
}

impl<'a> SwitchGuard<'a> {
    pub fn new(state: InheritedState<'a, bool>) -> Self {
        let value = state.as_ref().clone();
        Self { state, value }
    }

    pub fn switch(&'a mut self) {
        let v = &mut *self.state.as_mut();
        *v = !*v;
        self.value = !self.value;
    }
}

impl<'a> Deref for SwitchGuard<'a> {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
