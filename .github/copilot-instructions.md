# Workspace Instructions - Flexbox GPU Renderer

- [x] Projects scaffolded and compiled.
- [x] Rust/Wasm core integrated with Vite.
- [x] GPU Layout kernels updated for text-based intrinsic sizing.
- [x] Documentation (README.md, ARCHITECTURE.md) complete.

## Summary
The renderer now uses a static layout with 4 divs. Each div calculates its intrinsic size on the GPU based on its text length ($width = length \times 10$; $height = lines \times 20$).

## Next Steps
- Add font rendering.
- Implement more Flexbox constraints.
