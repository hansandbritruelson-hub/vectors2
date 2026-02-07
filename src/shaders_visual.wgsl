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
    signals_finished: u32,
    text_length: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0) var<storage, read> nodes: array<Node>;

struct Uniforms {
    screen_width: f32,
    screen_height: f32,
};
@group(0) @binding(1) var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    let node = nodes[instance_index];
    
    // Skip invisible/container nodes if they have no size
    // For now, we render everything.
    
    var pos = vec2<f32>(0.0, 0.0);
    let x1 = node.final_x;
    let y1 = node.final_y;
    let x2 = node.final_x + node.final_width;
    let y2 = node.final_y + node.final_height;
    
    // Quad vertices
    if (vertex_index == 0u) { pos = vec2<f32>(x1, y1); }
    else if (vertex_index == 1u) { pos = vec2<f32>(x2, y1); }
    else if (vertex_index == 2u) { pos = vec2<f32>(x1, y2); }
    else if (vertex_index == 3u) { pos = vec2<f32>(x1, y2); }
    else if (vertex_index == 4u) { pos = vec2<f32>(x2, y1); }
    else if (vertex_index == 5u) { pos = vec2<f32>(x2, y2); }
    
    // Convert to NDC (-1 to 1)
    let ndc_x = (pos.x / uniforms.screen_width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pos.y / uniforms.screen_height) * 2.0;
    
    var out: VertexOutput;
    out.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    
    // Generate a pseudo-random color based on index
    let r = f32(instance_index * 123u % 255u) / 255.0;
    let g = f32(instance_index * 456u % 255u) / 255.0;
    let b = f32(instance_index * 789u % 255u) / 255.0;
    out.color = vec3<f32>(r, g, b);
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
