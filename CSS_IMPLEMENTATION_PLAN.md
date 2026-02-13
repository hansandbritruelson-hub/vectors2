# CSS Properties Implementation Plan
## GPU-Optimized Flexbox Renderer

**Current Implementation Status:**
- Properties Supported: `width`, `height`, `background-color`, `flex-direction`, `position`, `top`, `left`, `z-index`
- Architecture: GPU-based layout with 4-pass compute shader pipeline + style resolution pass
- Storage: Binary serialization in `class_defs` buffer, accessed via WGSL compute shaders

---

## Implementation Strategy Overview

### Core Principles
1. **GPU-First**: All properties computed in compute shaders during layout or style resolution
2. **Binary Encoding**: Properties serialized to `class_defs` buffer as `[prop_id, ...data]`
3. **Render-Time Access**: Visual properties read in fragment shader from Node struct
4. **Layout Integration**: Dimensional properties affect 4-pass layout algorithm
5. **Minimal Branching**: Use numeric flags/modes to minimize shader divergence

### Memory Management
- **Node Struct Limit**: Currently ~176 bytes. Target max 256 bytes (cache line considerations)
- **Overflow Strategy**: Group related properties into separate buffers when Node exceeds limit
- **Precision**: Use f32 for most values; pack flags/enums as u32

---

## CSS Properties by Category

### **1. BOX MODEL**

#### **1.1 margin (top, right, bottom, left, inline-start, inline-end, block-start, block-end)**
**Implementation:**
- Add to Node: `margin_top/right/bottom/left: f32` (16 bytes)
- Serialize: `[prop_id][value:f32][unit:u32]`
- Layout Impact: Modify `width_bottom_up` to subtract margins from natural width; add margins in `final_layout` between siblings
- GPU Shader: In relative positioning, offset `final_x/y` by margin; in flex parent, add margin to child spacing calculations
- Units: Support px, %, em (resolve % against parent width/height)

#### **1.2 padding (top, right, bottom, left)**
**Implementation:**
- Add to Node: `padding_top/right/bottom/left: f32` (16 bytes)
- Serialize: `[prop_id][value:f32][unit:u32]`
- Layout Impact: Content box shrinks by padding amount. Modify `final_width` calculation: `content_width = final_width - padding_left - padding_right`
- GPU Shader: In `layout_text()`, reduce available width by padding. In rendering, offset quad vertices inward
- Text Layout: Text wraps within padded content box

#### **1.3 border-width (top, right, bottom, left)**
**Implementation:**
- Add to Node: `border_width_top/right/bottom/left: f32` (16 bytes)
- Serialize: `[prop_id][value:f32][unit:u32]`
- Layout Impact: Similar to padding—reduce content box size
- Rendering: Draw border as separate geometry OR use SDF in fragment shader with thickness parameter
- Interaction: Borders consume space in layout (box-sizing: border-box behavior)

#### **1.4 border-color (top, right, bottom, left)**
**Implementation:**
- Add to Node: `border_color_[side]_rgba: vec4<f32>` (64 bytes) OR use single packed u32 per side (16 bytes)
- Serialize: `[prop_id][r:f32][g:f32][b:f32][a:f32]` per side
- Rendering: Pass to fragment shader; blend with background in SDF border rendering
- Optimization: If all sides identical, store once with flag

#### **1.5 border-radius (top-left, top-right, bottom-right, bottom-left)**
**Implementation:**
- Add to Node: `border_radius_tl/tr/br/bl: f32` (16 bytes)
- Serialize: `[prop_id][tl:f32][tr:f32][br:f32][bl:f32]`
- Rendering: Fragment shader SDF for rounded corners. Calculate distance to rounded rectangle boundary; discard/alpha blend
- Algorithm: Per-corner ellipse SDF in pixel coordinates. Requires derivatives for AA
- Performance: Negligible cost—single SDF evaluation per fragment

#### **1.6 border-style (solid, dashed, dotted, double, groove, ridge, inset, outset)**
**Implementation:**
- Add to Node: `border_style_[side]: u32` enum (16 bytes for 4 sides, or 4 bytes if uniform)
- Serialize: `[prop_id][style:u32]` (0=none, 1=solid, 2=dashed, etc.)
- Rendering: Fragment shader pattern generation. For dashed/dotted: modulo operation on perimeter distance
- Complex Styles: groove/ridge/inset/outset require lighting calculations (fake bevel effect via gradient)

---

### **2. LAYOUT & POSITIONING**

#### **2.1 display (block, inline, inline-block, flex, inline-flex, grid, none)**
**Implementation:**
- Add to Node: `display_mode: u32` (4 bytes, already have implicit flex)
- Serialize: `[prop_id][mode:u32]`
- Layout Impact: 
  - `none`: Set flags bit, skip in layout passes
  - `inline`: Not compatible with current block-based layout—defer or error
  - `grid`: Requires separate 2D constraint solver—major feature
  - `flex`: Current default behavior
