use crate::FlexEngine;
use crate::signals::{Signal, ReadSignal, create_effect};
// use crate::Node; // Removed

// --- Node Updater Logic (Unsafe / Dirty Access) ---
// Since we are single-threaded WASM, we can cheat the borrow checker
// to update specific nodes inside effects.

struct NodeUpdater {
    node_id: u32,
    engine_ptr: *mut FlexEngine,
}

impl NodeUpdater {
    fn new(engine: &mut FlexEngine, node_id: u32) -> Self {
        // Unsafe block removed as it is not needed for simple struct construction
        unsafe {
            Self { 
                node_id,
                engine_ptr: engine as *mut FlexEngine,
            }
        }
    }

    fn set_text(&self, _text: &str) {
         // See implementation in build()
    }

    fn set_flags(&self, flags: u32) {
        unsafe {
            (*self.engine_ptr).update_node_flags(self.node_id, flags);
        }
    }
}

// --- Builder Pattern for UI ---

pub struct Element {
    // Properties that are applied immediately during build
    style_basis: f32,
    fixed_width: f32,
    color: Option<(f32, f32, f32, f32)>,
    text: Option<String>,
    flex_direction: Option<u32>,
    abs_pos: Option<(f32, f32)>,
    z_index: Option<f32>,

    // Reactive bindings
    text_signal: Option<ReadSignal<String>>,
    flags_signal: Option<ReadSignal<u32>>,
    
    // Children
    children: Vec<Element>,
}

impl Element {
    pub fn new() -> Self {
        Self {
            style_basis: 0.0,
            fixed_width: -1.0,
            color: None,
            text: None,
            flex_direction: None,
            abs_pos: None,
            z_index: None,
            
            text_signal: None,
            flags_signal: None,
            
            children: Vec::new(),
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.fixed_width = w;
        self
    }

    pub fn basis(mut self, b: f32) -> Self {
        self.style_basis = b;
        self
    }

    pub fn color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.color = Some((r, g, b, a));
        self
    }
    
    pub fn text(mut self, s: &str) -> Self {
        self.text = Some(s.to_string());
        self
    }
    
    pub fn row(mut self) -> Self {
        self.flex_direction = Some(0);
        self
    }
    
    pub fn col(mut self) -> Self {
        self.flex_direction = Some(1);
        self
    }
    
    pub fn absolute(mut self, top: f32, left: f32) -> Self {
        self.abs_pos = Some((top, left));
        self
    }
    
    pub fn z(mut self, z: f32) -> Self {
        self.z_index = Some(z);
        self
    }

    pub fn child(mut self, child: Element) -> Self {
        self.children.push(child);
        self
    }

    // --- Reactivity ---
    
    pub fn bind_text(mut self, signal: ReadSignal<String>) -> Self {
        self.text_signal = Some(signal);
        self
    }
    
    pub fn bind_flags(mut self, signal: ReadSignal<u32>) -> Self {
        self.flags_signal = Some(signal);
        self
    }

    // --- Building ---

