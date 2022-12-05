//! This implementation of KeyboardEventListeners

use std::cell::RefCell;

use druid_shell::KeyEvent;
use slotmap::SlotMap;

slotmap::new_key_type! { pub struct CallbackKey; }

#[derive(Default)]
pub struct KeyboardEventListeners {
    callbacks: SlotMap<CallbackKey, *const dyn Fn(KeyEvent)>,
}

impl KeyboardEventListeners {
    /// Registers a callback which will be called when a keyboard event is received.
    ///
    /// ## Safety:
    ///
    /// Value `callback` points to must live until [`unregister`] called.
    ///
    /// In other words, you need to remove this callback, by calling [`remove`]
    /// with [`CallbackKey`] returned from this function, before that `callback` is
    /// going to be dropped.
    pub unsafe fn register<'a>(&mut self, callback: *const (dyn Fn(KeyEvent) + 'a)) -> CallbackKey {
        self.callbacks.insert(std::mem::transmute(callback))
    }

    pub fn unregister(&mut self, key: CallbackKey) {
        self.callbacks.remove(key);
    }

    pub fn len(&self) -> usize {
        self.callbacks.len()
    }

    pub(crate) fn iter<'a>(&'a self) -> impl Iterator<Item = &'a dyn Fn(KeyEvent)> {
        self.callbacks.iter().map(|(_, f)| {
            // Safety: `callback` is valid as ensured by registrars to `KeyboardEventListeners`.
            unsafe { &**f }
        })
    }
}

thread_local! {
    /// Todo: Optimize this with something like SlotMap<(&f, PreviousNode, NextNode)> ?
    /// Basically linked tree but with advantages of slot map...?
    pub static KEYBOARD_EVENT_LISTENERS: RefCell<KeyboardEventListeners>  = Default::default();
}