- GPU Shader: Early exit in all passes if `display_mode == NONE`

#### **2.2 position: fixed**
**Implementation:**
- Extend `position_mode: u32` with value 2 for fixed
- Serialize: Reuse PROP_POSITION_MODE
- Layout Impact: Fixed nodes ignore scroll offset (future feature). Position relative to viewport, not parent
- GPU Shader: In `final_layout`, if `position_mode == 2`, use `top/left` directly relative to screen origin

#### **2.3 position: sticky**
**Implementation:**
- Add `position_mode` value 3
- Requires scroll position tracking (not yet implemented)
- GPU Shader: Compare scroll offset to threshold; switch between relative and fixed behavior
- Deferred: Requires scroll system

#### **2.4 right, bottom (positioning)**
**Implementation:**
- Add to Node: `right_offset, bottom_offset: f32` (8 bytes)
- Serialize: `[prop_id][value:f32][unit:u32]`
- Layout Impact: In absolute positioning, calculate from parent's right/bottom edge
- GPU Shader: `final_x = parent.final_x + parent.final_width - right_offset - node.final_width`
- Conflict Resolution: If both `left` and `right` specified, `left` wins (or use to determine width)

#### **2.5 inset (shorthand for top/right/bottom/left)**
**Implementation:**
- CSS Parser: Expand to individual properties during serialization
- No GPU change—purely host-side convenience

---

### **3. FLEXBOX (Extended)**

#### **3.1 flex-wrap (nowrap, wrap, wrap-reverse)**
**Implementation:**
- Add to Node: `flex_wrap: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Layout Impact: In `width_bottom_up`, if `wrap && children_width > parent_width`, distribute over multiple lines
- GPU Shader: Complex—requires line-breaking algorithm. Count lines, compute per-line heights. May need auxiliary line buffer
- Lines Buffer: `lines: array<LineInfo>` with `{start_child, end_child, width, height}`

#### **3.2 justify-content (flex-start, center, flex-end, space-between, space-around, space-evenly)**
**Implementation:**
- Add to Node: `justify_content: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Layout Impact: Affects spacing calculation in main axis during `final_layout`
- GPU Shader: Calculate total child width, remaining space. Distribute via mode:
  - `center`: offset = (remaining_space / 2)
  - `space-between`: gap = remaining_space / (child_count - 1)
  - `space-around`: gap = remaining_space / child_count, half-gap at edges
- No iterative solving needed—pure distribution math

