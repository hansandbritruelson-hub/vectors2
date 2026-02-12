use wasm_bindgen::prelude::*;
use ttf_parser::{Face, GlyphId, OutlineBuilder};
use std::collections::HashMap;

pub mod renderer;
pub mod ui;
pub mod web_bindings;
pub mod signals;
pub mod texture_atlas;
// REMOVED: pub mod generated_ui;
#[cfg(test)]
mod tests;
pub use renderer::FlexRenderer;

pub fn log(s: &str) {
    #[cfg(target_arch = "wasm32")]
    web_bindings::log(s);
    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", s);
}

// REMOVED: include!(concat!(env!("OUT_DIR"), "/generated_assets.rs"));

const FONT_DATA: &[u8] = include_bytes!("../roboto.ttf");

#[derive(Clone, Debug)]
pub enum StyleValue {
    Px(f32),
    Percent(f32),
    Em(f32),
    Vh(f32),
    Vw(f32),
    Color(f32, f32, f32, f32),
    Ident(String),
    String(String),
    Auto,
}

#[derive(Clone, Debug)]
pub struct StyleRule {
    pub selector: String,
    pub declarations: HashMap<String, StyleValue>,
}

#[derive(Default, Clone, Debug)]
pub struct StyleSheet {
    pub rules: Vec<StyleRule>,
}

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
    // --- Padding to 32 bytes (16-byte alignment for stride) ---
    pub _pad0: f32, pub _pad1: f32, pub _pad2: f32,
}

