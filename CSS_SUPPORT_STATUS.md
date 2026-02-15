# CSS Support Status & Roadmap

This document outlines the current state of CSS property support, identified gaps, and the technical process for expanding our CSS capabilities.

## 1. Currently Supported Properties

Our current system supports ~50 properties, primarily focused on layout (Flexbox) and basic visual styles.

| Category | Properties | Status |
| :--- | :--- | :--- |
| **Sizing** | `width`, `height`, `min-width`, `max-width`, `min-height`, `max-height` | Functional (PX only) |
| **Flexbox** | `flex-direction`, `justify-content`, `align-items` | Functional |
| **Positioning** | `position`, `top`, `left`, `z-index` | Functional |
| **Box Model** | `padding`, `margin`, `border-width`, `border-color` | Functional (all 4 sides) |
| **Visuals** | `background-color`, `border-radius`, `box-shadow` | Functional (SDF-based) |
| **Typography** | `font-size`, `text-color`, `line-height`, `text-align`, `letter/word-spacing`, `font-weight`, `font-style` | Functional |
| **Special** | `fill-color`, `stroke-color`, `stroke-width` | Functional (for SVGs) |

> [!NOTE]
> Most sizing properties currently only support `px` units in the shader, even if the compiler parses others.

---

## 2. Identified Gaps (Critical Missing Properties)

To achieve "modern" CSS parity, we need to implement the following high-priority features:

### **A. Units & Sizing**
- **Percent (%) Support**: Crucial for responsive layouts. Requires resolving `%` against parent dimensions in the `width_top_down` and `height_top_down` passes.
- **Aspect Ratio**: `aspect-ratio` support would simplify media-heavy layouts.

### **B. Advanced Flexbox**
- **Flex Grow/Shrink/Basis**: The core of Flexbox. Currently, items share space proportionally to their "natural" size, but explicit `grow`/`shrink` factors are not yet implemented.
- **Flex Wrap**: Multi-line flex containers are not supported. This requires a more complex layout algorithm that tracks "lines".
- **Align Self**: Overriding parent alignment on a per-child basis.

### **C. Visual Effects & Transforms**
- **Opacity**: `opacity: 0.5` is a common requirement. It needs to be passed to the fragment shader and applied to the final color.
- **Transforms**: `translate`, `rotate`, `scale`. These are best handled in the vertex shader for performance.
- **Background Images**: Support for `background-image`, `background-size` (cover/contain), and `background-repeat`.
- **Gradients**: `linear-gradient` and `radial-gradient` support.

### **D. Interactivity & Clipping**
- **Overflow**: `overflow: hidden` is essential for containing content. It requires a "clip rect" check in the fragment shader.
- **Display: none**: While we have a `Visible` flag, a true `display: none` should remove the element from layout entirely.

---

## 3. The "Implementation Tax"
To add support for a new CSS property, the following 5-step process is required:

1.  **Define (style_defs.toml)**: Register a new `prop_id` and define its binary format (e.g., `[f32][u32]` for value+unit).
2.  **Store (lib.rs & WGSL)**: Add fields to the `GpuNode` struct in Rust and the `Node` struct in both Compute and Visual shaders.
3.  **Resolve (shaders_compute.wgsl)**: Add a case to the `apply_style_stream` function to parse the property from the style buffer into the `Node` struct.
4.  **Execute (Layout/Visual Shaders)**:
    -   *If Layout*: Update `width_bottom_up`, `width_top_down`, etc., to use the new property.
    -   *If Visual*: Update `vs_main` or `fs_main` in `shaders_visual.wgsl`.
5.  **Compile (ui-compiler)**: Update `src/css.rs` to parse the property and potentially expand shorthands.

---

## 4. Proposed Priority Roadmap

### **Phase 1: Responsive Core (Low Effort, High Impact)**
- [ ] Implement `%` unit resolution in `width_top_down`.
- [x] Implement `min-height`, `max-width`, `max-height`.
- [ ] Implement `opacity`.

### **Phase 2: Flexbox Completion**
- [ ] Add `flex-grow`, `flex-shrink`, `flex-basis`.
- [ ] Support `align-self`.

### **Phase 3: Visual Polish**
- [ ] Implement `transform: translate/rotate/scale`.
- [ ] Add `overflow: hidden` support.
- [ ] Implementation of `background-image`.
