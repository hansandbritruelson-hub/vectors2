struct Node {
    style_min_width: f32,
    style_basis: f32,
    desired_width: f32,
    final_width: f32,
    
    desired_height: f32,
    final_height: f32,
    final_x: f32,
    final_y: f32,
    
    parent_index: u32,
    child_start_index: u32,
    child_count: u32,
    signals_finished: atomic<u32>,
    text_start: u32,
    text_length: u32,
    flex_direction: u32, // 0 = Row, 1 = Column
    _pad0: u32,
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
};

struct GlyphData {
    advance: f32,
    bearing_x: f32,
    width: f32,
    height: f32,
};

@group(0) @binding(0) var<storage, read_write> nodes: array<Node>;
@group(0) @binding(1) var<uniform> uniforms: Uniforms;
@group(0) @binding(2) var<storage, read_write> characters: array<Character>;
@group(0) @binding(3) var<storage, read> glyph_data: array<GlyphData>;

struct LayoutResult {
    width: f32,
    height: f32,
};

fn layout_text(node_id: u32, max_width: f32, write_output: bool) -> LayoutResult {
    let start = nodes[node_id].text_start;
    let len = nodes[node_id].text_length;
    let font_size_scale = 1.0; // Assume metrics are pre-scaled or this is 1.0
    let line_height = 24.0; // Based on font size
    
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
            characters[idx].y = cursor_y; 
            characters[idx].width = glyph.width;
            characters[idx].height = glyph.height;
        }
        
        cursor_x += advance;
        
        max_x = max(max_x, cursor_x);
    }
    
    return LayoutResult(max_x, cursor_y + line_height);
}

// PASS 1: Intrinsic Width (Bottom-Up)
@compute @workgroup_size(64)
fn width_bottom_up(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let node_id = global_id.x;
    if (node_id >= arrayLength(&nodes)) {
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

fn process_node_width(id: u32) {
    let count = nodes[id].child_count;
    
    if (count == 0u) {
        // Pass 1: Measure with infinite width
        let result = layout_text(id, 100000.0, false);
        nodes[id].desired_width = result.width;
    } else {
        var result_width = 0.0;
        let start = nodes[id].child_start_index;
        
        if (nodes[id].flex_direction == 1u) { // Column
            for (var i = 0u; i < count; i = i + 1u) {
                result_width = max(result_width, nodes[start + i].desired_width);
            }
        } else { // Row (Default)
            for (var i = 0u; i < count; i = i + 1u) {
                result_width += nodes[start + i].desired_width;
            }
        }
        nodes[id].desired_width = max(nodes[id].style_min_width, result_width);
    }
}

// PASS 2: Resolve Width (Top-Down)
@compute @workgroup_size(64)
fn width_top_down(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x;
    if (id >= arrayLength(&nodes)) {
        return;
    }
    
    atomicStore(&nodes[id].signals_finished, 0u);

    let parent_id = nodes[id].parent_index;
    if (parent_id == id) {
        nodes[id].final_width = uniforms.screen_width;
    } else {
        // Check Parent Direction
        if (nodes[parent_id].flex_direction == 1u) { // Column
            // In Column, children get the parent's final width
            nodes[id].final_width = nodes[parent_id].final_width;
        } else { // Row
            let parent_desired = nodes[parent_id].desired_width;
            let parent_final = nodes[parent_id].final_width;
            let my_desired = nodes[id].desired_width;
            
            if (parent_desired > 0.0) {
                let ratio = parent_final / parent_desired;
                nodes[id].final_width = my_desired * ratio;
            } else {
                nodes[id].final_width = 0.0;
            }
        }
    }
}

// PASS 3: Intrinsic Height (Bottom-Up)
@compute @workgroup_size(64)
fn height_bottom_up(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let node_id = global_id.x;
    if (node_id >= arrayLength(&nodes)) {
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
    let count = nodes[id].child_count;
    if (count == 0u) {
        // Pass 3: Layout text characters with wrapping and Write Coordinates
        let result = layout_text(id, nodes[id].final_width, true);
        nodes[id].desired_height = result.height;
    } else {
        var result_height = 0.0;
        let start = nodes[id].child_start_index;
        
        if (nodes[id].flex_direction == 1u) { // Column
            for (var i = 0u; i < count; i = i + 1u) {
                 result_height += nodes[start + i].desired_height;
            }
        } else { // Row
            for (var i = 0u; i < count; i = i + 1u) {
                result_height = max(result_height, nodes[start + i].desired_height);
            }
        }
        nodes[id].desired_height = result_height;
    }
}

// PASS 4: Final Layout (Top-Down)
@compute @workgroup_size(64)
fn final_layout(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x;
    if (id >= arrayLength(&nodes)) {
        return;
    }
    
    atomicStore(&nodes[id].signals_finished, 0u);
    nodes[id].final_height = nodes[id].desired_height;

    let parent_id = nodes[id].parent_index;
    
    if (parent_id == id) {
        nodes[id].final_x = 0.0;
        nodes[id].final_y = 0.0;
        nodes[id].final_height = uniforms.screen_height; 
    } else {
        if (nodes[parent_id].flex_direction == 1u) { // Column
            nodes[id].final_x = nodes[parent_id].final_x;
            
            var y_cursor = nodes[parent_id].final_y;
            let start = nodes[parent_id].child_start_index;
            
            for (var i = start; i < id; i = i + 1u) {
                y_cursor += nodes[i].final_height;
            }
            nodes[id].final_y = y_cursor;
            
        } else { // Row
            nodes[id].final_y = nodes[parent_id].final_y;
            
            var x_cursor = nodes[parent_id].final_x;
            let start = nodes[parent_id].child_start_index;
            
            for (var i = start; i < id; i = i + 1u) {
                x_cursor += nodes[i].final_width;
            }
            nodes[id].final_x = x_cursor;
        }
    }
}