#[test]
fn test_glyph_data_size() {
    assert_eq!(std::mem::size_of::<GlyphData>(), 32);
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
pub struct GpuNode {
    // --- Layout Inputs ---
    pub fixed_width: f32,   // -1.0 = auto
    pub min_width: f32,
    pub fixed_height: f32,  // -1.0 = auto
    
    // --- Computed Values ---
    pub final_width: f32,
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

    // --- Tree Topology (GPU Linear) ---
    pub parent_index: u32,
    pub child_start_index: u32,
    pub child_count: u32,
    
    // --- Synchronization ---
    pub signals_finished: u32,
    pub text_start: u32,
    pub text_length: u32,
    pub flags: u32,       // Bit 0 = Visible, Bit 1 = Has Image
    pub natural_content_width: f32,

    // --- Texture Atlas UVs ---
    pub uv_min_x: f32, 
    pub uv_min_y: f32, 
    pub uv_max_x: f32, 
    pub uv_max_y: f32,
    
    // --- Padding to 128 bytes ---
    pub cpu_index: u32, pub _pad5: u32, pub _pad6: u32, // Removed pad7 due to fixed_height
}

#[test]
fn test_gpu_node_size() {
    assert_eq!(std::mem::size_of::<GpuNode>(), 128);
}

impl GpuNode {
    pub fn new() -> Self {
        Self {
            fixed_width: -1.0,
            min_width: 0.0,
            fixed_height: -1.0,
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
            flags: 1, // Default to 1 (Visible)
            natural_content_width: 0.0,
            uv_min_x: 0.0, uv_min_y: 0.0, uv_max_x: 0.0, uv_max_y: 0.0,
            cpu_index: 0, _pad5: 0, _pad6: 0,
        }
    }
}

// --- The Logical CPU Node (DOM) ---
#[derive(Clone)]
pub struct CpuNode {
    // Topology (Linked List)
    pub parent: Option<usize>,
    pub first_child: Option<usize>,
    pub next_sibling: Option<usize>,
    pub last_child: Option<usize>, // Optimization for append

    // Properties (Mirrored from GpuNode Inputs)
    pub fixed_width: f32,
    pub min_width: f32,
    pub fixed_height: f32,
    pub color: (f32, f32, f32, f32),
    pub top_offset: f32,
    pub left_offset: f32,
    pub z_index: Option<f32>,
    pub position_mode: u32,
    pub flex_direction: u32,
    pub flags: u32,
    
    // Text
    pub text: Option<String>,
    pub image_asset_id: Option<String>,
    
    // CSS Styles
    pub classes: Vec<String>,
    pub inline_styles: HashMap<String, StyleValue>,
    
    // Cache
    #[cfg(target_arch = "wasm32")] // Or just always
    pub cached_texture: Option<std::rc::Rc<texture_atlas::TextureHandle>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub cached_texture: Option<std::rc::Rc<texture_atlas::TextureHandle>>,
    
    // Events
    pub on_click: Option<std::rc::Rc<dyn Fn()>>,
}

impl CpuNode {
    pub fn new() -> Self {
        Self {
            parent: None,
            first_child: None,
            next_sibling: None,
            last_child: None,
            
            fixed_width: -1.0,
            min_width: 0.0,
            fixed_height: -1.0,
            color: (0.0, 0.0, 0.0, 0.0),
            top_offset: 0.0,
            left_offset: 0.0,
            z_index: None,
            position_mode: 0,
            flex_direction: 0, // Row
            flags: 1,
            
            text: None,
            image_asset_id: None,
            classes: Vec::new(),
            inline_styles: HashMap::new(),
            cached_texture: None,
            on_click: None,
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
    #[wasm_bindgen(skip)]
    pub(crate) cpu_nodes: Vec<CpuNode>, // The Logical DOM (Linked List)
    #[wasm_bindgen(skip)]
    pub(crate) gpu_nodes: Vec<GpuNode>, // The Render Tree (Flattened)
    
    #[wasm_bindgen(skip)]
    pub characters: Vec<Character>,
    #[wasm_bindgen(skip)]
    pub glyph_data: Vec<GlyphData>,
    #[wasm_bindgen(skip)]
    pub kerning_table: Vec<KerningRecord>,
    
    // New GPU Vector Graphics Data
    #[wasm_bindgen(skip)]
    pub curves: Vec<GpuCurve>,
    #[wasm_bindgen(skip)]
    pub glyph_infos: Vec<GpuGlyphInfo>,

    // Font Metrics
    pub ascender: f32,
    pub descender: f32,
    pub line_gap: f32,
    
    #[wasm_bindgen(skip)]
    pub face: Option<Face<'static>>,

    // --- Image Support (Texture Atlas) ---
    #[wasm_bindgen(skip)]
    pub texture_atlas: texture_atlas::TextureAtlas,
    #[wasm_bindgen(skip)]
    pub assets: HashMap<String, Vec<u8>>,

    #[wasm_bindgen(skip)]
    pub stylesheet: StyleSheet,

    pub dirty: bool,
}

#[wasm_bindgen]
impl FlexEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FlexEngine {
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();
        log("FlexEngine Initialized via WebAssembly (CpuNode Topology)");
        
        let mut engine = FlexEngine {
            cpu_nodes: Vec::new(),
            gpu_nodes: Vec::new(),
            characters: Vec::new(),
            glyph_data: Vec::new(),
            kerning_table: Vec::new(),
            curves: Vec::new(),
            glyph_infos: Vec::new(),
            ascender: 0.0,
            descender: 0.0,
            line_gap: 0.0,
            face: None,
            texture_atlas: texture_atlas::TextureAtlas::new(2048, 2048),
            assets: HashMap::new(),
            stylesheet: StyleSheet::default(),
            dirty: false, // Start clean, mark_dirty will be called during build_ui
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

            let advance = face.glyph_hor_advance(gid).unwrap_or(0) as f32 * scale;
            let _rect = bbox; // Alias for clarity if needed, or just use bbox
            self.glyph_data.push(GlyphData {
                advance,
                bearing_x: bbox.x_min as f32 * scale - 1.0, // 1px padding
                bearing_y: bbox.y_max as f32 * scale + 1.0,
                width: (bbox.x_max - bbox.x_min) as f32 * scale + 2.0,
                height: (bbox.y_max - bbox.y_min) as f32 * scale + 2.0,
                _pad0: 0.0, _pad1: 0.0, _pad2: 0.0,
            });
        }
        
        self.face = Some(face);
    }
    


// ...




    pub fn add_node(&mut self, min_width: f32) -> u32 {
        let mut node = CpuNode::new();
        node.min_width = min_width;
        let index = self.cpu_nodes.len() as u32;
        self.cpu_nodes.push(node);
        self.mark_dirty();
        index
    }


    // --- Topology Management (Linked List Wiring) ---

    pub fn set_parent(&mut self, child_id: u32, parent_id: u32) {
        let child_idx = child_id as usize;
        let parent_idx = parent_id as usize;
        
        if child_idx >= self.cpu_nodes.len() || parent_idx >= self.cpu_nodes.len() {
            log("Invalid node index in set_parent");
            return;
        }
        
        // 1. Unlink from old parent (if any)
        self.remove_from_parent(child_id);

        // 2. Link to new parent
        self.cpu_nodes[child_idx].parent = Some(parent_idx);
        
        // 3. Append to parent's child list
        if let Some(last_child) = self.cpu_nodes[parent_idx].last_child {
            // Parent has children. Append to last.
            self.cpu_nodes[last_child].next_sibling = Some(child_idx);
            self.cpu_nodes[parent_idx].last_child = Some(child_idx);
        } else {
            // First child
            self.cpu_nodes[parent_idx].first_child = Some(child_idx);
            self.cpu_nodes[parent_idx].last_child = Some(child_idx);
        }
        
        self.mark_dirty();
    }

    pub fn insert_after_node(&mut self, child_id: u32, parent_id: u32, after_id: Option<u32>) {
        let child_idx = child_id as usize;
        let parent_idx = parent_id as usize;
        
        if child_idx >= self.cpu_nodes.len() || parent_idx >= self.cpu_nodes.len() {
            log("Invalid node index in insert_after_node");
            return;
        }

        // Unlink
        self.remove_from_parent(child_id);

        self.cpu_nodes[child_idx].parent = Some(parent_idx);

        if let Some(after_idx) = after_id.map(|id| id as usize) {
            // Insert after specific node
            let next = self.cpu_nodes[after_idx].next_sibling;
            self.cpu_nodes[after_idx].next_sibling = Some(child_idx);
            self.cpu_nodes[child_idx].next_sibling = next;

            if next.is_none() {
                // Was last, update parent's last_child
                self.cpu_nodes[parent_idx].last_child = Some(child_idx);
            }
        } else {
            // Insert at the very beginning
            let next = self.cpu_nodes[parent_idx].first_child;
            self.cpu_nodes[parent_idx].first_child = Some(child_idx);
            self.cpu_nodes[child_idx].next_sibling = next;

            if next.is_none() {
                self.cpu_nodes[parent_idx].last_child = Some(child_idx);
            }
        }

        self.mark_dirty();
    }

    pub fn remove_from_parent(&mut self, node_id: u32) {
        let node_idx = node_id as usize;
        if node_idx >= self.cpu_nodes.len() { return; }

        let parent_idx = match self.cpu_nodes[node_idx].parent {
            Some(p) => p,
            None => return,
        };

        // Find prev sibling
        let mut prev_sibling = None;
        let mut curr = self.cpu_nodes[parent_idx].first_child;
        while let Some(curr_idx) = curr {
            if curr_idx == node_idx { break; }
            prev_sibling = Some(curr_idx);
            curr = self.cpu_nodes[curr_idx].next_sibling;
        }

        let next_sibling = self.cpu_nodes[node_idx].next_sibling;

        if let Some(prev) = prev_sibling {
            self.cpu_nodes[prev].next_sibling = next_sibling;
        } else {
            // It was the first child
            self.cpu_nodes[parent_idx].first_child = next_sibling;
        }

        if next_sibling.is_none() {
            // It was the last child
            self.cpu_nodes[parent_idx].last_child = prev_sibling;
        }

        self.cpu_nodes[node_idx].parent = None;
        self.cpu_nodes[node_idx].next_sibling = None;
        self.mark_dirty();
    }

    pub fn clear_children(&mut self, parent_id: u32) {
        let parent_idx = parent_id as usize;
        if parent_idx >= self.cpu_nodes.len() { return; }

        let mut curr = self.cpu_nodes[parent_idx].first_child;
        while let Some(child_idx) = curr {
            let next = self.cpu_nodes[child_idx].next_sibling;
            self.cpu_nodes[child_idx].parent = None;
            self.cpu_nodes[child_idx].next_sibling = None;
            curr = next;
        }

        self.cpu_nodes[parent_idx].first_child = None;
        self.cpu_nodes[parent_idx].last_child = None;
        self.mark_dirty();
    }
}

impl FlexEngine {
    pub fn add_style_rule(&mut self, selector: String, declarations: HashMap<String, StyleValue>) {
        self.stylesheet.rules.push(StyleRule { selector, declarations });
        self.mark_dirty();
    }

    fn apply_styles(&mut self, node_idx: usize) {
        let mut resolved: HashMap<String, StyleValue> = HashMap::new();

        // 1. Classes (very basic matching for now)
        let classes = self.cpu_nodes[node_idx].classes.clone();
        for class_name in &classes {
            let dot_selector = format!(".{}", class_name);
            for rule in &self.stylesheet.rules {
                if rule.selector == *dot_selector || rule.selector == *class_name {
                    for (prop, val) in &rule.declarations {
                        resolved.insert(prop.clone(), val.clone());
                    }
                }
            }
        }

        // 2. Inline overrides
        let inline = self.cpu_nodes[node_idx].inline_styles.clone();
        for (prop, val) in inline {
            resolved.insert(prop, val);
        }

        // Update CpuNode fields based on resolved styles
        for (prop, val) in resolved {
            match prop.as_str() {
                "width" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].fixed_width = v; }
                }
                "height" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].fixed_height = v; }
                }
                "color" | "background-color" => {
                    if let StyleValue::Color(r, g, b, a) = val { self.cpu_nodes[node_idx].color = (r, g, b, a); }
                }
                "flex-direction" => {
                    if let StyleValue::Ident(s) = val {
                        match s.as_str() {
                            "row" => self.cpu_nodes[node_idx].flex_direction = 0,
                            "column" => self.cpu_nodes[node_idx].flex_direction = 1,
                            _ => {}
                        }
                    }
                }
                "z-index" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].z_index = Some(v); }
                }
                "top" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].top_offset = v; }
                }
                "left" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].left_offset = v; }
                }
                "position" => {
                    if let StyleValue::Ident(s) = val {
                        match s.as_str() {
                            "relative" => self.cpu_nodes[node_idx].position_mode = 0,
                            "absolute" => self.cpu_nodes[node_idx].position_mode = 1,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // --- Topology Management (Linked List Wiring) ---

    // ... (existing topology methods)



    // Deprecated / No-op in linked list mode (implicit)
    pub fn set_child_start(&mut self, _parent_index: u32, _start_index: u32) {
        // No-op
    }

    // --- Flattening (CPU -> GPU) ---
    // This is the bridge. Rebuilds gpu_nodes from cpu_nodes.
    fn flatten(&mut self) {
        // 0. Pre-pass: Style Application & Texture Management
        self.texture_atlas.process_deletions();

        for i in 0..self.cpu_nodes.len() {
             self.apply_styles(i);
             
             // We work around borrow checker by extracting needed data first if possible,
             // or just carefully using indices.
             // We need to mutate `cached_texture`.
             
             // Clone ID to avoid borrow issues while mutating
             let (image_id, w, h) = {
                 let node = &self.cpu_nodes[i];
                 if let Some(ref id) = node.image_asset_id {
                     let w = if node.fixed_width > 0.0 { node.fixed_width as u32 } else { 64 };
                     let h = if node.fixed_height > 0.0 { node.fixed_height as u32 } else { 64 };
                     (Some(id.clone()), w, h)
                 } else {
                     (None, 0, 0)
                 }
             };

             if let Some(id) = image_id {
                 // Check if current cache is valid
                 let mut needs_update = true;
                 if let Some(handle) = &self.cpu_nodes[i].cached_texture {
                      // We assume if handle exists, it matches the ID (since we clear on ID change)
                      // We just check dims.
                      if handle.region.width == w && handle.region.height == h {
                          needs_update = false;
                      }
                 }

                 if needs_update {
                      // Try to get existing handle from Atlas Cache
                      let key = texture_atlas::CacheKey { id: id.clone(), width: w, height: h };
                      
                      let new_handle = if let Some(h) = self.texture_atlas.get_handle(&key) {
                           Some(h)
                      } else {
                           // Not in atlas. Need to load/resize/allocate.
                           if let Some(bytes) = self.assets.get(&id).cloned() {
                                let is_svg = id.ends_with(".svg") || (bytes.len() > 4 && bytes.as_slice().starts_with(b"<svg"));
                                let pixels: Option<Vec<u8>> = if is_svg {
                                     let opt = usvg::Options::default();
                                     if let Ok(tree) = usvg::Tree::from_data(&bytes, &opt) {
                                          let mut pixmap = tiny_skia::Pixmap::new(w, h).unwrap_or(tiny_skia::Pixmap::new(1, 1).unwrap());
                                          let current_w = tree.size.width();
                                          let current_h = tree.size.height();
                                          let sx = w as f32 / current_w as f32;
                                          let sy = h as f32 / current_h as f32;
                                          let ts = tiny_skia::Transform::from_scale(sx, sy);
                                          resvg::render(&tree, usvg::FitTo::Size(w, h), ts, pixmap.as_mut());
                                          Some(pixmap.data().to_vec())
                                     } else {  log(&format!("Failed to parse SVG for atlas: {}", id)); None }
                                } else {
                                     if let Ok(img) = image::load_from_memory(&bytes) {
                                          let resized = img.resize_exact(w, h, image::imageops::FilterType::Lanczos3);
                                          Some(resized.to_rgba8().into_raw())
                                     } else { log(&format!("Failed to load image for atlas: {}", id)); None }
                                };
                                
                                if let Some(p) = pixels {
                                    self.texture_atlas.allocate(key, p)
                                } else {
                                    None
                                }
                           } else {
                               // Asset not found yet
                               None
                           }
                      };
                      
                      self.cpu_nodes[i].cached_texture = new_handle;
                 }
             }
        }

        self.gpu_nodes.clear();
        self.characters.clear(); // Rebuild chars too since they depend on node index
        
        // We need to map CPU Index -> GPU Index to fix up text/chars
        // But for now, let's just Traverse.
        
        // Queue for BFS or Stack for DFS?
        // Layout engine (compute shader) expects:
        // - Parents appear before children? (Top Down passes)
        // - Children of a parent are contiguous? (Bottom Up width summation)
        // YES. Contiguous children is the Hard Constraint.
        // DFS Pre-order traversal does NOT guarantee contiguous children in global array?
        // Wait. Parent A. Children B, C.
        // DFS: A, B, [B's children...], C...
        // In array: [A, B, ..., C] -> B and C are NOT contiguous. C is far away.
        
        // Compute Shader `width_bottom_up` loops `start` to `start + count`.
        // It IMPLICITLY assumes `nodes[start + i]` accesses the i-th child.
        // This means children MUST be contiguous in the buffer.
        
        // PROBLEM: DFS Pre-order separates siblings by the entire subtree of the sibling.
        // SOLUTION: Layout order must be: [Parent, Child1, Child2, Child3, Child1_Subtree...] ?
        // No. If Parent refers to `start`, and loop iterates `count`, then Child1...ChildN MUST be adjacent.
        // [Parent, Child1, Child2, Child3, ... Grandchildren ... ]
        // This implies Breadth-First-ish grouping?
        
        // Structure:
        // [Roots...]
        // [Layer 1 Children...]
        // [Layer 2 Children...]
        
        // BUT: Parent needs to know `child_start_index`.
        // If we put all Layer 1 children together, they are contiguous.
        // Parent A (at 0) has children B, C (at 10, 11).
        // Parent D (at 1) has children E, F (at 12, 13).
        // This works! BFS Layout.
        
        // Algorithm:
        // 1. Queue<CpuIndex>
        // 2. While Queue not empty:
        //    Pop Parent.
        //    If Parent has children:
        //       Record `child_start` = current_gpu_len.
        //       Iterate list (first_child -> next_sibling):
        //           Push Child to Queue.
        //           Append Child to gpu_nodes.
        //           (Map cpu_idx -> gpu_idx for referencing if needed)
        
        // Wait, `parent_index` in GpuNode points back to Parent.
        // If we do BFS, Parent is already placed. We know its GPU index.
        
        // Let's implement BFS Flattening.
        
        // Map CPU Node Index -> GPU Node Index
        let mut cpu_to_gpu: std::collections::HashMap<usize, u32> = std::collections::HashMap::new();
        
        if self.cpu_nodes.is_empty() { return; }
        
        // We assume Node 0 is Root?
        // Let's find all roots (nodes with no parent).
        // For simple single-root UI:
        let root_idx = 0; // Assumption
        
        // To handle "forests", we might iterate all, but let's assume single root for now.
        
        // Step 1: Push Root
        self.gpu_nodes.push(GpuNode::new()); // Placeholder for Root
        // We'll update Root's data later.
        cpu_to_gpu.insert(root_idx, 0);
        
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root_idx);
        
        while let Some(cpu_idx) = queue.pop_front() {
            let gpu_idx = *cpu_to_gpu.get(&cpu_idx).unwrap();
            
            // 1. Copy Data from CPU Node to GPU Node
            // (We do this here to ensure mapped index is ready)
            let cpu_node = &self.cpu_nodes[cpu_idx];
            
            // We need to mutate the GPU node which is already in the vec
            // But we also need to append children, which might realloc.
            // Safe indices.
            
            // Calculate Children Range
            let start_child_gpu_idx = self.gpu_nodes.len() as u32;
            let mut child_count = 0;
            
            // Iterate Children to reserve/push placeholders
            let mut curr_child = cpu_node.first_child;
            while let Some(child_cpu_idx) = curr_child {
                let kid_gpu_idx = self.gpu_nodes.len() as u32;
                self.gpu_nodes.push(GpuNode::new()); // Placeholder
                cpu_to_gpu.insert(child_cpu_idx, kid_gpu_idx);
                
                queue.push_back(child_cpu_idx);
                
                child_count += 1;
                curr_child = self.cpu_nodes[child_cpu_idx].next_sibling;
            }
            
            // Now update the Parent Node with child info
            let mut parent_z = 0.0;
            let mut parent_gpu_idx = 0;
            if let Some(p_cpu) = cpu_node.parent {
                if let Some(&p_gpu) = cpu_to_gpu.get(&p_cpu) {
                     parent_gpu_idx = p_gpu;
                     parent_z = self.gpu_nodes[p_gpu as usize].z_index;
                }
            }

            {
                let gpu_node = &mut self.gpu_nodes[gpu_idx as usize];
                // Mirror Props
                gpu_node.fixed_width = cpu_node.fixed_width;
                gpu_node.min_width = cpu_node.min_width;
                gpu_node.fixed_height = cpu_node.fixed_height;
                gpu_node.color_r = cpu_node.color.0;
                gpu_node.color_g = cpu_node.color.1;
                gpu_node.color_b = cpu_node.color.2;
                gpu_node.color_a = cpu_node.color.3;
                gpu_node.top_offset = cpu_node.top_offset;
                gpu_node.left_offset = cpu_node.left_offset;
                gpu_node.position_mode = cpu_node.position_mode;
                gpu_node.flex_direction = cpu_node.flex_direction;
                gpu_node.flags = cpu_node.flags; // Visibility
                gpu_node.cpu_index = cpu_idx as u32;
                
                // Topology
                gpu_node.child_start_index = start_child_gpu_idx;
                gpu_node.child_count = child_count;
                
                // Parent Ref
                gpu_node.parent_index = if cpu_node.parent.is_some() { parent_gpu_idx } else { 0 };

                gpu_node.z_index = cpu_node.z_index.unwrap_or(parent_z);
                
                // Image UV Resolution
                if let Some(handle) = &cpu_node.cached_texture {
                    let region = &handle.region;
                    gpu_node.uv_min_x = region.u_min;
                    gpu_node.uv_min_y = region.v_min;
                    gpu_node.uv_max_x = region.u_max;
                    gpu_node.uv_max_y = region.v_max;
                    gpu_node.flags |= 2; // Ensure flag is set
                }
                
                // Text Handling (Rebuild Characters)
                if let Some(text_content) = &cpu_node.text {
                     let chars_start = self.characters.len() as u32;
                     let chars_vec: Vec<char> = text_content.chars().collect();
                     let chars_len = chars_vec.len() as u32;
                     

                     
                     for (i, &c) in chars_vec.iter().enumerate() {
                        let val = c as u32;
                        let glyph_id = if let Some(face) = &self.face {
                            face.glyph_index(c).map(|g| g.0).unwrap_or(0)
                        } else { 0 };
                        
                        let next_glyph_id = if i < chars_vec.len() - 1 {
                             if let Some(face) = &self.face {
                                face.glyph_index(chars_vec[i+1]).map(|g| g.0).unwrap_or(0)
                            } else { 0 }
                        } else { 0 };
            
                        self.characters.push(Character::new(val, glyph_id as u32, next_glyph_id as u32, gpu_idx));
                        

                     }
                     
                     gpu_node.text_start = chars_start;
                     gpu_node.text_length = chars_len;
                }
            }
        }
    }
    
    pub fn set_flex_direction(&mut self, node_index: u32, direction: u32) {
        if (node_index as usize) < self.cpu_nodes.len() {
            self.cpu_nodes[node_index as usize].flex_direction = direction;
            self.mark_dirty();
        }
    }

    pub fn set_fixed_width(&mut self, node_index: u32, width: f32) {
        if (node_index as usize) < self.cpu_nodes.len() {
            self.cpu_nodes[node_index as usize].fixed_width = width;
            self.mark_dirty();
        }
    }
    
    pub fn set_fixed_height(&mut self, node_index: u32, height: f32) {
        if (node_index as usize) < self.cpu_nodes.len() {
            self.cpu_nodes[node_index as usize].fixed_height = height;
            self.mark_dirty();
        }
    }

    pub fn set_flags(&mut self, node_index: u32, flags: u32) {
        if (node_index as usize) < self.cpu_nodes.len() {
            self.cpu_nodes[node_index as usize].flags = flags;
            self.mark_dirty();
        }
    }
    
    // New Setters
    pub fn set_color(&mut self, node_index: u32, r: f32, g: f32, b: f32, a: f32) {
        if (node_index as usize) < self.cpu_nodes.len() {
            self.cpu_nodes[node_index as usize].color = (r, g, b, a);
            self.mark_dirty();
        }
    }

    pub fn set_position_absolute(&mut self, node_index: u32, top: f32, left: f32) {
        if (node_index as usize) < self.cpu_nodes.len() {
            let node = &mut self.cpu_nodes[node_index as usize];
            node.position_mode = 1;
            node.top_offset = top;
            node.left_offset = left;
            self.mark_dirty();
        }
    }

    pub fn set_z_index(&mut self, node_index: u32, z_index: f32) {
        if (node_index as usize) < self.cpu_nodes.len() {
            self.cpu_nodes[node_index as usize].z_index = Some(z_index);
            self.mark_dirty();
        }
    }

    pub fn set_text_length(&mut self, _node_index: u32, _length: u32) {
        // Automatically handled in flatten now
    }
}

#[wasm_bindgen]
impl FlexEngine {
    pub fn set_text(&mut self, node_index: u32, text: &str) {
        if (node_index as usize) >= self.cpu_nodes.len() { return; }
        self.cpu_nodes[node_index as usize].text = Some(text.to_string());
        self.mark_dirty();
    }

    pub fn set_image_asset_id(&mut self, node_index: u32, asset_id: &str) {
        if (node_index as usize) >= self.cpu_nodes.len() { return; }
        self.cpu_nodes[node_index as usize].image_asset_id = Some(asset_id.to_string());
        // Clear value cache to force re-fetch
        self.cpu_nodes[node_index as usize].cached_texture = None;
        
        self.cpu_nodes[node_index as usize].flags |= 2;
        self.mark_dirty();
    }

    pub fn get_nodes_ptr(&self) -> *const GpuNode {
        // Panic if we access before flatten? Or just return?
        self.gpu_nodes.as_ptr()
    }

    pub fn get_node_count(&self) -> usize {
        // Return GPU node count so the renderer allocates enough buffer
        // This implies render() must be called BEFORE this.
        self.gpu_nodes.len()
    }
    
    pub fn get_node_size(&self) -> usize {
        std::mem::size_of::<GpuNode>()
    }

    pub fn get_nodes_buffer(&self) -> js_sys::Uint8Array {
        // We MUST assume render() was called or call it here.
        // But render() is likely usually called by the JS loop?
        // Let's call flatten just in case dirty.
        // self.render(); // recursive ref cell issue potentially if not careful?
        // Assuming renderer calls render() then asks for buffer.
        
        let size = self.gpu_nodes.len() * std::mem::size_of::<GpuNode>();
        let ptr = self.gpu_nodes.as_ptr() as *const u8;
        unsafe {
            js_sys::Uint8Array::view(std::slice::from_raw_parts(ptr, size))
        }
    }
    
    // --- Public Render Entry ---
    pub fn render(&mut self) {
        // Renamed from internal implicit to explicit
        if self.dirty {
            self.flatten();
            self.dirty = false;
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

    
    pub fn update_node_flags(&mut self, node_id: u32, flags: u32) {
        if (node_id as usize) < self.cpu_nodes.len() {
             self.cpu_nodes[node_id as usize].flags = flags;
             self.mark_dirty();
        }
    }
    
     pub fn update_node_color(&mut self, node_id: u32, r: f32, g: f32, b: f32, a: f32) {
              let node = &mut self.cpu_nodes[node_id as usize];
              node.color = (r, g, b, a);
              self.mark_dirty();
          }
    
    // Kerning getters omitted
    
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    
    pub fn mark_dirty(&mut self) {
        if !self.dirty {
            self.dirty = true;
            #[cfg(target_arch = "wasm32")]
            web_bindings::request_render_frame();
        }
    }
    
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

impl FlexEngine {
    pub fn set_on_click(&mut self, node_id: u32, f: std::rc::Rc<dyn Fn()>) {
        if let Some(node) = self.cpu_nodes.get_mut(node_id as usize) {
            node.on_click = Some(f);
        }
    }
}

#[wasm_bindgen]
pub fn render_svg(_svg_content: &str, width: u32, height: u32) -> Vec<u8> {
    log(&format!("Rendering SVG (Disabled for migration): {}x{}", width, height));
    Vec::new()
    /*
    let opt = Options::default();
    // ... (rest of the code)
    */
}


pub async fn load_image_to_engine(engine: std::rc::Rc<std::cell::RefCell<FlexEngine>>, url: String) {
    let bytes: Vec<u8>;

    if url.starts_with("asset:") {
         log("Renderer does not handle asset: URLs internally anymore. Use load_asset_bytes from App.");
         return;
    } else {
        // HTTP Download
        // log(&format!("Downloading image: {}", url));
        let promise = crate::web_bindings::download_image(&url);
        let js_val = wasm_bindgen_futures::JsFuture::from(promise).await;
        
        if let Ok(val) = js_val {
            let uint8_array = js_sys::Uint8Array::new(&val);
            bytes = uint8_array.to_vec();
        } else {
            log("Failed to download image");
            return;
        }
    }

    // Determine if SVG or Image
    // Simple check: extension or magic bytes. 
    // Since we control assets, extension is fine. For HTTP, we might need to guess.
    // Let's check if it looks like SVG (starts with <svg or has .svg extension in URL)
    
    // Simplified: Just store the source bytes. Flatten will handle resizing/rendering.
    // Determine ID from URL
    let id = url.clone(); 
    
    engine.borrow_mut().load_asset_bytes(&id, bytes);
}

#[wasm_bindgen]
impl FlexEngine {
    pub fn loaded_assets(&self) -> Vec<String> {
        self.assets.keys().cloned().collect()
    }
}

// --- Internal Methods (Not exposed to JS) ---
impl FlexEngine {
    pub fn handle_click(&mut self, x: f32, y: f32) -> Vec<std::rc::Rc<dyn Fn()>> {
        let mut hit_idx = None;
        for i in (0..self.gpu_nodes.len()).rev() {
            let n = &self.gpu_nodes[i];
            
            // Skip invisible if flag bit 0 is Not set
            if (n.flags & 1) == 0 { continue; }

            let left = n.final_x;
            let top = n.final_y;
            let right = left + n.final_width;
            let bottom = top + n.final_height;

            if x >= left && x <= right && y >= top && y <= bottom {
                hit_idx = Some(i);
                break;
            }
        }

        let mut callbacks = Vec::new();

        if let Some(gpu_idx) = hit_idx {
            let initial_cpu_idx = self.gpu_nodes[gpu_idx].cpu_index as usize;
            
            // Event Bubbling
            let mut current_cpu_idx = Some(initial_cpu_idx);

            while let Some(cpu_idx) = current_cpu_idx {
                if let Some(node) = self.cpu_nodes.get(cpu_idx) {
                    if let Some(cb) = &node.on_click {
                        callbacks.push(cb.clone());
                        break;
                    }
                    current_cpu_idx = node.parent;
                } else {
                    break;
                }
            }
        }

        callbacks
    }

    pub fn add_image_to_atlas(&mut self, id: String, width: u32, height: u32, data: Vec<u8>) -> Option<std::rc::Rc<texture_atlas::TextureHandle>> {
        let key = texture_atlas::CacheKey { id, width, height };
        let handle = self.texture_atlas.allocate(key, data);
        if handle.is_some() {
            self.mark_dirty();
        }
        handle
    }

    pub fn get_asset(&self, id: &str) -> Option<Vec<u8>> {
        self.assets.get(id).cloned()
    }

    pub fn assign_image_to_node(&mut self, node_id: u32, _region: texture_atlas::AtlasRegion) {
        let idx = node_id as usize;
        if idx < self.cpu_nodes.len() {
           // We need to store this in CPU node?
           // Currently CpuNode doesn't have UVs.
           // We should add UVs to CpuNode to mirror GpuNode.
        }
    }
    
    // Stores raw asset data for resizing later
    pub fn load_asset_bytes(&mut self, id: &str, data: Vec<u8>) {
         self.assets.insert(id.to_string(), data);
    }
    
    pub fn get_asset_data(&self, id: &str) -> Option<&Vec<u8>> {
        self.assets.get(id)
    }
}
