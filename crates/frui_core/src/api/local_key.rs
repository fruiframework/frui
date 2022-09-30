use std::any::{Any, TypeId};

use crate::prelude::{BuildContext, SingleChildWidget};

use super::{implementors::WidgetDerive, Widget, WidgetKind};

/// LocalKey is a widget that allows you to annotate the key for a `child`
/// widget.
pub struct LocalKey<K: 'static + PartialEq, W: Widget> {
    pub key: K,
    pub child: W,
}

impl<K: 'static + PartialEq, W: Widget> LocalKey<K, W> {
    pub fn new(key: K, child: W) -> Self {
        Self { key, child }
    }
}

//
// Widget implementation.

#[doc(hidden)]
pub enum LocalKeyUniqueTypeId {}

impl<K: 'static + PartialEq, W: Widget> Widget for LocalKey<K, W> {
    fn unique_type(&self) -> TypeId {
        TypeId::of::<LocalKeyUniqueTypeId>()
    }

    fn kind(&self) -> WidgetKind {
        WidgetKind::SingleChild(self)
    }
}

impl<K: 'static + PartialEq, W: Widget> WidgetDerive for LocalKey<K, W> {
    type Widget<'a> = &'a W where Self: 'a;

    type UniqueTypeId = LocalKeyUniqueTypeId;
}

impl<K: 'static + PartialEq, W: Widget> SingleChildWidget for LocalKey<K, W> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }
}

//
// LocalKeyAny

pub struct LocalKeyAny<'a> {
    key: &'a dyn PartialEqAny,
}

impl PartialEq for LocalKeyAny<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(other.key)
    }
}

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

//
// Helpers

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
            unsafe { return self.eq(&*(other as *const _ as *const T)) }
        } else {
            false
        }
    }
}