#### **3.3 align-items (flex-start, center, flex-end, stretch, baseline)**
**Implementation:**
- Add to Node: `align_items: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Layout Impact: Affects cross-axis positioning in `final_layout`
- GPU Shader: In row layout, adjust child `final_y` based on mode:
  - `center`: `final_y = parent_y + (parent_height - child_height) / 2`
  - `stretch`: Set child height to parent height (requires modifying height pass)
- Baseline: Complex—requires font metrics for text nodes; defer

#### **3.4 align-content (for multi-line flex)**
**Implementation:**
- Add to Node: `align_content: u32` (4 bytes)
- Requires `flex-wrap`. Aligns entire lines in cross axis
- GPU Shader: After computing all line heights, distribute vertical space between lines
- Dependencies: Needs line buffer from `flex-wrap`

#### **3.5 flex-grow, flex-shrink, flex-basis**
**Implementation:**
- Add to Node: `flex_grow, flex_shrink, flex_basis: f32` (12 bytes)
- Serialize: `[prop_id][value:f32]`
- Layout Impact: Core flex algorithm. Modify `width_top_down`:
  - Collect all children's `flex_basis` (or natural width if auto)
  - Compute available space: `parent_width - sum(basis)`
  - If positive: distribute via `flex_grow` ratios
  - If negative: shrink via `flex_shrink` ratios
- GPU Shader: Two-pass within `width_top_down`: first pass computes total flex factors, second distributes space

#### **3.6 align-self (override parent's align-items)**
**Implementation:**
- Add to Node: `align_self: u32` (4 bytes, 255 = inherit)
- Serialize: `[prop_id][mode:u32]`
- Layout Impact: Per-child override in cross-axis positioning
- GPU Shader: Check `align_self` before falling back to parent's `align_items`

#### **3.7 order**
**Implementation:**
- Add to Node: `order: i32` (4 bytes)
- Serialize: `[prop_id][order:i32]`
- Layout Impact: Reorder children before layout. Requires CPU-side sort OR GPU radix sort
- GPU Shader: Pre-layout pass to sort child indices by order value. Use auxiliary buffer for sorted indices
- Complexity: GPU radix sort for up to ~1024 children per container

---

### **4. SIZING CONSTRAINTS**

#### **4.1 min-width, max-width, min-height, max-height**
**Implementation:**
- Add to Node: `max_width, min_height, max_height: f32` (12 bytes, `min_width` exists)
- Serialize: `[prop_id][value:f32][unit:u32]`
- Layout Impact: Clamp final dimensions after flex distribution
- GPU Shader: After computing `final_width` in `width_top_down`, apply `clamp(final_width, min_width, max_width)`
- Iterative Constraint: If child clamped, parent may need reflow—requires multi-iteration or accept approximation

#### **4.2 aspect-ratio**
**Implementation:**
- Add to Node: `aspect_ratio: f32` (4 bytes, -1.0 = auto)
- Serialize: `[prop_id][ratio:f32]`
- Layout Impact: If width known, set `height = width / aspect_ratio` in height pass
- GPU Shader: In `height_bottom_up`, override `desired_height` if aspect ratio set
- Interaction: Conflicts with explicit height—aspect ratio wins unless height is constrained by min/max

---

### **5. TEXT & TYPOGRAPHY**

#### **5.1 color (text color)**
**Implementation:**
- Add to Node: `text_color_rgba: vec4<f32>` (16 bytes)
- Serialize: `[prop_id][r:f32][g:f32][b:f32][a:f32]`
- Rendering: Pass to text fragment shader; multiply glyph alpha channel by text color
- Current: Uses `color_r/g/b/a` (background). Rename to `bg_color_*`, add `text_color_*`

#### **5.2 font-size**
**Implementation:**
- Add to Node: `font_size: f32` (4 bytes)
- Serialize: `[prop_id][value:f32][unit:u32]`
- Layout Impact: Affects glyph metrics. Requires dynamic glyph atlas OR pre-generate multiple sizes
- GPU Shader: Scale glyph dimensions by `font_size / base_font_size` in `layout_text()`
- Atlas Strategy: Use SDF glyphs (single-channel distance field) for arbitrary scaling

#### **5.3 font-family**
**Implementation:**
- Add to Node: `font_id: u32` (4 bytes, index into font registry)
- Serialize: `[prop_id][font_id:u32]`
- Layout Impact: Each font has separate glyph buffer and metrics
- GPU Shader: Index into `glyph_buffers[font_id]` for metrics
- Multi-Font: Requires multiple `glyph_data` buffers OR interleaved with font_id prefix

#### **5.4 font-weight (100-900, bold)**
**Implementation:**
- Add to Node: `font_weight: u32` (4 bytes)
- Serialize: `[prop_id][weight:u32]`
- Rendering: Load different font variant OR use variable font (requires axis parameter in glyph rendering)
- SDF Approach: Adjust SDF threshold to fake weight changes (limited quality)

#### **5.5 font-style (normal, italic, oblique)**
**Implementation:**
- Add to Node: `font_style: u32` (4 bytes)
- Serialize: `[prop_id][style:u32]`
- Rendering: Load italic font variant. Oblique: apply skew matrix in vertex shader
- GPU Shader: Vertex shader transformation for oblique: `x += y * tan(slant_angle)`

#### **5.6 text-align (left, center, right, justify)**
**Implementation:**
- Add to Node: `text_align: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Layout Impact: Affects horizontal positioning of text within content box
- GPU Shader: In `layout_text()`, after computing line width:
  - `center`: `offset_x = (content_width - line_width) / 2`
  - `right`: `offset_x = content_width - line_width`
  - `justify`: Adjust inter-word spacing (requires word boundary detection)

#### **5.7 line-height**
**Implementation:**
- Currently global `uniforms.line_height`. Make per-node: `line_height: f32` (4 bytes)
- Serialize: `[prop_id][value:f32][unit:u32]`
- Layout Impact: Direct effect on text height calculation
- GPU Shader: Use `node.line_height` in `layout_text()` instead of uniform

#### **5.8 letter-spacing, word-spacing**
**Implementation:**
- Add to Node: `letter_spacing, word_spacing: f32` (8 bytes)
- Serialize: `[prop_id][value:f32][unit:u32]`
- Layout Impact: Modify advance in `layout_text()`: `cursor_x += advance + letter_spacing`
- Word Spacing: Detect space character (U+0020), add `word_spacing`

#### **5.9 text-transform (uppercase, lowercase, capitalize)**
**Implementation:**
- Add to Node: `text_transform: u32` (4 bytes)
- Layout Impact: Transform on CPU before uploading to `characters` buffer
- GPU Shader: Not needed—CPU pre-processes text content
- Alternative: GPU transform—lookup uppercase/lowercase mapping in texture

#### **5.10 text-decoration (underline, overline, line-through)**
**Implementation:**
- Add to Node: `text_decoration: u32` flags (4 bytes)
- Serialize: `[prop_id][flags:u32]` (bit 0=underline, 1=overline, 2=line-through)
- Rendering: Fragment shader draws horizontal line at appropriate baseline offset
- GPU Shader: In text fragment, detect if pixel y-coord intersects decoration line; blend color

