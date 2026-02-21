#![allow(unused_imports)]
#![allow(dead_code)]
use crate::signals::{create_effect, ReadSignal};
use crate::FlexEngine;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

// --- Node Updater Logic ---

struct NodeUpdater {
    node_id: u32,
    engine: Rc<RefCell<FlexEngine>>,
}

impl NodeUpdater {
    fn new(engine: Rc<RefCell<FlexEngine>>, node_id: u32) -> Self {
        Self { node_id, engine }
    }
}

// --- Builder Pattern for Elements ---

pub struct Element {
    text_content: Option<String>,
    image_id: Option<String>,
    flags: u32,

    value_signal: Option<ReadSignal<String>>,
    path_signal: Option<ReadSignal<String>>,
    flags_signal: Option<ReadSignal<u32>>,

    on_click: Option<Rc<dyn Fn(crate::UiEvent)>>,
    on_mouse_enter: Option<Rc<dyn Fn(crate::UiEvent)>>,
    on_mouse_leave: Option<Rc<dyn Fn(crate::UiEvent)>>,
    classes: Vec<String>,
    inline_styles: HashMap<String, crate::StyleValue>,
    children: Vec<Element>,
    path_data: Option<String>,

    input_type: Option<String>,
    on_update_model_value: Option<Rc<dyn Fn(String)>>,
}

impl Element {
    pub fn new() -> Self {
        Self {
            text_content: None,
            image_id: None,
            flags: 1, // Visible
            value_signal: None,
            path_signal: None,
            flags_signal: None,
            on_click: None,
            on_mouse_enter: None,
            on_mouse_leave: None,
            classes: Vec::new(),
            inline_styles: HashMap::new(),
            children: Vec::new(),
            path_data: None,
            input_type: None,
            on_update_model_value: None,
        }
    }

    pub fn class(mut self, name: &str) -> Self {
        for token in name.split_whitespace() {
            if !token.is_empty() {
                self.classes.push(token.to_string());
            }
        }
        self
    }

    pub fn style(mut self, prop: &str, val: crate::StyleValue) -> Self {
        self.inline_styles.insert(prop.to_string(), val);
        self
    }

