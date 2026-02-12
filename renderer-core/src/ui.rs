use crate::FlexEngine;
use crate::signals::{ReadSignal, create_effect};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
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

    text_signal: Option<ReadSignal<String>>,
    flags_signal: Option<ReadSignal<u32>>,
    
    on_click: Option<Rc<dyn Fn()>>,
    classes: Vec<String>,
    inline_styles: HashMap<String, crate::StyleValue>,
    children: Vec<Element>,
}

impl Element {
    pub fn new() -> Self {
        Self {
            text_content: None,
            image_id: None,
            flags: 1, // Visible
            text_signal: None,
            flags_signal: None,
            on_click: None,
            classes: Vec::new(),
            inline_styles: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn class(mut self, name: &str) -> Self {
        self.classes.push(name.to_string());
        self
    }

    pub fn style(mut self, prop: &str, val: crate::StyleValue) -> Self {
        self.inline_styles.insert(prop.to_string(), val);
        self
    }

    // Deprecated helpers - will be removed or made to use .style()
    pub fn width(self, w: f32) -> Self { self.style("width", crate::StyleValue::Px(w)) }
    pub fn height(self, h: f32) -> Self { self.style("height", crate::StyleValue::Px(h)) }
    pub fn color(self, r: f32, g: f32, b: f32, a: f32) -> Self { self.style("color", crate::StyleValue::Color(r, g, b, a)) }
    pub fn row(self) -> Self { self.style("flex-direction", crate::StyleValue::Ident("row".into())) }
    pub fn col(self) -> Self { self.style("flex-direction", crate::StyleValue::Ident("column".into())) }
    pub fn absolute(self, top: f32, left: f32) -> Self { 
        self.style("position", crate::StyleValue::Ident("absolute".into()))
            .style("top", crate::StyleValue::Px(top))
            .style("left", crate::StyleValue::Px(left))
    }
    pub fn z(self, z: f32) -> Self { self.style("z-index", crate::StyleValue::Px(z)) }
    pub fn child(mut self, child: Element) -> Self { self.children.push(child); self }
    pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self { self.on_click = Some(Rc::new(f)); self }

    pub fn bind_text(mut self, signal: ReadSignal<String>) -> Self { self.text_signal = Some(signal); self }
    pub fn bind_flags(mut self, signal: ReadSignal<u32>) -> Self { self.flags_signal = Some(signal); self }

    pub fn text(mut self, s: &str) -> Self { self.text_content = Some(s.to_string()); self }
    pub fn image(mut self, id: &str) -> Self { self.image_id = Some(id.to_string()); self }

    pub fn build(self, engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>) -> u32 {
        self.build_after(engine, parent, None)
    }

    pub fn build_after(self, engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, after: Option<u32>) -> u32 {
        let node_id = engine.borrow_mut().add_node(0.0);
        
        {
            let mut e = engine.borrow_mut();
            let node = &mut e.cpu_nodes[node_id as usize];
            node.classes = self.classes;
            node.inline_styles = self.inline_styles;
            node.flags = self.flags;
            if let Some(s) = self.text_content { node.text = Some(s); }
            if let Some(id) = self.image_id { node.image_asset_id = Some(id); }
            if let Some(f) = self.on_click { node.on_click = Some(f); }
            
            if let Some(p) = parent {
                if let Some(a) = after {
                    e.insert_after_node(node_id, p, Some(a));
                } else {
                    e.set_parent(node_id, p);
                }
            }
        }

        if let Some(sig) = self.flags_signal {
             let engine_clone = engine.clone();
             create_effect(move || {
                 let val = sig.get();
                 engine_clone.borrow_mut().update_node_flags(node_id, val);
             });
        }

        if let Some(sig) = self.text_signal {
             let engine_clone = engine.clone();
             create_effect(move || {
                 let val = sig.get();
                 engine_clone.borrow_mut().set_text(node_id, &val);
             });
        }

        for child in self.children {
            child.build(engine.clone(), Some(node_id));
        }

        node_id
    }
}

pub fn div() -> Element { Element::new() }
pub fn text(content: &str) -> Element { Element::new().text(content) }

// --- Block System ---

/// mount_list handles dynamic lists (v-for) with a stable anchor for O(1) insertions.
pub fn mount_list<T, F, K>(
    engine: Rc<RefCell<FlexEngine>>, 
    parent: u32, 
    items_sig: ReadSignal<Vec<T>>, 
    key_fn: K,
    template: F
) 
where 
    T: Clone + 'static,
    F: Fn(T) -> Element + 'static,
    K: Fn(&T) -> String + 'static
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

    create_effect(move || {
        let items = items_sig.get();
        let mut new_mounted: HashMap<String, u32> = HashMap::new();
        let mut new_order: Vec<String> = Vec::new();
        
        // 1. Reconciliation Pass
        let mut last_id = anchor_id;
        for item in items {
            let key = key_fn(&item);
            new_order.push(key.clone());
            
            let node_id = if let Some(&existing_id) = mounted_nodes.get(&key) {
                // Reuse existing node!
                engine.borrow_mut().insert_after_node(existing_id, parent, Some(last_id));
                existing_id
            } else {
                // Build new node
                let element = template(item);
                element.build_after(engine.clone(), Some(parent), Some(last_id))
            };
            
            new_mounted.insert(key, node_id);
            last_id = node_id;
        }

        // 2. Cleanup Pass: Remove nodes no longer in the list
        {
            let mut e = engine.borrow_mut();
            for (key, id) in mounted_nodes.drain() {
                if !new_mounted.contains_key(&key) {
                    e.remove_from_parent(id);
                }
            }
        }

        mounted_nodes = new_mounted;
    });
}

// --- UI Construction ---

// --- UI Construction ---

// build_ui removed. Logic moved to app crate.

