//! This example shows how to construct an [`InheritedWidget`] and use it in
//! your code.
//!
//! [`InheritedWidget`] allows you to access its state from anywhere in its
//! subtree. Additionally, whenever its state is updated, it will mark every
//! widget that is dependent on it for a rebuild. That way, only necessary
//! widgets are going to be rebuilt instead of the whole subtree.
//!
//! You can observe the above behavior in this example. [`ChangesOnRebuild`]
//! inside of the [`App`] widget remains the same even though
//! [`InheritedSwitchConsumer`] reacts to [`InheritedSwitch`] state changes
//! triggered by the [`InheritedSwitchDispatcher`] and rebuilds.

#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;
use misc::{ChangesOnRebuild, SwitchGuard};

// (1)
//
// First we will define an InheritedWidget which will be inserted at some point
// into the widget tree. State of that widget is going to be boolean. That
// boolean will be then accessed later from widgets which want to depend on
// that InheritedSwitch widget.

#[derive(InheritedWidget)]
struct InheritedSwitch<W: Widget> {
    child: W,
}

impl<W: Widget> WidgetState for InheritedSwitch<W> {
    type State = bool;

    fn create_state<'a>(&'a self) -> Self::State {
        false
    }
}

impl<W: Widget> InheritedWidget for InheritedSwitch<W> {
    fn build<'w>(&'w self) -> Self::Widget<'w> {
        &self.child
    }
}

// (2)
//
// Now we will define the InheritedSwitch::of method, which will simplify
// access to the state of the InheritedWidget.

impl InheritedSwitch<()> {
    pub fn of<'a, T>(ctx: BuildContext<'a, T>) -> SwitchGuard<'a> {
        let state = ctx.depend_on_inherited_widget::<Self>().unwrap();

        SwitchGuard::new(state)
    }
}

// (3)
//
// Here we will define a widget which will trigger change in InheritedSwitch
// state. It will react to a keyboard event and will flip the boolean value
// by using InheritedSwitch::of method we defined earlier.

#[derive(ViewWidget)]
struct InheritedSwitchDispatcher;

impl ViewWidget for InheritedSwitchDispatcher {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w> {
        KeyboardEventDetector {
            on_event: |_| InheritedSwitch::of(ctx).switch(),
            child: (),
        }
    }
}

// (4)
//
// Here we will define a widget that reacts to changes in InheritedSwitch state.
// Again we access the state through InheritedSwitch::of and display it.

#[derive(ViewWidget)]
struct InheritedSwitchConsumer;

impl ViewWidget for InheritedSwitchConsumer {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w> {
        Text::new(InheritedSwitch::of(ctx).to_string())
    }
}

// (5)
//
// We will now insert previously defined InheritedSwitchDispatcher and
// InheritedSwitchConsumer to the App.
//
// Additionally, we will add ChangesOnRebuild widget to prove that nothing
// except for the dependent widgets gets rebuilt.

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        Center::child(Column::builder().children((
            Text::new(concat!(
                "Following widget doesn't depend on inherited widget, ",
                "so it should not be rebuilt (number should stay the same).",
            )),
            ChangesOnRebuild,
            Text::new("\n"),
            //
            Text::new(concat!(
                "Following widget depends on inherited widget, ",
                "so its value should update when you press a key.",
            )),
            InheritedSwitchConsumer,
            InheritedSwitchDispatcher,
        )))
    }
}

// (6)
//
// Finally, we wrap our App in InheritedSwitch widget so that
// InheritedSwitchDispatcher and InheritedSwitchConsumer can access its state.

fn main() {
    run_app(InheritedSwitch { child: App });
}

#[cfg(all(test, feature = "miri"))]
mod test {
    use super::*;
    use frui::{
        app::runner::miri::MiriRunner,
        druid_shell::{keyboard_types::Key, Modifiers},
    };

    #[test]
    pub fn inherited_widget() {
        let mut runner = MiriRunner::new(InheritedSwitch { child: App });

        for _ in 0..4 {
            runner.key_down(KeyEvent::for_test(
                Modifiers::default(),
                Key::Character(" ".into()),
            ));
            runner.update(true);
        }
    }
}
