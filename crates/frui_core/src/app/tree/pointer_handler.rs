use std::{cell::RefCell, collections::HashMap, rc::Rc};

use druid_shell::kurbo::Affine;

use crate::prelude::{context::HitTestCtxOS, events::PointerExit, PointerEvent};

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
                let results = HitTestResults::default();

                self.hit_test(root, &results, &event);

                for (node, affine) in results.borrow_mut().iter() {
                    self.handle_event(&node, event.transform(affine));
                }
            }
            PointerEvent::PointerMove(_) => {
                let new_results = HitTestResults::default();

                self.hit_test(root, &new_results, &event);

                // Dispatch to all widgets that got hit.
                for (node, affine) in new_results.borrow_mut().iter() {
                    self.handle_event(&node, event.transform(affine));
                }

                // Dispatch to widgets that lost "hover status" by this event.
                // Used to correctly dispatch PointerExit event.
                for (node, affine) in self
                    .pointer_hover_results_last
                    .borrow_mut()
                    .iter()
                    .filter(|(last, _)| !new_results.borrow_mut().contains_key(last))
                {
                    let event = event.transform(affine).raw();
                    let event = PointerEvent::PointerExit(PointerExit(event));
                    self.handle_event(&node, event);
                }

                self.pointer_hover_results_last = new_results;
            }
            _ => unreachable!(),
        }
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
        node.widget().raw().handle_event_os(ctx.clone(), &event);
    }
}
