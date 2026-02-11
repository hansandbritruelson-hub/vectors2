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
    signals_finished: u32,
    text_start: u32,
    text_length: u32,
    flags: u32, // Bit 0 = Visible
    natural_content_width: f32,
    
    // --- Padding to 128 bytes ---
    _pad0: u32, _pad1: u32, _pad2: u32, _pad3: u32,
    _pad4: u32, _pad5: u32, _pad6: u32, // Removed pad7
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

struct Curve {
    p0: vec2<f32>,
    p1: vec2<f32>,
    p2: vec2<f32>,
    p3: vec2<f32>,
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
    node_count: f32,
    _pad0: f32, _pad1: f32, _pad2: f32,
};
@group(0) @binding(1) var<uniform> uniforms: Uniforms;
@group(0) @binding(2) var<storage, read> characters: array<Character>;
@group(0) @binding(3) var<storage, read> curves: array<Curve>;
@group(0) @binding(4) var<storage, read> glyph_infos: array<GlyphInfo>;
@group(0) @binding(5) var t_diffuse: texture_2d<f32>;
@group(0) @binding(6) var s_diffuse: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) local_pos: vec2<f32>,
    @location(2) @interpolate(flat) glyph_index: u32,
    @location(3) @interpolate(flat) flags: u32,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    let node = nodes[instance_index];
    
    // Visibility Check
    if ((node.flags & 1u) == 0u) {
        var out: VertexOutput;
        out.position = vec4<f32>(0.0, 0.0, 0.0, 0.0); // Degenerate
        return out;
    }

    var pos = vec2<f32>(0.0, 0.0);
    var uv = vec2<f32>(0.0, 0.0);

    let x1 = node.final_x;
    let y1 = node.final_y;
    let x2 = node.final_x + node.final_width;
    let y2 = node.final_y + node.final_height;
    
    if (vertex_index == 0u) { pos = vec2<f32>(x1, y1); uv = vec2<f32>(0.0, 0.0); }
    else if (vertex_index == 1u) { pos = vec2<f32>(x2, y1); uv = vec2<f32>(1.0, 0.0); }
    else if (vertex_index == 2u) { pos = vec2<f32>(x1, y2); uv = vec2<f32>(0.0, 1.0); }
    else if (vertex_index == 3u) { pos = vec2<f32>(x1, y2); uv = vec2<f32>(0.0, 1.0); }
    else if (vertex_index == 4u) { pos = vec2<f32>(x2, y1); uv = vec2<f32>(1.0, 0.0); }
    else if (vertex_index == 5u) { pos = vec2<f32>(x2, y2); uv = vec2<f32>(1.0, 1.0); }
    
    let ndc_x = (pos.x / uniforms.screen_width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pos.y / uniforms.screen_height) * 2.0;

    // Z-Index Mapping: 0..10000 -> 1.0..0.0 (Near is 0.0, Far is 1.0)
    let z = 1.0 - (node.z_index / 10000.0);
    
    var out: VertexOutput;
    out.position = vec4<f32>(ndc_x, ndc_y, z, 1.0);
    out.color = vec4<f32>(node.color_r, node.color_g, node.color_b, node.color_a);
    out.local_pos = uv;
    out.glyph_index = 0u;
    out.flags = node.flags;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Check if Image Flag (Bit 1 = Value 2) is set
    if ((in.flags & 2u) != 0u) {
        let tex_color = textureSampleLevel(t_diffuse, s_diffuse, in.local_pos, 0.0);
        return tex_color;
        // Optionally mix with background color? For now just replace.
    }
    return in.color;
}

@vertex
fn vs_text(@builtin(vertex_index) vertex_index: u32, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    let char = characters[instance_index];
    let node = nodes[char.node_index];
    
    // Visibility Check
    if ((node.flags & 1u) == 0u) {
        var out: VertexOutput;
        out.position = vec4<f32>(0.0, 0.0, 0.0, 0.0); // Degenerate
        return out;
    }

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

    // Text slightly in front of background (margin 0.0001)
    let z = 1.0 - (node.z_index / 10000.0) - 0.0001;

    var out: VertexOutput;
    out.position = vec4<f32>(ndc_x, ndc_y, z, 1.0);
    out.color = vec4<f32>(1.0, 1.0, 1.0, 1.0); // White Text
    out.local_pos = corner - vec2(1.0, 1.0);
    out.glyph_index = char.glyph_index;
    return out;
}

