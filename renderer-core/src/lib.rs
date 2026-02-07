use wasm_bindgen::prelude::*;
use ttf_parser::{Face, GlyphId};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const FONT_DATA: &[u8] = include_bytes!("../roboto.ttf");

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GlyphData {
    pub advance: f32,
    pub bearing_x: f32,
    pub width: f32,   // Bounding box width
    pub height: f32,  // Bounding box height
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KerningRecord {
    pub left: u32,
    pub right: u32,
    pub value: f32,
    pub _pad: u32, // Padding
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
    pub text_start: u32,       // Index into character buffer
    pub text_length: u32,      // Length of the text content
    pub flex_direction: u32,   // 0 = Row, 1 = Column
    pub _padding: u32,         // Maintain 16-byte alignment
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
            text_start: 0,
            text_length: 0,
            flex_direction: 0,
            _padding: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Character {
    pub value: u32,      // unicode codepoint
    pub glyph_index: u32, // glyph index in font
    pub next_glyph_index: u32, // for kerning
    pub node_index: u32, // owner node

    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Character {
    pub fn new(val: u32, glyph: u32, next_glyph: u32, node_index: u32) -> Self {
        Self {
            value: val,
            glyph_index: glyph,
            next_glyph_index: next_glyph,
            node_index,
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

#[wasm_bindgen]
pub struct FlexEngine {
    nodes: Vec<Node>,
    characters: Vec<Character>,
    glyph_data: Vec<GlyphData>,
    kerning_table: Vec<KerningRecord>,
    #[wasm_bindgen(skip)]
    pub face: Option<Face<'static>>,
}

#[wasm_bindgen]
impl FlexEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FlexEngine {
        log("FlexEngine Initialized via WebAssembly");
        
        let mut engine = FlexEngine {
            nodes: Vec::new(),
            characters: Vec::new(),
            glyph_data: Vec::new(),
            kerning_table: Vec::new(),
            face: None,
        };
        
        engine.parse_font();
        engine
    }
    
    fn parse_font(&mut self) {
        // Simple fixed size for demo
        let face = Face::parse(FONT_DATA, 0).expect("Error parsing font");
        let units_per_em = face.units_per_em() as f32;
        let font_size = 24.0; // Fixed 24px font size for now
        let scale = font_size / units_per_em;
        
        log(&format!("Font loaded. UnitsPerEm: {}, Scale: {}", units_per_em, scale));
        
        // Resize glyph_data to num_glyphs
        let num_glyphs = face.number_of_glyphs();
        self.glyph_data.reserve(num_glyphs as usize);
        
        for id in 0..num_glyphs {
            let gid = GlyphId(id);
            let advance = face.glyph_hor_advance(gid).unwrap_or(0) as f32 * scale;
            let bbox = face.glyph_bounding_box(gid).unwrap_or(ttf_parser::Rect { x_min: 0, y_min: 0, x_max: 0, y_max: 0 });
            
            self.glyph_data.push(GlyphData {
                advance,
                bearing_x: bbox.x_min as f32 * scale,
                width: (bbox.x_max - bbox.x_min) as f32 * scale,
                height: (bbox.y_max - bbox.y_min) as f32 * scale,
            });
        }
        
        self.face = Some(face);
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
    
    pub fn set_flex_direction(&mut self, node_index: u32, direction: u32) {
        if (node_index as usize) < self.nodes.len() {
            self.nodes[node_index as usize].flex_direction = direction;
        }
    }

    pub fn set_text_length(&mut self, node_index: u32, length: u32) {
        if (node_index as usize) < self.nodes.len() {
            self.nodes[node_index as usize].text_length = length;
        }
    }

    pub fn set_text(&mut self, node_index: u32, text: &str) {
        if (node_index as usize) >= self.nodes.len() { return; }
        
        let start = self.characters.len() as u32;
        let chars_vec: Vec<char> = text.chars().collect();
        let len = chars_vec.len() as u32;
        
        for (i, &c) in chars_vec.iter().enumerate() {
            let val = c as u32;
            let glyph_id = if let Some(face) = &self.face {
                face.glyph_index(c).map(|g| g.0).unwrap_or(0)
            } else {
                0
            };
            
            let next_glyph_id = if i < chars_vec.len() - 1 {
                 if let Some(face) = &self.face {
                    face.glyph_index(chars_vec[i+1]).map(|g| g.0).unwrap_or(0)
                } else { 0 }
            } else { 0 };

            self.characters.push(Character::new(val, glyph_id as u32, next_glyph_id as u32, node_index));
        }

        self.nodes[node_index as usize].text_start = start;
        self.nodes[node_index as usize].text_length = len;
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

    pub fn get_character_count(&self) -> usize {
        self.characters.len()
    }
    
    pub fn get_character_size(&self) -> usize {
        std::mem::size_of::<Character>()
    }

    pub fn get_characters_buffer(&self) -> js_sys::Uint8Array {
        let size = self.characters.len() * std::mem::size_of::<Character>();
        let ptr = self.characters.as_ptr() as *const u8;
        unsafe {
            js_sys::Uint8Array::view(std::slice::from_raw_parts(ptr, size))
        }
    }
    
    pub fn get_glyph_data_count(&self) -> usize {
        self.glyph_data.len()
    }
    
    pub fn get_glyph_data_size(&self) -> usize {
        std::mem::size_of::<GlyphData>()
    }
    
    pub fn get_glyph_data_buffer(&self) -> js_sys::Uint8Array {
        let size = self.glyph_data.len() * std::mem::size_of::<GlyphData>();
        let ptr = self.glyph_data.as_ptr() as *const u8;
        unsafe {
            js_sys::Uint8Array::view(std::slice::from_raw_parts(ptr, size))
        }
    }
    
    // Kerning getters omitted for brevity but similar pattern if needed
}
