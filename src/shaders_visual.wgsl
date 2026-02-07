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
    font_ascender: f32,
    line_height: f32,
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

fn solve_cubic(a: f32, b: f32, c: f32) -> vec3<f32> {
    let p = b - a * a / 3.0;
    let p3 = p * p * p;
    let q = a * (2.0 * a * a - 9.0 * b) / 27.0 + c;
    let d = q * q + 4.0 * p3 / 27.0;
    let offset = -a / 3.0;
    
    if (d > 0.0) {
        let z = sqrt(d);
        let u = (-q + z) / 2.0;
        let v = (-q - z) / 2.0;
        let u1 = sign(u) * pow(abs(u), 1.0/3.0);
        let v1 = sign(v) * pow(abs(v), 1.0/3.0);
        return vec3<f32>(u1 + v1 + offset, -1.0, -1.0);
    } else if (d == 0.0) {
        let u = -q / 2.0;
        let u1 = sign(u) * pow(abs(u), 1.0/3.0);
        return vec3<f32>(2.0 * u1 + offset, -u1 + offset, -1.0);
    } else {
        let u = sqrt(-p / 3.0);
        let v = acos(-sqrt(-27.0 / p3) * q / 2.0) / 3.0;
        let m = cos(v);
        let n = sin(v) * 1.732050808;
        return vec3<f32>(2.0 * u * m + offset, -u * (m + n) + offset, -u * (m - n) + offset);
    }
}

fn sd_bezier_sq(pos: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>) -> f32 {
    let A = p1 - p0;
    let B = p0 - 2.0 * p1 + p2;
    let C = p0 - pos;

    let dotBB = dot(B, B);

    if (dotBB < 0.0001) {
        let v = p2 - p0;
        let w = pos - p0;
        let c1 = dot(w, v);
        let c2 = dot(v, v);
        let b = clamp(c1 / c2, 0.0, 1.0);
        let dist = distance(pos, p0 + b * v);
        return dist * dist;
    }

    let k = 1.0 / dotBB;
    let c2 = 3.0 * dot(A, B) * k;
    let c1 = (2.0 * dot(A, A) + dot(C, B)) * k;
    let c0 = dot(C, A) * k;
    
    let roots = solve_cubic(c2, c1, c0);
    
    var min_d2 = dot(C, C);
    let d2_at_1 = dot(p2 - pos, p2 - pos);
    min_d2 = min(min_d2, d2_at_1);

    if (roots.x > 0.0 && roots.x < 1.0) {
        let t = roots.x;
        let q = C + 2.0 * t * A + t * t * B;
        min_d2 = min(min_d2, dot(q, q));
    }
    if (roots.y > 0.0 && roots.y < 1.0) {
        let t = roots.y;
        let q = C + 2.0 * t * A + t * t * B;
        min_d2 = min(min_d2, dot(q, q));
    }
    if (roots.z > 0.0 && roots.z < 1.0) {
        let t = roots.z;
        let q = C + 2.0 * t * A + t * t * B;
        min_d2 = min(min_d2, dot(q, q));
    }
    
    return min_d2;
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
    var min_dist_sq = 1e20; // Init to large value
    
    for (var i = 0u; i < count; i = i + 1u) {
        let curve = curves[start + i];
        let p0 = curve.p0;
        let p1 = curve.p1;
        let p2 = curve.p2;
        
        // 1. Distance Calculation (True Distance)
        let d2 = sd_bezier_sq(p, p0, p1, p2);
        if (d2 < min_dist_sq) {
            min_dist_sq = d2;
        }

        // 2. Winding Number Logic (Ray to right)
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
        
        if (num_roots > 0) {
            check_intersection(t0, p, p0, p1, p2, &winding);
        }
        if (num_roots > 1) {
            check_intersection(t1, p, p0, p1, p2, &winding);
        }
    }
    
    let dist = sqrt(min_dist_sq);
    
    // Signed Distance: Negative if inside (winding != 0), Positive if outside
    // Actually, traditionally SDF is negative inside.
    // Let's assume winding != 0 means inside.
    
    var sd = dist;
    if (winding != 0) {
        sd = -dist;
    }
    
    // Anti-Aliasing
    // screen pixel range is roughly 0.5 to 1.0 depending on preference.
    // Using fwidth on dist is theoretically better for scale independence,
    // but since we are computing in pixel space (initially), 0.5 is fine.
    // Actually, local_pos scale depends on vertex shader.
    // vs_text: out.local_pos = corner; corner is in Layout Units (pixels).
    // If dpr=2, then 1 layout unit = 2 physical pixels.
    // fwidth(dist) will be approx 1.0/dpr? or just magnitude of gradient?
    // Let's use fwidth to be safe against scaling.
    
    // fwidth requires uniform flow control, but we are after the loop.
    let afwidth = fwidth(dist);
    let alpha = 1.0 - smoothstep(-afwidth, afwidth, sd); 
    
    // Use standard 0.5 for crispness if fwidth is weird (e.g. constant regions)
    // let alpha = 1.0 - smoothstep(-0.5, 0.5, sd);

    if (alpha <= 0.0) {
        discard;
    }
    
    return vec4<f32>(in.color, alpha);
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