@fragment
fn fs_text(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.glyph_index >= arrayLength(&glyph_infos)) { discard; }
    let info = glyph_infos[in.glyph_index];
    if (info.count == 0u) { discard; }

    var total_coverage = 0.0;
    let SAMPLES = 4u;
    let STEP = 1.0 / f32(SAMPLES);
    
    for (var sy = 0u; sy < SAMPLES; sy = sy + 1u) {
        for (var sx = 0u; sx < SAMPLES; sx = sx + 1u) {
            let p = in.local_pos + vec2<f32>(f32(sx) + 0.5, f32(sy) + 0.5) * STEP;
            if (is_inside(p, info)) {
                total_coverage += 1.0;
            }
        }
    }
    
    let alpha = total_coverage / f32(SAMPLES * SAMPLES);
    if (alpha <= 0.001) { discard; }
    return vec4<f32>(in.color.rgb, alpha * in.color.a);
}

fn is_inside(p: vec2<f32>, info: GlyphInfo) -> bool {
    var winding = 0;
    let start = info.start_index;
    let count = info.count;
    
    for (var i = 0u; i < count; i = i + 1u) {
        let curve = curves[start + i];
        let p0 = curve.p0;
        let p1 = curve.p1;
        let p2 = curve.p2;
        let p3 = curve.p3;
        
        // 1. Find roots of dy/dt = 0 to decompose into monotonic segments
        // dy/dt = 3at^2 + 2bt + c
        let y0 = p0.y;
        let y1 = p1.y;
        let y2 = p2.y;
        let y3 = p3.y;
        
        let a = -y0 + 3.0*y1 - 3.0*y2 + y3;
        let b = 3.0*y0 - 6.0*y1 + 3.0*y2;
        let c = -3.0*y0 + 3.0*y1;

        var splits = vec2<f32>(-1.0);
        var num_splits = 0u;
        
        // Quad solver for dy/dt = 0
        let q_a = 3.0 * a;
        let q_b = 2.0 * b;
        let q_c = c;
        
        if (abs(q_a) < 1e-6) {
            if (abs(q_b) > 1e-6) {
                let t = -q_c / q_b;
                if (t > 0.0 && t < 1.0) { splits[0] = t; num_splits = 1u; }
            }
        } else {
            let disc = q_b * q_b - 4.0 * q_a * q_c;
            if (disc >= 0.0) {
                let s_disc = sqrt(disc);
                let t1 = (-q_b - s_disc) / (2.0 * q_a);
                let t2 = (-q_b + s_disc) / (2.0 * q_a);
                if (t1 > 0.0 && t1 < 1.0) { splits[num_splits] = t1; num_splits++; }
                if (t2 > 0.0 && t2 < 1.0) {
                    if (num_splits == 0u || abs(t2 - splits[0]) > 1e-6) {
                        splits[num_splits] = t2; num_splits++;
                    }
                }
            }
        }
        
        // Ensure sorted splits
        if (num_splits == 2u && splits[0] > splits[1]) {
            let tmp = splits[0]; splits[0] = splits[1]; splits[1] = tmp;
        }
        
        // 2. Process each monotonic segment
        var t_prev = 0.0;
        for (var k = 0u; k <= num_splits; k++) {
            var t_next = 1.0;
            if (k < num_splits) { t_next = splits[k]; }
            
            process_monotonic_segment(t_prev, t_next, p, p0, p1, p2, p3, &winding);
            t_prev = t_next;
        }
    }
    return winding != 0;
}

