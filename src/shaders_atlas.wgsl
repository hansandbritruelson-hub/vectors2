struct GlyphData {
    glyph_id: u32,
    start_curve: u32,
    curve_count: u32,
};

struct Curve {
    p0: vec2<f32>,
    p1: vec2<f32>,
    p2: vec2<f32>,
};

@group(0) @binding(0) var<storage, read> glyph_data: array<GlyphData>;
@group(0) @binding(1) var<storage, read> curves: array<f32>; // packed as floats: x0,y0,x1,y1,x2,y2...

struct AtlasInfo {
    width: u32,
    height: u32,
    glyph_count: u32,
    grid_cols: u32,
    cell_size: u32,
    upem: u32,
};
@group(0) @binding(2) var<uniform> info: AtlasInfo;
@group(0) @binding(3) var atlas_tex: texture_storage_2d<rgba8unorm, write>;

fn read_curve(idx: u32) -> Curve {
    let base = idx * 6u;
    return Curve(
        vec2<f32>(curves[base], curves[base + 1u]),
        vec2<f32>(curves[base + 2u], curves[base + 3u]),
        vec2<f32>(curves[base + 4u], curves[base + 5u])
    );
}

fn intersect_quadratic(p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, y: f32) -> i32 {
    // Solve (1-t)^2*y0 + 2t(1-t)*y1 + t^2*y2 = y
    // Formula: a*t^2 + b*t + c = 0
    let a = p0.y - 2.0 * p1.y + p2.y;
    let b = 2.0 * (p1.y - p0.y);
    let c = p0.y - y;

    var count = 0;
    if (abs(a) < 0.0001) {
        let t = -c / b;
        if (t >= 0.0 && t <= 1.0) {
            let x = (1.0 - t) * (1.0 - t) * p0.x + 2.0 * t * (1.0 - t) * p1.x + t * t * p2.x;
            count += 1;
        }
    } else {
        let det = b * b - 4.0 * a * c;
        if (det >= 0.0) {
            let sdet = sqrt(det);
            let t1 = (-b + sdet) / (2.0 * a);
            let t2 = (-b - sdet) / (2.0 * a);
            if (t1 >= 0.0 && t1 <= 1.0) { count += 1; }
            if (t2 >= 0.0 && t2 <= 1.0) { count += 1; }
        }
    }
    return count;
}

// Implicit Curve Rasterization (simplified Winding Number)
@compute @workgroup_size(8, 8)
fn generate_atlas(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let px = global_id.x;
    let py = global_id.y;

    if (px >= info.width || py >= info.height) { return; }

    // Map pixel to Glyph
    let cell_size = info.cell_size;
    let col = px / cell_size;
    let row = py / cell_size;
    let glyph_idx = row * info.grid_cols + col;
    
    if (glyph_idx >= info.glyph_count) { return; }
    
    // Pixel coordinate within the cell (normalized -1 to 1)
    let cx = f32(px % cell_size) / f32(cell_size);
    let cy = f32(py % cell_size) / f32(cell_size);
    
    // In font units (using actual upem from font)
    let upem = f32(info.upem);
    let font_x = cx * upem; 
    let font_y = (1.0 - cy) * upem; 
    let p = vec2<f32>(font_x, font_y);

    // Winding Number Algorithm
    var winding = 0;
    
    let g_data = glyph_data[glyph_idx];
    let start = g_data.start_curve;
    let count = g_data.curve_count;
    
    for (var i = 0u; i < count; i = i + 1u) {
        let c = read_curve(start + i);
        
        // Ray casting for quadratic Bezier
        // We solve for t where y(t) = p.y
        // Then check if x(t) > p.x
        let a = c.p0.y - 2.0 * c.p1.y + c.p2.y;
        let b = 2.0 * (c.p1.y - c.p0.y);
        let c_val = c.p0.y - p.y;

        if (abs(a) < 0.0001) {
            let t = -c_val / b;
            if (t >= 0.0 && t <= 1.0) {
                let x = (1.0 - t) * (1.0 - t) * c.p0.x + 2.0 * t * (1.0 - t) * c.p1.x + t * t * c.p2.x;
                if (x > p.x) {
                    if (c.p0.y < c.p2.y) { winding++; } else { winding--; }
                }
            }
        } else {
            let det = b * b - 4.0 * a * c_val;
            if (det >= 0.0) {
                let sdet = sqrt(det);
                let t1 = (-b + sdet) / (2.0 * a);
                let t2 = (-b - sdet) / (2.0 * a);
                
                if (t1 >= 0.0 && t1 <= 1.0) {
                    let x1 = (1.0 - t1) * (1.0 - t1) * c.p0.x + 2.0 * t1 * (1.0 - t1) * c.p1.x + t1 * t1 * c.p2.x;
                    if (x1 > p.x) {
                        // Determine direction of crossing
                        // Derivative dy/dt = 2at + b
                        let dy1 = 2.0 * a * t1 + b;
                        if (dy1 > 0.0) { winding++; } else { winding--; }
                    }
                }
                if (t2 >= 0.0 && t2 <= 1.0) {
                    let x2 = (1.0 - t2) * (1.0 - t2) * c.p0.x + 2.0 * t2 * (1.0 - t2) * c.p1.x + t2 * t2 * c.p2.x;
                    if (x2 > p.x) {
                        let dy2 = 2.0 * a * t2 + b;
                        if (dy2 > 0.0) { winding++; } else { winding--; }
                    }
                }
            }
        }
    }
    
    let is_inside = winding != 0;
    
    if (is_inside) {
        textureStore(atlas_tex, vec2<i32>(global_id.xy), vec4<f32>(1.0, 1.0, 1.0, 1.0));
    } else {
        textureStore(atlas_tex, vec2<i32>(global_id.xy), vec4<f32>(0.0, 0.0, 0.0, 0.0));
    }
}