#### **5.11 white-space (normal, nowrap, pre, pre-wrap, pre-line)**
**Implementation:**
- Add to Node: `white_space: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Layout Impact: 
  - `nowrap`: Disable word wrapping in `layout_text()`
  - `pre`: Preserve newlines/spaces; no wrapping
  - `pre-wrap`: Preserve newlines, enable wrapping
- GPU Shader: Conditional in `layout_text()` word-wrap check

#### **5.12 text-overflow (clip, ellipsis)**
**Implementation:**
- Add to Node: `text_overflow: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Rendering: If `ellipsis`, detect truncation in `layout_text()`, append "…" character
- GPU Shader: Track if text exceeds bounds; replace last visible chars with ellipsis glyph

#### **5.13 vertical-align (for inline content)**
**Implementation:**
- Add to Node: `vertical_align: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]` (baseline, top, middle, bottom, sub, super)
- Layout Impact: Inline layout not yet implemented—defer
- GPU Shader: Would adjust y-offset of inline boxes relative to baseline

---

### **6. VISUAL EFFECTS**

#### **6.1 opacity**
**Implementation:**
- Add to Node: `opacity: f32` (4 bytes)
- Serialize: `[prop_id][value:f32]`
- Rendering: Multiply final fragment alpha by opacity
- GPU Shader: In fragment shader: `return vec4(color.rgb, color.a * node.opacity)`
- Blending: Requires proper alpha blending enabled (already necessary for text)

#### **6.2 box-shadow**
**Implementation:**
- Add to Node: `shadow_offset_x/y, shadow_blur, shadow_spread: f32, shadow_color: vec4<f32>` (32 bytes)
- Serialize: `[prop_id][offset_x][offset_y][blur][spread][r][g][b][a]`
- Rendering: Fragment shader SDF for shadow. Render shadow pass before main geometry OR compute in single pass
- GPU Shader: Distance to box edge; apply Gaussian blur approximation (erf function). Blend shadow color
- Multiple Shadows: Array of shadow structs; iterate in fragment shader

#### **6.3 text-shadow**
**Implementation:**
- Similar to box-shadow but for glyph shapes
- Add to Node: Same shadow params or reuse box-shadow (4 bytes flag to apply to text)
- Rendering: In text fragment shader, sample glyph at offset positions, blend shadow color
- Performance: Multiple texture samples per fragment—expensive. Limit to 1-2 shadows

#### **6.4 filter (blur, brightness, contrast, grayscale, etc.)**
**Implementation:**
- Requires post-processing pass. Render node to texture, apply filter, composite
- Add to Node: `filter_flags: u32, filter_params: vec4<f32>` (20 bytes)
- Serialize: `[prop_id][filter_type][params...]`
- GPU Shader: After main render pass, dispatch compute shader for filter effects
- Blur: Gaussian blur via separable convolution (horizontal + vertical pass)
- Color Filters: Matrix multiplication in fragment shader

#### **6.5 backdrop-filter**
**Implementation:**
- Requires reading framebuffer before rendering node
- Add to Node: Same as `filter`
- Rendering: Before drawing node, copy framebuffer region to texture, apply filter, draw with blend
- GPU Shader: Use `textureSample()` on background texture, apply filter matrix
- Performance: Expensive—requires render target copy per node

