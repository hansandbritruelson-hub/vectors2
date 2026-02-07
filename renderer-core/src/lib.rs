use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Node {
    // --- Layout Inputs (Style) ---
    pub style_min_width: f32,
    pub style_basis: f32,

    // --- Computed Values (Phase A - Width) ---
    pub desired_width: f32, // Result of Pass 1
    pub final_width: f32,   // Result of Pass 2

    // --- Computed Values (Phase B - Height/Pos) ---
    pub desired_height: f32, // Result of Pass 3
    pub final_height: f32,   // Result of Pass 4
    pub final_x: f32,
    pub final_y: f32,

    // --- Tree Topology ---
    pub parent_index: u32,
    pub child_start_index: u32,
    pub child_count: u32,
    
    // --- Synchronization ---
    pub signals_finished: u32, // Atomic counter for Bottom-Up
}

impl Node {
    pub fn new() -> Self {
        Self {
            style_min_width: 0.0,
            style_basis: 100.0,
            desired_width: 0.0,
            final_width: 0.0,
            desired_height: 0.0,
            final_height: 0.0,
            final_x: 0.0,
            final_y: 0.0,
            parent_index: 0,
            child_start_index: 0,
            child_count: 0,
            signals_finished: 0,
        }
    }
}

#[wasm_bindgen]
pub struct FlexEngine {
    nodes: Vec<Node>, 
}

#[wasm_bindgen]
impl FlexEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FlexEngine {
        log("FlexEngine Initialized via WebAssembly");
        FlexEngine {
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, style_basis: f32) -> u32 {
        let mut node = Node::new();
        node.style_basis = style_basis;
        let index = self.nodes.len() as u32;
        
        // Default parent to self (root) or 0 to be safe
        node.parent_index = if index == 0 { 0 } else { 0 }; 
        
        self.nodes.push(node);
        index
    }

    // Basic tree building - set parent
    pub fn set_parent(&mut self, child_index: u32, parent_index: u32) {
        if (child_index as usize) < self.nodes.len() && (parent_index as usize) < self.nodes.len() {
            self.nodes[child_index as usize].parent_index = parent_index;
            // Note: In a real implementation we would also need to update 
            // parent.child_start_index and parent.child_count, or use a separate child array 
            // and linking step. For the "Last Worker" pattern, parent_index is critical.
            
            // Increment child count on parent (naive implementation for now)
            self.nodes[parent_index as usize].child_count += 1;
        }
    }

    pub fn set_child_start(&mut self, parent_index: u32, start_index: u32) {
        if (parent_index as usize) < self.nodes.len() {
            self.nodes[parent_index as usize].child_start_index = start_index;
        }
    }

    pub fn get_nodes_ptr(&self) -> *const Node {
        self.nodes.as_ptr()
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }
    
    pub fn get_node_size(&self) -> usize {
        std::mem::size_of::<Node>()
    }

    pub fn get_nodes_buffer(&self) -> js_sys::Uint8Array {
        let size = self.nodes.len() * std::mem::size_of::<Node>();
        let ptr = self.nodes.as_ptr() as *const u8;
        unsafe {
            js_sys::Uint8Array::view(std::slice::from_raw_parts(ptr, size))
        }
    }
}
