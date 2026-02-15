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
    justify_content: u32,
    align_items: u32,

    parent_index: u32,
    child_start_index: u32,
    child_count: u32,
    signals_finished: u32,
    text_start: u32,
    text_length: u32,
    flags: u32, // Bit 0 = Visible
    natural_content_width: f32,
    
    // --- Texture Atlas UVs ---
    uv_min_x: f32, 
    uv_min_y: f32, 
    uv_max_x: f32, 
    uv_max_y: f32,
    
    // --- Padding to 128 bytes ---
    cpu_index: u32, 
    curve_start_index: u32, 
    curve_count: u32, 

    // --- GPU Style System ---
    class_data_offset: u32,
    
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
    @location(4) @interpolate(flat) curve_start: u32,
    @location(5) @interpolate(flat) curve_count: u32,
    @location(6) @interpolate(flat) dimensions: vec2<f32>,
    @location(7) @interpolate(flat) expansion: f32,
    @location(8) @interpolate(flat) instance_index: u32,
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

    // Calculate Expansion (Outline width + offset, if positive)
    // Calculate Expansion
    let outline_expansion = node.outline_width + max(0.0, node.outline_offset);
    let shadow_expansion = max(abs(node.box_shadow_h_offset), abs(node.box_shadow_v_offset)) + node.box_shadow_spread + node.box_shadow_blur * 2.0;
    let expansion = max(outline_expansion, shadow_expansion);

    let x1 = node.final_x - expansion;
    let y1 = node.final_y - expansion;
    let x2 = node.final_x + node.final_width + expansion;
    let y2 = node.final_y + node.final_height + expansion;
    
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
    out.dimensions = vec2<f32>(node.final_width, node.final_height);
    out.expansion = expansion;

    // local_pos should be in PIXELS relative to (final_x, final_y)
    let pixel_x = mix(-expansion, node.final_width + expansion, uv.x);
    let pixel_y = mix(-expansion, node.final_height + expansion, uv.y);
    out.local_pos = vec2<f32>(pixel_x, pixel_y);

    out.glyph_index = 0u;
    out.flags = node.flags;
    out.curve_start = node.curve_start_index;
    out.curve_count = node.curve_count;
    out.instance_index = instance_index;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate derivatives at the very beginning to ensure uniform control flow.
    // Derivatives must be calculated BEFORE any divergent branching (returns/discards).
    let dx = dpdx(in.local_pos);
    let dy = dpdy(in.local_pos);

    let node = nodes[in.instance_index];
    let pixel_pos = in.local_pos; // Pixel units relative to (final_x, final_y)
    let dims = in.dimensions;

    // 1. Distance from the node's main box [0, 0] to [dims.x, dims.y]
    let dist_x = max(-pixel_pos.x, pixel_pos.x - dims.x);
    let dist_y = max(-pixel_pos.y, pixel_pos.y - dims.y);
    let box_dist = max(dist_x, dist_y);

    // --- Content & Border (On Top) ---
    if (box_dist <= 0.0) {
        // Border
        if (pixel_pos.y < node.border_top_width) {
            return unpack4x8unorm(node.border_color_top);
        } else if (pixel_pos.y > dims.y - node.border_bottom_width) {
            return unpack4x8unorm(node.border_color_bottom);
        } else if (pixel_pos.x < node.border_left_width) {
            return unpack4x8unorm(node.border_color_left);
        } else if (pixel_pos.x > dims.x - node.border_right_width) {
            return unpack4x8unorm(node.border_color_right);
        }

        // Content
        let content_uv = pixel_pos / dims;
        if ((in.flags & 2u) != 0u) { // Image
            let atlas_uv = vec2<f32>(
                mix(node.uv_min_x, node.uv_max_x, content_uv.x),
                mix(node.uv_min_y, node.uv_max_y, content_uv.y)
            );
            return textureSampleLevel(t_diffuse, s_diffuse, atlas_uv, 0.0);
        }
        if ((in.flags & 4u) != 0u) { // Shape
            if (in.curve_count == 0u) { discard; }
            var total_coverage = 0.0;
            let SAMPLES = 4u;
            for (var sy = 0u; sy < SAMPLES; sy = sy + 1u) {
                for (var sx = 0u; sx < SAMPLES; sx = sx + 1u) {
                    let sub_offset = (vec2<f32>(f32(sx), f32(sy)) / f32(SAMPLES)) - 0.5;
                    let p_sub = pixel_pos + sub_offset.x * dx + sub_offset.y * dy;
                    if (calculate_winding(p_sub, in.curve_start, in.curve_count) != 0) {
                        total_coverage += 1.0;
                    }
                }
            }
            let alpha = total_coverage / f32(SAMPLES * SAMPLES);
            return vec4<f32>(in.color.rgb, alpha * in.color.a);
        }
        return in.color;
    }

    // --- Outline (Middle Layer) ---
    let outline_inner = node.outline_offset;
    let outline_outer = node.outline_offset + node.outline_width;
    if (box_dist > outline_inner && box_dist <= outline_outer) {
        if (pixel_pos.y < 0.0 && abs(pixel_pos.y) >= abs(pixel_pos.x) && abs(pixel_pos.y) >= abs(pixel_pos.x - dims.x)) {
            return unpack4x8unorm(node.outline_color_top);
        } else if (pixel_pos.y > dims.y && abs(pixel_pos.y - dims.y) >= abs(pixel_pos.x) && abs(pixel_pos.y - dims.y) >= abs(pixel_pos.x - dims.x)) {
            return unpack4x8unorm(node.outline_color_bottom);
        } else if (pixel_pos.x < 0.0) {
            return unpack4x8unorm(node.outline_color_left);
        } else {
            return unpack4x8unorm(node.outline_color_right);
        }
    }

    // --- Box Shadow (Bottom Layer) ---
    let shadow_color = unpack4x8unorm(node.box_shadow_color);
    if (shadow_color.a > 0.0) {
        let shadow_pos = pixel_pos - vec2<f32>(node.box_shadow_h_offset, node.box_shadow_v_offset);
        let s_dist_x = max(-shadow_pos.x - node.box_shadow_spread, shadow_pos.x - dims.x - node.box_shadow_spread);
        let s_dist_y = max(-shadow_pos.y - node.box_shadow_spread, shadow_pos.y - dims.y - node.box_shadow_spread);
        let shadow_dist = max(s_dist_x, s_dist_y);
        
        var shadow_alpha = 0.0;
        if (node.box_shadow_blur <= 0.0) {
            shadow_alpha = select(0.0, 1.0, shadow_dist <= 0.0);
        } else {
            shadow_alpha = 1.0 - smoothstep(-node.box_shadow_blur, node.box_shadow_blur, shadow_dist);
        }
        
        if (shadow_alpha > 0.0) {
             return vec4<f32>(shadow_color.rgb, shadow_color.a * shadow_alpha);
        }
    }

    discard;
    return vec4<f32>(0.0);
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

    let font_size = node.font_size;
    let base_x = node.final_x + char.x;
    let base_y = node.final_y + char.y;
    let w = char.width;
    let h = char.height;
    
    // 1px padding for AA. 
    let padding = 1.0; 
    
    var corner = vec2<f32>(0.0, 0.0);
    if (vertex_index == 0u) { corner = vec2(-padding, -padding); }
    else if (vertex_index == 1u) { corner = vec2(w + padding, -padding); }
    else if (vertex_index == 2u) { corner = vec2(-padding, h + padding); }
    else if (vertex_index == 3u) { corner = vec2(-padding, h + padding); }
    else if (vertex_index == 4u) { corner = vec2(w + padding, -padding); }
    else if (vertex_index == 5u) { corner = vec2(w + padding, h + padding); }

    // Fake italic/oblique via geometric shear.
    if (node.font_style == 1u || node.font_style == 2u) {
        let shear = select(0.12, 0.18, node.font_style == 2u);
        corner.x = corner.x + (h - corner.y) * shear;
    }
    
    let pos = vec2(base_x + corner.x, base_y + corner.y);
    let ndc_x = (pos.x / uniforms.screen_width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pos.y / uniforms.screen_height) * 2.0;

    // Text slightly in front of background
    let z = 1.0 - (node.z_index / 10000.0) - 0.0001;

    var out: VertexOutput;
    out.position = vec4<f32>(ndc_x, ndc_y, z, 1.0);
    out.color = vec4<f32>(node.text_color_r, node.text_color_g, node.text_color_b, node.text_color_a);
    
    // Map local_pos to EM units
    out.local_pos = corner / font_size;
    
    out.glyph_index = char.glyph_index;
    out.flags = 0u;
    out.curve_start = 0u;
    out.curve_count = 0u;
    out.dimensions = vec2(0.0, 0.0);
    out.instance_index = char.node_index;
    return out;
}

