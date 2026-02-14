struct Node {
    fixed_width: f32, // -1.0 = auto
    min_width: f32,
    fixed_height: f32, // -1.0 = auto
    final_width: f32,
    
    desired_height: f32,
    final_height: f32,
    final_x: f32,
    final_y: f32,
    
    color_r: f32,
    color_g: f32,
    color_b: f32,
    color_a: f32,
    
    top_offset: f32,
    left_offset: f32,
    z_index: f32,
    position_mode: u32,
    flex_direction: u32,

    parent_index: u32,
    child_start_index: u32,
    child_count: u32,
    signals_finished: atomic<u32>,
    text_start: u32,
    text_length: u32,
    flags: u32, // Bit 0 = Visible
    natural_content_width: f32,
    
    // --- Texture Atlas UVs ---
    uv_min_x: f32, 
    uv_min_y: f32, 
    uv_max_x: f32, 
    uv_max_y: f32,
    
    // --- Misc ---
    cpu_index: u32,
    curve_start_index: u32,
    curve_count: u32,

    // --- GPU Style System ---
    class_data_offset: u32,  // offset into node_class_list
    
    // --- Padding ---
    padding_top: f32,
    padding_right: f32,
    padding_bottom: f32,
    padding_left: f32,

    // --- Margin ---
    margin_top: f32,
    margin_right: f32,
    margin_bottom: f32,
    margin_left: f32,

    // --- Border & Outline ---
    border_top_width: f32,
    border_right_width: f32,
    border_bottom_width: f32,
    border_left_width: f32,

    border_color_top: u32,
    border_color_right: u32,
    border_color_bottom: u32,
    border_color_left: u32,

    outline_width: f32,
    outline_offset: f32,
    outline_color_top: u32,
    outline_color_right: u32,
    outline_color_bottom: u32,
    outline_color_left: u32,

    box_shadow_h_offset: f32,
    box_shadow_v_offset: f32,
    box_shadow_blur: f32,
    box_shadow_spread: f32,
    box_shadow_color: u32,

    text_color_r: f32,
    text_color_g: f32,
    text_color_b: f32,
    text_color_a: f32,

    font_size: f32,

    fill_color: u32,
    stroke_color: u32,
    stroke_width: f32,
    _pad_styles: u32,
};

struct Character {
    value: u32,
    glyph_index: u32,
    next_glyph_index: u32,
    node_index: u32,

    x: f32,
    y: f32,
    width: f32,
    height: f32,
};

struct Uniforms {
    screen_width: f32,
    screen_height: f32,
    font_ascender: f32,
    line_height: f32,
    node_count: f32,
    _pad0: f32, _pad1: f32, _pad2: f32,
};

struct GlyphData {
    advance: f32,
    bearing_x: f32,
    bearing_y: f32,
    width: f32,
    height: f32,
    // --- Padding to 32 bytes ---
    _pad0: f32, _pad1: f32, _pad2: f32,
};

@group(0) @binding(0) var<storage, read_write> nodes: array<Node>;
@group(0) @binding(1) var<uniform> uniforms: Uniforms;
@group(0) @binding(2) var<storage, read_write> characters: array<Character>;
@group(0) @binding(3) var<storage, read> glyph_data: array<GlyphData>;
@group(0) @binding(4) var<storage, read> class_defs: array<u32>;
@group(0) @binding(5) var<storage, read> node_class_list: array<u32>;

// --- Style Constants (generated) ---
// These are injected by the renderer at shader compilation time.
// See style_defs.toml for the source of truth.

@compute @workgroup_size(64)
fn reset_signals(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x;
    if (id >= u32(uniforms.node_count)) {
        return;
    }
    atomicStore(&nodes[id].signals_finished, 0u);
}

struct LayoutResult {
    width: f32,
    height: f32,
};

