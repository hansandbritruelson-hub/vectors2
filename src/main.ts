import init, { create_app_renderer, FlexRenderer } from 'renderer-core';
// import shadersAtlas from './shaders_atlas.wgsl?raw';

async function run() {
    await init();

    let renderer: FlexRenderer | undefined;
    let pendingFrame = false;

    const renderFrame = () => {
        pendingFrame = false;
        if (renderer) {
            renderer.render();
        }
    };

    (window as any).requestRenderFrame = () => {
        if (!pendingFrame) {
            pendingFrame = true;
            requestAnimationFrame(renderFrame);
        }
    };

    // Initialize the App (moved until after we have device/context)

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

    renderer = create_app_renderer(device, context);

    await renderer.init();

    // Initial render
    renderer.render();

    canvas.addEventListener('mousedown', async (e) => {
        if (!renderer) return;
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        console.log(`Click at (UI pixels): ${x}, ${y}`);
        await renderer.handle_click(x, y);
    });

    window.addEventListener('click', (e) => {
        if (renderer) {
            renderer.handle_click(e.clientX, e.clientY);
        }
    });

    window.addEventListener('mousemove', (e) => {
        if (renderer) {
            renderer.handle_mousemove(e.clientX, e.clientY);
        }
    });

    window.addEventListener('keydown', (e) => {
        if (!renderer) return;
        renderer.handle_keydown(e.key);
    });

    // Expose debug for manual inspection
    (window as any).debugRenderer = () => {
        if (renderer) renderer.debug();
    };

    console.log("Renderer initialized. Periodic updates enabled for testing.");
};

run();
