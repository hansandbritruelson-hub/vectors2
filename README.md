# Flexbox GPU Renderer

A high-performance Flexbox-inspired UI rendering engine that runs its entire layout constraint solving process on the GPU using WebGPU and Rust/WASM.

## Features
- **GPU-Accelerated Layout:** All 4 passes (Intrinsic Width, Resolve Width, Intrinsic Height, Final Layout) are executed in parallel on the GPU.
- **Rust/WASM Core:** Tree management and node synchronization logic in Rust.
- **Dynamic Text Wrapping:** Computes height based on resolved width and text content length.

## Getting Started

### Prerequisites
- Node.js
- Rust & `wasm-pack`
- A browser with WebGPU support (Chrome/Edge Canary)

### Setup
1. Install dependencies:
   ```bash
   npm install
   ```
2. Build WASM and start dev server:
   ```bash
   npm run dev
   ```

## Architecture
See [ARCHITECTURE.md](ARCHITECTURE.md) for a detailed breakdown of the 4-pass GPU layout pipeline.
