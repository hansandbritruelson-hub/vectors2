use wasm_bindgen::prelude::*;
use ttf_parser::{Face, GlyphId, OutlineBuilder};
use std::collections::HashMap;

pub mod renderer;
pub mod ui;
pub mod web_bindings;
pub mod signals;
pub mod texture_atlas;
// REMOVED: pub mod generated_ui;
mod style_constants {
    include!(concat!(env!("OUT_DIR"), "/style_constants.rs"));
}
use style_constants::*;

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

    // For font parsing (y-up, variable scale)
    fn tx_font(&self, x: f32) -> f32 { (x - self.x_min) * self.scale }
    fn ty_font(&self, y: f32) -> f32 { (self.y_max - y) * self.scale }

    // For shape parsing (y-down, 1:1 scale usually)
    fn tx_shape(&self, x: f32) -> f32 { x }
    fn ty_shape(&self, y: f32) -> f32 { y }
    
    fn add_line(&mut self, x: f32, y: f32, is_font: bool) {
        let p0_x = self.current_x;
        let p0_y = self.current_y;
        let p3_x = if is_font { self.tx_font(x) } else { self.tx_shape(x) };
        let p3_y = if is_font { self.ty_font(y) } else { self.ty_shape(y) };

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
    
    fn add_quad(&mut self, x1: f32, y1: f32, x: f32, y: f32, is_font: bool) {
        let p0_x = self.current_x;
        let p0_y = self.current_y;
        let cp_x = if is_font { self.tx_font(x1) } else { self.tx_shape(x1) };
        let cp_y = if is_font { self.ty_font(y1) } else { self.ty_shape(y1) };
        let p3_x = if is_font { self.tx_font(x) } else { self.tx_shape(x) };
        let p3_y = if is_font { self.ty_font(y) } else { self.ty_shape(y) };

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

    fn add_cubic(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32, is_font: bool) {
        let p0_x = self.current_x;
        let p0_y = self.current_y;
        let p1_x = if is_font { self.tx_font(x1) } else { self.tx_shape(x1) };
        let p1_y = if is_font { self.ty_font(y1) } else { self.ty_shape(y1) };
        let p2_x = if is_font { self.tx_font(x2) } else { self.tx_shape(x2) };
        let p2_y = if is_font { self.ty_font(y2) } else { self.ty_shape(y2) };
        let p3_x = if is_font { self.tx_font(x) } else { self.tx_shape(x) };
        let p3_y = if is_font { self.ty_font(y) } else { self.ty_shape(y) };

        self.curves.push(GpuCurve {
            p0_x, p0_y,
            p1_x, p1_y,
            p2_x, p2_y,
            p3_x, p3_y,
        });
        self.current_x = p3_x;
        self.current_y = p3_y;
    }

    fn move_to_pos(&mut self, x: f32, y: f32, is_font: bool) {
        self.current_x = if is_font { self.tx_font(x) } else { self.tx_shape(x) };
        self.current_y = if is_font { self.ty_font(y) } else { self.ty_shape(y) };
        self.start_x = self.current_x;
        self.start_y = self.current_y;
    }
}

impl OutlineBuilder for PathCollector {
    fn move_to(&mut self, x: f32, y: f32) {
        self.move_to_pos(x, y, true);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.add_line(x, y, true);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.add_quad(x1, y1, x, y, true);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.add_cubic(x1, y1, x2, y2, x, y, true);
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

impl PathCollector {
    fn parse_svg_path(&mut self, path: &str) {
        let mut chars = path.chars().peekable();
        
        // Helper to read float consuming chars
        fn read_float(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<f32> {
            // skip ws
            while let Some(c) = chars.peek() {
                 if c.is_whitespace() || *c == ',' { chars.next(); } else { break; }
            }
            
            let mut s = String::new();
            if let Some(&c) = chars.peek() {
                if c == '-' || c == '+' { s.push(chars.next()?); }
            }
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' {
                    s.push(chars.next()?);
                } else {
                    break;
                }
            }
            if s.is_empty() { return None; }
            s.parse::<f32>().ok()
        }

        let mut cmd = ' ';
        
        loop {
            // Skip WS
            while let Some(c) = chars.peek() {
                 if c.is_whitespace() || *c == ',' { chars.next(); } else { break; }
            }
            
            if chars.peek().is_none() { break; }
            
            if let Some(&c) = chars.peek() {
                if c.is_ascii_alphabetic() {
                    cmd = chars.next().unwrap();
                }
            }
            
            match cmd {
                'M' => {
                     let x = read_float(&mut chars).unwrap_or(0.0);
                     let y = read_float(&mut chars).unwrap_or(0.0);
                     self.move_to_pos(x, y, false);
                }
                'L' => {
                     let x = read_float(&mut chars).unwrap_or(0.0);
                     let y = read_float(&mut chars).unwrap_or(0.0);
                     self.add_line(x, y, false);
                }
                'Q' => {
                     let x1 = read_float(&mut chars).unwrap_or(0.0);
                     let y1 = read_float(&mut chars).unwrap_or(0.0);
                     let x = read_float(&mut chars).unwrap_or(0.0);
                     let y = read_float(&mut chars).unwrap_or(0.0);
                     self.add_quad(x1, y1, x, y, false);
                }
                'C' => {
                     let x1 = read_float(&mut chars).unwrap_or(0.0);
                     let y1 = read_float(&mut chars).unwrap_or(0.0);
                     let x2 = read_float(&mut chars).unwrap_or(0.0);
                     let y2 = read_float(&mut chars).unwrap_or(0.0);
                     let x = read_float(&mut chars).unwrap_or(0.0);
                     let y = read_float(&mut chars).unwrap_or(0.0);
                     self.add_cubic(x1, y1, x2, y2, x, y, false);
                }
                'Z' | 'z' => {
                     self.close();
                     // Z doesn't consume args, so next loop will pick up next cmd
                     // But if there is no next cmd, we might loop forever if we enforce implicit cmd?
                     // 'Z' typically resets implicit command to Move/Line?
                     cmd = ' '; // Reset cmd to force explicit next command or exit
                }
                ' ' => {
                    // No command yet? consumes 1 char to advance
                     chars.next();
                }
                _ => {
                    // Unknown, skip
                    chars.next();
                }
            }
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
    pub max_width: f32,     // -1.0 = none
    pub fixed_height: f32,  // -1.0 = auto
    pub min_height: f32,
    pub max_height: f32,    // -1.0 = none
    
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
    pub justify_content: u32, // 0 = flex-start, 1 = center, 2 = flex-end, 3 = space-between, 4 = space-around, 5 = space-evenly
    pub align_items: u32, // 0 = stretch, 1 = flex-start, 2 = center, 3 = flex-end

    // --- Tree Topology (GPU Linear) ---
    pub parent_index: u32,
    pub child_start_index: u32,
    pub child_count: u32,
    
    // --- Synchronization ---
    pub signals_finished: u32,
    pub text_start: u32,
    pub text_length: u32,
    pub flags: u32,       // Bit 0 = Visible, Bit 1 = Has Image, Bit 2 = Is Shape, Bit 3 = Is Input, Bit 4 = Hovered
    pub natural_content_width: f32,

    // --- Texture Atlas UVs ---
    pub uv_min_x: f32, 
    pub uv_min_y: f32, 
    pub uv_max_x: f32, 
    pub uv_max_y: f32,
    
    // --- Misc ---
    pub cpu_index: u32, 
    pub curve_start_index: u32, 
    pub curve_count: u32, 

    // --- GPU Style System ---
    pub class_data_offset: u32,  // offset into node_class_list_and_inline_styles buffer
    
    // --- Padding ---
    pub padding_top: f32,
    pub padding_right: f32,
    pub padding_bottom: f32,
    pub padding_left: f32,

    // --- Margin ---
    pub margin_top: f32,
    pub margin_right: f32,
    pub margin_bottom: f32,
    pub margin_left: f32,

    // --- Border & Outline ---
    pub border_top_width: f32,
    pub border_right_width: f32,
    pub border_bottom_width: f32,
    pub border_left_width: f32,

    pub border_color_top: u32,
    pub border_color_right: u32,
    pub border_color_bottom: u32,
    pub border_color_left: u32,

    pub outline_width: f32,
    pub outline_offset: f32,
    pub outline_color_top: u32,
    pub outline_color_right: u32,
    pub outline_color_bottom: u32,
    pub outline_color_left: u32,

    pub box_shadow_h_offset: f32,
    pub box_shadow_v_offset: f32,
    pub box_shadow_blur: f32,
    pub box_shadow_spread: f32,
    pub box_shadow_color: u32,

    pub text_color_r: f32,
    pub text_color_g: f32,
    pub text_color_b: f32,
    pub text_color_a: f32,

    pub text_align: u32,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub word_spacing: f32,
    pub font_weight: u32,
    pub font_style: u32,
    pub font_size: f32,

    // --- SVG Styles ---
    pub fill_color: u32,
    pub stroke_color: u32,
    pub stroke_width: f32,
    
    pub _pad_styles: u32, // Padding for alignment
}

#[test]
fn test_gpu_node_size() {
    assert_eq!(std::mem::size_of::<GpuNode>(), 320);
}

impl GpuNode {
    pub fn new() -> Self {
        Self {
            fixed_width: -1.0,
            min_width: 0.0,
            max_width: -1.0,
            fixed_height: -1.0,
            min_height: 0.0,
            max_height: -1.0,
            final_width: 0.0,
            desired_height: 0.0,
            final_height: 0.0,
            final_x: 0.0,
            final_y: 0.0,
            color_r: 0.0,
            color_g: 0.0,
            color_b: 0.0,
            color_a: 0.0,
            text_color_r: 1.0, // Default text to white?
            text_color_g: 1.0,
            text_color_b: 1.0,
            text_color_a: 1.0,
            text_align: 0,
            line_height: 1.0,
            letter_spacing: 0.0,
            word_spacing: 0.0,
            font_weight: 400,
            font_style: 0,
            top_offset: 0.0,
            left_offset: 0.0,
            z_index: 0.0,
            position_mode: 0,
            flex_direction: 0,
            justify_content: 0,
            align_items: 0,
            parent_index: 0,
            child_start_index: 0,
            child_count: 0,
            signals_finished: 0,
            text_start: 0,
            text_length: 0,
            flags: 1, // Default to 1 (Visible)
            natural_content_width: 0.0,
            uv_min_x: 0.0, uv_min_y: 0.0, uv_max_x: 0.0, uv_max_y: 0.0,
            cpu_index: 0, 
            curve_start_index: 0, 
            curve_count: 0,
            class_data_offset: 0,
            padding_top: 0.0,
            padding_right: 0.0,
            padding_bottom: 0.0,
            padding_left: 0.0,
            margin_top: 0.0,
            margin_right: 0.0,
            margin_bottom: 0.0,
            margin_left: 0.0,
            border_top_width: 0.0,
            border_right_width: 0.0,
            border_bottom_width: 0.0,
            border_left_width: 0.0,
            border_color_top: 0,
            border_color_right: 0,
            border_color_bottom: 0,
            border_color_left: 0,
            outline_width: 0.0,
            outline_offset: 0.0,
            outline_color_top: 0,
            outline_color_right: 0,
            outline_color_bottom: 0,
            outline_color_left: 0,
            box_shadow_h_offset: 0.0,
            box_shadow_v_offset: 0.0,
            box_shadow_blur: 0.0,
            box_shadow_spread: 0.0,
            box_shadow_color: 0,
            font_size: 24.0,
            fill_color: 0,
            stroke_color: 0,
            stroke_width: 0.0,
            _pad_styles: 0,
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
    pub max_width: f32,
    pub fixed_height: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub color: (f32, f32, f32, f32),
    pub top_offset: f32,
    pub left_offset: f32,
    pub z_index: Option<f32>,
    pub position_mode: u32,
    pub flex_direction: u32,
    pub justify_content: u32,
    pub align_items: u32,
    pub flags: u32,
    
    // Text
    pub text: Option<String>,

    pub image_asset_id: Option<String>,
    pub shape_data: Option<String>,
    
    // CSS Styles
    pub classes: Vec<String>,
    pub inline_styles: HashMap<String, StyleValue>,
    
    // Cache
    pub cached_texture: Option<std::rc::Rc<texture_atlas::TextureHandle>>,
    
    // Events
    pub on_click: Option<std::rc::Rc<dyn Fn()>>,
    
    // Input Support
    pub input_type: Option<String>,
    pub on_update_model_value: Option<std::rc::Rc<dyn Fn(String)>>,

    pub scope: Option<crate::signals::ScopeId>,
    pub hovered: bool,
    pub font_size: f32,
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
            max_width: -1.0,
            fixed_height: -1.0,
            min_height: 0.0,
            max_height: -1.0,
            color: (0.0, 0.0, 0.0, 0.0),
            top_offset: 0.0,
            left_offset: 0.0,
            z_index: None,
            position_mode: 0,
            flex_direction: 0, // Row
            justify_content: 0, // flex-start
            align_items: 0, // stretch
            flags: 1,
            
            text: None,
            image_asset_id: None,
            shape_data: None,
            classes: Vec::new(),
            inline_styles: HashMap::new(),
            cached_texture: None,
            on_click: None,
            input_type: None,
            on_update_model_value: None,
            scope: None,
            hovered: false,
            font_size: 24.0,
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
    pub(crate) hit_test_nodes: Vec<GpuNode>, // Stable nodes for hit testing (updated via readback)
    
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
    
    // Curves that are permanent (e.g. from font) and should not be cleared on flatten
    pub permanent_curve_count: usize,

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
    pub asset_ref_counts: HashMap<String, usize>,
    #[wasm_bindgen(skip)]
    pub stylesheet: StyleSheet,

    #[wasm_bindgen(skip)]
    pub free_nodes: Vec<u32>,

    #[wasm_bindgen(skip)]
    pub root_scope_id: Option<crate::signals::ScopeId>,

    // --- GPU Style Buffers ---
    #[wasm_bindgen(skip)]
    pub class_defs: Vec<u32>,         // serialized class property data
    #[wasm_bindgen(skip)]
    pub node_class_list_and_inline_styles: Vec<u32>,    // per-node [count, offset0, offset1, ..., CTRL_END]
    #[wasm_bindgen(skip)]
    pub class_offsets: HashMap<String, u32>,  // selector -> offset in class_defs
    #[wasm_bindgen(skip)]
    pub leaf_nodes: Vec<u32>,                 // indices of nodes with no children
    #[wasm_bindgen(skip)]
    pub panic_data: Vec<u32>,                 // [0] = error_code (prop_id)

    pub focused_node: Option<u32>,
    pub last_hover_target: Option<usize>,
    #[wasm_bindgen(skip)]
    pub last_hover_chain: Vec<usize>,
    pub dirty: bool,
    pub device_pixel_ratio: f32,
}

#[wasm_bindgen]
impl FlexEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FlexEngine {
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();
        log("FlexEngine Initialized via WebAssembly (CpuNode Topology)");
        
        let dpr = 1.0;
        #[cfg(target_arch = "wasm32")]
        let dpr = if let Some(win) = web_bindings::get_window() {
             win.device_pixel_ratio() as f32
        } else {
             1.0
        };
        
        let mut engine = FlexEngine {
            cpu_nodes: Vec::new(),
            gpu_nodes: Vec::new(),
            characters: Vec::new(),
            glyph_data: Vec::new(),
            kerning_table: Vec::new(),
            curves: Vec::new(),
            glyph_infos: Vec::new(),
            permanent_curve_count: 0,
            ascender: 0.0,
            descender: 0.0,
            line_gap: 0.0,
            face: None,
            texture_atlas: texture_atlas::TextureAtlas::new(2048, 2048),
            assets: HashMap::new(),
            asset_ref_counts: HashMap::new(),
            stylesheet: StyleSheet::default(),
            free_nodes: Vec::new(),
            root_scope_id: None,
            class_defs: Vec::new(),
            node_class_list_and_inline_styles: Vec::new(),
            class_offsets: HashMap::new(),
            leaf_nodes: Vec::new(),
            panic_data: vec![0], // Initialize with 0 (no error)
            focused_node: None,
            last_hover_target: None,
            last_hover_chain: Vec::new(),
            hit_test_nodes: Vec::new(),
            dirty: false, // Start clean, mark_dirty will be called during build_ui
            device_pixel_ratio: dpr,
        };
        
        engine.parse_font();
        engine
    }
}

impl Drop for FlexEngine {
    fn drop(&mut self) {
        if let Some(scope_id) = self.root_scope_id {
            crate::signals::Scope { id: scope_id }.dispose();
        }
    }
}

#[wasm_bindgen]
impl FlexEngine {
    
    fn parse_font(&mut self) {
        // Simple fixed size for demo
        let face = Face::parse(FONT_DATA, 0).expect("Error parsing font");
        let units_per_em = face.units_per_em() as f32;
        // let font_size = 24.0; // REMOVED: No longer scaling to fixed 24px here
        let scale = 1.0 / units_per_em; // Normalized to 1.0 EM units

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
                bearing_x: bbox.x_min as f32 * scale, 
                bearing_y: bbox.y_max as f32 * scale,
                width: (bbox.x_max - bbox.x_min) as f32 * scale,
                height: (bbox.y_max - bbox.y_min) as f32 * scale,
                _pad0: 0.0, _pad1: 0.0, _pad2: 0.0,
            });
        }
        
        self.face = Some(face);
        self.permanent_curve_count = self.curves.len();
    }
    


// ...




    pub fn add_node(&mut self, min_width: f32) -> u32 {
        let index = if let Some(idx) = self.free_nodes.pop() {
            let node = &mut self.cpu_nodes[idx as usize];
            *node = CpuNode::new();
            node.min_width = min_width;
            idx
        } else {
            let mut node = CpuNode::new();
            node.min_width = min_width;
            let index = self.cpu_nodes.len() as u32;
            self.cpu_nodes.push(node);
            index
        };
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
            self.delete_node_recursive(child_idx as u32);
            curr = next;
        }

        self.cpu_nodes[parent_idx].first_child = None;
        self.cpu_nodes[parent_idx].last_child = None;
        self.mark_dirty();
    }

    pub fn delete_node_recursive(&mut self, node_id: u32) {
        let node_idx = node_id as usize;
        if node_idx >= self.cpu_nodes.len() { return; }

        // 1. Recurse to children
        let mut curr = self.cpu_nodes[node_idx].first_child;
        while let Some(child_idx) = curr {
            let next = self.cpu_nodes[child_idx].next_sibling;
            self.delete_node_recursive(child_idx as u32);
            curr = next;
        }

        // 2. Unlink from parent
        self.remove_from_parent(node_id);

        // 3. Dispose of Signal Scope
        if let Some(scope_id) = self.cpu_nodes[node_idx].scope {
            crate::signals::Scope { id: scope_id }.dispose();
            self.cpu_nodes[node_idx].scope = None;
        }

        // 4. Mark as free
        // Decrement asset ref count if this node was holding one
        if let Some(asset_id) = self.cpu_nodes[node_idx].image_asset_id.take() {
            self.decrement_asset_ref(&asset_id);
        }
        self.cpu_nodes[node_idx] = CpuNode::new();
        self.free_nodes.push(node_id);
        self.mark_dirty();
    }
}

impl FlexEngine {
    pub fn add_style_rule(&mut self, selector: String, declarations: HashMap<String, StyleValue>) {
        if let Some(pos) = self.stylesheet.rules.iter().position(|r| r.selector == selector) {
            self.stylesheet.rules[pos].declarations = declarations;
        } else {
            self.stylesheet.rules.push(StyleRule { selector, declarations });
        }
        self.mark_dirty();
    }

    fn apply_styles(&mut self, node_idx: usize) {
        let resolved = self.resolve_styles_for_node(node_idx);

        // Update CpuNode fields based on resolved styles
        for (prop, val) in resolved {
            match prop.as_str() {
                "width" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].fixed_width = v; }
                }
                "height" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].fixed_height = v; }
                }
                "min-width" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].min_width = v.max(0.0); }
                }
                "max-width" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].max_width = v.max(0.0); }
                }
                "min-height" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].min_height = v.max(0.0); }
                }
                "max-height" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].max_height = v.max(0.0); }
                }
                "background-color" => {
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
                "justify-content" => {
                    if let StyleValue::Ident(s) = val {
                        self.cpu_nodes[node_idx].justify_content = match s.as_str() {
                            "center" => 1,
                            "flex-end" | "end" | "right" => 2,
                            "space-between" => 3,
                            "space-around" => 4,
                            "space-evenly" => 5,
                            _ => 0, // flex-start
                        };
                    }
                }
                "align-items" => {
                    if let StyleValue::Ident(s) = val {
                        self.cpu_nodes[node_idx].align_items = match s.as_str() {
                            "flex-start" | "start" | "left" => 1,
                            "center" => 2,
                            "flex-end" | "end" | "right" => 3,
                            _ => 0, // stretch (and baseline fallback)
                        };
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
                "font-size" => {
                    if let StyleValue::Px(v) = val { self.cpu_nodes[node_idx].font_size = v; }
                }
                _ => {}
            }
        }
    }

    fn resolve_styles_for_node(&self, node_idx: usize) -> HashMap<String, StyleValue> {
        let mut resolved: HashMap<String, StyleValue> = HashMap::new();
        let hovered = self.cpu_nodes[node_idx].hovered;
        let classes = self.cpu_nodes[node_idx].classes.clone();
        for class_name in &classes {
            let dot_selector = format!(".{}", class_name);
            let dot_hover_selector = format!("{}:hover", dot_selector);
            let plain_hover_selector = format!("{}:hover", class_name);
            for rule in &self.stylesheet.rules {
                let selector = rule.selector.as_str();
                let is_match = selector == dot_selector || selector == class_name;
                let is_hover_match = hovered && (selector == dot_hover_selector || selector == plain_hover_selector);
                if is_match || is_hover_match {
                    for (prop, val) in &rule.declarations {
                        resolved.insert(prop.clone(), val.clone());
                    }
                }
            }
        }

        // Inline overrides
        let inline = self.cpu_nodes[node_idx].inline_styles.clone();
        for (prop, val) in inline {
            resolved.insert(prop, val);
        }
        resolved
    }

    fn resolve_text_transform_for_node(&self, node_idx: usize) -> String {
        let mut idx = Some(node_idx);
        while let Some(current) = idx {
            let resolved = self.resolve_styles_for_node(current);
            if let Some(StyleValue::Ident(mode)) = resolved.get("text-transform") {
                return mode.clone();
            }
            idx = self.cpu_nodes[current].parent;
        }
        "none".to_string()
    }

    fn apply_text_transform(text: &str, transform: &str) -> String {
        match transform {
            "uppercase" => text.to_uppercase(),
            "lowercase" => text.to_lowercase(),
            "capitalize" => {
                let mut out = String::with_capacity(text.len());
                let mut capitalize_next = true;
                for ch in text.chars() {
                    if ch.is_whitespace() {
                        capitalize_next = true;
                        out.push(ch);
                    } else if capitalize_next {
                        for up in ch.to_uppercase() {
                            out.push(up);
                        }
                        capitalize_next = false;
                    } else {
                        out.push(ch);
                    }
                }
                out
            }
            _ => text.to_string(),
        }
    }

    // --- Topology Management (Linked List Wiring) ---

    // ... (existing topology methods)



    // Deprecated / No-op in linked list mode (implicit)
    pub fn set_child_start(&mut self, _parent_index: u32, _start_index: u32) {
        // No-op
    }

    // --- GPU Style Buffer Construction ---

    /// Serializes all stylesheet rules into the `class_defs` buffer.
    /// Each class becomes: [prop_id, ...value_data]* [CTRL_END]
    /// Records each selector's offset in `class_offsets`.
    fn build_class_buffers(&mut self) {
        self.class_defs.clear();
        self.class_offsets.clear();
        self.node_class_list_and_inline_styles.clear();

        // Group rules by base selector (e.g. .btn and .btn:hover)
        let mut grouped: HashMap<String, (HashMap<String, StyleValue>, HashMap<String, StyleValue>)> = HashMap::new();

        for rule in &self.stylesheet.rules {
            let (base, is_hover) = if let Some(stripped) = rule.selector.strip_suffix(":hover") {
                (stripped.to_string(), true)
            } else {
                (rule.selector.clone(), false)
            };

            let entry = grouped.entry(base).or_insert((HashMap::new(), HashMap::new()));
            if is_hover {
                for (k, v) in &rule.declarations {
                    entry.1.insert(k.clone(), v.clone());
                }
            } else {
                for (k, v) in &rule.declarations {
                    entry.0.insert(k.clone(), v.clone());
                }
            }
        }

        // Sort base selectors for deterministic output
        let mut base_selectors: Vec<_> = grouped.keys().cloned().collect();
        base_selectors.sort();

        for selector in base_selectors {
            let (base_decls, hover_decls) = grouped.get(&selector).unwrap();
            let offset = self.class_defs.len() as u32;
            self.class_offsets.insert(selector, offset);

            // Write base rules
            let mut props: Vec<_> = base_decls.keys().collect();
            props.sort();
            for prop in props {
                Self::serialize_property(&mut self.class_defs, prop, base_decls.get(prop).unwrap());
            }

            // Write hover rules if present
            if !hover_decls.is_empty() {
                self.class_defs.push(CTRL_HOVER_START);
                let mut h_props: Vec<_> = hover_decls.keys().collect();
                h_props.sort();
                for prop in h_props {
                    Self::serialize_property(&mut self.class_defs, prop, hover_decls.get(prop).unwrap());
                }
            }

            self.class_defs.push(CTRL_END);
        }
    }

    fn pack_color(r: f32, g: f32, b: f32, a: f32) -> u32 {
        let r = (r.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (g.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (b.clamp(0.0, 1.0) * 255.0) as u32;
        let a = (a.clamp(0.0, 1.0) * 255.0) as u32;
        (a << 24) | (b << 16) | (g << 8) | r
    }

    /// Serializes a single CSS property into `class_defs`.
    /// Format: [prop_id: u32] [value data: variable u32s depending on property]
    /// Serializes a single CSS property into the provided buffer.
    fn serialize_property(dest: &mut Vec<u32>, prop: &str, val: &StyleValue) {
        match prop {
            "background-color" => {
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(PROP_BACKGROUND_COLOR_RGBA);
                    dest.push(r.to_bits());
                    dest.push(g.to_bits());
                    dest.push(b.to_bits());
                    dest.push(a.to_bits());
                }
            }
            "color" => {
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(PROP_TEXT_COLOR_RGBA);
                    dest.push(r.to_bits());
                    dest.push(g.to_bits());
                    dest.push(b.to_bits());
                    dest.push(a.to_bits());
                }
            }
            "width" => {
                dest.push(PROP_WIDTH);
                match val {
                    StyleValue::Px(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_PX);
                    }
                    StyleValue::Percent(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_PERCENT);
                    }
                    _ => {
                        dest.push(0f32.to_bits());
                        dest.push(UNIT_PX);
                    }
                }
            }
            "height" => {
                dest.push(PROP_HEIGHT);
                match val {
                    StyleValue::Px(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_PX);
                    }
                    StyleValue::Percent(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_PERCENT);
                    }
                    _ => {
                        dest.push(0f32.to_bits());
                        dest.push(UNIT_PX);
                    }
                }
            }
            "min-width" => {
                dest.push(PROP_MIN_WIDTH);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "max-width" => {
                dest.push(PROP_MAX_WIDTH);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push((-1.0f32).to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "min-height" => {
                dest.push(PROP_MIN_HEIGHT);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "max-height" => {
                dest.push(PROP_MAX_HEIGHT);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push((-1.0f32).to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "flex-direction" => {
                dest.push(PROP_FLEX_DIRECTION);
                if let StyleValue::Ident(s) = val {
                    match s.as_str() {
                        "column" => dest.push(1),
                        _ => dest.push(0), // row
                    }
                } else {
                    dest.push(0);
                }
            }
            "justify-content" => {
                dest.push(PROP_JUSTIFY_CONTENT);
                if let StyleValue::Ident(s) = val {
                    let mode = match s.as_str() {
                        "center" => 1,
                        "flex-end" | "end" | "right" => 2,
                        "space-between" => 3,
                        "space-around" => 4,
                        "space-evenly" => 5,
                        _ => 0, // flex-start
                    };
                    dest.push(mode);
                } else {
                    dest.push(0);
                }
            }
            "align-items" => {
                dest.push(PROP_ALIGN_ITEMS);
                if let StyleValue::Ident(s) = val {
                    let mode = match s.as_str() {
                        "flex-start" | "start" | "left" => 1,
                        "center" => 2,
                        "flex-end" | "end" | "right" => 3,
                        _ => 0, // stretch (and baseline fallback)
                    };
                    dest.push(mode);
                } else {
                    dest.push(0);
                }
            }
            "position" => {
                dest.push(PROP_POSITION_MODE);
                if let StyleValue::Ident(s) = val {
                    match s.as_str() {
                        "absolute" => dest.push(1),
                        _ => dest.push(0), // relative
                    }
                } else {
                    dest.push(0);
                }
            }
            "top" => {
                dest.push(PROP_TOP);
                match val {
                    StyleValue::Px(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_PX);
                    }
                    _ => {
                        dest.push(0f32.to_bits());
                        dest.push(UNIT_PX);
                    }
                }
            }
            "left" => {
                dest.push(PROP_LEFT);
                match val {
                    StyleValue::Px(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_PX);
                    }
                    _ => {
                        dest.push(0f32.to_bits());
                        dest.push(UNIT_PX);
                    }
                }
            }
            "z-index" => {
                dest.push(PROP_Z_INDEX);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                } else {
                    dest.push(0f32.to_bits());
                }
            }
            "padding-top" => {
                dest.push(PROP_PADDING_TOP);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "padding-right" => {
                dest.push(PROP_PADDING_RIGHT);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "padding-bottom" => {
                dest.push(PROP_PADDING_BOTTOM);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "padding-left" => {
                dest.push(PROP_PADDING_LEFT);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "margin-top" => {
                dest.push(PROP_MARGIN_TOP);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "margin-right" => {
                dest.push(PROP_MARGIN_RIGHT);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "margin-bottom" => {
                dest.push(PROP_MARGIN_BOTTOM);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "margin-left" => {
                dest.push(PROP_MARGIN_LEFT);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "border-top-width" => {
                dest.push(PROP_BORDER_TOP_WIDTH);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "border-right-width" => {
                dest.push(PROP_BORDER_RIGHT_WIDTH);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "border-bottom-width" => {
                dest.push(PROP_BORDER_BOTTOM_WIDTH);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "border-left-width" => {
                dest.push(PROP_BORDER_LEFT_WIDTH);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "border-color-top" => {
                dest.push(PROP_BORDER_COLOR_TOP);
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(Self::pack_color(*r, *g, *b, *a));
                } else {
                    dest.push(0);
                }
            }
            "border-color-right" => {
                dest.push(PROP_BORDER_COLOR_RIGHT);
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(Self::pack_color(*r, *g, *b, *a));
                } else {
                    dest.push(0);
                }
            }
            "border-color-bottom" => {
                dest.push(PROP_BORDER_COLOR_BOTTOM);
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(Self::pack_color(*r, *g, *b, *a));
                } else {
                    dest.push(0);
                }
            }
            "border-color-left" => {
                dest.push(PROP_BORDER_COLOR_LEFT);
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(Self::pack_color(*r, *g, *b, *a));
                } else {
                    dest.push(0);
                }
            }
            "outline-width" => {
                dest.push(PROP_OUTLINE_WIDTH);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "outline-offset" => {
                dest.push(PROP_OUTLINE_OFFSET);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "outline-color-top" => {
                dest.push(PROP_OUTLINE_COLOR_TOP);
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(Self::pack_color(*r, *g, *b, *a));
                } else {
                    dest.push(0);
                }
            }
            "outline-color-right" => {
                dest.push(PROP_OUTLINE_COLOR_RIGHT);
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(Self::pack_color(*r, *g, *b, *a));
                } else {
                    dest.push(0);
                }
            }
            "outline-color-bottom" => {
                dest.push(PROP_OUTLINE_COLOR_BOTTOM);
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(Self::pack_color(*r, *g, *b, *a));
                } else {
                    dest.push(0);
                }
            }
            "outline-color-left" => {
                dest.push(PROP_OUTLINE_COLOR_LEFT);
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(Self::pack_color(*r, *g, *b, *a));
                } else {
                    dest.push(0);
                }
            }
            "box-shadow-h-offset" => {
                dest.push(PROP_BOX_SHADOW_H_OFFSET);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "box-shadow-v-offset" => {
                dest.push(PROP_BOX_SHADOW_V_OFFSET);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "box-shadow-blur" => {
                dest.push(PROP_BOX_SHADOW_BLUR);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "box-shadow-spread" => {
                dest.push(PROP_BOX_SHADOW_SPREAD);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            "box-shadow-color" => {
                dest.push(PROP_BOX_SHADOW_COLOR);
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(Self::pack_color(*r, *g, *b, *a));
                } else {
                    dest.push(0);
                }
            }
            "font-size" => {
                dest.push(PROP_FONT_SIZE);
                match val {
                    StyleValue::Px(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_PX);
                    }
                    StyleValue::Em(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_EM);
                    }
                    _ => {
                        dest.push(24f32.to_bits());
                        dest.push(UNIT_PX);
                    }
                }
            }
            "text-align" => {
                dest.push(PROP_TEXT_ALIGN);
                let mode = if let StyleValue::Ident(s) = val {
                    match s.as_str() {
                        "center" => 1,
                        "right" | "end" => 2,
                        _ => 0,
                    }
                } else {
                    0
                };
                dest.push(mode);
            }
            "line-height" => {
                dest.push(PROP_LINE_HEIGHT);
                match val {
                    // Unitless values are already parsed as Px by this parser; treat as multiplier.
                    StyleValue::Px(v) | StyleValue::Em(v) => dest.push(v.to_bits()),
                    _ => dest.push(1.0f32.to_bits()),
                }
            }
            "letter-spacing" => {
                dest.push(PROP_LETTER_SPACING);
                match val {
                    StyleValue::Px(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_PX);
                    }
                    StyleValue::Em(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_EM);
                    }
                    _ => {
                        dest.push(0f32.to_bits());
                        dest.push(UNIT_PX);
                    }
                }
            }
            "word-spacing" => {
                dest.push(PROP_WORD_SPACING);
                match val {
                    StyleValue::Px(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_PX);
                    }
                    StyleValue::Em(v) => {
                        dest.push(v.to_bits());
                        dest.push(UNIT_EM);
                    }
                    _ => {
                        dest.push(0f32.to_bits());
                        dest.push(UNIT_PX);
                    }
                }
            }
            "font-weight" => {
                dest.push(PROP_FONT_WEIGHT);
                let weight = match val {
                    StyleValue::Px(v) => (*v).round() as u32,
                    StyleValue::Ident(s) => match s.as_str() {
                        "normal" => 400,
                        "bold" => 700,
                        "bolder" => 800,
                        "lighter" => 300,
                        _ => 400,
                    },
                    _ => 400,
                };
                dest.push(weight.clamp(100, 900));
            }
            "font-style" => {
                dest.push(PROP_FONT_STYLE);
                let style = if let StyleValue::Ident(s) = val {
                    match s.as_str() {
                        "italic" => 1,
                        "oblique" => 2,
                        _ => 0,
                    }
                } else {
                    0
                };
                dest.push(style);
            }
            // Applied on CPU while generating per-character glyphs.
            "text-transform" => {}
            "fill" => {
                dest.push(PROP_FILL_COLOR);
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(Self::pack_color(*r, *g, *b, *a));
                } else {
                    dest.push(0);
                }
            }
            "stroke" => {
                dest.push(PROP_STROKE_COLOR);
                if let StyleValue::Color(r, g, b, a) = val {
                    dest.push(Self::pack_color(*r, *g, *b, *a));
                } else {
                    dest.push(0);
                }
            }
            "stroke-width" => {
                dest.push(PROP_STROKE_WIDTH);
                if let StyleValue::Px(v) = val {
                    dest.push(v.to_bits());
                    dest.push(UNIT_PX);
                } else {
                    dest.push(0f32.to_bits());
                    dest.push(UNIT_PX);
                }
            }
            _ => {
                panic!(
                    "Unknown CSS property '{}' with value {:?}. Add support in renderer-core/src/lib.rs::serialize_property and renderer-core/src/shaders_compute.wgsl::apply_style_stream.",
                    prop,
                    val
                );
            }
        }
    }

    /// Builds a node's style entry and returns the offset.
    /// Format: [class_count: u32, class_def_offset_0, ..., prop_id, val..., CTRL_END]
    fn build_node_class_and_inline_styles_entry(&mut self, cpu_idx: usize) -> u32 {
        let offset = self.node_class_list_and_inline_styles.len() as u32;
        let node = &self.cpu_nodes[cpu_idx];
        let classes = &node.classes;

        // 1. Classes
        let mut matching_offsets: Vec<u32> = Vec::new();
        for class_name in classes {
            let dot_selector = format!(".{}", class_name);
            if let Some(&def_offset) = self.class_offsets.get(&dot_selector) {
                matching_offsets.push(def_offset);
            } else if let Some(&def_offset) = self.class_offsets.get(class_name) {
                matching_offsets.push(def_offset);
            }
        }

        self.node_class_list_and_inline_styles.push(matching_offsets.len() as u32);
        for off in matching_offsets {
            self.node_class_list_and_inline_styles.push(off);
        }

        // 2. Inline Styles
        let mut inline_styles = node.inline_styles.clone(); // Clone to avoid borrow issues while writing to buffer
        
        // Merge imperative fields into inline styles if they are non-default
        if node.fixed_width >= 0.0 { inline_styles.insert("width".into(), StyleValue::Px(node.fixed_width)); }
        if node.fixed_height >= 0.0 { inline_styles.insert("height".into(), StyleValue::Px(node.fixed_height)); }
        if node.min_width != 0.0 { inline_styles.insert("min-width".into(), StyleValue::Px(node.min_width)); }
        if node.max_width != -1.0 { inline_styles.insert("max-width".into(), StyleValue::Px(node.max_width)); }
        if node.min_height != 0.0 { inline_styles.insert("min-height".into(), StyleValue::Px(node.min_height)); }
        if node.max_height != -1.0 { inline_styles.insert("max-height".into(), StyleValue::Px(node.max_height)); }

        for (prop, val) in inline_styles {
            Self::serialize_property(&mut self.node_class_list_and_inline_styles, &prop, &val);
        }

        // 3. End Token
        self.node_class_list_and_inline_styles.push(CTRL_END);

        offset
    }

    // --- Flattening (CPU -> GPU) ---
    // This is the bridge. Rebuilds gpu_nodes from cpu_nodes.
    fn flatten(&mut self) {
        // 0. Pre-pass: Build class buffers & Texture Management
        self.build_class_buffers();
        self.texture_atlas.process_deletions();

        // Build style map from previous frame's hit_test_nodes
        let style_map: HashMap<u32, (u32, u32, f32, u32, u32, u32, u32)> = self.hit_test_nodes.iter()
            .map(|n| (n.cpu_index, (
                n.fill_color, 
                n.stroke_color, 
                n.stroke_width,
                n.text_color_r.to_bits(),
                n.text_color_g.to_bits(),
                n.text_color_b.to_bits(),
                n.text_color_a.to_bits()
            )))
            .collect();

        for i in 0..self.cpu_nodes.len() {
             
             // We work around borrow checker by extracting needed data first if possible,
             // or just carefully using indices.
             // We need to mutate `cached_texture`.
             
             // Clone ID to avoid borrow issues while mutating
             let (image_id, w_logical, h_logical) = {
                 let node = &self.cpu_nodes[i];
                 if let Some(ref id) = node.image_asset_id {
                     let w = if node.fixed_width > 0.0 { node.fixed_width } else { 64.0 };
                     let h = if node.fixed_height > 0.0 { node.fixed_height } else { 64.0 };
                     (Some(id.clone()), w, h)
                 } else {
                     (None, 0.0, 0.0)
                 }
             };

             // Clone shape data


             if let Some(id) = image_id {
                 // Check if current cache is valid
                 let (fill, stroke, stroke_w, tr, tg, tb, ta) = style_map.get(&(i as u32)).cloned().unwrap_or((0, 0, 0.0, 1.0f32.to_bits(), 1.0f32.to_bits(), 1.0f32.to_bits(), 1.0f32.to_bits()));
                 
                 // Apply Device Pixel Ratio
                 let w = (w_logical * self.device_pixel_ratio).ceil() as u32;
                 let h = (h_logical * self.device_pixel_ratio).ceil() as u32;

                 let text_color_packed = Self::pack_color(
                     f32::from_bits(tr), 
                     f32::from_bits(tg), 
                     f32::from_bits(tb), 
                     f32::from_bits(ta)
                 );
                 
                 // Always try to get existing handle from Atlas Cache to ensure style match
                 let key = texture_atlas::CacheKey { 
                     id: id.clone(), 
                     width: w, 
                     height: h,
                     fill_color: fill,
                     stroke_color: stroke,
                     stroke_width: stroke_w.to_bits(),
                     text_color: text_color_packed,
                 };
                 
                 let new_handle = if let Some(h) = self.texture_atlas.get_handle(&key) {
                           Some(h)
                      } else {
                           // Not in atlas. Need to load/resize/allocate.
                           if let Some(bytes) = self.assets.get(&id).cloned() {
                                let is_svg = id.ends_with(".svg") || (bytes.len() > 4 && bytes.as_slice().starts_with(b"<svg"));
                                let pixels: Option<Vec<u8>> = if is_svg {
                                     let mut final_bytes = bytes.clone();
                                     // Inject styles if present
                                     if fill != 0 || stroke != 0 || stroke_w > 0.0 || text_color_packed != 0xFFFFFFFF {
                                         let mut style = String::new();
                                         // Unpack fill
                                         if fill != 0 {
                                             let r = fill & 0xFF;
                                             let g = (fill >> 8) & 0xFF;
                                             let b = (fill >> 16) & 0xFF;
                                             let a = ((fill >> 24) & 0xFF) as f32 / 255.0;
                                             style.push_str(&format!("fill: rgba({},{},{},{}); ", r,g,b,a));
                                         }
                                         // Unpack stroke
                                         if stroke != 0 {
                                             let r = stroke & 0xFF;
                                             let g = (stroke >> 8) & 0xFF;
                                             let b = (stroke >> 16) & 0xFF;
                                             let a = ((stroke >> 24) & 0xFF) as f32 / 255.0;
                                             style.push_str(&format!("stroke: rgba({},{},{},{}); ", r,g,b,a));
                                         }
                                         // Stroke Width
                                         if stroke_w > 0.0 {
                                             style.push_str(&format!("stroke-width: {}px; ", stroke_w));
                                         }

                                         // Inject Color property (for currentColor support)
                                         let cr = f32::from_bits(tr);
                                         let cg = f32::from_bits(tg);
                                         let cb = f32::from_bits(tb);
                                         let ca = f32::from_bits(ta);
                                         style.push_str(&format!("color: rgba({},{},{},{}); ", (cr*255.0) as u8, (cg*255.0) as u8, (cb*255.0) as u8, ca));
                                         
                                         if let Ok(s) = std::str::from_utf8(&final_bytes) {
                                              // Find <svg tag to inject style
                                              // Simple search for first <svg
                                              if let Some(idx) = s.to_lowercase().find("<svg") {
                                                  let mut new_s = s.to_string();
                                                  // Insert after <svg
                                                  new_s.insert_str(idx + 4, &format!(" style=\"{}\" ", style));
                                                  final_bytes = new_s.into_bytes();
                                              }
                                         }
                                     }

                                     let opt = usvg::Options::default();
                                     if let Ok(tree) = usvg::Tree::from_data(&final_bytes, &opt) {
                                          let mut pixmap = tiny_skia::Pixmap::new(w, h).unwrap_or(tiny_skia::Pixmap::new(1, 1).unwrap());
                                          // FitTo::Size already handles scaling to the target raster size.
                                          // Applying an additional scale transform shrinks icons into the top-left.
                                          let ts = tiny_skia::Transform::identity();
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

        self.gpu_nodes.clear();
        self.node_class_list_and_inline_styles.clear();
        self.characters.clear(); // Rebuild chars too since they depend on node index
        self.curves.truncate(self.permanent_curve_count);
        
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

        self.leaf_nodes.clear();
        self.panic_data[0] = 0;
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
            
            // Extract all needed data from cpu_node into locals (drop immutable borrow early)
            let cn_first_child = self.cpu_nodes[cpu_idx].first_child;
            let cn_parent = self.cpu_nodes[cpu_idx].parent;
            let cn_min_width = self.cpu_nodes[cpu_idx].min_width;
            let cn_max_width = self.cpu_nodes[cpu_idx].max_width;
            let cn_min_height = self.cpu_nodes[cpu_idx].min_height;
            let cn_max_height = self.cpu_nodes[cpu_idx].max_height;
            let cn_flags = self.cpu_nodes[cpu_idx].flags;
            let cn_cached_texture_region = self.cpu_nodes[cpu_idx].cached_texture.as_ref().map(|h| {
                (h.region.u_min, h.region.v_min, h.region.u_max, h.region.v_max)
            });
            let cn_text = self.cpu_nodes[cpu_idx].text.clone();
            let cn_shape_data = self.cpu_nodes[cpu_idx].shape_data.clone();
            let cn_font_size = self.cpu_nodes[cpu_idx].font_size;
            let transformed_text = if let Some(text_content) = &cn_text {
                let text_transform = self.resolve_text_transform_for_node(cpu_idx);
                Some(Self::apply_text_transform(text_content, text_transform.as_str()))
            } else {
                None
            };
            
            // Calculate Children Range
            let start_child_gpu_idx = self.gpu_nodes.len() as u32;
            let mut child_count = 0;
            
            // Iterate Children to reserve/push placeholders
            let mut curr_child = cn_first_child;
            while let Some(child_cpu_idx) = curr_child {
                let kid_gpu_idx = self.gpu_nodes.len() as u32;
                self.gpu_nodes.push(GpuNode::new()); // Placeholder
                cpu_to_gpu.insert(child_cpu_idx, kid_gpu_idx);
                
                queue.push_back(child_cpu_idx);
                
                child_count += 1;
                curr_child = self.cpu_nodes[child_cpu_idx].next_sibling;
            }
            
            if cn_first_child.is_none() {
                self.leaf_nodes.push(gpu_idx);
            }

            let mut parent_gpu_idx = 0;
            if let Some(p_cpu) = cn_parent {
                if let Some(&p_gpu) = cpu_to_gpu.get(&p_cpu) {
                     parent_gpu_idx = p_gpu;
                }
            }

            // Build style entry (borrows &mut self)
            let class_data_offset = self.build_node_class_and_inline_styles_entry(cpu_idx);

            {
                let gpu_node = &mut self.gpu_nodes[gpu_idx as usize];
                // Mirror non-style props (class-resolved props handled by GPU resolve_styles pass)
                gpu_node.min_width = cn_min_width;
                gpu_node.max_width = cn_max_width;
                gpu_node.min_height = cn_min_height;
                gpu_node.max_height = cn_max_height;
                gpu_node.flags = cn_flags;
                if self.cpu_nodes[cpu_idx].hovered {
                    gpu_node.flags |= 16; // Bit 4 = Hovered
                }
                
                gpu_node.cpu_index = cpu_idx as u32;
                
                // Topology
                gpu_node.child_start_index = start_child_gpu_idx;
                gpu_node.child_count = child_count;
                
                // Parent Ref
                gpu_node.parent_index = if cn_parent.is_some() { parent_gpu_idx } else { 0 };

                // Class data offset for GPU style resolution
                gpu_node.class_data_offset = class_data_offset;
                
                // Image UV Resolution
                if let Some((u_min, v_min, u_max, v_max)) = cn_cached_texture_region {
                    gpu_node.uv_min_x = u_min;
                    gpu_node.uv_min_y = v_min;
                    gpu_node.uv_max_x = u_max;
                    gpu_node.uv_max_y = v_max;
                    gpu_node.flags |= 2; // Ensure flag is set
                }
                
                // Text Handling (Rebuild Characters)
                if let Some(text_content) = transformed_text.as_ref() {
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

                // Shape Handling
                if let Some(path_str) = &cn_shape_data {
                     let start_idx = self.curves.len() as u32;
                     let mut collector = PathCollector::new(1.0, 0.0, 0.0);
                     collector.parse_svg_path(path_str);
                     self.curves.extend(collector.curves);
                     let end_idx = self.curves.len() as u32;
                     
                     if end_idx > start_idx {
                         gpu_node.curve_start_index = start_idx;
                         gpu_node.curve_count = end_idx - start_idx;
                         gpu_node.flags |= 4; // Bit 2 = Shape
                     }
                }

                gpu_node.font_size = cn_font_size;
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

    pub fn set_image_asset_id(&mut self, node_id: u32, asset_id: &str) {
        let old_id = if let Some(node) = self.cpu_nodes.get_mut(node_id as usize) {
            node.image_asset_id.take()
        } else {
            return;
        };
        
        if let Some(id) = old_id {
            self.decrement_asset_ref(&id);
        }
        
        self.increment_asset_ref(asset_id);

        if let Some(node) = self.cpu_nodes.get_mut(node_id as usize) {
            node.image_asset_id = Some(asset_id.to_string());
            node.cached_texture = None;
            node.flags |= 2;
        }
        
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

    pub fn get_class_defs_buffer(&self) -> js_sys::Uint8Array {
        let size = self.class_defs.len() * std::mem::size_of::<u32>();
        let ptr = self.class_defs.as_ptr() as *const u8;
        unsafe {
            js_sys::Uint8Array::view(std::slice::from_raw_parts(ptr, size))
        }
    }
    pub fn get_node_class_list_and_inline_styles_buffer(&self) -> js_sys::Uint8Array {
        let size = self.node_class_list_and_inline_styles.len() * std::mem::size_of::<u32>();
        let ptr = self.node_class_list_and_inline_styles.as_ptr() as *const u8;
        unsafe {
            js_sys::Uint8Array::view(std::slice::from_raw_parts(ptr, size))
        }
    }

    pub fn get_class_defs_count(&self) -> usize {
        self.class_defs.len()
    }

    pub fn get_node_class_list_and_inline_styles_count(&self) -> usize {
        self.node_class_list_and_inline_styles.len()
    }

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

    pub fn unload_asset(&mut self, id: &str) {
        self.assets.remove(id);
        // We don't necessarily clear the texture atlas cache here,
        // because existing nodes might still be using the texture.
        // The atlas cache uses Weak refs, and TextureHandle drop handles deallocation.
    }

    fn increment_asset_ref(&mut self, id: &str) {
        let count = self.asset_ref_counts.entry(id.to_string()).or_insert(0);
        *count += 1;
    }

    fn decrement_asset_ref(&mut self, id: &str) {
        if let Some(count) = self.asset_ref_counts.get_mut(id) {
            if *count > 0 {
                *count -= 1;
                if *count == 0 {
                    self.unload_asset(id);
                    self.asset_ref_counts.remove(id);
                }
            }
        }
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
        for i in (0..self.hit_test_nodes.len()).rev() {
            let n = &self.hit_test_nodes[i];
            
            // Skip invisible if flag bit 0 is Not set
            if (n.flags & 1) == 0 { continue; }

            let left = n.final_x;
            let top = n.final_y;
            let right = left + n.final_width;
            let bottom = top + n.final_height;

            if x >= left && x <= right && y >= top && y <= bottom {
                hit_idx = Some(i);
                log(&format!("Hit Node: {} at ({} , {}). rect: [{}, {}, {}, {}]", i, x, y, left, top, right, bottom));
                break;
            }
        }

        if hit_idx.is_none() {
            log(&format!("No hit at ({} , {})", x, y));
            self.focused_node = None;
        }

        let mut callbacks = Vec::new();

        if let Some(gpu_idx) = hit_idx {
            let initial_cpu_idx = self.hit_test_nodes[gpu_idx].cpu_index as usize;
            
            // Event Bubbling
            let mut current_cpu_idx = Some(initial_cpu_idx);

            while let Some(cpu_idx) = current_cpu_idx {
                if let Some(node) = self.cpu_nodes.get(cpu_idx) {
                    if let Some(cb) = &node.on_click {
                        callbacks.push(cb.clone());
                        break;
                    }
                    
                    // Focus handling for inputs
                    if node.input_type.is_some() {
                        self.focused_node = Some(cpu_idx as u32);
                        log(&format!("Focused Input Node: {}", cpu_idx));
                    }
                    
                    current_cpu_idx = node.parent;
                }
            }
        }

        callbacks
    }

    pub fn handle_mousemove(&mut self, x: f32, y: f32) {
        let mut hit_idx = None;
        // Search backwards to find topmost element
        for i in (0..self.hit_test_nodes.len()).rev() {
            let n = &self.hit_test_nodes[i];
            
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

        let target_cpu_idx = hit_idx.map(|idx| self.hit_test_nodes[idx].cpu_index as usize);
        
        // Only update if the leaf hover target has changed
        if target_cpu_idx == self.last_hover_target {
            return;
        }

        let mut changed_hover = false;

        // Build target chain root->leaf for efficient diff against previous hover chain.
        let mut new_chain = Vec::new();
        let mut curr = target_cpu_idx;
        while let Some(idx) = curr {
            if idx >= self.cpu_nodes.len() {
                break;
            }
            new_chain.push(idx);
            curr = self.cpu_nodes[idx].parent;
        }
        new_chain.reverse();

        // Find common prefix to avoid touching unaffected nodes.
        let mut common = 0usize;
        while common < self.last_hover_chain.len()
            && common < new_chain.len()
            && self.last_hover_chain[common] == new_chain[common]
        {
            common += 1;
        }

        // Un-hover nodes no longer in the chain.
        for &idx in self.last_hover_chain[common..].iter().rev() {
            if idx < self.cpu_nodes.len() && self.cpu_nodes[idx].hovered {
                self.cpu_nodes[idx].hovered = false;
                changed_hover = true;
            }
        }

        // Hover newly entered nodes.
        for &idx in &new_chain[common..] {
            if idx < self.cpu_nodes.len() && !self.cpu_nodes[idx].hovered {
                self.cpu_nodes[idx].hovered = true;
                changed_hover = true;
            }
        }

        self.last_hover_target = target_cpu_idx;
        self.last_hover_chain = new_chain;

        if changed_hover {
            self.mark_dirty();
        }
    }

    pub fn handle_keydown(&mut self, key: String) -> Option<(std::rc::Rc<dyn Fn(String)>, String)> {
        let node_id = match self.focused_node {
            Some(id) => id,
            None => return None,
        };

        let (current_text, input_type, on_update) = {
            let node = match self.cpu_nodes.get(node_id as usize) {
                Some(n) => n,
                None => return None,
            };
            
            (
                node.text.clone().unwrap_or_default(),
                node.input_type.clone().unwrap_or_else(|| "text".to_string()),
                node.on_update_model_value.clone()
            )
        };

        let mut next_text = current_text.clone();

        if key == "Backspace" {
            next_text.pop();
        } else if key.len() == 1 {
            // Validation based on type
            let c = key.chars().next().unwrap();
            let mut allowed = true;
            
            if input_type == "float64" {
                allowed = c.is_ascii_digit() || c == '.' || (c == '-' && next_text.is_empty());
            } else if input_type == "int64" {
                allowed = c.is_ascii_digit() || (c == '-' && next_text.is_empty());
            }

            if allowed {
                next_text.push(c);
            }
        }

        if next_text != current_text {
            if let Some(cb) = on_update {
                return Some((cb, next_text));
            }
        }
        None
    }

    pub fn add_image_to_atlas(&mut self, id: String, width: u32, height: u32, data: Vec<u8>) -> Option<std::rc::Rc<texture_atlas::TextureHandle>> {
        let key = texture_atlas::CacheKey { 
            id, 
            width, 
            height,
            fill_color: 0,
            stroke_color: 0,
            stroke_width: 0,
            text_color: 0,
        };
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
