use std::{cell::RefCell, sync::Mutex};

use druid_shell::{
    kurbo::Rect,
    piet::{Color, RenderContext},
    Cursor, IdleToken, KeyEvent, MouseEvent,
};

use crate::{
    api::{pointer_events::events::PointerEvent, WidgetPtr},
    app::{
        listeners::keyboard::KEYBOARD_EVENT_LISTENERS,
        tree::{NodeRef, WidgetTree},
        TEXT_FACTORY,
    },
    prelude::Widget,
    render::*,
};

use super::{Application, Canvas, FruiWindowHandler, IdleHandle, WindowHandle};

thread_local! {
    pub(crate) static APP_HANDLE: std::cell::RefCell<Option<IdleHandle>> = RefCell::new(None);
}

thread_local! {
    pub(crate) static NEED_REBUILD: Mutex<Vec<NodeRef>>  = Mutex::new(Vec::with_capacity(100));
}

pub struct WindowHandler {
    /// Current size of main window.
    window_size: Size,
    /// Clone of window handle received from `connect`.
    window_handle: WindowHandle,

    pending_update: bool,
    widget_tree: WidgetTree,

    /// Temporary field to store root widget before constructing the widget tree
    /// (which requires WindowHandle which can be obtained only after `connect`).
    root_temp: Option<WidgetPtr<'static>>,
}

impl WindowHandler {
    pub fn new<W: Widget + 'static>(widget: W) -> Self {
        Self {
            window_size: Size::default(),
            window_handle: WindowHandle::default(),
            pending_update: true,
            widget_tree: WidgetTree::default(),
            root_temp: Some(WidgetPtr::from_owned(Box::new(widget))),
        }
    }

    /// Will schedule an update for the next frame.
    pub fn schedule_update(&mut self) {
        if !self.pending_update {
            self.pending_update = true;
            self.window_handle.invalidate();
            self.window_handle.request_anim_frame();
        }
    }

    fn rebuild_dirty(&mut self) {
        NEED_REBUILD.with(|need_rebuild| {
            let mut idx = 0;

            loop {
                let need_rebuild = need_rebuild.lock().unwrap();

                // We use idx instead of iterator, since every call to `update_subtree` may add
                // new widgets that need to be rebuilt.
                if idx == need_rebuild.len() {
                    break;
                }

                // Todo: Sort widgets according to their depth, which will allow us to avoid
                // rebuilding some widgets multiple times. Remember to sort those widgets every
                // time you update a widget (don't sort if length of vec didn't change).

                // Acquire node reference.
                let node = need_rebuild[idx].clone();

                // Drop the lock.
                drop(need_rebuild);

                if node.is_alive() {
                    if node.borrow().dirty {
                        node.update_subtree();
                    }
                }

                idx += 1;
            }

            need_rebuild.lock().unwrap().clear();
        });
    }
}

impl FruiWindowHandler for WindowHandler {
    fn connect(&mut self, handle: &WindowHandle) {
        APP_HANDLE.with(|r| *r.borrow_mut() = Some(handle.get_idle_handle().unwrap()));

        if !cfg!(feature = "miri") {
            TEXT_FACTORY.with(|f| f.set(self.window_handle.text()));
        }

        let root_widget = std::mem::take(&mut self.root_temp);
        self.widget_tree = WidgetTree::new(root_widget.unwrap());
        self.window_handle = handle.clone();

        self.window_handle.set_cursor(&Cursor::Arrow);
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut Canvas, _invalid: &druid_shell::Region) {
        //
        // Fill screen with one color (temp).

        let size = self.window_size;
        let rect = Rect::new(0., 0., size.width, size.height);
        let brush = &piet.solid_brush(Color::from_hex_str("#202324").unwrap());

        druid_shell::piet::RenderContext::fill(piet, rect, brush);

        //
        // Rebuild widget tree.

        self.pending_update = false;
        self.rebuild_dirty();

        //
        // Layout & Paint

        // Todo: Optimize layout.
        self.widget_tree
            .layout(Constraints::new_tight(self.window_size));

        // Todo: Optimize paint.
        self.widget_tree.paint(piet);
    }

    fn size(&mut self, size: druid_shell::kurbo::Size) {
        self.window_size = size.into();
    }

    fn idle(&mut self, _token: IdleToken) {
        self.schedule_update();
    }

    fn destroy(&mut self) {
        Application::global().quit()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    // Events:

    fn mouse_down(&mut self, event: &MouseEvent) {
        self.widget_tree
            .handle_pointer_event(PointerEvent::new(event, "down"));
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        self.widget_tree
            .handle_pointer_event(PointerEvent::new(event, "up"));
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        self.widget_tree
            .handle_pointer_event(PointerEvent::new(event, "move"));

        self.window_handle.set_cursor(&Cursor::Arrow);
    }

    fn wheel(&mut self, event: &MouseEvent) {
        self.widget_tree
            .handle_pointer_event(PointerEvent::new(event, "wheel"));
    }

    fn key_down(&mut self, event: KeyEvent) -> bool {
        KEYBOARD_EVENT_LISTENERS.with(|listeners| {
            for listener in listeners.borrow_mut().iter() {
                listener(event.clone());
            }
        });

        true
    }

    fn request_close(&mut self) {
        self.window_handle.close();
    }
}