#### **6.6 mix-blend-mode (multiply, screen, overlay, etc.)**
**Implementation:**
- Add to Node: `blend_mode: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Rendering: Fragment shader implements blend equations
- GPU Shader: Read destination color (requires framebuffer fetch), apply blend formula
- WebGPU Support: Check for framebuffer fetch extension; fallback to separate pass

---

### **7. TRANSFORMS**

#### **7.1 transform: translate(x, y)**
**Implementation:**
- Add to Node: `transform_translate_x/y: f32` (8 bytes)
- Serialize: `[prop_id][tx][ty][unit_x][unit_y]`
- Rendering: Apply in vertex shader: `position.xy += vec2(tx, ty)`
- GPU Shader: Offset final screen position, NOT layout position (transform doesn't affect layout)

#### **7.2 transform: rotate(angle)**
**Implementation:**
- Add to Node: `transform_rotate: f32` (4 bytes, radians)
- Serialize: `[prop_id][angle:f32]`
- Rendering: Vertex shader rotation matrix around center point
- GPU Shader: `rot_mat = [[cos, -sin], [sin, cos]]`; apply to `(vertex - center)` then add center back

#### **7.3 transform: scale(x, y)**
**Implementation:**
- Add to Node: `transform_scale_x/y: f32` (8 bytes, 1.0 = no scale)
- Serialize: `[prop_id][sx][sy]`
- Rendering: Vertex shader: `position = center + (position - center) * scale`
- GPU Shader: Multiply vertex offset by scale factors

#### **7.4 transform-origin**
**Implementation:**
- Add to Node: `transform_origin_x/y: f32` (8 bytes)
- Serialize: `[prop_id][x][y][unit_x][unit_y]`
- Rendering: Change center point for rotate/scale operations
- GPU Shader: Use `origin` instead of geometric center in transform calculations

#### **7.5 transform: matrix(a, b, c, d, e, f)**
**Implementation:**
- Add to Node: `transform_matrix: mat3x2<f32>` (24 bytes for 2D affine)
- Serialize: `[prop_id][a][b][c][d][e][f]`
- Rendering: Single matrix multiply in vertex shader
- GPU Shader: `new_pos = transform_matrix * vec3(pos, 1.0)`
- Advantage: Combines all transforms; computed on CPU, uploaded once

#### **7.6 perspective (3D)**
**Implementation:**
- Add to Node: `perspective: f32` (4 bytes, distance)
- Serialize: `[prop_id][distance:f32]`
- Rendering: Modify projection matrix OR vertex shader Z-divide
- GPU Shader: `clip_z = -1.0 / (z / perspective)`; apply to vertex position
- Requires: 3D transform support (transform: translate3d, rotateX/Y/Z)

---

### **8. BACKGROUNDS & IMAGES**

#### **8.1 background-image**
**Implementation:**
- Add to Node: `bg_image_id: u32` (4 bytes, texture index)
- Serialize: `[prop_id][image_id:u32]`
- Rendering: Fragment shader samples background texture
- GPU Shader: `textureSample(bg_textures[image_id], sampler, uv)`
- Multi-Texture: Bindless textures OR texture array (WebGPU limits: 16 textures per bind group)

#### **8.2 background-size (cover, contain, specific size)**
**Implementation:**
- Add to Node: `bg_size_mode: u32, bg_size_x/y: f32` (12 bytes)
- Serialize: `[prop_id][mode][x][y]`
- Rendering: Fragment shader calculates UV scaling
- GPU Shader: 
  - `cover`: scale = max(node_width/img_width, node_height/img_height)
  - `contain`: scale = min(...)
  - Adjust UV: `uv = (uv - 0.5) / scale + 0.5`

#### **8.3 background-position**
**Implementation:**
- Add to Node: `bg_position_x/y: f32` (8 bytes)
- Serialize: `[prop_id][x][y][unit_x][unit_y]`
- Rendering: Offset UV in fragment shader
- GPU Shader: `uv -= vec2(bg_position_x / img_width, bg_position_y / img_height)`

#### **8.4 background-repeat (repeat, repeat-x, repeat-y, no-repeat)**
**Implementation:**
- Add to Node: `bg_repeat: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Rendering: Fragment shader UV wrapping
- GPU Shader: `uv = fract(uv)` for repeat; clamp for no-repeat

#### **8.5 background-attachment (scroll, fixed)**
**Implementation:**
- Add to Node: `bg_attachment: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Rendering: Fixed background uses viewport coordinates, not element coordinates
- GPU Shader: If fixed, use screen-space UV instead of element-local UV

#### **8.6 linear-gradient, radial-gradient**
**Implementation:**
- Add to Node: `gradient_type: u32, gradient_data: [u32; 16]` (68 bytes)
- Serialize: `[prop_id][type][stop_count][stops...]` (stop = position:f32 + color:rgba)
- Rendering: Fragment shader gradient calculation
- GPU Shader: 
  - Linear: `t = dot(uv - start, direction)`
  - Radial: `t = length(uv - center) / radius`
  - Lookup color from stops via linear interpolation
- Optimization: Pre-bake gradient to 1D texture (256 pixels), sample with `t`

---

### **9. OVERFLOW & CLIPPING**

#### **9.1 overflow (visible, hidden, scroll, auto)**
**Implementation:**
- Add to Node: `overflow_x/y: u32` (8 bytes)
- Serialize: `[prop_id][mode_x][mode_y]`
- Rendering: Enable scissor test for `hidden`. For `scroll`, requires clip planes
- GPU Shader: Fragment shader discards if pixel outside bounds: `if (pixel_pos > bounds) { discard; }`
- Scrolling: Requires scroll offset tracking (future feature)

#### **9.2 clip-path (circle, ellipse, polygon, inset)**
**Implementation:**
- Add to Node: `clip_path_type: u32, clip_data: [f32; 8]` (36 bytes)
- Serialize: `[prop_id][type][data...]`
- Rendering: Fragment shader SDF for clip shape
- GPU Shader: 
  - Circle: `if (length(pixel - center) > radius) { discard; }`
  - Polygon: Point-in-polygon test via winding number
- Performance: Single SDF evaluation per fragment; cheap

---

### **10. GRID LAYOUT**

#### **10.1 display: grid**
**Implementation:**
- Fundamentally different constraint solver. Requires separate implementation
- Add to Node: `grid_template_cols/rows: u32` (offsets into grid track definition buffer)
- Layout Impact: Two-phase: size tracks (rows/cols), then place items
- GPU Shader: 
  - Pass 1: Compute intrinsic track sizes (similar to flex natural width)
  - Pass 2: Distribute available space across fr units
  - Pass 3: Place items in grid cells
- Track Buffer: `grid_tracks: array<GridTrack>` with `{size, unit, min, max}`

#### **10.2 grid-template-columns, grid-template-rows**
**Implementation:**
- Serialize: `[prop_id][track_count][track_1_size][track_1_unit]...`
- Add to Node: `grid_col_tracks_offset, grid_row_tracks_offset: u32` (8 bytes)
- GPU Shader: Read track definitions from separate buffer, compute final sizes

#### **10.3 grid-column, grid-row (item placement)**
**Implementation:**
- Add to Node: `grid_col_start/end, grid_row_start/end: u32` (16 bytes)
- Serialize: `[prop_id][start][end]`
- GPU Shader: In grid layout pass, place item at specified cell coordinates

#### **10.4 grid-auto-flow (row, column, dense)**
**Implementation:**
- Add to Node: `grid_auto_flow: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- GPU Shader: Auto-placement algorithm. Track occupied cells in bitfield; find next free slot