fn layout_text(node_id: u32, max_width: f32, write_output: bool) -> LayoutResult {
    let start = nodes[node_id].text_start;
    let len = nodes[node_id].text_length;
    let line_height = uniforms.line_height;
    let font_ascender = uniforms.font_ascender;
    
    var cursor_x = 0.0;
    var cursor_y = 0.0;
    var max_x = 0.0;
    
    let font_size = nodes[node_id].font_size;
    let scale = font_size;
    
    // Safety check for empty text
    if (len == 0u) {
        return LayoutResult(0.0, 0.0);
    }
    
    for (var i = 0u; i < len; i = i + 1u) {
        let idx = start + i;
        let char = characters[idx]; 
        let glyph_idx = char.glyph_index; 
        
        let glyph = glyph_data[glyph_idx];
        let advance = glyph.advance;
        
        // Word wrap check
        if (cursor_x + advance * scale > max_width + 0.01 && cursor_x > 0.0) {
            cursor_x = 0.0;
            cursor_y += line_height * scale;
        }
        
        if (write_output) {
            characters[idx].x = cursor_x + glyph.bearing_x * scale;
            characters[idx].y = cursor_y + font_ascender * scale - glyph.bearing_y * scale; 
            characters[idx].width = glyph.width * scale;
            characters[idx].height = glyph.height * scale;
        }
        
        cursor_x += advance * scale;
        
        max_x = max(max_x, cursor_x);
    }
    
    return LayoutResult(max_x, cursor_y + line_height * scale);
}

// PASS 1: Natural Width (Bottom-Up)
@compute @workgroup_size(64)
fn width_bottom_up(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let node_id = global_id.x;
    if (node_id >= u32(uniforms.node_count)) {
        return;
    }

    var current_id = node_id;
    
    loop {
        let count = nodes[current_id].child_count;
        
        if (current_id == node_id && count > 0u) {
            return;
        }

        process_node_width(current_id);
        
        let parent_id = nodes[current_id].parent_index;
        
        if (current_id == parent_id) {
            break;
        }

        let status = atomicAdd(&nodes[parent_id].signals_finished, 1u);
        let parent_child_count = nodes[parent_id].child_count;

        if (status == parent_child_count - 1u) {
            current_id = parent_id;
        } else {
            break;
        }
    }
}

fn get_negotiated_outer_width(id: u32) -> f32 {
    var base_w = 0.0;
    if (nodes[id].fixed_width >= 0.0) {
        base_w = nodes[id].fixed_width;
    } else {
        base_w = max(nodes[id].min_width, nodes[id].natural_content_width);
    }
    return base_w + nodes[id].margin_left + nodes[id].margin_right;
}

fn process_node_width(id: u32) {
    let count = nodes[id].child_count;

    // Visibility Check
    if ((nodes[id].flags & 1u) == 0u) {
        nodes[id].natural_content_width = 0.0;
        return;
    }

    let border_h = nodes[id].border_left_width + nodes[id].border_right_width;

    if (count == 0u) {
        // Pass 1: Measure with infinite width
        let result = layout_text(id, 100000.0, false);
        nodes[id].natural_content_width = result.width + nodes[id].padding_left + nodes[id].padding_right + border_h;
    } else {
        var result_width = 0.0;
        let start = nodes[id].child_start_index;

        if (nodes[id].flex_direction == 1u) { // Column
            for (var i = 0u; i < count; i = i + 1u) {
                // POS ABSOLUTE CHECK
                if (nodes[start + i].position_mode == 0u && (nodes[start + i].flags & 1u) != 0u) {
                    result_width = max(result_width, get_negotiated_outer_width(start + i));
                }
            }
        } else { // Row (Default)
            for (var i = 0u; i < count; i = i + 1u) {
                 // POS ABSOLUTE CHECK
                if (nodes[start + i].position_mode == 0u && (nodes[start + i].flags & 1u) != 0u) {
                    result_width += get_negotiated_outer_width(start + i);
                }
            }
        }
        nodes[id].natural_content_width = result_width + nodes[id].padding_left + nodes[id].padding_right + border_h;
    }
}

