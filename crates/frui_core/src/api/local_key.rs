use std::any::{Any, TypeId};

use crate::prelude::{BuildCx, ViewWidget, Widget};

/// LocalKey is a widget that allows you to annotate the key for a `child`
/// widget.
#[derive(ViewWidget)]
pub struct LocalKey<K: 'static + PartialEq, W: Widget> {
    pub key: K,
    pub child: W,
}

impl<K: 'static + PartialEq, W: Widget> LocalKey<K, W> {
    pub fn new(key: K, child: W) -> Self {
        Self { key, child }
    }
}

impl<K: 'static + PartialEq, W: Widget> ViewWidget for LocalKey<K, W> {
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }
}

pub struct LocalKeyAny<'a> {
    key: &'a dyn PartialEqAny,
}

impl PartialEq for LocalKeyAny<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(other.key)
    }
}

trait PartialEqAny: Any {
    fn type_id(&self) -> TypeId;
    fn eq(&self, other: &dyn PartialEqAny) -> bool;
}

impl<T: 'static + PartialEq> PartialEqAny for T {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn eq(&self, other: &dyn PartialEqAny) -> bool {
        if TypeId::of::<T>() == PartialEqAny::type_id(other) {
            // Safety: T is 'static and types match.
            unsafe { return self.eq(&*(other as *const _ as *const T)) }
        } else {
            false
        }
    }
}

//
//

pub trait WidgetLocalKey {
    fn local_key(&self) -> Option<LocalKeyAny>;
}

impl<T> WidgetLocalKey for T {
    default fn local_key(&self) -> Option<LocalKeyAny> {
        None
    }
}

impl<K: 'static + PartialEq, W: Widget> WidgetLocalKey for LocalKey<K, W> {
    fn local_key(&self) -> Option<LocalKeyAny> {
        Some(LocalKeyAny { key: &self.key })
    }
}
