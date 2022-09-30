use std::cell::{RefCell, RefMut};

use druid_shell::piet::PietText;

pub mod listeners;
pub mod runner;
pub mod tree;

pub struct TextFactory(RefCell<Option<PietText>>);

impl TextFactory {
    pub(crate) fn new() -> Self {
        TextFactory(RefCell::new(None))
    }

    pub(crate) fn set(&self, f: PietText) {
        *self.0.borrow_mut() = Some(f);
    }

    pub fn get(&self) -> RefMut<PietText> {
        RefMut::map(self.0.borrow_mut(), |b| {
            b.as_mut().expect("TextFactory was not set")
        })
    }
}

thread_local! {
    pub static TEXT_FACTORY: TextFactory = TextFactory::new();
}