// PASS 2: Resolve Width (Top-Down)
@compute @workgroup_size(64)
fn width_top_down(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x;
    if (id >= u32(uniforms.node_count)) {
        return;
    }
    
    let parent_id = nodes[id].parent_index;
    if (parent_id == id) {
        nodes[id].final_width = uniforms.screen_width;
        atomicStore(&nodes[id].signals_finished, 1u);
    } else {
        if (atomicLoad(&nodes[parent_id].signals_finished) == 1u && atomicLoad(&nodes[id].signals_finished) == 0u) {
            
            if (nodes[id].position_mode == 1u) {
                // Absolute: Use negotiated width (excluding its own margin from the final_width property, 
                // because absolute positioning usually defines the box size via width)
                if (nodes[id].fixed_width >= 0.0) {
                    nodes[id].final_width = nodes[id].fixed_width;
                } else {
                    nodes[id].final_width = max(nodes[id].min_width, nodes[id].natural_content_width);
                }
            } else {
                // Relative/Flex
                let available_parent_inner_width = max(0.0, nodes[parent_id].final_width - nodes[parent_id].padding_left - nodes[parent_id].padding_right - nodes[parent_id].border_left_width - nodes[parent_id].border_right_width);
                
                if (nodes[parent_id].flex_direction == 1u) { // Column
                    if (nodes[id].fixed_width >= 0.0) {
                        nodes[id].final_width = nodes[id].fixed_width;
                    } else {
                        nodes[id].final_width = max(0.0, available_parent_inner_width - nodes[id].margin_left - nodes[id].margin_right);
                    }
                } else { // Row
                    if (nodes[id].fixed_width >= 0.0) {
                        nodes[id].final_width = nodes[id].fixed_width;
                    } else {
                        let parent_natural_inner = max(0.0, nodes[parent_id].natural_content_width - nodes[parent_id].padding_left - nodes[parent_id].padding_right - nodes[parent_id].border_left_width - nodes[parent_id].border_right_width);
                        let my_negotiated_outer = get_negotiated_outer_width(id);
                        
                        if (parent_natural_inner > 0.0) {
                            let ratio = available_parent_inner_width / parent_natural_inner;
                            let my_final_outer = my_negotiated_outer * ratio;
                            nodes[id].final_width = max(0.0, my_final_outer - nodes[id].margin_left - nodes[id].margin_right);
                        } else {
                            nodes[id].final_width = 0.0;
                        }
                    }
                }
            }
            atomicStore(&nodes[id].signals_finished, 1u);
        }
    }
}

// PASS 3: Natural Height (Bottom-Up)
@compute @workgroup_size(64)
fn height_bottom_up(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let node_id = global_id.x;
    if (node_id >= u32(uniforms.node_count)) {
        return;
    }

    var current_id = node_id;
    
    loop {
        let count = nodes[current_id].child_count;
        if (current_id == node_id && count > 0u) {
            return;
        }

        process_node_height(current_id);
        
        let parent_id = nodes[current_id].parent_index;
        if (current_id == parent_id) { break; }

        let status = atomicAdd(&nodes[parent_id].signals_finished, 1u);
        let parent_child_count = nodes[parent_id].child_count;

        if (status == parent_child_count - 1u) {
            current_id = parent_id;
        } else {
            break;
        }
    }
}

fn process_node_height(id: u32) {
    // Visibility Check
    if ((nodes[id].flags & 1u) == 0u) {
        nodes[id].desired_height = 0.0;
        return;
    }

    if (nodes[id].fixed_height >= 0.0) {
        nodes[id].desired_height = nodes[id].fixed_height;
        return;
    }

    let count = nodes[id].child_count;
    let border_v = nodes[id].border_top_width + nodes[id].border_bottom_width;

    if (count == 0u) {
        let available_text_width = max(0.0, nodes[id].final_width - nodes[id].padding_left - nodes[id].padding_right - nodes[id].border_left_width - nodes[id].border_right_width);
        let result = layout_text(id, available_text_width, true);
        nodes[id].desired_height = result.height + nodes[id].padding_top + nodes[id].padding_bottom + border_v;
    } else {
        var result_height = 0.0;
        let start = nodes[id].child_start_index;
        
        if (nodes[id].flex_direction == 1u) { // Column
            for (var i = 0u; i < count; i = i + 1u) {
                 if (nodes[start + i].position_mode == 0u && (nodes[start + i].flags & 1u) != 0u) {
                    result_height += nodes[start + i].desired_height + nodes[start + i].margin_top + nodes[start + i].margin_bottom;
                 }
            }
        } else { // Row
            for (var i = 0u; i < count; i = i + 1u) {
                if (nodes[start + i].position_mode == 0u && (nodes[start + i].flags & 1u) != 0u) {
                    result_height = max(result_height, nodes[start + i].desired_height + nodes[start + i].margin_top + nodes[start + i].margin_bottom);
                }
            }
        }
        nodes[id].desired_height = result_height + nodes[id].padding_top + nodes[id].padding_bottom + border_v;
    }
}