    // Deprecated helpers - will be removed or made to use .style()
    pub fn width(self, w: f32) -> Self {
        self.style("width", crate::StyleValue::Px(w))
    }
    pub fn height(self, h: f32) -> Self {
        self.style("height", crate::StyleValue::Px(h))
    }
    pub fn min_width(self, w: f32) -> Self {
        self.style("min-width", crate::StyleValue::Px(w))
    }
    pub fn max_width(self, w: f32) -> Self {
        self.style("max-width", crate::StyleValue::Px(w))
    }
    pub fn min_height(self, h: f32) -> Self {
        self.style("min-height", crate::StyleValue::Px(h))
    }
    pub fn max_height(self, h: f32) -> Self {
        self.style("max-height", crate::StyleValue::Px(h))
    }
    pub fn color(self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.style("color", crate::StyleValue::Color(r, g, b, a))
    }
    pub fn row(self) -> Self {
        self.style("flex-direction", crate::StyleValue::Ident("row".into()))
    }
    pub fn col(self) -> Self {
        self.style("flex-direction", crate::StyleValue::Ident("column".into()))
    }
    pub fn absolute(self, top: f32, left: f32) -> Self {
        self.style("position", crate::StyleValue::Ident("absolute".into()))
            .style("top", crate::StyleValue::Px(top))
            .style("left", crate::StyleValue::Px(left))
    }
    pub fn z(self, z: f32) -> Self {
        self.style("z-index", crate::StyleValue::Px(z))
    }
    pub fn padding(self, v: f32) -> Self {
        self.style("padding", crate::StyleValue::Px(v))
    }
    pub fn margin(self, v: f32) -> Self {
        self.style("margin", crate::StyleValue::Px(v))
    }
    pub fn child(mut self, child: Element) -> Self {
        self.children.push(child);
        self
    }
    pub fn on_click<F: Fn(crate::UiEvent) + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Rc::new(f));
        self
    }
    pub fn on_mouse_enter<F: Fn(crate::UiEvent) + 'static>(mut self, f: F) -> Self {
        self.on_mouse_enter = Some(Rc::new(f));
        self
    }
    pub fn on_mouse_leave<F: Fn(crate::UiEvent) + 'static>(mut self, f: F) -> Self {
        self.on_mouse_leave = Some(Rc::new(f));
        self
    }
    pub fn value(mut self, signal: ReadSignal<String>) -> Self {
        self.value_signal = Some(signal);
        self
    }
    pub fn bind_text(self, signal: ReadSignal<String>) -> Self {
        self.value(signal)
    } // Deprecated
    pub fn bind_path(mut self, signal: ReadSignal<String>) -> Self {
        self.path_signal = Some(signal);
        self
    }
    pub fn bind_flags(mut self, signal: ReadSignal<u32>) -> Self {
        self.flags_signal = Some(signal);
        self
    }

    pub fn text(mut self, s: &str) -> Self {
        self.text_content = Some(s.to_string());
        self
    }
    pub fn image(mut self, id: &str) -> Self {
        self.image_id = Some(id.to_string());
        self
    }
    pub fn path(mut self, d: &str) -> Self {
        self.path_data = Some(d.to_string());
        self
    }

    pub fn input_type(mut self, t: &str) -> Self {
        self.input_type = Some(t.to_string());
        self
    }
    pub fn on_update_model_value<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_update_model_value = Some(Rc::new(f));
        self
    }

    pub fn build(self, engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>) -> u32 {
        self.build_after(engine, parent, None)
    }

    pub fn build_after(
        self,
        engine: Rc<RefCell<FlexEngine>>,
        parent: Option<u32>,
        after: Option<u32>,
    ) -> u32 {
        let node_id = engine.borrow_mut().add_node(0.0);

        {
            let mut e = engine.borrow_mut();
            let node = &mut e.cpu_nodes[node_id as usize];
            node.classes = self.classes;
            node.inline_styles = self.inline_styles;
            node.flags = self.flags;
            if let Some(s) = self.text_content {
                node.text = Some(s);
            }
            if let Some(id) = self.image_id {
                node.image_asset_id = Some(id);
            }
            if let Some(p) = self.path_data {
                node.shape_data = Some(p);
            }
            if let Some(f) = self.on_click {
                node.on_click = Some(f);
            }
            if let Some(f) = self.on_mouse_enter {
                node.on_mouse_enter = Some(f);
            }
            if let Some(f) = self.on_mouse_leave {
                node.on_mouse_leave = Some(f);
            }
            if let Some(t) = self.input_type {
                node.input_type = Some(t);
                node.flags |= crate::NODE_FLAG_IS_INPUT;
            }
            if let Some(f) = self.on_update_model_value {
                node.on_update_model_value = Some(f);
            }

            if let Some(p) = parent {
                if let Some(a) = after {
                    e.insert_after_node(node_id, p, Some(a));
                } else {
                    e.set_parent(node_id, p);
                }
            }
        }

        if let Some(sig) = self.flags_signal {
            let engine_weak = Rc::downgrade(&engine);
            create_effect(move || {
                if let Some(engine) = engine_weak.upgrade() {
                    let val = sig.get();
                    engine.borrow_mut().update_node_flags(node_id, val);
                }
            });
        }

        if let Some(sig) = self.value_signal {
            let engine_weak = Rc::downgrade(&engine);
            create_effect(move || {
                if let Some(engine) = engine_weak.upgrade() {
                    let val = sig.get();
                    engine.borrow_mut().set_text(node_id, &val);
                }
            });
        }

        if let Some(sig) = self.path_signal {
            let engine_weak = Rc::downgrade(&engine);
            create_effect(move || {
                if let Some(engine) = engine_weak.upgrade() {
                    let val = sig.get();
                    engine.borrow_mut().set_shape_data(node_id, &val);
                }
            });
        }

        for child in self.children {
            child.build(engine.clone(), Some(node_id));
        }

        node_id
    }
}

