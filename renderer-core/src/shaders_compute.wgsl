struct Node {
    fixed_width: f32, // -1.0 = auto
    min_width: f32,
    max_width: f32, // -1.0 = none
    fixed_height: f32, // -1.0 = auto
    min_height: f32,
    max_height: f32, // -1.0 = none
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
    right_offset: f32,
    bottom_offset: f32,
    z_index: f32,
    fixed_width_unit: u32,
    fixed_height_unit: u32,
    min_width_unit: u32,
    max_width_unit: u32,
    min_height_unit: u32,
    max_height_unit: u32,
    top_offset_unit: u32,
    left_offset_unit: u32,
    right_offset_unit: u32,
    bottom_offset_unit: u32,
    z_index_specified: u32,
    position_mode: u32,
    flex_direction: u32,
    justify_content: u32,
    align_items: u32,

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

    text_align: u32,
    line_height: f32,
    letter_spacing: f32,
    word_spacing: f32,
    font_weight: u32,
    font_style: u32,
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
    leaf_count: f32,
    mouse_x: f32,
    mouse_y: f32,
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
@group(0) @binding(5) var<storage, read> node_class_list_and_inline_styles: array<u32>;

struct Panic {
    error_prop_id: atomic<u32>,
};
@group(0) @binding(6) var<storage, read_write> panic_buffer: Panic;
@group(0) @binding(7) var<storage, read> leaf_nodes: array<u32>;

fn has_gpu_panic() -> bool {
    return atomicLoad(&panic_buffer.error_prop_id) != 0u;
}

// --- Style Constants (generated) ---
// These are injected by the renderer at shader compilation time.
// See style_defs.toml for the source of truth.

@compute @workgroup_size(64)
fn reset_signals(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (has_gpu_panic()) {
        return;
    }
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

fn line_shift(align: u32, line_width: f32, content_width: f32) -> f32 {
    let slack = max(0.0, content_width - line_width);
    if (align == 1u) {
        return slack * 0.5;
    }
    if (align == 2u) {
        return slack;
    }
    return 0.0;
}

fn apply_line_alignment(start_idx: u32, end_idx: u32, shift: f32) {
    if (shift == 0.0 || end_idx <= start_idx) {
        return;
    }
    for (var i = start_idx; i < end_idx; i = i + 1u) {
        characters[i].x = characters[i].x + shift;
    }
}

fn layout_text(node_id: u32, max_width: f32, write_output: bool) -> LayoutResult {
    let start = nodes[node_id].text_start;
    let len = nodes[node_id].text_length;
    let font_ascender = uniforms.font_ascender;
    
    var cursor_x = 0.0;
    var cursor_y = 0.0;
    var max_x = 0.0;
    
    let font_size = nodes[node_id].font_size;
    let scale = font_size;
    let line_height = uniforms.line_height * max(0.1, nodes[node_id].line_height);
    let letter_spacing = nodes[node_id].letter_spacing;
    let word_spacing = nodes[node_id].word_spacing;
    let text_align = nodes[node_id].text_align;
    let available_width = max(0.0, max_width);
    var line_start = start;

    // Safety check for empty text
    if (len == 0u) {
        return LayoutResult(0.0, 0.0);
    }
    
    for (var i = 0u; i < len; i = i + 1u) {
        let idx = start + i;
        let char = characters[idx]; 
        let glyph_idx = char.glyph_index; 
        
        let glyph = glyph_data[glyph_idx];
        let is_space = char.value == 32u;
        var extra_spacing = letter_spacing;
        if (is_space) {
            extra_spacing = extra_spacing + word_spacing;
        }
        let advance = glyph.advance * scale + extra_spacing;
        
        // Word wrap check
        if (cursor_x + advance > available_width + 0.01 && cursor_x > 0.0) {
            if (write_output) {
                let shift = line_shift(text_align, cursor_x, available_width);
                apply_line_alignment(line_start, idx, shift);
            }
            cursor_x = 0.0;
            cursor_y += line_height * scale;
            line_start = idx;
        }
        
        if (write_output) {
            characters[idx].x = cursor_x + glyph.bearing_x * scale;
            characters[idx].y = cursor_y + font_ascender * scale - glyph.bearing_y * scale; 
            characters[idx].width = glyph.width * scale;
            characters[idx].height = glyph.height * scale;
        }
        
        cursor_x += advance;
        
        max_x = max(max_x, cursor_x);
    }

    if (write_output) {
        let end = start + len;
        let shift = line_shift(text_align, cursor_x, available_width);
        apply_line_alignment(line_start, end, shift);
    }
    
    return LayoutResult(max_x, cursor_y + line_height * scale);
}

// PASS 1: Natural Width (Bottom-Up)
@compute @workgroup_size(64)
fn width_bottom_up(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (has_gpu_panic()) {
        return;
    }
    let t_id = global_id.x;
    if (t_id >= u32(uniforms.leaf_count)) {
        return;
    }

    var current_id = leaf_nodes[t_id];
    
    loop {
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

fn resolve_length_horizontal(value: f32, unit: u32, percent_base: f32) -> f32 {
    switch (unit) {
        case UNIT_PX: { return value; }
        case UNIT_PERCENT: { return percent_base * value * 0.01; }
        case UNIT_VW: { return uniforms.screen_width * value * 0.01; }
        case UNIT_VH: { return uniforms.screen_height * value * 0.01; }
        default: { return value; }
    }
}

fn resolve_length_vertical(value: f32, unit: u32, percent_base: f32) -> f32 {
    switch (unit) {
        case UNIT_PX: { return value; }
        case UNIT_PERCENT: { return percent_base * value * 0.01; }
        case UNIT_VH: { return uniforms.screen_height * value * 0.01; }
        case UNIT_VW: { return uniforms.screen_width * value * 0.01; }
        default: { return value; }
    }
}

fn resolve_min_width(id: u32, percent_base: f32) -> f32 {
    if (nodes[id].min_width_unit == 0u) {
        return 0.0;
    }
    return max(0.0, resolve_length_horizontal(nodes[id].min_width, nodes[id].min_width_unit, percent_base));
}

fn resolve_max_width(id: u32, percent_base: f32) -> f32 {
    if (nodes[id].max_width_unit == 0u) {
        return -1.0;
    }
    return max(0.0, resolve_length_horizontal(nodes[id].max_width, nodes[id].max_width_unit, percent_base));
}

fn resolve_min_height(id: u32, percent_base: f32) -> f32 {
    if (nodes[id].min_height_unit == 0u) {
        return 0.0;
    }
    return max(0.0, resolve_length_vertical(nodes[id].min_height, nodes[id].min_height_unit, percent_base));
}

fn resolve_max_height(id: u32, percent_base: f32) -> f32 {
    if (nodes[id].max_height_unit == 0u) {
        return -1.0;
    }
    return max(0.0, resolve_length_vertical(nodes[id].max_height, nodes[id].max_height_unit, percent_base));
}

fn resolve_specified_width(id: u32, percent_base: f32) -> f32 {
    if (nodes[id].fixed_width_unit == 0u) {
        return -1.0;
    }
    return max(0.0, resolve_length_horizontal(nodes[id].fixed_width, nodes[id].fixed_width_unit, percent_base));
}

fn resolve_specified_height(id: u32, percent_base: f32) -> f32 {
    if (nodes[id].fixed_height_unit == 0u) {
        return -1.0;
    }
    return max(0.0, resolve_length_vertical(nodes[id].fixed_height, nodes[id].fixed_height_unit, percent_base));
}

fn clamp_width(id: u32, w: f32, percent_base: f32) -> f32 {
    var clamped = max(0.0, w);
    clamped = max(clamped, resolve_min_width(id, percent_base));
    let max_width = resolve_max_width(id, percent_base);
    if (max_width >= 0.0) {
        clamped = min(clamped, max_width);
    }
    return clamped;
}

fn get_negotiated_outer_width(id: u32) -> f32 {
    var base_w = 0.0;
    let specified = resolve_specified_width(id, uniforms.screen_width);
    if (specified >= 0.0) {
        base_w = specified;
    } else {
        base_w = nodes[id].natural_content_width;
    }
    return clamp_width(id, base_w, uniforms.screen_width) + nodes[id].margin_left + nodes[id].margin_right;
}

fn clamp_height(id: u32, h: f32, percent_base: f32) -> f32 {
    var clamped = max(0.0, h);
    clamped = max(clamped, resolve_min_height(id, percent_base));
    let max_height = resolve_max_height(id, percent_base);
    if (max_height >= 0.0) {
        clamped = min(clamped, max_height);
    }
    return clamped;
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
    if (has_gpu_panic()) {
        return;
    }
    let id = global_id.x;
    if (id >= u32(uniforms.node_count)) {
        return;
    }
    
    let parent_id = nodes[id].parent_index;
    if (parent_id == id) {
        let root_specified = resolve_specified_width(id, uniforms.screen_width);
        let root_width = select(uniforms.screen_width, root_specified, root_specified >= 0.0);
        nodes[id].final_width = clamp_width(id, root_width, uniforms.screen_width);
        atomicStore(&nodes[id].signals_finished, 1u);
    } else {
        if (atomicLoad(&nodes[parent_id].signals_finished) == 1u && atomicLoad(&nodes[id].signals_finished) == 0u) {
            let available_parent_inner_width = max(
                0.0,
                nodes[parent_id].final_width
                    - nodes[parent_id].padding_left
                    - nodes[parent_id].padding_right
                    - nodes[parent_id].border_left_width
                    - nodes[parent_id].border_right_width,
            );
            let specified_width = resolve_specified_width(id, available_parent_inner_width);

            if (nodes[id].position_mode == 1u) {
                let has_left = nodes[id].left_offset_unit != 0u;
                let has_right = nodes[id].right_offset_unit != 0u;
                let left = select(
                    0.0,
                    resolve_length_horizontal(nodes[id].left_offset, nodes[id].left_offset_unit, available_parent_inner_width),
                    has_left,
                );
                let right = select(
                    0.0,
                    resolve_length_horizontal(nodes[id].right_offset, nodes[id].right_offset_unit, available_parent_inner_width),
                    has_right,
                );

                if (specified_width >= 0.0) {
                    nodes[id].final_width = specified_width;
                } else if (has_left && has_right) {
                    nodes[id].final_width = max(
                        0.0,
                        available_parent_inner_width
                            - left
                            - right
                            - nodes[id].margin_left
                            - nodes[id].margin_right,
                    );
                } else {
                    nodes[id].final_width = max(
                        0.0,
                        nodes[id].natural_content_width - nodes[id].margin_left - nodes[id].margin_right,
                    );
                }
            } else {
                // Relative/Flex
                if (nodes[parent_id].flex_direction == 1u) { // Column
                    if (specified_width >= 0.0) {
                        nodes[id].final_width = specified_width;
                    } else {
                        nodes[id].final_width = max(0.0, available_parent_inner_width - nodes[id].margin_left - nodes[id].margin_right);
                    }
                } else { // Row
                    if (specified_width >= 0.0) {
                        nodes[id].final_width = specified_width;
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
            nodes[id].final_width = clamp_width(id, nodes[id].final_width, available_parent_inner_width);
            atomicStore(&nodes[id].signals_finished, 1u);
        }
    }
}

// PASS 3: Natural Height (Bottom-Up)
@compute @workgroup_size(64)
fn height_bottom_up(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (has_gpu_panic()) {
        return;
    }
    let t_id = global_id.x;
    if (t_id >= u32(uniforms.leaf_count)) {
        return;
    }

    var current_id = leaf_nodes[t_id];
    
    loop {
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

    let specified_height = resolve_specified_height(id, uniforms.screen_height);
    if (specified_height >= 0.0 && nodes[id].fixed_height_unit != UNIT_PERCENT) {
        nodes[id].desired_height = clamp_height(id, specified_height, uniforms.screen_height);
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
    nodes[id].desired_height = clamp_height(id, nodes[id].desired_height, uniforms.screen_height);
}

fn justify_distribution(mode: u32, free_space: f32, item_count: u32) -> vec2<f32> {
    // Returns (leading_space, between_space) along the main axis.
    if (item_count == 0u || free_space <= 0.0) {
        return vec2<f32>(0.0, 0.0);
    }

    switch (mode) {
        case 1u: { // center
            return vec2<f32>(free_space * 0.5, 0.0);
        }
        case 2u: { // flex-end
            return vec2<f32>(free_space, 0.0);
        }
        case 3u: { // space-between
            if (item_count > 1u) {
                return vec2<f32>(0.0, free_space / f32(item_count - 1u));
            }
            return vec2<f32>(0.0, 0.0);
        }
        case 4u: { // space-around
            let gap = free_space / f32(item_count);
            return vec2<f32>(gap * 0.5, gap);
        }
        case 5u: { // space-evenly
            let gap = free_space / f32(item_count + 1u);
            return vec2<f32>(gap, gap);
        }
        default: { // flex-start
            return vec2<f32>(0.0, 0.0);
        }
    }
}

fn align_offset(mode: u32, free_space: f32) -> f32 {
    // stretch and flex-start both align from the start edge.
    if (free_space <= 0.0) {
        return 0.0;
    }
    if (mode == 2u) { // center
        return free_space * 0.5;
    }
    if (mode == 3u) { // flex-end
        return free_space;
    }
    return 0.0;
}

// PASS 4: Final Layout (Top-Down)
@compute @workgroup_size(64)
fn final_layout(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (has_gpu_panic()) {
        return;
    }
    let id = global_id.x;
    if (id >= u32(uniforms.node_count)) {
        return;
    }
    
    let parent_id = nodes[id].parent_index;
    
    if (parent_id == id) {
        nodes[id].final_x = 0.0;
        nodes[id].final_y = 0.0;
        let root_specified_height = resolve_specified_height(id, uniforms.screen_height);
        let root_height = select(uniforms.screen_height, root_specified_height, root_specified_height >= 0.0);
        nodes[id].final_height = clamp_height(id, root_height, uniforms.screen_height);
        atomicStore(&nodes[id].signals_finished, 1u);
    } else {
        if (atomicLoad(&nodes[parent_id].signals_finished) == 1u && atomicLoad(&nodes[id].signals_finished) == 0u) {
            let parent_inner_x = nodes[parent_id].final_x + nodes[parent_id].padding_left + nodes[parent_id].border_left_width;
            let parent_inner_y = nodes[parent_id].final_y + nodes[parent_id].padding_top + nodes[parent_id].border_top_width;
            let available_parent_inner_width = max(
                0.0,
                nodes[parent_id].final_width
                    - nodes[parent_id].padding_left
                    - nodes[parent_id].padding_right
                    - nodes[parent_id].border_left_width
                    - nodes[parent_id].border_right_width,
            );
            let available_parent_inner_height = max(
                0.0,
                nodes[parent_id].final_height
                    - nodes[parent_id].padding_top
                    - nodes[parent_id].padding_bottom
                    - nodes[parent_id].border_top_width
                    - nodes[parent_id].border_bottom_width,
            );
            if (nodes[id].z_index_specified == 0u) {
                nodes[id].z_index = nodes[parent_id].z_index;
            }

            if (nodes[id].position_mode == 1u) { // Absolute
                let has_left = nodes[id].left_offset_unit != 0u;
                let has_right = nodes[id].right_offset_unit != 0u;
                let has_top = nodes[id].top_offset_unit != 0u;
                let has_bottom = nodes[id].bottom_offset_unit != 0u;
                let left = select(0.0, resolve_length_horizontal(nodes[id].left_offset, nodes[id].left_offset_unit, available_parent_inner_width), has_left);
                let right = select(0.0, resolve_length_horizontal(nodes[id].right_offset, nodes[id].right_offset_unit, available_parent_inner_width), has_right);
                let top = select(0.0, resolve_length_vertical(nodes[id].top_offset, nodes[id].top_offset_unit, available_parent_inner_height), has_top);
                let bottom = select(0.0, resolve_length_vertical(nodes[id].bottom_offset, nodes[id].bottom_offset_unit, available_parent_inner_height), has_bottom);

                let specified_height = resolve_specified_height(id, available_parent_inner_height);
                if (specified_height >= 0.0) {
                    nodes[id].final_height = specified_height;
                } else if (has_top && has_bottom) {
                    nodes[id].final_height = max(
                        0.0,
                        available_parent_inner_height - top - bottom - nodes[id].margin_top - nodes[id].margin_bottom,
                    );
                } else {
                    nodes[id].final_height = nodes[id].desired_height;
                }
                nodes[id].final_height = clamp_height(id, nodes[id].final_height, available_parent_inner_height);

                if (has_left) {
                    nodes[id].final_x = parent_inner_x + left + nodes[id].margin_left;
                } else if (has_right) {
                    nodes[id].final_x = parent_inner_x + available_parent_inner_width - right - nodes[id].final_width - nodes[id].margin_right;
                } else {
                    nodes[id].final_x = parent_inner_x + nodes[id].margin_left;
                }

                if (has_top) {
                    nodes[id].final_y = parent_inner_y + top + nodes[id].margin_top;
                } else if (has_bottom) {
                    nodes[id].final_y = parent_inner_y + available_parent_inner_height - bottom - nodes[id].final_height - nodes[id].margin_bottom;
                } else {
                    nodes[id].final_y = parent_inner_y + nodes[id].margin_top;
                }
            } else { // Relative
                let start = nodes[parent_id].child_start_index;
                let end = start + nodes[parent_id].child_count;
                let specified_height = resolve_specified_height(id, available_parent_inner_height);

                if (nodes[parent_id].flex_direction == 1u) { // Column
                    let parent_natural_inner_height = max(0.0, nodes[parent_id].desired_height - nodes[parent_id].padding_top - nodes[parent_id].padding_bottom - nodes[parent_id].border_top_width - nodes[parent_id].border_bottom_width);
                    
                    var ratio = 1.0;
                    if (parent_natural_inner_height > 0.0) {
                        ratio = available_parent_inner_height / parent_natural_inner_height;
                    }
                    
                    if (specified_height >= 0.0) {
                        nodes[id].final_height = specified_height;
                    } else {
                        let my_outer_main = (nodes[id].desired_height + nodes[id].margin_top + nodes[id].margin_bottom) * ratio;
                        nodes[id].final_height = max(0.0, my_outer_main - nodes[id].margin_top - nodes[id].margin_bottom);
                    }
                    nodes[id].final_height = clamp_height(id, nodes[id].final_height, available_parent_inner_height);

                    var relative_count = 0u;
                    var total_outer_main = 0.0;
                    for (var i = start; i < end; i = i + 1u) {
                        if (nodes[i].position_mode == 0u && (nodes[i].flags & 1u) != 0u) {
                            total_outer_main += (nodes[i].desired_height + nodes[i].margin_top + nodes[i].margin_bottom) * ratio;
                            relative_count += 1u;
                        }
                    }

                    let free_main = max(0.0, available_parent_inner_height - total_outer_main);
                    let main_distribution = justify_distribution(nodes[parent_id].justify_content, free_main, relative_count);

                    var main_before = 0.0;
                    for (var i = start; i < id; i = i + 1u) {
                        if (nodes[i].position_mode == 0u && (nodes[i].flags & 1u) != 0u) {
                            main_before += (nodes[i].desired_height + nodes[i].margin_top + nodes[i].margin_bottom) * ratio + main_distribution.y;
                        }
                    }

                    nodes[id].final_y = parent_inner_y + main_distribution.x + main_before + nodes[id].margin_top;

                    let my_outer_cross = nodes[id].final_width + nodes[id].margin_left + nodes[id].margin_right;
                    let free_cross = max(0.0, available_parent_inner_width - my_outer_cross);
                    let cross_shift = align_offset(nodes[parent_id].align_items, free_cross);
                    nodes[id].final_x = parent_inner_x + cross_shift + nodes[id].margin_left;
                    
                } else { // Row
                    if (specified_height >= 0.0) {
                        nodes[id].final_height = specified_height;
                    } else {
                        if (nodes[parent_id].align_items == 0u) { // stretch
                            nodes[id].final_height = max(0.0, available_parent_inner_height - nodes[id].margin_top - nodes[id].margin_bottom);
                        } else {
                            nodes[id].final_height = max(0.0, nodes[id].desired_height);
                        }
                    }
                    nodes[id].final_height = clamp_height(id, nodes[id].final_height, available_parent_inner_height);

                    var relative_count = 0u;
                    var total_outer_main = 0.0;
                    for (var i = start; i < end; i = i + 1u) {
                        if (nodes[i].position_mode == 0u && (nodes[i].flags & 1u) != 0u) {
                            total_outer_main += nodes[i].final_width + nodes[i].margin_left + nodes[i].margin_right;
                            relative_count += 1u;
                        }
                    }

                    let free_main = max(0.0, available_parent_inner_width - total_outer_main);
                    let main_distribution = justify_distribution(nodes[parent_id].justify_content, free_main, relative_count);

                    var main_before = 0.0;
                    for (var i = start; i < id; i = i + 1u) {
                        if (nodes[i].position_mode == 0u && (nodes[i].flags & 1u) != 0u) {
                            main_before += nodes[i].final_width + nodes[i].margin_left + nodes[i].margin_right + main_distribution.y;
                        }
                    }

                    nodes[id].final_x = parent_inner_x + main_distribution.x + main_before + nodes[id].margin_left;

                    let my_outer_cross = nodes[id].final_height + nodes[id].margin_top + nodes[id].margin_bottom;
                    let free_cross = max(0.0, available_parent_inner_height - my_outer_cross);
                    let cross_shift = align_offset(nodes[parent_id].align_items, free_cross);
                    nodes[id].final_y = parent_inner_y + cross_shift + nodes[id].margin_top;
                }
            }
            atomicStore(&nodes[id].signals_finished, 1u);
        }
    }
}

// Helper to read from either class_defs or node_class_list_and_inline_styles
fn get_style_val(buffer_selector: u32, pos: u32) -> u32 {
    if (buffer_selector == 0u) {
        return class_defs[pos];
    } else {
        return node_class_list_and_inline_styles[pos];
    }
}

// Subroutine to apply a stream of properties until CTRL_END
fn apply_style_stream(id: u32, buffer_selector: u32, start_pos: u32, is_hovered: bool) -> u32 {
    var pos = start_pos;
    var in_hover = false;
    loop {
        let prop_id = get_style_val(buffer_selector, pos);
        if (prop_id == CTRL_END) { 
            pos = pos + 1u;
            break; 
        }
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
                    nodes[id].color_r = bitcast<f32>(get_style_val(buffer_selector, pos));
                    nodes[id].color_g = bitcast<f32>(get_style_val(buffer_selector, pos + 1u));
                    nodes[id].color_b = bitcast<f32>(get_style_val(buffer_selector, pos + 2u));
                    nodes[id].color_a = bitcast<f32>(get_style_val(buffer_selector, pos + 3u));
                }
                pos = pos + 4u;
                break;
            }
            case PROP_WIDTH: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].fixed_width = val;
                    nodes[id].fixed_width_unit = unit;
                }
                pos = pos + 2u;
                break;
            }
            case PROP_HEIGHT: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].fixed_height = val;
                    nodes[id].fixed_height_unit = unit;
                }
                pos = pos + 2u;
                break;
            }
            case PROP_MIN_WIDTH: {
                if (apply) {
                    let val = max(0.0, bitcast<f32>(get_style_val(buffer_selector, pos)));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].min_width = val;
                    nodes[id].min_width_unit = unit;
                }
                pos = pos + 2u;
                break;
            }
            case PROP_MAX_WIDTH: {
                if (apply) {
                    let val = max(0.0, bitcast<f32>(get_style_val(buffer_selector, pos)));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].max_width = val;
                    nodes[id].max_width_unit = unit;
                }
                pos = pos + 2u;
                break;
            }
            case PROP_MIN_HEIGHT: {
                if (apply) {
                    let val = max(0.0, bitcast<f32>(get_style_val(buffer_selector, pos)));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].min_height = val;
                    nodes[id].min_height_unit = unit;
                }
                pos = pos + 2u;
                break;
            }
            case PROP_MAX_HEIGHT: {
                if (apply) {
                    let val = max(0.0, bitcast<f32>(get_style_val(buffer_selector, pos)));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].max_height = val;
                    nodes[id].max_height_unit = unit;
                }
                pos = pos + 2u;
                break;
            }
            case PROP_FLEX_DIRECTION: {
                if (apply) { nodes[id].flex_direction = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_JUSTIFY_CONTENT: {
                if (apply) { nodes[id].justify_content = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_ALIGN_ITEMS: {
                if (apply) { nodes[id].align_items = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_POSITION_MODE: {
                if (apply) { nodes[id].position_mode = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_TOP: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].top_offset = val;
                    nodes[id].top_offset_unit = unit;
                }
                pos = pos + 2u;
                break;
            }
            case PROP_LEFT: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].left_offset = val;
                    nodes[id].left_offset_unit = unit;
                }
                pos = pos + 2u;
                break;
            }
            case PROP_RIGHT: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].right_offset = val;
                    nodes[id].right_offset_unit = unit;
                }
                pos = pos + 2u;
                break;
            }
            case PROP_BOTTOM: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].bottom_offset = val;
                    nodes[id].bottom_offset_unit = unit;
                }
                pos = pos + 2u;
                break;
            }
            case PROP_Z_INDEX: {
                if (apply) {
                    nodes[id].z_index = bitcast<f32>(get_style_val(buffer_selector, pos));
                    nodes[id].z_index_specified = 1u;
                }
                pos = pos + 1u;
                break;
            }
            case PROP_PADDING_TOP: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].padding_top = resolve_length_vertical(val, unit, uniforms.screen_height);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_PADDING_RIGHT: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].padding_right = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_PADDING_BOTTOM: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].padding_bottom = resolve_length_vertical(val, unit, uniforms.screen_height);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_PADDING_LEFT: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].padding_left = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_MARGIN_TOP: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].margin_top = resolve_length_vertical(val, unit, uniforms.screen_height);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_MARGIN_RIGHT: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].margin_right = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_MARGIN_BOTTOM: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].margin_bottom = resolve_length_vertical(val, unit, uniforms.screen_height);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_MARGIN_LEFT: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].margin_left = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_BORDER_TOP_WIDTH: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].border_top_width = resolve_length_vertical(val, unit, uniforms.screen_height);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_BORDER_RIGHT_WIDTH: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].border_right_width = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_BORDER_BOTTOM_WIDTH: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].border_bottom_width = resolve_length_vertical(val, unit, uniforms.screen_height);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_BORDER_LEFT_WIDTH: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].border_left_width = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_BORDER_COLOR_TOP: {
                if (apply) { nodes[id].border_color_top = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_BORDER_COLOR_RIGHT: {
                if (apply) { nodes[id].border_color_right = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_BORDER_COLOR_BOTTOM: {
                if (apply) { nodes[id].border_color_bottom = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_BORDER_COLOR_LEFT: {
                if (apply) { nodes[id].border_color_left = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_OUTLINE_WIDTH: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].outline_width = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_OUTLINE_OFFSET: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].outline_offset = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_OUTLINE_COLOR_TOP: {
                if (apply) { nodes[id].outline_color_top = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_OUTLINE_COLOR_RIGHT: {
                if (apply) { nodes[id].outline_color_right = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_OUTLINE_COLOR_BOTTOM: {
                if (apply) { nodes[id].outline_color_bottom = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_OUTLINE_COLOR_LEFT: {
                if (apply) { nodes[id].outline_color_left = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_BOX_SHADOW_H_OFFSET: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].box_shadow_h_offset = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_BOX_SHADOW_V_OFFSET: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].box_shadow_v_offset = resolve_length_vertical(val, unit, uniforms.screen_height);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_BOX_SHADOW_BLUR: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].box_shadow_blur = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_BOX_SHADOW_SPREAD: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].box_shadow_spread = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            case PROP_BOX_SHADOW_COLOR: {
                if (apply) { nodes[id].box_shadow_color = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_TEXT_COLOR_RGBA: {
                if (apply) {
                    nodes[id].text_color_r = bitcast<f32>(get_style_val(buffer_selector, pos));
                    nodes[id].text_color_g = bitcast<f32>(get_style_val(buffer_selector, pos + 1u));
                    nodes[id].text_color_b = bitcast<f32>(get_style_val(buffer_selector, pos + 2u));
                    nodes[id].text_color_a = bitcast<f32>(get_style_val(buffer_selector, pos + 3u));
                }
                pos = pos + 4u;
                break;
            }
            case PROP_FONT_SIZE: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    if (unit == UNIT_PX) { nodes[id].font_size = val; }
                    else if (unit == UNIT_EM) { nodes[id].font_size = val * nodes[id].font_size; }
                    else if (unit == UNIT_VH) { nodes[id].font_size = uniforms.screen_height * val * 0.01; }
                    else if (unit == UNIT_VW) { nodes[id].font_size = uniforms.screen_width * val * 0.01; }
                }
                pos = pos + 2u;
                break;
            }
            case PROP_TEXT_ALIGN: {
                if (apply) { nodes[id].text_align = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_LINE_HEIGHT: {
                if (apply) { nodes[id].line_height = max(0.1, bitcast<f32>(get_style_val(buffer_selector, pos))); }
                pos = pos + 1u;
                break;
            }
            case PROP_LETTER_SPACING: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    if (unit == UNIT_PX) { nodes[id].letter_spacing = val; }
                    else if (unit == UNIT_EM) { nodes[id].letter_spacing = val * nodes[id].font_size; }
                    else if (unit == UNIT_VH) { nodes[id].letter_spacing = uniforms.screen_height * val * 0.01; }
                    else if (unit == UNIT_VW) { nodes[id].letter_spacing = uniforms.screen_width * val * 0.01; }
                }
                pos = pos + 2u;
                break;
            }
            case PROP_WORD_SPACING: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    if (unit == UNIT_PX) { nodes[id].word_spacing = val; }
                    else if (unit == UNIT_EM) { nodes[id].word_spacing = val * nodes[id].font_size; }
                    else if (unit == UNIT_VH) { nodes[id].word_spacing = uniforms.screen_height * val * 0.01; }
                    else if (unit == UNIT_VW) { nodes[id].word_spacing = uniforms.screen_width * val * 0.01; }
                }
                pos = pos + 2u;
                break;
            }
            case PROP_FONT_WEIGHT: {
                if (apply) { nodes[id].font_weight = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_FONT_STYLE: {
                if (apply) { nodes[id].font_style = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_FILL_COLOR: {
                if (apply) { nodes[id].fill_color = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_STROKE_COLOR: {
                if (apply) { nodes[id].stroke_color = get_style_val(buffer_selector, pos); }
                pos = pos + 1u;
                break;
            }
            case PROP_STROKE_WIDTH: {
                if (apply) {
                    let val = bitcast<f32>(get_style_val(buffer_selector, pos));
                    let unit = get_style_val(buffer_selector, pos + 1u);
                    nodes[id].stroke_width = resolve_length_horizontal(val, unit, uniforms.screen_width);
                }
                pos = pos + 2u;
                break;
            }
            default: { 
                // CRITICAL: Unknown property behavior.
                atomicMax(&panic_buffer.error_prop_id, prop_id);
                return pos; 
            }
        }
    }
    return pos;
}

// PASS 0: Resolve Styles (runs before layout)
@compute @workgroup_size(64)
fn resolve_styles(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (has_gpu_panic()) {
        return;
    }
    let id = global_id.x;
    if (id >= u32(uniforms.node_count)) {
        return;
    }

    // 4. Default Layout Values (reset for each resolve pass)
    nodes[id].fixed_width = 0.0;
    nodes[id].fixed_width_unit = 0u;
    nodes[id].min_width = 0.0;
    nodes[id].min_width_unit = 0u;
    nodes[id].max_width = 0.0;
    nodes[id].max_width_unit = 0u;
    nodes[id].fixed_height = 0.0;
    nodes[id].fixed_height_unit = 0u;
    nodes[id].min_height = 0.0;
    nodes[id].min_height_unit = 0u;
    nodes[id].max_height = 0.0;
    nodes[id].max_height_unit = 0u;
    nodes[id].color_r = 0.0;
    nodes[id].color_g = 0.0;
    nodes[id].color_b = 0.0;
    nodes[id].color_a = 0.0;
    nodes[id].top_offset = 0.0;
    nodes[id].top_offset_unit = 0u;
    nodes[id].left_offset = 0.0;
    nodes[id].left_offset_unit = 0u;
    nodes[id].right_offset = 0.0;
    nodes[id].right_offset_unit = 0u;
    nodes[id].bottom_offset = 0.0;
    nodes[id].bottom_offset_unit = 0u;
    nodes[id].z_index = 0.0;
    nodes[id].z_index_specified = 0u;
    nodes[id].position_mode = 0u;
    nodes[id].flex_direction = 0u;
    nodes[id].justify_content = 0u;
    nodes[id].align_items = 0u;
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
    nodes[id].box_shadow_h_offset = 0.0;
    nodes[id].box_shadow_v_offset = 0.0;
    nodes[id].box_shadow_blur = 0.0;
    nodes[id].box_shadow_spread = 0.0;
    nodes[id].box_shadow_color = 0u;
    nodes[id].text_color_r = 1.0;
    nodes[id].text_color_g = 1.0;
    nodes[id].text_color_b = 1.0;
    nodes[id].text_color_a = 1.0;
    nodes[id].text_align = 0u;
    nodes[id].line_height = 1.0;
    nodes[id].letter_spacing = 0.0;
    nodes[id].word_spacing = 0.0;
    nodes[id].font_weight = 400u;
    nodes[id].font_style = 0u;
    nodes[id].font_size = 24.0;
    nodes[id].fill_color = 0u;
    nodes[id].stroke_color = 0u;
    nodes[id].stroke_width = 0.0;
    nodes[id]._pad_styles = 0u;

    let is_hovered = (nodes[id].flags & 16u) != 0u;
    let list_offset = nodes[id].class_data_offset;
    let count = node_class_list_and_inline_styles[list_offset];

    // 1. Process Classes
    for (var c = 0u; c < count; c = c + 1u) {
        let class_offset = node_class_list_and_inline_styles[list_offset + 1u + c];
        apply_style_stream(id, 0u, class_offset, is_hovered);
        if (has_gpu_panic()) {
            return;
        }
    }

    // 2. Process Inline Styles
    let inline_start = list_offset + 1u + count;
    apply_style_stream(id, 1u, inline_start, is_hovered);
}

// PASS 0.5: Inherit Styles
@compute @workgroup_size(64)
fn inherit_styles(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (has_gpu_panic()) {
        return;
    }
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
        if (nodes[id].z_index_specified == 0u) {
            nodes[id].z_index = nodes[parent_id].z_index;
        }

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
        if (nodes[id].text_align == 0u) {
            nodes[id].text_align = nodes[parent_id].text_align;
        }
        if (nodes[id].line_height == 1.0) {
            nodes[id].line_height = nodes[parent_id].line_height;
        }
        if (nodes[id].letter_spacing == 0.0) {
            nodes[id].letter_spacing = nodes[parent_id].letter_spacing;
        }
        if (nodes[id].word_spacing == 0.0) {
            nodes[id].word_spacing = nodes[parent_id].word_spacing;
        }
        if (nodes[id].font_weight == 400u) {
            nodes[id].font_weight = nodes[parent_id].font_weight;
        }
        if (nodes[id].font_style == 0u) {
            nodes[id].font_style = nodes[parent_id].font_style;
        }

        atomicStore(&nodes[id].signals_finished, 1u);
    }
}

struct HitTestResult {
    max_z_index: atomic<i32>,
    any_change: atomic<u32>,
    cpu_needed: atomic<u32>,
    _pad: u32,
};

@group(0) @binding(8) var<storage, read_write> hit_test_result: HitTestResult;

var<workgroup> wg_max_z: atomic<i32>;

const NODE_FLAG_VISIBLE: u32 = 1u << 0u;
const NODE_FLAG_HOVERED: u32 = 1u << 4u;
const NODE_FLAG_HAS_MOUSE_ENTER_LISTENER: u32 = 1u << 5u;
const NODE_FLAG_HAS_MOUSE_LEAVE_LISTENER: u32 = 1u << 6u;
const NODE_FLAG_MOUSE_ENTER_TRIGGERED: u32 = 1u << 7u;
const NODE_FLAG_MOUSE_LEAVE_TRIGGERED: u32 = 1u << 8u;

fn float_to_int_z(z: f32) -> i32 {
    return i32(z);
}

fn is_effectively_visible(node_index: u32) -> bool {
    var current = node_index;
    loop {
        if ((nodes[current].flags & NODE_FLAG_VISIBLE) == 0u) {
            return false;
        }
        if (current == 0u) {
            break;
        }
        let parent = nodes[current].parent_index;
        if (parent == current) {
            break;
        }
        current = parent;
    }
    return true;
}

@compute @workgroup_size(64)
fn hit_test_pass_1(@builtin(global_invocation_id) global_id: vec3<u32>, @builtin(local_invocation_id) local_id: vec3<u32>) {
    if (local_id.x == 0u) {
        atomicStore(&wg_max_z, -2147483648);
    }
    workgroupBarrier();

    let id = global_id.x;
    if (id < u32(uniforms.node_count)) {
        if (is_effectively_visible(id)) {
            let mx = uniforms.mouse_x;
            let my = uniforms.mouse_y;
            if mx >= nodes[id].final_x && mx <= nodes[id].final_x + nodes[id].final_width &&
                my >= nodes[id].final_y && my <= nodes[id].final_y + nodes[id].final_height {
                let z_int = float_to_int_z(nodes[id].z_index);
                atomicMax(&wg_max_z, z_int);
            }
        }
    }
    
    workgroupBarrier();
    
    if (local_id.x == 0u) {
        let local_max = atomicLoad(&wg_max_z);
        if (local_max > -2147483648) {
            atomicMax(&hit_test_result.max_z_index, local_max);
        }
    }
}

@compute @workgroup_size(64)
fn hit_test_pass_2(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x;
    if (id >= u32(uniforms.node_count)) {
        return;
    }

    var node_flags = nodes[id].flags;
    let was_hovered = (node_flags & NODE_FLAG_HOVERED) != 0u;
    var is_hovered = false;

    if (is_effectively_visible(id)) {
        let mx = uniforms.mouse_x;
        let my = uniforms.mouse_y;
        if mx >= nodes[id].final_x && mx <= nodes[id].final_x + nodes[id].final_width &&
            my >= nodes[id].final_y && my <= nodes[id].final_y + nodes[id].final_height {
            let max_z = atomicLoad(&hit_test_result.max_z_index);
            if float_to_int_z(nodes[id].z_index) == max_z {
                is_hovered = true;
            }
        }
    }

    node_flags &= ~(NODE_FLAG_MOUSE_ENTER_TRIGGERED | NODE_FLAG_MOUSE_LEAVE_TRIGGERED);

    if (is_hovered != was_hovered) {
        atomicStore(&hit_test_result.any_change, 1u);
        if (is_hovered) {
            node_flags |= NODE_FLAG_HOVERED;
        } else {
            node_flags &= ~NODE_FLAG_HOVERED;
        }

        let has_enter = (node_flags & NODE_FLAG_HAS_MOUSE_ENTER_LISTENER) != 0u;
        let has_leave = (node_flags & NODE_FLAG_HAS_MOUSE_LEAVE_LISTENER) != 0u;
        if (is_hovered && has_enter) {
            node_flags |= NODE_FLAG_MOUSE_ENTER_TRIGGERED;
            atomicStore(&hit_test_result.cpu_needed, 1u);
        }
        if ((!is_hovered) && has_leave) {
            node_flags |= NODE_FLAG_MOUSE_LEAVE_TRIGGERED;
            atomicStore(&hit_test_result.cpu_needed, 1u);
        }
    }

    nodes[id].flags = node_flags;
}