// PASS 4: Final Layout (Top-Down)
@compute @workgroup_size(64)
fn final_layout(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x;
    if (id >= u32(uniforms.node_count)) {
        return;
    }
    
    let parent_id = nodes[id].parent_index;
    
    if (parent_id == id) {
        nodes[id].final_x = 0.0;
        nodes[id].final_y = 0.0;
        nodes[id].final_height = uniforms.screen_height; 
        atomicStore(&nodes[id].signals_finished, 1u);
    } else {
        if (atomicLoad(&nodes[parent_id].signals_finished) == 1u && atomicLoad(&nodes[id].signals_finished) == 0u) {
            nodes[id].final_height = nodes[id].desired_height;

            if (nodes[id].position_mode == 1u) { // Absolute
                 nodes[id].final_x = nodes[parent_id].final_x + nodes[parent_id].padding_left + nodes[parent_id].border_left_width + nodes[id].left_offset + nodes[id].margin_left;
                 nodes[id].final_y = nodes[parent_id].final_y + nodes[parent_id].padding_top + nodes[parent_id].border_top_width + nodes[id].top_offset + nodes[id].margin_top;
            } else { // Relative
                if (nodes[parent_id].flex_direction == 1u) { // Column
                    nodes[id].final_x = nodes[parent_id].final_x + nodes[parent_id].padding_left + nodes[parent_id].border_left_width + nodes[id].margin_left;
                    
                    var y_cursor = nodes[parent_id].final_y + nodes[parent_id].padding_top + nodes[parent_id].border_top_width;
                    let start = nodes[parent_id].child_start_index;
                    
                    // Sum previous siblings only if they are relative
                    for (var i = start; i < id; i = i + 1u) {
                        if (nodes[i].position_mode == 0u && (nodes[i].flags & 1u) != 0u) {
                            y_cursor += nodes[i].final_height + nodes[i].margin_top + nodes[i].margin_bottom;
                        }
                    }
                    nodes[id].final_y = y_cursor + nodes[id].margin_top;
                    
                } else { // Row
                    nodes[id].final_y = nodes[parent_id].final_y + nodes[parent_id].padding_top + nodes[parent_id].border_top_width + nodes[id].margin_top;
                    
                    var x_cursor = nodes[parent_id].final_x + nodes[parent_id].padding_left + nodes[parent_id].border_left_width;
                    let start = nodes[parent_id].child_start_index;
                    
                     // Sum previous siblings only if they are relative
                    for (var i = start; i < id; i = i + 1u) {
                        if (nodes[i].position_mode == 0u && (nodes[i].flags & 1u) != 0u) {
                            x_cursor += nodes[i].final_width + nodes[i].margin_left + nodes[i].margin_right;
                        }
                    }
                    nodes[id].final_x = x_cursor + nodes[id].margin_left;
                }
            }
            atomicStore(&nodes[id].signals_finished, 1u);
        }
    }
}

