import { FlexRenderer } from 'renderer-core';
import initWasm from 'renderer-core';

const init = async () => {
    await initWasm();

    if (!navigator.gpu) {
        document.body.innerHTML = "WebGPU not supported.";
        return;
    }

    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) {
        document.body.innerHTML = "No GPU adapter found.";
        return;
    }

    const device = await adapter.requestDevice();

    const canvas = document.createElement('canvas');
    // Handle High-DPI (Retina) displays
    const dpr = window.devicePixelRatio || 1;
    canvas.width = window.innerWidth * dpr;
    canvas.height = window.innerHeight * dpr;
    // Keep the display size matching the window (CSS pixels)
    canvas.style.width = window.innerWidth + 'px';
    canvas.style.height = window.innerHeight + 'px';

    document.body.appendChild(canvas);

    const context = canvas.getContext('webgpu') as GPUCanvasContext;
    if (!context) {
        console.error("WebGPU context lost");
        return;
    }

    const presentationFormat = navigator.gpu.getPreferredCanvasFormat();
    context.configure({
        device,
        format: presentationFormat,
    });

    const renderer = new FlexRenderer(device, context);
    await renderer.init();

    // Push-based Render Loop
    let pendingFrame = false;
    const renderFrame = () => {
        pendingFrame = false;
        renderer.render();
    };

    (window as any).requestRenderFrame = () => {
        if (!pendingFrame) {
            pendingFrame = true;
            requestAnimationFrame(renderFrame);
        }
    };

    // Initial render
    renderer.render();

    // Expose debug for manual inspection
    (window as any).debugRenderer = () => {
        renderer.debug();
    };

    console.log("Renderer initialized. Periodic updates enabled for testing.");
};

init();