#### **10.5 gap (row-gap, column-gap)**
**Implementation:**
- Add to Node: `gap_row, gap_col: f32` (8 bytes)
- Serialize: `[prop_id][row_gap][col_gap][unit]`
- GPU Shader: Add gap spacing between tracks when calculating positions

---

### **11. ANIMATION & TRANSITION**

#### **11.1 transition-property, transition-duration, transition-timing-function**
**Implementation:**
- Requires time-based interpolation. Add to Node: `transition_flags: u32, transition_duration: f32` (8 bytes)
- Serialize: `[prop_id][property_flags][duration][timing_func]`
- CPU-Side: Track old/new values, compute interpolated values based on elapsed time
- GPU Shader: Receive already-interpolated values; no GPU animation (too complex)
- Alternative: GPU Animation—store old/new in separate buffers, interpolate in compute shader with time uniform

#### **11.2 animation (keyframes)**
**Implementation:**
- Keyframe Buffer: `keyframes: array<Keyframe>` with `{time, property_id, value}`
- Add to Node: `animation_id, animation_time: u32, f32` (8 bytes)
- CPU-Side: Evaluate keyframes on CPU, upload interpolated values
- GPU-Side: Compute shader reads keyframe buffer, interpolates based on `animation_time`, writes to Node properties
- Performance: GPU animation avoids CPU bottleneck for thousands of animated nodes

---

### **12. INTERACTION & CURSOR**

#### **12.1 cursor (pointer, text, move, etc.)**
**Implementation:**
- CPU-Only: Detect hover via mouse position, query Node bounds, set CSS cursor
- Add to Node: `cursor_type: u32` (4 bytes) for potential future use (custom cursor rendering)
- GPU Shader: Not needed—cursor is OS-level

#### **12.2 pointer-events (none, auto)**
**Implementation:**
- Add to Node: `pointer_events: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- CPU-Side: Skip hit-testing if `pointer_events == none`
- GPU Shader: Not needed for layout/rendering; affects CPU event handling only

---

### **13. VISIBILITY & DISPLAY**

#### **13.1 visibility (visible, hidden, collapse)**
**Implementation:**
- Add to Node: `visibility: u32` (4 bytes, currently using `flags` bit 0)
- Serialize: `[prop_id][mode:u32]`
- Layout Impact: 
  - `hidden`: Render invisible but occupy layout space
  - `collapse`: Remove from layout (like `display: none`)
- GPU Shader: Skip rendering in vertex shader if `visibility != visible`, but don't skip layout passes

---

### **14. TABLE LAYOUT (Deferred)**

Properties like `display: table`, `table-layout`, `border-collapse`, `border-spacing` require a dedicated table layout algorithm. Implementation would mirror grid complexity. **Recommendation: Phase 2 feature.**

---

### **15. MULTI-COLUMN LAYOUT (Deferred)**

Properties like `column-count`, `column-width`, `column-gap` require fragmenting content across columns. **Recommendation: Phase 2 feature.**

---

### **16. WRITING MODES & DIRECTIONALITY**

#### **16.1 direction (ltr, rtl)**
**Implementation:**
- Add to Node: `direction: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Layout Impact: Reverse child order in flex row layout. Text rendering reverses glyph order
- GPU Shader: In `final_layout`, iterate children in reverse if RTL. In `layout_text()`, place glyphs right-to-left