@fragment
fn fs_text(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.glyph_index >= arrayLength(&glyph_infos)) { discard; }
    let info = glyph_infos[in.glyph_index];
    if (info.count == 0u) { discard; }

    let node = nodes[in.instance_index];
    let pixel_size = 1.0 / node.font_size;
    let weight_delta = max(0.0, (f32(node.font_weight) - 400.0) / 500.0);
    let embolden_px = weight_delta * 0.8 * pixel_size;

    var total_coverage = 0.0;
    let SAMPLES = 4u;
    let STEP = pixel_size / f32(SAMPLES);
    
    for (var sy = 0u; sy < SAMPLES; sy = sy + 1u) {
        for (var sx = 0u; sx < SAMPLES; sx = sx + 1u) {
            let p = in.local_pos + vec2<f32>(f32(sx) + 0.5, f32(sy) + 0.5) * STEP;
            if (is_inside(p, info) || (embolden_px > 0.0 && is_inside(p - vec2<f32>(embolden_px, 0.0), info))) {
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
    
    return calculate_winding(p, start, count) != 0;
}

fn calculate_winding(p: vec2<f32>, start: u32, count: u32) -> i32 {
    var winding = 0;

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
                if (t1 > 0.0 && t1 < 1.0) { splits[num_splits] = t1; num_splits += 1u; }
                if (t2 > 0.0 && t2 < 1.0) {
                    if (num_splits == 0u || abs(t2 - splits[0]) > 1e-6) {
                        splits[num_splits] = t2; num_splits += 1u;
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
        for (var k = 0u; k <= num_splits; k += 1u) {
            var t_next = 1.0;
            if (k < num_splits) { t_next = splits[k]; }
            
            process_monotonic_segment(t_prev, t_next, p, p0, p1, p2, p3, &winding);
            t_prev = t_next;
        }
    }
    return winding;
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
