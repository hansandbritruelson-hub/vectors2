# Flexbox GPU Renderer Architecture

## 1. System Overview
This project is a high-performance, Flexbox-inspired UI rendering engine.
**The Goal:** A layout engine where the entire constraint solving process—from "intrinsic size" calculation to final pixel positioning—runs on the GPU.
**The Stack:** 
- **Rust/Host:** Manages the UI tree structure (adding/removing "divs").
- **WebGPU:** Executes the layout logic in massive parallel passes.

---

## 2. The Rendering Pipeline (The "Big Picture")
The layout algorithm follows the standard web layout dependency: **Width determines Height** (due to text wrapping). Therefore, we cannot solve X and Y simultaneously. We must split the frame into two distinct phases, resulting in a **4-Pass Flow**.

### Phase A: The Width Axis
*Goal: Determine exactly how wide every element is.*

**1. Pass 1: "Intrinsic Width" (Bottom-Up)**
- **Logic:** We start at the leaves.
- **Simulation:** Leaves calculate their "Desired Width" based on content (e.g., a random number simulating a text string's length or an image size).
- **Aggregation:** Parents sum the desired widths of their children (or take the max, depending on simulated flex-direction) to determine their own desired width.
- **Result:** The Root knows how much space the entire tree *wants*.

**2. Pass 2: "Resolve Width" (Top-Down)**
- **Logic:** We start at the Root.
- **Constraint:** The Root is constrained to the Window Width.
- **Distribution:** Parents take their *given* width and divide it among children based on the children's "Desired Widths" calculated in Pass 1.
- **Result:** Every node now has a rigid `final_width`. 

### Phase B: The Height Axis
*Goal: Determine how tall elements are and where they sit (X, Y).*

**3. Pass 3: "Intrinsic Height" (Bottom-Up)**
- **Logic:** We start at the leaves again.
- **Crucial Dependency:** Nodes now use their `final_width` (from Pass 2) to calculate height.
- **Simulation:** A text node checks: "If I have 500px width, I wrap to 2 lines (height=40). If I have 100px width, I wrap to 10 lines (height=200)."
- **Aggregation:** Parents sum child heights to find their own desired height.

**4. Pass 4: "Final Layout" (Top-Down)**
- **Logic:** Start at Root with Window Height.
- **Distribution:** Parents allocate height to children.
- **Positioning:** This pass also calculates absolute `final_x` and `final_y` by summing offsets from the parent.
- **Result:** A fully solved tree ready for rendering.

---

## 3. GPU Execution Model
To make this fast, we map the logical passes above to specific advanced GPU patterns.

### For Bottom-Up (Passes 1 & 3) -> "Last Worker Continues"
Instead of dispatching one kernel per tree level (which requires CPU synchronization), we launch **one single dispatch** for all nodes.

**The Algorithm (WGSL):**
1.  **Thread Execution:** A thread computes its `Node[i]`.
2.  **Completion Signal:** When done, it performs an increment on its parent:
    ```wgsl
    let status = atomicAdd(&nodes[parent_id].signals_finished, 1);
    ```
3.  **The Check:**
    - If `status < (parent.child_count - 1)`: Thread is NOT the last child. It exits.
    - If `status == (parent.child_count - 1)`: Thread **is the last child**. It **becomes the parent** (swaps ID to `parent_id`) and computes the parent's logic.
4.  **Recursion:** This ripple continues until the Root is reached.

### For Top-Down (Passes 2 & 4) -> "Cascading Indirect Dispatches"
Since parents determine the work for children, we can't easily wait (fan-out). We use **Indirect Dispatching**.

**The Algorithm:**
1.  **CPU Setup:** Pre-record 32 `dispatchWorkgroupsIndirect(Buffer_Level_N)` commands.
2.  **Level 0 (Root):**
    - Computes layout.
    - Writes children IDs to `Queue_Level_1`.
    - Writes `[ceil(child_count/64), 1, 1]` to `Indirect_Args_Level_1`.
3.  **Level 1 (Children):**
    - GPU executes the dispatch automatically.
    - Threads read from `Queue_Level_1`.
    - Compute layout, then populate `Queue_Level_2` + `Indirect_Args_Level_2`.
4.  **Result:** The GPU expands the tree level-by-level without CPU intervention.

---

## 4. Data Model (Rust -> GPU)

### The Memory Layout
Rust maintains a flat `Vec<Node>` which maps byte-for-byte to a WebGPU storage buffer.

**Rust `Node` (repr(C)):**
*Must match WGSL byte-for-byte. 16-byte alignment is critical.*
```rust
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Node {
    // --- Layout Inputs (Style) ---
    style_min_width: f32,
    style_basis: f32,

    // --- Computed Values (Phase A - Width) ---
    desired_width: f32, // Result of Pass 1
    final_width: f32,   // Result of Pass 2

    // --- Computed Values (Phase B - Height/Pos) ---
    desired_height: f32, // Result of Pass 3
    final_height: f32,   // Result of Pass 4
    final_x: f32,
    final_y: f32,

    // --- Tree Topology ---
    parent_index: u32,
    child_start_index: u32,
    child_count: u32,
    
    // --- Synchronization ---
    signals_finished: u32, // Atomic counter for Bottom-Up
}
```

**WGSL `Node`:**
```wgsl
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
};
```

---

## 5. Implementation Roadmap
1. **Structures:** Define `Node` struct in Rust and WGSL.
2. **Buffer Management:** Create the mechanism to sync `Vec<Node>` to `GPUBuffer`.
3. **Pass 1 Shader:** Implement Bottom-Up width accumulation.
4. **Pass 2 Shader:** Implement Top-Down width distribution.
5. **Pass 3/4 Shaders:** Implement Height and Positioning logic.