#### **16.2 writing-mode (horizontal-tb, vertical-rl, vertical-lr)**
**Implementation:**
- Add to Node: `writing_mode: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Layout Impact: Swap width/height axes. Vertical writing requires rotating glyph coordinates
- GPU Shader: Conditional logic in layout passes to swap main/cross axis

---

### **17. CONTENT & GENERATED CONTENT**

#### **17.1 content (for ::before, ::after)**
**Implementation:**
- Requires pseudo-element support. Each pseudo is a synthetic Node
- Add to Node: `pseudo_type: u32` (4 bytes, 0=none, 1=before, 2=after)
- Serialize: `[prop_id][content_type][content_data...]` (string index or counter value)
- GPU Shader: Render pseudo-nodes as children; layout treats them as regular nodes

#### **17.2 counter-increment, counter-reset**
**Implementation:**
- CPU-Side: Track counters during tree traversal, generate content values
- GPU Shader: Not applicable—counters are logical, not visual

---

### **18. LISTS**

#### **18.1 list-style-type (disc, circle, square, decimal, etc.)**
**Implementation:**
- Add to Node: `list_style_type: u32` (4 bytes)
- Serialize: `[prop_id][type:u32]`
- Rendering: Generate marker as pseudo-element OR render in fragment shader
- GPU Shader: For shape markers (disc/circle), SDF rendering. For text markers (decimal), generate text node

#### **18.2 list-style-position (inside, outside)**
**Implementation:**
- Add to Node: `list_style_position: u32` (4 bytes)
- Layout Impact: `outside` marker doesn't affect content box width; `inside` does
- GPU Shader: Adjust content offset and marker placement

---

### **19. MISC PROPERTIES**

#### **19.1 box-sizing (content-box, border-box)**
**Implementation:**
- Add to Node: `box_sizing: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Layout Impact: Changes interpretation of width/height
  - `content-box`: width = content only
  - `border-box`: width = content + padding + border
- GPU Shader: Adjust calculations in all layout passes based on box-sizing mode

#### **19.2 resize (none, both, horizontal, vertical)**
**Implementation:**
- Interactive feature. Requires drag handles and size constraints
- Add to Node: `resize_mode: u32` (4 bytes)
- CPU-Side: Render resize handles, update width/height on drag
- GPU Shader: Not needed—resize is input-driven

#### **19.3 object-fit (fill, contain, cover, none, scale-down)**
**Implementation:**
- Add to Node: `object_fit: u32` (4 bytes)
- Serialize: `[prop_id][mode:u32]`
- Rendering: Fragment shader adjusts image UV scaling (similar to background-size)
- GPU Shader: Calculate scale factor, apply to UV

#### **19.4 object-position**
**Implementation:**
- Add to Node: `object_position_x/y: f32` (8 bytes)
- Serialize: `[prop_id][x][y][unit_x][unit_y]`
- Rendering: Offset image within frame (similar to background-position)

---

## Implementation Phasing

### **Phase 1: Core Layout Enhancements** (50 properties)
- All box model (margin, padding, border-width/color/radius/style)
- Sizing constraints (min/max width/height, aspect-ratio)
- Extended positioning (right, bottom, fixed)
- Core flexbox (justify-content, align-items, flex-grow/shrink/basis, flex-wrap)
- Text alignment and spacing
- Opacity
- Box-sizing

**Rationale:** Foundational for any real UI. No major architecture changes—extends existing system.

### **Phase 2: Visual Effects** (30 properties)
- Text properties (font-size, font-family, font-weight, line-height, text-decoration)
- Shadows (box-shadow, text-shadow)
- Transforms (translate, rotate, scale, transform-origin)
- Backgrounds (background-image, -size, -position, -repeat)
- Gradients
- Overflow and clipping

**Rationale:** Enhances visual fidelity. Requires fragment shader complexity but no layout changes.

### **Phase 3: Advanced Layout** (40 properties)
- Grid layout (all grid-* properties)
- Multi-line flex (align-content)
- Writing modes (direction, writing-mode)
- Visibility modes

**Rationale:** Major features requiring new layout algorithms. Grid is largest undertaking.

### **Phase 4: Animations & Interactions** (20 properties)
- Transitions and animations
- Filters (blur, brightness, etc.)
- Blend modes
- Pseudo-elements and generated content

**Rationale:** Requires time-based updates and complex rendering. Lower priority for static layouts.

### **Phase 5: Specialized Features** (20 properties)
- Table layout
- Multi-column layout
- Lists
- Counters

**Rationale:** Niche use cases. Can defer until core features mature.

---

## Technical Challenges & Solutions

### **Challenge 1: Node Struct Size Limit**
**Problem:** 160+ properties would require ~500 bytes, exceeding cache efficiency.

**Solution:** 
- **Property Grouping:** Separate buffers for rarely-used properties (e.g., `extended_styles: array<ExtendedStyle>` indexed by `node.extended_style_idx`)
- **Default Elision:** Only store non-default values. Use bitfield to indicate which properties are set.
- **Compression:** Pack flags/enums into bit fields. E.g., 4 border styles = 8 bits.

### **Challenge 2: Shader Compilation Time**
**Problem:** Massive switch statement in `resolve_styles` with 160+ cases.

**Solution:**
- **Jump Table Emulation:** Use binary search tree structure in shader.
- **Specialization:** Generate shader variants for common property subsets.
- **Inline Expansion:** Unroll critical properties; handle rare ones in fallback.

### **Challenge 3: Grid Layout Complexity**
**Problem:** 2D constraint solving with auto-sizing and spanning items.

