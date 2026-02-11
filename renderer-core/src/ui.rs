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
    min_width: f32,
    fixed_width: f32,
    fixed_height: f32,
    color: Option<(f32, f32, f32, f32)>,
    text: Option<String>,
    image_id: Option<String>,
    flex_direction: Option<u32>,
    abs_pos: Option<(f32, f32)>,
    z_index: Option<f32>,
    flags: u32,

    text_signal: Option<ReadSignal<String>>,
    flags_signal: Option<ReadSignal<u32>>,
    
    on_click: Option<Rc<dyn Fn()>>,
    children: Vec<Element>,
}

impl Element {
    pub fn new() -> Self {
        Self {
            min_width: 0.0,
            fixed_width: -1.0,
            fixed_height: -1.0,
            color: None,
            text: None,
            image_id: None,
            flex_direction: None,
            abs_pos: None,
            z_index: None,
            flags: 1, // Visible
            text_signal: None,
            flags_signal: None,
            on_click: None,
            children: Vec::new(),
        }
    }

    pub fn width(mut self, w: f32) -> Self { self.fixed_width = w; self }
    pub fn height(mut self, h: f32) -> Self { self.fixed_height = h; self }
    pub fn min_width(mut self, mw: f32) -> Self { self.min_width = mw; self }
    pub fn flags(mut self, f: u32) -> Self { self.flags = f; self }
    pub fn color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self { self.color = Some((r, g, b, a)); self }
    pub fn text(mut self, s: &str) -> Self { self.text = Some(s.to_string()); self }
    pub fn image(mut self, id: &str) -> Self { self.image_id = Some(id.to_string()); self }
    pub fn row(mut self) -> Self { self.flex_direction = Some(0); self }
    pub fn col(mut self) -> Self { self.flex_direction = Some(1); self }
    pub fn absolute(mut self, top: f32, left: f32) -> Self { self.abs_pos = Some((top, left)); self }
    pub fn z(mut self, z: f32) -> Self { self.z_index = Some(z); self }
    pub fn child(mut self, child: Element) -> Self { self.children.push(child); self }
    pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self { self.on_click = Some(Rc::new(f)); self }

    pub fn bind_text(mut self, signal: ReadSignal<String>) -> Self { self.text_signal = Some(signal); self }
    pub fn bind_flags(mut self, signal: ReadSignal<u32>) -> Self { self.flags_signal = Some(signal); self }

    pub fn build(self, engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>) -> u32 {
        self.build_after(engine, parent, None)
    }

    pub fn build_after(self, engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, after: Option<u32>) -> u32 {
        let node_id = engine.borrow_mut().add_node(self.min_width);
        
        {
            let mut e = engine.borrow_mut();
            e.set_fixed_width(node_id, self.fixed_width);
            e.set_fixed_height(node_id, self.fixed_height);
            e.set_flags(node_id, self.flags);
            if let Some((r,g,b,a)) = self.color { e.set_color(node_id, r, g, b, a); }
            if let Some(s) = self.text { e.set_text(node_id, &s); }
            if let Some(id) = self.image_id { e.set_image_asset_id(node_id, &id); }
            if let Some(dir) = self.flex_direction { e.set_flex_direction(node_id, dir); }
            if let Some((t, l)) = self.abs_pos { e.set_position_absolute(node_id, t, l); }
            if let Some(z) = self.z_index { e.set_z_index(node_id, z); }
            if let Some(f) = self.on_click { e.set_on_click(node_id, f); }
            
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

pub fn build_ui(engine: Rc<RefCell<FlexEngine>>) {
    // Call the generated UI
    crate::generated_ui::app::build_generated_ui(engine.clone());

    // Trigger Asset Loads for images used in the template
    {
        let engine_clone = engine.clone();
        spawn_local(async move {
            // Load Asset
            crate::load_image_to_engine(engine_clone, "asset:paintbrush.svg".to_string()).await;
        });
    }
}