pub fn div() -> Element {
    Element::new()
}
pub fn text(content: &str) -> Element {
    Element::new().text(content)
}
pub fn input() -> Element {
    Element::new()
}
pub fn img() -> Element {
    Element::new()
}

// --- Block System ---

/// mount_list handles dynamic lists (v-for) with a stable anchor for O(1) insertions.
pub fn mount_list<T, F, K>(
    engine: Rc<RefCell<FlexEngine>>,
    parent: u32,
    items_sig: ReadSignal<Vec<T>>,
    key_fn: K,
    template: F,
) where
    T: Clone + 'static,
    F: Fn(T) -> Element + 'static,
    K: Fn(&T) -> String + 'static,
{
    // Anchor node marks where the list starts in the linked-list logical tree.
    let anchor_id = {
        let mut e = engine.borrow_mut();
        let id = e.add_node(0.0);
        e.set_fixed_width(id, 0.0);
        e.set_parent(id, parent);
        id
    };

    // Tracks currently mounted nodes by their key.
    // Infrastructure for Keyed Diffing: even if items move, we keep their nodes.
    let mut mounted_nodes: HashMap<String, u32> = HashMap::new();

    let engine_weak = Rc::downgrade(&engine);
    create_effect(move || {
        if let Some(engine) = engine_weak.upgrade() {
            let items = items_sig.get();
            let mut new_mounted: HashMap<String, u32> = HashMap::new();

            // 1. Reconciliation Pass
            let mut last_id = anchor_id;
            for item in items {
                let key = key_fn(&item);

                let node_id = if let Some(&existing_id) = mounted_nodes.get(&key) {
                    // Reuse existing node!
                    engine
                        .borrow_mut()
                        .insert_after_node(existing_id, parent, Some(last_id));
                    existing_id
                } else {
                    // Build new node in a new scope
                    let engine_clone = engine.clone();
                    let (id, scope) = crate::signals::create_root(|s| {
                        (
                            template(item).build_after(engine_clone, Some(parent), Some(last_id)),
                            s,
                        )
                    });
                    engine.borrow_mut().cpu_nodes[id as usize].scope = Some(scope.id);
                    id
                };

                new_mounted.insert(key, node_id);
                last_id = node_id;
            }

            // 2. Cleanup Pass: Remove nodes no longer in the list
            {
                let mut e = engine.borrow_mut();
                for (key, id) in mounted_nodes.drain() {
                    if !new_mounted.contains_key(&key) {
                        e.delete_node_recursive(id);
                    }
                }
            }

            mounted_nodes = new_mounted;
        }
    });
}

/// mount_if handles conditional rendering (v-if).
pub fn mount_if<F>(
    engine: Rc<RefCell<FlexEngine>>,
    parent: u32,
    condition: ReadSignal<bool>,
    template: F,
) where
    F: Fn() -> u32 + 'static,
{
    // Anchor node marks the position for the conditional content.
    let anchor_id = {
        let mut e = engine.borrow_mut();
        let id = e.add_node(0.0);
        e.set_fixed_width(id, 0.0);
        e.set_parent(id, parent);
        // Hide anchor from rendering
        e.update_node_flags(id, 0);
        id
    };

    let mounted_id = Rc::new(RefCell::new(None));

    let engine_weak = Rc::downgrade(&engine);
    create_effect(move || {
        if let Some(engine) = engine_weak.upgrade() {
            let is_true = condition.get();
            let mut current = mounted_id.borrow_mut();

            if is_true {
                if current.is_none() {
                    // Create in a new scope
                    let (id, scope) = crate::signals::create_root(|s| (template(), s));

                    engine.borrow_mut().cpu_nodes[id as usize].scope = Some(scope.id);
                    // Ensure it's correctly placed after the anchor
                    engine
                        .borrow_mut()
                        .insert_after_node(id, parent, Some(anchor_id));
                    *current = Some(id);
                }
            } else {
                if let Some(id) = current.take() {
                    engine.borrow_mut().delete_node_recursive(id);
                }
            }
        }
    });
}

// --- UI Construction ---

// --- UI Construction ---

// build_ui removed. Logic moved to app crate.
