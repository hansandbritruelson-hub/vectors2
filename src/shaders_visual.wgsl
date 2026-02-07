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
    text_start: u32,
    text_length: u32,
    _pad0: u32,
    _pad1: u32,
};

struct Character {
    value: u32,
    prev: u32,
    next: u32,
    node_index: u32,

    x: f32,
    y: f32,
    width: f32,
    height: f32,
};

@group(0) @binding(0) var<storage, read> nodes: array<Node>;

struct Uniforms {
    screen_width: f32,
    screen_height: f32,
};
@group(0) @binding(1) var<uniform> uniforms: Uniforms;
@group(0) @binding(2) var<storage, read> characters: array<Character>;
@group(0) @binding(3) var font_texture: texture_2d<f32>;
@group(0) @binding(4) var font_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    let node = nodes[instance_index];
    
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
    out.uv = vec2<f32>(0.0, 0.0);
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

@vertex
fn vs_text(@builtin(vertex_index) vertex_index: u32, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    let char = characters[instance_index];
    let node = nodes[char.node_index];
    
    let base_x = node.final_x + char.x;
    let base_y = node.final_y + char.y;
    // Shrink slightly to see individual chars
    let w = char.width - 2.0; 
    let h = char.height - 2.0;
    let off_x = 1.0;
    let off_y = 1.0;
    
    var corner = vec2<f32>(0.0, 0.0);
    if (vertex_index == 0u) { corner = vec2(0.0, 0.0); }
    else if (vertex_index == 1u) { corner = vec2(w, 0.0); }
    else if (vertex_index == 2u) { corner = vec2(0.0, h); }
    else if (vertex_index == 3u) { corner = vec2(0.0, h); }
    else if (vertex_index == 4u) { corner = vec2(w, 0.0); }
    else if (vertex_index == 5u) { corner = vec2(w, h); }
    
    let pos = vec2(base_x + off_x + corner.x, base_y + off_y + corner.y);
    
    // Convert to NDC (-1 to 1) y=0 -> 1, y=H -> -1
    let ndc_x = (pos.x / uniforms.screen_width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pos.y / uniforms.screen_height) * 2.0;
    
    // UV calc
    // Atlas: 512x512, Cell: 32x64
    // Cols = 16 (512/32)
    let char_code = char.value;
    var glyph_index = 0u;
    if (char_code >= 32u && char_code < 127u) {
        glyph_index = char_code - 32u;
    }
    
    let grid_cols = 16u;
    let col = glyph_index % grid_cols;
    let row = glyph_index / grid_cols;
    
    let cell_w = 32.0;
    let cell_h = 64.0;
    let tex_w = 512.0;
    let tex_h = 512.0;
    
    // UV within the cell
    var uv_local = vec2<f32>(0.0, 0.0);
    if (vertex_index == 0u) { uv_local = vec2(0.0, 0.0); } // TL
    else if (vertex_index == 1u) { uv_local = vec2(1.0, 0.0); } // TR
    else if (vertex_index == 2u) { uv_local = vec2(0.0, 1.0); } // BL
    else if (vertex_index == 3u) { uv_local = vec2(0.0, 1.0); } // BL
    else if (vertex_index == 4u) { uv_local = vec2(1.0, 0.0); } // TR
    else if (vertex_index == 5u) { uv_local = vec2(1.0, 1.0); } // BR

    let base_u = (f32(col) * cell_w) / tex_w;
    let base_v = (f32(row) * cell_h) / tex_h;
    
    let u = base_u + uv_local.x * (cell_w / tex_w);
    let v = base_v + uv_local.y * (cell_h / tex_h);

    var out: VertexOutput;
    out.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.color = vec3<f32>(1.0, 1.0, 1.0); 
    out.uv = vec2<f32>(u, v);
    return out;
}

@fragment
fn fs_text(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(font_texture, font_sampler, in.uv);
    // Discard transparent pixels for cleaner text
    if (tex_color.a < 0.1) {
        discard;
    }
    return tex_color * vec4<f32>(in.color, 1.0);
}