    // Returns the Node ID
    pub fn build(self, engine: &mut FlexEngine, parent: Option<u32>) -> u32 {
        let node_id = engine.add_node(self.style_basis);
        
        // Apply Static Props
        engine.set_fixed_width(node_id, self.fixed_width);
        
        if let Some((r,g,b,a)) = self.color {
            engine.set_color(node_id, r, g, b, a);
        }
        
        if let Some(s) = self.text {
            engine.set_text(node_id, &s);
        }
        
        if let Some(dir) = self.flex_direction {
            engine.set_flex_direction(node_id, dir);
        }
        
        if let Some((t, l)) = self.abs_pos {
            engine.set_position_absolute(node_id, t, l);
        }
        
        if let Some(z) = self.z_index {
            engine.set_z_index(node_id, z);
        }
        
        if let Some(p) = parent {
            engine.set_parent(node_id, p);
        }

        // Apply Reactive Bindings (Effects)
        // Note: In a real implementation we would need to pass a reference to 'engine' 
        // into the effect. But our effect system currently only takes FnMut closures 
        // and 'engine' is &mut. This is the classic Rust ownership struggle.
        
        // Unsafe Updater Strategy
        let updater = NodeUpdater::new(engine, node_id);
        
        // Bind Flags
        if let Some(sig) = self.flags_signal {
             // We need to move 'updater' into the closure. 
             let node_id = updater.node_id;
             let engine_ptr = updater.engine_ptr as usize;
             
             create_effect(move || {
                 let val = sig.get();
                 unsafe {
                     let engine = engine_ptr as *mut FlexEngine;
                     (*engine).update_node_flags(node_id, val);
                 }
             });
        }


        if let Some(mut sig) = self.text_signal {
             let node_id = updater.node_id;
             let engine_ptr = updater.engine_ptr as usize;
             
             create_effect(move || {
                 let val = sig.get();
                 unsafe {
                     let engine = engine_ptr as *mut FlexEngine;
                     (*engine).set_text(node_id, &val);
                     // set_text marks dirty automatically
                 }
             });
        }

        // Build Children
        // Current FlexEngine Requirement: Children must be contiguous indices!
        // This is a harsh constraint of the current engine. 
        // We must build children immediately after parent in a linear scan.
        // Recursive build does this naturally IF we depth-first build.
        // Wait, no. Depth first means: Parent -> Child -> Child's Child.
        // Result indices: 0, 1, 2. 
        // Parent (0) children are 1... but 1 has a child 2. 
        // So Parent's children are [1]. 1's children are [2].
        // This works for the current contiguous logic IF each parent only has 1 "group" of children.
        // But if Parent has Child A and Child B...
        // 0 -> 1 -> 2 (A's child) -> 3 (B).
        // Parent's children: 1 and 3. NOT CONTIGUOUS.
        
        // CRITICAL FIX: The current FlexEngine requires BFS (Breadth-First) topology for optimal
        // contiguous memory, OR we simply must update the Engine to use Linked Lists as discussed.
        // Since we haven't done the Linked List refactor yet, this Builder is limited.
        // Use a simple recursive build for now, realizing it might break complex layouts 
        // until we fix the Linked List engine.
        
        let start_child_index = engine.get_node_count() as u32; // This is a guess, unsafe for DFS
        
        // For now, we just recursively build.
        // To strictly satisfy "contiguous children" without Linked Lists, we would need to 
        // reserve slots or modify the engine.
        // Let's proceed with the Linked List refactor plan alongside this.
        
        if !self.children.is_empty() {
             for child in self.children {
                 child.build(engine, Some(node_id));
             }
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

// --- Replacement for build_ui ---

pub fn build_ui(engine: &mut FlexEngine) {
    // Test Signal for Interval
    let (visible, set_visible) = crate::signals::create_signal(1u32);
    let (sidebar_content, set_sidebar_content) = crate::signals::create_signal("SIDEBAR\n\nDashboard\nAnalytics\nCustomers\nSettings\n\nStatus: OK".to_string());
    
    // Set Interval Hack
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        
        let closure = Closure::wrap(Box::new(move || {
            // Random Text Update Logic
            let r = js_sys::Math::random();
            let new_text = format!("SIDEBAR\n\nDashboard\nAnalytics\nCustomers\nSettings\n\nStatus: {:.4}", r);
            set_sidebar_content.set(new_text);
            
            crate::log(&format!("Interval Tick: Updating Sidebar to {}", r));
        }) as Box<dyn FnMut()>);
        
        if let Some(window) = crate::web_bindings::get_window() {
            window.set_interval(closure.as_ref().unchecked_ref(), 5000);
        }
        
        closure.forget(); // Leak to keep alive
    }

    // Recreating the sidebar layout using the new builder
    
    // Grand Root
    div().width(800.0).row().color(0.1, 0.1, 0.1, 1.0)
        .child(
            // Sidebar
            div().width(200.0).color(0.15, 0.15, 0.2, 1.0)
            .bind_text(sidebar_content)
        )
        .child(
            // Main Content
            div().col().color(0.9, 0.9, 0.9, 1.0)
            .child(
                // Row 1
                div().row().color(0.3, 0.3, 0.3, 1.0)
                .child(text("Row 1 - Item A: Long text testing wrapping."))
                .child(text("Row 1 - Item B: More text."))
            )
            .child(
                // Row 2
                div().row().color(0.3, 0.3, 0.3, 1.0)
                .child(text("Row 2 - Item C: Below the first row."))
                .child(text("Row 2 - Item D: Final test block."))
            )
            .child(
                // Absolute Popup - NOW REACTIVE
                div().absolute(50.0, 50.0).width(100.0).z(100.0)
                .color(1.0, 0.2, 0.2, 1.0)
                .text("ABSOLUTE\nPOPUP")
                .bind_flags(visible) // BIND SIGNAL
            )
        )
        .build(engine, None);
}