fn process_monotonic_segment(t0: f32, t1: f32, p: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>, winding: ptr<function, i32>) {
    let y0_seg = get_cubic_coordinate(t0, p0.y, p1.y, p2.y, p3.y);
    let y1_seg = get_cubic_coordinate(t1, p0.y, p1.y, p2.y, p3.y);
    
    let min_y = min(y0_seg, y1_seg);
    let max_y = max(y0_seg, y1_seg);
    
    // Half-open interval rule [min_y, max_y)
    if (p.y >= min_y && p.y < max_y) {
        // Solve y(t) = p.y in [t0, t1]
        // Since it's monotonic, we can use the cubic solver or even binary search.
        // Let's use the cubic solver and find a root in [t0, t1].
        let y0 = p0.y;
        let y1 = p1.y;
        let y2 = p2.y;
        let y3 = p3.y;
        
        let a = -y0 + 3.0*y1 - 3.0*y2 + y3;
        let b = 3.0*y0 - 6.0*y1 + 3.0*y2;
        let c = -3.0*y0 + 3.0*y1;
        let d = y0 - p.y;
        
        var roots = vec3<f32>(-1.0);
        let n = solve_cubic(a, b, c, d, &roots);
        
        for (var i = 0u; i < n; i++) {
            let t = roots[i];
            // Allow a small epsilon for endpoint inclusion if needed, but strict interval on the monotonic segment
            if (t >= min(t0, t1) - 1e-5 && t <= max(t0, t1) + 1e-5) {
                let x_at_t = get_cubic_coordinate(t, p0.x, p1.x, p2.x, p3.x);
                if (x_at_t > p.x) {
                    if (y1_seg > y0_seg) {
                        *winding = *winding + 1;
                    } else {
                        *winding = *winding - 1;
                    }
                    return; // Monotonic segment has at most one root
                }
            }
        }
    }
}

fn get_cubic_coordinate(t: f32, c0: f32, c1: f32, c2: f32, c3: f32) -> f32 {
    let mt = 1.0 - t;
    return mt*mt*mt*c0 + 3.0*mt*mt*t*c1 + 3.0*mt*t*t*c2 + t*t*t*c3;
}

fn solve_cubic(a: f32, b: f32, c: f32, d: f32, roots: ptr<function, vec3<f32>>) -> u32 {
    if (abs(a) < 1e-6) {
        if (abs(b) < 1e-6) {
            if (abs(c) < 1e-6) { return 0u; }
            (*roots)[0] = -d / c;
            return 1u;
        }
        let disc = c * c - 4.0 * b * d;
        if (disc < 0.0) { return 0u; }
        if (disc == 0.0) {
            (*roots)[0] = -c / (2.0 * b);
            return 1u;
        }
        let s_disc = sqrt(disc);
        (*roots)[0] = (-c - s_disc) / (2.0 * b);
        (*roots)[1] = (-c + s_disc) / (2.0 * b);
        return 2u;
    }

    let A = b / a;
    let B = c / a;
    let C = d / a;

    let Q = (3.0 * B - A * A) / 9.0;
    let R = (9.0 * A * B - 27.0 * C - 2.0 * A * A * A) / 54.0;
    let D = Q * Q * Q + R * R; 

    if (D > 0.0) {
        let s_D = sqrt(D);
        let s1 = R + s_D;
        let s2 = R - s_D;
        let S = sign(s1) * pow(abs(s1), 1.0/3.0);
        let T = sign(s2) * pow(abs(s2), 1.0/3.0);
        (*roots)[0] = -A/3.0 + (S + T);
        return 1u;
    } else if (D == 0.0) {
        let S = sign(R) * pow(abs(R), 1.0/3.0);
        (*roots)[0] = -A/3.0 + 2.0 * S;
        (*roots)[1] = -A/3.0 - S;
        return 2u;
    } else {
        let Q_neg = -Q;
        let arg = R / sqrt(Q_neg * Q_neg * Q_neg);
        let theta = acos(clamp(arg, -1.0, 1.0));
        let s_Q = 2.0 * sqrt(Q_neg);
        (*roots)[0] = s_Q * cos(theta / 3.0) - A / 3.0;
        (*roots)[1] = s_Q * cos((theta + 2.0 * 3.14159265) / 3.0) - A / 3.0;
        (*roots)[2] = s_Q * cos((theta + 4.0 * 3.14159265) / 3.0) - A / 3.0;
        return 3u;
    }
}
