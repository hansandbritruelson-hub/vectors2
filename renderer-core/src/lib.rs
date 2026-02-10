use wasm_bindgen::prelude::*;
use ttf_parser::{Face, GlyphId, OutlineBuilder};
use tiny_skia::{Pixmap, Transform};
// use usvg::{Options, Tree, FitTo};

pub mod renderer;
pub mod ui;
pub use renderer::FlexRenderer;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const FONT_DATA: &[u8] = include_bytes!("../roboto.ttf");

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GpuCurve {
    pub p0_x: f32, pub p0_y: f32,
    pub p1_x: f32, pub p1_y: f32,
    pub p2_x: f32, pub p2_y: f32,
    pub p3_x: f32, pub p3_y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GpuGlyphInfo {
    pub start_index: u32,
    pub count: u32,
    pub _pad0: u32,
    pub _pad1: u32,
}

struct PathCollector {
    curves: Vec<GpuCurve>,
    scale: f32,
    x_min: f32,
    y_max: f32,
    start_x: f32,
    start_y: f32,
    current_x: f32,
    current_y: f32,
}

impl PathCollector {
    fn new(scale: f32, x_min: f32, y_max: f32) -> Self {
        Self {
            curves: Vec::new(),
            scale,
            x_min,
            y_max,
            start_x: 0.0,
            start_y: 0.0,
            current_x: 0.0,
            current_y: 0.0,
        }
    }

    fn tx(&self, x: f32) -> f32 { (x - self.x_min) * self.scale }
    fn ty(&self, y: f32) -> f32 { (self.y_max - y) * self.scale }
    
    fn add_line(&mut self, x: f32, y: f32) {
        let p0_x = self.current_x;
        let p0_y = self.current_y;
        let p3_x = self.tx(x);
        let p3_y = self.ty(y);

        // Represent line as cubic
        let p1_x = p0_x + (p3_x - p0_x) / 3.0;
        let p1_y = p0_y + (p3_y - p0_y) / 3.0;
        let p2_x = p0_x + 2.0 * (p3_x - p0_x) / 3.0;
        let p2_y = p0_y + 2.0 * (p3_y - p0_y) / 3.0;

        self.curves.push(GpuCurve {
            p0_x, p0_y,
            p1_x, p1_y,
            p2_x, p2_y,
            p3_x, p3_y,
        });
        self.current_x = p3_x;
        self.current_y = p3_y;
    }
    
    fn add_quad(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let p0_x = self.current_x;
        let p0_y = self.current_y;
        let cp_x = self.tx(x1);
        let cp_y = self.ty(y1);
        let p3_x = self.tx(x);
        let p3_y = self.ty(y);

        // Represent quadratic as cubic
        let p1_x = p0_x + 2.0 * (cp_x - p0_x) / 3.0;
        let p1_y = p0_y + 2.0 * (cp_y - p0_y) / 3.0;
        let p2_x = p3_x + 2.0 * (cp_x - p3_x) / 3.0;
        let p2_y = p3_y + 2.0 * (cp_y - p3_y) / 3.0;
        
        self.curves.push(GpuCurve {
            p0_x, p0_y,
            p1_x, p1_y,
            p2_x, p2_y,
            p3_x, p3_y,
        });
        self.current_x = p3_x;
        self.current_y = p3_y;
    }

    fn add_cubic(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let p0_x = self.current_x;
        let p0_y = self.current_y;
        let p1_x = self.tx(x1);
        let p1_y = self.ty(y1);
        let p2_x = self.tx(x2);
        let p2_y = self.ty(y2);
        let p3_x = self.tx(x);
        let p3_y = self.ty(y);

        self.curves.push(GpuCurve {
            p0_x, p0_y,
            p1_x, p1_y,
            p2_x, p2_y,
            p3_x, p3_y,
        });
        self.current_x = p3_x;
        self.current_y = p3_y;
    }
}

impl OutlineBuilder for PathCollector {
    fn move_to(&mut self, x: f32, y: f32) {
        self.current_x = self.tx(x);
        self.current_y = self.ty(y);
        self.start_x = self.current_x;
        self.start_y = self.current_y;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.add_line(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.add_quad(x1, y1, x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.add_cubic(x1, y1, x2, y2, x, y);
    }

    fn close(&mut self) {
        // If not at start, close it
        if (self.current_x - self.start_x).abs() > 0.001 || (self.current_y - self.start_y).abs() > 0.001 {
             let p0_x = self.current_x;
             let p0_y = self.current_y;
             let p3_x = self.start_x;
             let p3_y = self.start_y;
             
             // Represent line as cubic
             let p1_x = p0_x + (p3_x - p0_x) / 3.0;
             let p1_y = p0_y + (p3_y - p0_y) / 3.0;
             let p2_x = p0_x + 2.0 * (p3_x - p0_x) / 3.0;
             let p2_y = p0_y + 2.0 * (p3_y - p0_y) / 3.0;
             
             self.curves.push(GpuCurve {
                p0_x, p0_y,
                p1_x, p1_y,
                p2_x, p2_y,
                p3_x, p3_y,
             });
             self.current_x = p3_x;
             self.current_y = p3_y;
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GlyphData {
    pub advance: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
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
    // --- Layout Inputs ---
    pub fixed_width: f32,   // -1.0 = auto
    pub style_basis: f32,
    
    // --- Computed Values (Phase A - Width) ---
    pub desired_width: f32,
    pub final_width: f32,

    // --- Computed Values (Phase B - Height/Pos) ---
    pub desired_height: f32,
    pub final_height: f32,
    pub final_x: f32,
    pub final_y: f32,

    // --- Visuals ---
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    pub color_a: f32,

    // --- Positioning ---
    pub top_offset: f32,
    pub left_offset: f32,
    pub z_index: f32,
    pub position_mode: u32, // 0 = Relative, 1 = Absolute
    pub flex_direction: u32, // 0 = Row, 1 = Column

    // --- Tree Topology ---
    pub parent_index: u32,
    pub child_start_index: u32,
    pub child_count: u32,
    
    // --- Synchronization ---
    pub signals_finished: u32,
    pub text_start: u32,
    pub text_length: u32,
    pub _pad0: u32,
    pub _pad1: u32,
}

impl Node {
    pub fn new() -> Self {
        Self {
            fixed_width: -1.0,
            style_basis: 0.0,
            desired_width: 0.0,
            final_width: 0.0,
            desired_height: 0.0,
            final_height: 0.0,
            final_x: 0.0,
            final_y: 0.0,
            color_r: 0.0,
            color_g: 0.0,
            color_b: 0.0,
            color_a: 0.0,
            top_offset: 0.0,
            left_offset: 0.0,
            z_index: 0.0,
            position_mode: 0,
            flex_direction: 0,
            parent_index: 0,
            child_start_index: 0,
            child_count: 0,
            signals_finished: 0,
            text_start: 0,
            text_length: 0,
            _pad0: 0,
            _pad1: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Character {
    pub value: u32,
    pub glyph_index: u32,
    pub next_glyph_index: u32,
    pub node_index: u32,

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
    
    // New GPU Vector Graphics Data
    curves: Vec<GpuCurve>,
    glyph_infos: Vec<GpuGlyphInfo>,

    // Font Metrics
    pub ascender: f32,
    pub descender: f32,
    pub line_gap: f32,
    
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
            curves: Vec::new(),
            glyph_infos: Vec::new(),
            ascender: 0.0,
            descender: 0.0,
            line_gap: 0.0,
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

        self.ascender = face.ascender() as f32 * scale;
        self.descender = face.descender() as f32 * scale;
        self.line_gap = face.line_gap() as f32 * scale;
        
        log(&format!("Font loaded. UnitsPerEm: {}, Scale: {}", units_per_em, scale));
        log(&format!("Metrics: Ascender: {}, Descender: {}, LineGap: {}", self.ascender, self.descender, self.line_gap));
        
        // Resize glyph_data to num_glyphs
        let num_glyphs = face.number_of_glyphs();
        self.glyph_data.reserve(num_glyphs as usize);
        self.glyph_infos.reserve(num_glyphs as usize);
        
        for id in 0..num_glyphs {
            let gid = GlyphId(id);
            let advance = face.glyph_hor_advance(gid).unwrap_or(0) as f32 * scale;
            let bbox = face.glyph_bounding_box(gid).unwrap_or(ttf_parser::Rect { x_min: 0, y_min: 0, x_max: 0, y_max: 0 });
            
            // Generate Curves
            let start_index = self.curves.len() as u32;
            let mut collector = PathCollector::new(scale, bbox.x_min as f32, bbox.y_max as f32);
            if let Some(_) = face.outline_glyph(gid, &mut collector) {
                // collected
            }
            self.curves.extend(collector.curves);
            let end_index = self.curves.len() as u32;

            self.glyph_infos.push(GpuGlyphInfo {
                start_index,
                count: end_index - start_index,
                _pad0: 0, _pad1: 0,
            });

            self.glyph_data.push(GlyphData {
                advance,
                bearing_x: bbox.x_min as f32 * scale - 1.0, // 1px padding
                bearing_y: bbox.y_max as f32 * scale + 1.0,
                width: (bbox.x_max - bbox.x_min) as f32 * scale + 2.0,
                height: (bbox.y_max - bbox.y_min) as f32 * scale + 2.0,
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

    pub fn set_fixed_width(&mut self, node_index: u32, width: f32) {
        if (node_index as usize) < self.nodes.len() {
            self.nodes[node_index as usize].fixed_width = width;
        }
    }
    
    // New Setters
    pub fn set_color(&mut self, node_index: u32, r: f32, g: f32, b: f32, a: f32) {
        if (node_index as usize) < self.nodes.len() {
            let node = &mut self.nodes[node_index as usize];
            node.color_r = r;
            node.color_g = g;
            node.color_b = b;
            node.color_a = a;
        }
    }

    pub fn set_position_absolute(&mut self, node_index: u32, top: f32, left: f32) {
        if (node_index as usize) < self.nodes.len() {
            let node = &mut self.nodes[node_index as usize];
            node.position_mode = 1;
            node.top_offset = top;
            node.left_offset = left;
        }
    }

    pub fn set_z_index(&mut self, node_index: u32, z_index: f32) {
        if (node_index as usize) < self.nodes.len() {
            self.nodes[node_index as usize].z_index = z_index;
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

    // --- New Getters ---

    pub fn get_curve_buffer(&self) -> js_sys::Uint8Array {
        let size = self.curves.len() * std::mem::size_of::<GpuCurve>();
        let ptr = self.curves.as_ptr() as *const u8;
        unsafe {
            js_sys::Uint8Array::view(std::slice::from_raw_parts(ptr, size))
        }
    }

    pub fn get_glyph_info_buffer(&self) -> js_sys::Uint8Array {
        let size = self.glyph_infos.len() * std::mem::size_of::<GpuGlyphInfo>();
        let ptr = self.glyph_infos.as_ptr() as *const u8;
        unsafe {
            js_sys::Uint8Array::view(std::slice::from_raw_parts(ptr, size))
        }
    }

    pub fn get_ascender(&self) -> f32 {
        self.ascender
    }

    pub fn get_descender(&self) -> f32 {
        self.descender
    }

    pub fn get_line_gap(&self) -> f32 {
        self.line_gap
    }

    
    // Kerning getters omitted for brevity but similar pattern if needed
}

#[wasm_bindgen]
pub fn render_svg(svg_content: &str, width: u32, height: u32) -> Vec<u8> {
    log(&format!("Rendering SVG (Disabled for migration): {}x{}", width, height));
    Vec::new()
    /*
    let opt = Options::default();
    // ... (rest of the code)
    */
}
