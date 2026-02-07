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
    text_length: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0) var<storage, read_write> nodes: array<Node>;

struct Uniforms {
    screen_width: f32,
    screen_height: f32,
};
@group(0) @binding(1) var<uniform> uniforms: Uniforms;

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
        // Compute intrinsic width based on text length (10px per character)
        nodes[id].desired_width = f32(nodes[id].text_length) * 10.0;
    } else {
        var sum_width = 0.0;
        let start = nodes[id].child_start_index;
        for (var i = 0u; i < count; i = i + 1u) {
            sum_width += nodes[start + i].desired_width;
        }
        nodes[id].desired_width = max(nodes[id].style_min_width, sum_width);
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
        // Intrinsic height based on wrapping text
        let char_width = 10.0;
        let line_height = 20.0;
        let total_content_width = f32(nodes[id].text_length) * char_width;
        let width = max(char_width, nodes[id].final_width);
        let num_lines = ceil(total_content_width / width);
        nodes[id].desired_height = max(line_height, num_lines * line_height);
    } else {
        var max_height = 0.0;
        let start = nodes[id].child_start_index;
        for (var i = 0u; i < count; i = i + 1u) {
            max_height = max(max_height, nodes[start + i].desired_height);
        }
        nodes[id].desired_height = max_height;
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
        nodes[id].final_y = nodes[parent_id].final_y;
        
        var x_cursor = nodes[parent_id].final_x;
        let start = nodes[parent_id].child_start_index;
        
        for (var i = start; i < id; i = i + 1u) {
            x_cursor += nodes[i].final_width;
        }
        nodes[id].final_x = x_cursor;
    }
}
