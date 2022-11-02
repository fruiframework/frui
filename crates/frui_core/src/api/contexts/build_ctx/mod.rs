use std::{
    any::Any,
    cell::{Ref, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{app::tree::WidgetNodeRef, prelude::InheritedWidget};

pub mod widget_state;
pub use widget_state::WidgetState;

/// Set by framework when accessing state mutably shouldn't register widget for
/// state updates (e.g. in unmount/mount methods).
pub(crate) static STATE_UPDATE_SUPRESSED: AtomicBool = AtomicBool::new(false);

// `BuildContext` is borrowed to make it so that closures don't take ownership
// of it, which would be inconvenient - user would have to clone `BuildContext`
// before every closure, since otherwise the context would move.
pub type BuildContext<'a, T> = &'a _BuildContext<'a, T>;

#[repr(transparent)]
pub struct _BuildContext<'a, T> {
    node: WidgetNodeRef,
    _p: PhantomData<&'a T>,
}

impl<'a, T> _BuildContext<'a, T> {
    pub fn state(&self) -> StateGuard<T::State>
    where
        T: WidgetState,
    {
        StateGuard {
            guard: Ref::map(self.node.borrow(), |node| node.state.deref()),
            _p: PhantomData,
        }
    }

    pub fn state_mut(&self) -> StateGuardMut<T::State>
    where
        T: WidgetState,
    {
        if !STATE_UPDATE_SUPRESSED.load(Ordering::SeqCst) {
            self.node.mark_dirty();
        }

        StateGuardMut {
            guard: RefMut::map(self.node.borrow_mut(), |node| node.state.deref_mut()),
            _p: PhantomData,
        }
    }

    /// This method registers the widget of this `BuildContext` as a dependency of
    /// the closest `InheritedWidget` ancestor of type `W` in the tree. It then
    /// returns the state of that inherited widget or `None` if inherited ancestor
    /// doesn't exist.
    pub fn depend_on_inherited_widget<W>(&self) -> Option<InheritedState<W::State>>
    where
        W: InheritedWidget + WidgetState,
    {
        // Register and get inherited widget of specified key.
        let node = self
            .node
            .depend_on_inherited_widget_of_key::<W::UniqueTypeId>()?;

        // Todo:
        //
        // 1. Get node above.
        // 2. Increase rc/borrow count.
        // 3. Get reference to the widget's state (can be done at once in step
        //    above).
        // 4. Return InheritedGuard<'a> with that `node`, `refcell` guard, and
        //    extracted reference. Possibly transmute.

        Some(InheritedState {
            node,
            _p: PhantomData,
        })
    }
}

pub struct StateGuard<'a, T: 'static> {
    guard: Ref<'a, dyn Any>,
    _p: PhantomData<&'a T>,
}

impl<'a, T: 'static> Deref for StateGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.deref().downcast_ref().unwrap()
    }
}

pub struct StateGuardMut<'a, T: 'static> {
    guard: RefMut<'a, dyn Any>,
    _p: PhantomData<&'a T>,
}

impl<'a, T: 'static> Deref for StateGuardMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.deref().downcast_ref().unwrap()
    }
}

impl<'a, T: 'static> std::ops::DerefMut for StateGuardMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.deref_mut().downcast_mut().unwrap()
    }
}

pub struct InheritedState<'a, T: 'static> {
    pub(crate) node: WidgetNodeRef,
    pub(crate) _p: PhantomData<&'a T>,
}

impl<'a, T: 'static> InheritedState<'a, T> {
    pub fn as_ref(&'a self) -> InheritedStateRef<'a, T> {
        InheritedStateRef {
            state: Ref::map(self.node.borrow(), |node| node.state.deref()),
            _p: PhantomData,
        }
    }

    pub fn as_mut(&'a mut self) -> InheritedStateRefMut<'a, T> {
        if !STATE_UPDATE_SUPRESSED.load(Ordering::SeqCst) {
            self.node.mark_dirty();
            self.node.mark_dependent_widgets_as_dirty();
        }

        InheritedStateRefMut {
            state: RefMut::map(self.node.borrow_mut(), |node| node.state.deref_mut()),
            _p: PhantomData,
        }
    }
}

pub struct InheritedStateRef<'a, T: 'static> {
    state: Ref<'a, dyn Any>,
    _p: PhantomData<T>,
}

impl<'a, T> Deref for InheritedStateRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.state.downcast_ref().unwrap()
    }
}

pub struct InheritedStateRefMut<'a, T: 'static> {
    state: RefMut<'a, dyn Any>,
    _p: PhantomData<T>,
}

impl<'a, T> Deref for InheritedStateRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.state.downcast_ref().unwrap()
    }
}

impl<'a, T> DerefMut for InheritedStateRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.state.downcast_mut().unwrap()
    }
}
