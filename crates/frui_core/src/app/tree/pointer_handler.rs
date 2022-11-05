use std::{cell::RefCell, collections::HashMap, rc::Rc};

use druid_shell::kurbo::Affine;

use crate::prelude::{context::HitTestCtxOS, PointerEvent};

use super::WidgetNodeRef;

type HitTestResults = Rc<RefCell<HashMap<WidgetNodeRef, Affine>>>;

#[derive(Default)]
pub struct PointerHandler {
    /// Hit test results for last pointer down event.
    pointer_down_results: HitTestResults,
    /// Hit test results for the last hover event.
    pointer_hover_results_last: HitTestResults,
}

impl PointerHandler {
    pub fn handle_pointer_event(&mut self, root: WidgetNodeRef, event: PointerEvent) {
        match event {
            PointerEvent::PointerDown(_) => {
                self.hit_test(root, &self.pointer_down_results, &event);

                for (node, affine) in self.pointer_down_results.borrow_mut().iter() {
                    self.handle_event(&node, event.transform(affine));
                }
            }
            PointerEvent::PointerUp(_) => {
                // Call all nodes that were hit during PointerDown.
                for (node, affine) in self.pointer_down_results.borrow_mut().drain() {
                    self.handle_event(&node, event.transform(&affine));
                }
            }
            PointerEvent::PointerScroll(_) => {
                self.dispatch(root, event);
            }
            PointerEvent::PointerMove(_) => {
                // Dispatch almost directly.

                // Problem: Button should register pointer moves I think, even if it
                // is outside of the hit area; But for sure, it has to register the
                // PointerUp event outside of the are.
            }
        }

        // Todo: Change self.widget() to returns `&dyn RawWidget`.
    }

    fn hit_test(
        &self,
        node: WidgetNodeRef,
        new_hit_entries: &Rc<RefCell<HashMap<WidgetNodeRef, Affine>>>,
        event: &PointerEvent,
    ) {
        let ctx = HitTestCtxOS::new(&node, new_hit_entries.clone(), Affine::default());
        node.widget().raw().hit_test_os(ctx.clone(), event.pos());
    }

    fn handle_event(&self, node: &WidgetNodeRef, event: PointerEvent) {
        let ctx = HitTestCtxOS::new(node, Rc::new(RefCell::default()), Affine::default());
        node.widget()
            .raw()
            .handle_event_os(ctx.clone(), &event, false);
    }

    /// Dispatches event to all widgets at event position.
    fn dispatch(&mut self, root: WidgetNodeRef, event: PointerEvent) {
        let results = HitTestResults::default();

        self.hit_test(root, &results, &event);

        for (node, affine) in results.borrow_mut().iter() {
            self.handle_event(&node, event.transform(affine));
        }
    }
}