// PASS 0: Resolve Styles (runs before layout)
@compute @workgroup_size(64)
fn resolve_styles(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x;
    if (id >= u32(uniforms.node_count)) {
        return;
    }

    // Default values for style properties
    nodes[id].fixed_width = -1.0;
    nodes[id].fixed_height = -1.0;
    nodes[id].color_r = 0.0;
    nodes[id].color_g = 0.0;
    nodes[id].color_b = 0.0;
    nodes[id].color_a = 0.0;
    nodes[id].top_offset = 0.0;
    nodes[id].left_offset = 0.0;
    nodes[id].z_index = 0.0;
    nodes[id].position_mode = 0u;
    nodes[id].flex_direction = 0u;
    nodes[id].padding_top = 0.0;
    nodes[id].padding_right = 0.0;
    nodes[id].padding_bottom = 0.0;
    nodes[id].padding_left = 0.0;
    nodes[id].margin_top = 0.0;
    nodes[id].margin_right = 0.0;
    nodes[id].margin_bottom = 0.0;
    nodes[id].margin_left = 0.0;
    nodes[id].border_top_width = 0.0;
    nodes[id].border_right_width = 0.0;
    nodes[id].border_bottom_width = 0.0;
    nodes[id].border_left_width = 0.0;
    nodes[id].border_color_top = 0u;
    nodes[id].border_color_right = 0u;
    nodes[id].border_color_bottom = 0u;
    nodes[id].border_color_left = 0u;
    nodes[id].outline_width = 0.0;
    nodes[id].outline_offset = 0.0;
    nodes[id].outline_color_top = 0u;
    nodes[id].outline_color_right = 0u;
    nodes[id].outline_color_bottom = 0u;
    nodes[id].outline_color_left = 0u;
    nodes[id].box_shadow_spread = 0.0;
    nodes[id].box_shadow_color = 0u;
    nodes[id].text_color_r = 1.0;
    nodes[id].text_color_g = 1.0;
    nodes[id].text_color_b = 1.0;
    nodes[id].text_color_a = 1.0;
    nodes[id].font_size = 24.0;
    nodes[id].fill_color = 0u;
    nodes[id].stroke_color = 0u;
    nodes[id].stroke_width = 0.0;
    nodes[id]._pad_styles = 0u;

    let is_hovered = (nodes[id].flags & 16u) != 0u;
    let list_offset = nodes[id].class_data_offset;
    let count = node_class_list[list_offset];

    for (var c = 0u; c < count; c = c + 1u) {
        var pos = node_class_list[list_offset + 1u + c]; // offset into class_defs
        var in_hover = false;
        loop {
            let prop_id = class_defs[pos];
            if (prop_id == CTRL_END) { break; }
            if (prop_id == CTRL_HOVER_START) {
                in_hover = true;
                pos = pos + 1u;
                continue;
            }
            pos = pos + 1u;

            let apply = (in_hover && is_hovered) || (!in_hover);

            switch (prop_id) {
                case PROP_BACKGROUND_COLOR_RGBA: {
                    if (apply) {
                        nodes[id].color_r = bitcast<f32>(class_defs[pos]);
                        nodes[id].color_g = bitcast<f32>(class_defs[pos + 1u]);
                        nodes[id].color_b = bitcast<f32>(class_defs[pos + 2u]);
                        nodes[id].color_a = bitcast<f32>(class_defs[pos + 3u]);
                    }
                    pos = pos + 4u;
                }
                case PROP_WIDTH: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].fixed_width = val; }
                        else { nodes[id].fixed_width = -1.0; } // Fallback to auto
                    }
                    pos = pos + 2u;
                }
                case PROP_HEIGHT: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].fixed_height = val; }
                        else { nodes[id].fixed_height = -1.0; } // Fallback to auto
                    }
                    pos = pos + 2u;
                }
                case PROP_FLEX_DIRECTION: {
                    if (apply) { nodes[id].flex_direction = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case PROP_POSITION_MODE: {
                    if (apply) { nodes[id].position_mode = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case PROP_TOP: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].top_offset = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_LEFT: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].left_offset = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_Z_INDEX: {
                    if (apply) { nodes[id].z_index = bitcast<f32>(class_defs[pos]); }
                    pos = pos + 1u;
                }
                case PROP_PADDING_TOP: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].padding_top = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_PADDING_RIGHT: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].padding_right = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_PADDING_BOTTOM: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].padding_bottom = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_PADDING_LEFT: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].padding_left = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_MARGIN_TOP: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].margin_top = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_MARGIN_RIGHT: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].margin_right = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_MARGIN_BOTTOM: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].margin_bottom = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_MARGIN_LEFT: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].margin_left = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_BORDER_TOP_WIDTH: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].border_top_width = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_BORDER_RIGHT_WIDTH: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].border_right_width = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_BORDER_BOTTOM_WIDTH: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].border_bottom_width = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_BORDER_LEFT_WIDTH: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].border_left_width = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_BORDER_COLOR_TOP: {
                    if (apply) { nodes[id].border_color_top = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case PROP_BORDER_COLOR_RIGHT: {
                    if (apply) { nodes[id].border_color_right = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case PROP_BORDER_COLOR_BOTTOM: {
                    if (apply) { nodes[id].border_color_bottom = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case PROP_BORDER_COLOR_LEFT: {
                    if (apply) { nodes[id].border_color_left = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case PROP_OUTLINE_WIDTH: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].outline_width = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_OUTLINE_OFFSET: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].outline_offset = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_OUTLINE_COLOR_TOP: {
                    if (apply) { nodes[id].outline_color_top = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case PROP_OUTLINE_COLOR_RIGHT: {
                    if (apply) { nodes[id].outline_color_right = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case PROP_OUTLINE_COLOR_BOTTOM: {
                    if (apply) { nodes[id].outline_color_bottom = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case PROP_OUTLINE_COLOR_LEFT: {
                    if (apply) { nodes[id].outline_color_left = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case PROP_BOX_SHADOW_H_OFFSET: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].box_shadow_h_offset = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_BOX_SHADOW_V_OFFSET: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].box_shadow_v_offset = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_BOX_SHADOW_BLUR: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].box_shadow_blur = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_BOX_SHADOW_SPREAD: {
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].box_shadow_spread = val; }
                    }
                    pos = pos + 2u;
                }
                case PROP_BOX_SHADOW_COLOR: {
                    if (apply) { nodes[id].box_shadow_color = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case PROP_TEXT_COLOR_RGBA: {
                    if (apply) {
                        nodes[id].text_color_r = bitcast<f32>(class_defs[pos]);
                        nodes[id].text_color_g = bitcast<f32>(class_defs[pos + 1u]);
                        nodes[id].text_color_b = bitcast<f32>(class_defs[pos + 2u]);
                        nodes[id].text_color_a = bitcast<f32>(class_defs[pos + 3u]);
                    }
                    pos = pos + 4u;
                }
                case 37u: { // PROP_FONT_SIZE
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].font_size = val; }
                    }
                    pos = pos + 2u;
                }
                case 38u: { // PROP_FILL_COLOR
                    if (apply) { nodes[id].fill_color = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case 39u: { // PROP_STROKE_COLOR
                    if (apply) { nodes[id].stroke_color = class_defs[pos]; }
                    pos = pos + 1u;
                }
                case 40u: { // PROP_STROKE_WIDTH
                    if (apply) {
                        let val = bitcast<f32>(class_defs[pos]);
                        let unit = class_defs[pos + 1u];
                        if (unit == 1u) { nodes[id].stroke_width = val; }
                    }
                    pos = pos + 2u;
                }
                default: { break; }
            }
        }
    }
}

// PASS 0.5: Inherit Styles
@compute @workgroup_size(64)
fn inherit_styles(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x;
    if (id >= u32(uniforms.node_count)) {
        return;
    }

    let parent_id = nodes[id].parent_index;
    if (parent_id == id) {
        atomicStore(&nodes[id].signals_finished, 1u);
        return;
    }

    if (atomicLoad(&nodes[parent_id].signals_finished) == 1u && atomicLoad(&nodes[id].signals_finished) == 0u) {
        // Basic Inheritance Heuristic: 
        // If font_size is still 24.0 (default), inherit from parent.
        if (nodes[id].font_size == 24.0) {
            nodes[id].font_size = nodes[parent_id].font_size;
        }
        
        // Inherit text color if default white (1.0, 1.0, 1.0, 1.0)
        if (nodes[id].text_color_r == 1.0 && nodes[id].text_color_g == 1.0 && nodes[id].text_color_b == 1.0 && nodes[id].text_color_a == 1.0) {
            nodes[id].text_color_r = nodes[parent_id].text_color_r;
            nodes[id].text_color_g = nodes[parent_id].text_color_g;
            nodes[id].text_color_b = nodes[parent_id].text_color_b;
            nodes[id].text_color_a = nodes[parent_id].text_color_a;
        }

        atomicStore(&nodes[id].signals_finished, 1u);
    }
}