**Solution:**
- **Track-Based Approach:** Size tracks independently, then place items (simpler than full constraint solver).
- **Iterative Refinement:** Multiple passes to resolve min/max constraints (max 3 iterations).
- **GPU Radix Sort:** For dense packing, sort items by size/position.

### **Challenge 4: Animation Interpolation**
**Problem:** Smooth interpolation of hundreds of properties across thousands of nodes.

**Solution:**
- **Keyframe Buffer:** Store all keyframes in GPU buffer. Compute shader evaluates per-node.
- **Property Masks:** Only interpolate changed properties (tracked via bitfield).
- **Time Uniform:** Single global time; all animations synchronized.

### **Challenge 5: Text Rendering at Arbitrary Sizes**
**Problem:** Current glyph atlas is fixed resolution.

**Solution:**
- **SDF Glyphs:** Single-channel signed distance field per glyph. Scales without pixelation.
- **Multi-Resolution Atlas:** Pre-generate 3 sizes (small/medium/large), select based on font-size.
- **Runtime Rasterization:** CPU rasterizes new glyphs on demand (slow but flexible).

### **Challenge 6: Filter Effects (Blur)**
**Problem:** Gaussian blur requires large kernel; expensive in fragment shader.

**Solution:**
- **Separable Convolution:** Horizontal pass + vertical pass (O(n) instead of O(n²)).
- **Mipmap Approximation:** Downsample texture, sample lower mip for cheap blur.
- **Compute Shader:** Dispatch compute for blur; more efficient than fragment.

---

## Serialization Format Extension

### **Current Format:**
```
[prop_id: u32][value_data: variable]
```

### **Proposed Extensions:**

#### **Multi-Value Properties (e.g., margin)**
```
[PROP_MARGIN: u32][top: f32][right: f32][bottom: f32][left: f32][unit: u32]
```

#### **Flags/Enums (e.g., text-align)**
```
[PROP_TEXT_ALIGN: u32][mode: u32]  // 0=left, 1=center, 2=right, 3=justify
```

#### **Colors (rgba)**
```
[PROP_BORDER_COLOR: u32][r: f32][g: f32][b: f32][a: f32]
```

#### **Transform Matrices**
```
[PROP_TRANSFORM: u32][a: f32][b: f32][c: f32][d: f32][e: f32][f: f32]
```

#### **Arrays (e.g., box-shadow with multiple shadows)**
```
[PROP_BOX_SHADOW: u32][count: u32]
  [offset_x: f32][offset_y: f32][blur: f32][spread: f32][r: f32][g: f32][b: f32][a: f32]
  ... (repeated count times)
```

---

## Performance Estimates

Based on GPU workload analysis:

| Feature Category | GPU Cost | Bottleneck | Notes |
|------------------|----------|------------|-------|
| Box Model (margin/padding/border) | +10% layout time | Memory bandwidth | 4 loads per node |
| Flexbox Extended | +20% layout time | Compute | flex-grow requires iteration |
| Grid Layout | +40% layout time | Compute | 2D constraint solving |
| Transforms | +5% render time | ALU | Matrix multiply per vertex |
| Shadows | +50% render time | Texture/ALU | Multiple SDF samples |
| Filters (blur) | +200% render time | Memory | Separable convolution |
| Gradients | +10% render time | ALU | Interpolation per fragment |
| Text (SDF scaling) | +15% render time | Texture | Distance field evaluation |

**Overall:** With all Phase 1-2 properties, expect ~2x slowdown from current baseline. Still orders of magnitude faster than CPU layout.

---

## Testing Strategy

### **Per-Property Test Cases:**
1. **Isolated Test:** Single div with property at various values
2. **Interaction Test:** Property combined with related properties (e.g., margin + padding)
3. **Stress Test:** 1000+ nodes with property set
4. **Animation Test:** Interpolate property over time

### **Validation:**
- **Reference Comparison:** Render same layout in browser, compare pixel output
- **Regression Suite:** Automated tests for each property
- **Performance Profiling:** GPU trace analysis for each new property

---

## Conclusion

This plan covers **160+ CSS properties** with concrete GPU-optimized implementation strategies. Each property is designed to:
- Minimize shader divergence
- Maximize memory coalescing
- Leverage parallel compute
- Scale to thousands of nodes

**Estimated Implementation Time (for cheaper AI):**
- Phase 1: 50-70 hours (1-2 hours per property × 50 properties)
- Phase 2: 40-50 hours
- Phase 3: 60-80 hours (grid is complex)
- Phase 4: 30-40 hours
- Phase 5: 30-40 hours

**Total:** ~250 hours for comprehensive CSS support.

**Critical Path Items:**
1. Expand Node struct with property fields (batch allocation for Phase 1)
2. Extend `style_defs.toml` and rebuild system
3. Update `resolve_styles` shader with new property handling
4. Modify layout passes for dimensional properties
5. Enhance fragment shader for visual properties
6. Implement SDF-based rendering for borders/shadows/shapes
7. Build grid layout solver
8. Add animation interpolation system
