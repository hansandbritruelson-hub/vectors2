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
    glyph_index: u32, // Now strictly the font glyph index
    next_glyph_index: u32,
    node_index: u32,

    x: f32,
    y: f32,
    width: f32,
    height: f32,
};

struct Curve {
    p0: vec2<f32>,
    p1: vec2<f32>,
    p2: vec2<f32>,
    _pad: vec2<f32>, // Padding matches Rust GpuCurve
};

struct GlyphInfo {
    start_index: u32,
    count: u32,
    _pad0: u32,
    _pad1: u32,
};

@group(0) @binding(0) var<storage, read> nodes: array<Node>;

struct Uniforms {
    screen_width: f32,
    screen_height: f32,
};
@group(0) @binding(1) var<uniform> uniforms: Uniforms;
@group(0) @binding(2) var<storage, read> characters: array<Character>;

// New Buffers for Vector Rendering
@group(0) @binding(3) var<storage, read> curves: array<Curve>;
@group(0) @binding(4) var<storage, read> glyph_infos: array<GlyphInfo>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) local_pos: vec2<f32>,
    @location(2) @interpolate(flat) glyph_index: u32,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    let node = nodes[instance_index];
    
    var pos = vec2<f32>(0.0, 0.0);
    let x1 = node.final_x;
    let y1 = node.final_y;
    let x2 = node.final_x + node.final_width;
    let y2 = node.final_y + node.final_height;
    
    if (vertex_index == 0u) { pos = vec2<f32>(x1, y1); }
    else if (vertex_index == 1u) { pos = vec2<f32>(x2, y1); }
    else if (vertex_index == 2u) { pos = vec2<f32>(x1, y2); }
    else if (vertex_index == 3u) { pos = vec2<f32>(x1, y2); }
    else if (vertex_index == 4u) { pos = vec2<f32>(x2, y1); }
    else if (vertex_index == 5u) { pos = vec2<f32>(x2, y2); }
    
    let ndc_x = (pos.x / uniforms.screen_width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pos.y / uniforms.screen_height) * 2.0;
    
    var out: VertexOutput;
    out.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    
    let r = f32(instance_index * 123u % 255u) / 255.0;
    let g = f32(instance_index * 456u % 255u) / 255.0;
    let b = f32(instance_index * 789u % 255u) / 255.0;
    out.color = vec3<f32>(r, g, b);
    out.local_pos = vec2<f32>(0.0, 0.0);
    out.glyph_index = 0u;
    
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
    let w = char.width;
    let h = char.height;
    
    var corner = vec2<f32>(0.0, 0.0);
    if (vertex_index == 0u) { corner = vec2(0.0, 0.0); }
    else if (vertex_index == 1u) { corner = vec2(w, 0.0); }
    else if (vertex_index == 2u) { corner = vec2(0.0, h); }
    else if (vertex_index == 3u) { corner = vec2(0.0, h); }
    else if (vertex_index == 4u) { corner = vec2(w, 0.0); }
    else if (vertex_index == 5u) { corner = vec2(w, h); }
    
    let pos = vec2(base_x + corner.x, base_y + corner.y);
    
    let ndc_x = (pos.x / uniforms.screen_width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pos.y / uniforms.screen_height) * 2.0;

    var out: VertexOutput;
    out.position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.color = vec3<f32>(0.0, 0.0, 0.0); // Black text
    out.local_pos = corner; // Screen pixels 0..w, 0..h
    out.glyph_index = char.glyph_index;
    return out;
}

@fragment
fn fs_text(in: VertexOutput) -> @location(0) vec4<f32> {
    // 1. Get Glyph Info
    // Note: glyph_index in Character might be larger than our glyph_infos array if we didn't cache all
    // But since we built infos for ALL glyphs in the font, it should be fine.
    
    if (in.glyph_index >= arrayLength(&glyph_infos)) {
        discard;
    }
    let info = glyph_infos[in.glyph_index];
    let count = info.count;
    let start = info.start_index;
    
    if (count == 0u) {
        discard;
    }
    
    let p = in.local_pos; // Pixel coordinate relative to Glyph Box Origin
    
    // Winding Number Algorithm
    // Ray Cast to the Right (+X direction)
    var winding = 0;
    
    for (var i = 0u; i < count; i = i + 1u) {
        let curve = curves[start + i];
        let p0 = curve.p0;
        let p1 = curve.p1;
        let p2 = curve.p2;
        
        // Solve Intersection: Curve Y = p.y
        // Bezier Y(t) = (1-t)^2 y0 + 2(1-t)t y1 + t^2 y2
        // A t^2 + B t + C = 0
        
        let y0 = p0.y;
        let y1 = p1.y;
        let y2 = p2.y;
        
        let A = y0 - 2.0 * y1 + y2;
        let B = 2.0 * (y1 - y0);
        let C = y0 - p.y;
        
        // Quadratic Solver
        var t0 = -1.0;
        var t1 = -1.0;
        var num_roots = 0;
        
        if (abs(A) < 0.001) {
            // Linear case: Bt + C = 0 -> t = -C / B
            if (abs(B) > 0.001) {
                t0 = -C / B;
                num_roots = 1;
            }
        } else {
            let disc = B * B - 4.0 * A * C;
            if (disc >= 0.0) {
                let sqrt_disc = sqrt(disc);
                t0 = (-B - sqrt_disc) / (2.0 * A);
                t1 = (-B + sqrt_disc) / (2.0 * A);
                num_roots = 2;
            }
        }
        
        // Check roots
        // Note: we can optimize loop by skipping if y range doesn't cover p.y
        // But for brute force, just check t.
        
        if (num_roots > 0) {
            check_intersection(t0, p, p0, p1, p2, &winding);
        }
        if (num_roots > 1) {
            check_intersection(t1, p, p0, p1, p2, &winding);
        }
    }
    
    if (winding != 0) {
        return vec4<f32>(in.color, 1.0);
    } else {
        discard;
    }
    return vec4<f32>(0.0); // Unreachable
}

fn check_intersection(t: f32, p: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, winding: ptr<function, i32>) {
    if (t >= 0.0 && t < 1.0) {
        // Calculate X at t
        let mt = 1.0 - t;
        let x_at_t = (mt * mt * p0.x) + (2.0 * mt * t * p1.x) + (t * t * p2.x);
        
        if (x_at_t > p.x) {
            // Intersection to the right!
            // Determine direction (Derivative Y)
            // dy/dt = 2(1-t)(y1-y0) + 2t(y2-y1)
            let dy = 2.0 * mt * (p1.y - p0.y) + 2.0 * t * (p2.y - p1.y);
            
            if (dy > 0.0) {
                *winding += 1;
            } else {
                *winding -= 1;
            }
        }
    }
}
