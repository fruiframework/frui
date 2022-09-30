//! This implementation of KeyboardEventListeners

use std::cell::RefCell;

use druid_shell::KeyEvent;

#[derive(Debug, Clone, Copy)]
pub struct CallbackKey(usize);

pub struct KeyboardEventListeners {
    callbacks: Vec<*const dyn Fn(KeyEvent)>,
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
        self.callbacks.push(std::mem::transmute(callback));
        // Todo: Fix this. After unregister-ing one callback, calling unregister again will
        // cause a panic. Use SlotMap as a fix.
        CallbackKey(self.callbacks.len() - 1)
    }

    pub fn unregister(&mut self, key: &CallbackKey) {
        // Todo: Optimize this.
        self.callbacks.remove(key.0);
    }

    pub fn len(&self) -> usize {
        self.callbacks.len()
    }

    pub(crate) fn iter<'a>(&'a self) -> CallbackIter<'a> {
        CallbackIter {
            idx: 0,
            callbacks: &self.callbacks,
        }
    }
}

pub struct CallbackIter<'a> {
    idx: usize,
    callbacks: &'a Vec<*const dyn Fn(KeyEvent)>,
}

impl<'a> Iterator for CallbackIter<'a> {
    type Item = &'a dyn Fn(KeyEvent);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.callbacks.len() {
            return None;
        }

        self.idx += 1;

        // Safety: `callback` is valid as ensured by registrars to `KeyboardEventListeners`.
        unsafe { self.callbacks[self.idx - 1].as_ref() }
    }
}

thread_local! {
    /// Todo: Optimize this with something like SlotMap<(&f, PreviousNode, NextNode)> ?
    /// Basically linked tree but with advantages of slot map...?
    pub static KEYBOARD_EVENT_LISTENERS: RefCell<KeyboardEventListeners>  = RefCell::new(KeyboardEventListeners {
        callbacks: Vec::with_capacity(100),
    });
}
