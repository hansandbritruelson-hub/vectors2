struct Node {
    fixed_width: f32, // -1.0 = auto
    min_width: f32,
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
    
    // --- Padding to 128 bytes ---
    _pad0: u32, _pad1: u32, _pad2: u32, _pad3: u32,
    _pad4: u32, _pad5: u32, _pad6: u32, _pad7: u32,
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
        if (cursor_x + advance > max_width + 0.01 && cursor_x > 0.0) {
            cursor_x = 0.0;
            cursor_y += line_height;
        }
        
        if (write_output) {
            characters[idx].x = cursor_x + glyph.bearing_x;
            characters[idx].y = cursor_y + font_ascender - glyph.bearing_y; 
            characters[idx].width = glyph.width;
            characters[idx].height = glyph.height;
        }
        
        cursor_x += advance;
        
        max_x = max(max_x, cursor_x);
    }
    
    return LayoutResult(max_x, cursor_y + line_height);
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

fn get_negotiated_width(id: u32) -> f32 {
    if (nodes[id].fixed_width >= 0.0) {
        return nodes[id].fixed_width;
    }
    return max(nodes[id].min_width, nodes[id].natural_content_width);
}

fn process_node_width(id: u32) {
    let count = nodes[id].child_count;

    // Visibility Check
    if ((nodes[id].flags & 1u) == 0u) {
        nodes[id].natural_content_width = 0.0;
        return;
    }

    if (count == 0u) {
        // Pass 1: Measure with infinite width
        let result = layout_text(id, 100000.0, false);
        nodes[id].natural_content_width = result.width;
    } else {
        var result_width = 0.0;
        let start = nodes[id].child_start_index;

        if (nodes[id].flex_direction == 1u) { // Column
            for (var i = 0u; i < count; i = i + 1u) {
                // POS ABSOLUTE CHECK
                if (nodes[start + i].position_mode == 0u && (nodes[start + i].flags & 1u) != 0u) {
                    result_width = max(result_width, get_negotiated_width(start + i));
                }
            }
        } else { // Row (Default)
            for (var i = 0u; i < count; i = i + 1u) {
                 // POS ABSOLUTE CHECK
                if (nodes[start + i].position_mode == 0u && (nodes[start + i].flags & 1u) != 0u) {
                    result_width += get_negotiated_width(start + i);
                }
            }
        }
        nodes[id].natural_content_width = result_width;
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
                // Absolute: Use negotiated width
                nodes[id].final_width = get_negotiated_width(id);
            } else {
                // Relative/Flex
                if (nodes[parent_id].flex_direction == 1u) { // Column
                    if (nodes[id].fixed_width >= 0.0) {
                        nodes[id].final_width = nodes[id].fixed_width;
                    } else {
                        nodes[id].final_width = nodes[parent_id].final_width;
                    }
                } else { // Row
                    if (nodes[id].fixed_width >= 0.0) {
                        nodes[id].final_width = nodes[id].fixed_width;
                    } else {
                        let parent_natural = nodes[parent_id].natural_content_width;
                        let parent_final = nodes[parent_id].final_width;
                        let my_negotiated = get_negotiated_width(id);
                        
                        if (parent_natural > 0.0) {
                            let ratio = parent_final / parent_natural;
                            nodes[id].final_width = my_negotiated * ratio;
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

    let count = nodes[id].child_count;
    if (count == 0u) {
        let result = layout_text(id, nodes[id].final_width, true);
        nodes[id].desired_height = result.height;
    } else {
        var result_height = 0.0;
        let start = nodes[id].child_start_index;
        
        if (nodes[id].flex_direction == 1u) { // Column
            for (var i = 0u; i < count; i = i + 1u) {
                 if (nodes[start + i].position_mode == 0u && (nodes[start + i].flags & 1u) != 0u) {
                    result_height += nodes[start + i].desired_height;
                 }
            }
        } else { // Row
            for (var i = 0u; i < count; i = i + 1u) {
                if (nodes[start + i].position_mode == 0u && (nodes[start + i].flags & 1u) != 0u) {
                    result_height = max(result_height, nodes[start + i].desired_height);
                }
            }
        }
        nodes[id].desired_height = result_height;
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
                 nodes[id].final_x = nodes[parent_id].final_x + nodes[id].left_offset;
                 nodes[id].final_y = nodes[parent_id].final_y + nodes[id].top_offset;
            } else { // Relative
                if (nodes[parent_id].flex_direction == 1u) { // Column
                    nodes[id].final_x = nodes[parent_id].final_x;
                    
                    var y_cursor = nodes[parent_id].final_y;
                    let start = nodes[parent_id].child_start_index;
                    
                    // Sum previous siblings only if they are relative
                    for (var i = start; i < id; i = i + 1u) {
                        if (nodes[i].position_mode == 0u && (nodes[i].flags & 1u) != 0u) {
                            y_cursor += nodes[i].desired_height;
                        }
                    }
                    nodes[id].final_y = y_cursor;
                    
                } else { // Row
                    nodes[id].final_y = nodes[parent_id].final_y;
                    
                    var x_cursor = nodes[parent_id].final_x;
                    let start = nodes[parent_id].child_start_index;
                    
                     // Sum previous siblings only if they are relative
                    for (var i = start; i < id; i = i + 1u) {
                        if (nodes[i].position_mode == 0u && (nodes[i].flags & 1u) != 0u) {
                            x_cursor += nodes[i].final_width;
                        }
                    }
                    nodes[id].final_x = x_cursor;
                }
            }
            atomicStore(&nodes[id].signals_finished, 1u);
        }
    }
}


