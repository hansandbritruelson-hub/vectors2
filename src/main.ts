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
    canvas.style.display = 'block';
    canvas.style.position = 'fixed';
    canvas.style.inset = '0';

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
    renderer.sync_viewport();

    // Initial render
    renderer.render();

    const syncViewportAndRender = () => {
        if (renderer) {
            renderer.sync_viewport();
            renderer.handle_mousemove(-1, -1);
            renderer.render();
        }
    };

    window.addEventListener('resize', syncViewportAndRender);
    window.visualViewport?.addEventListener('resize', syncViewportAndRender);

    canvas.addEventListener('pointerdown', async (e) => {
        if (!renderer) return;
        if (e.button !== 0) return;
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        canvas.setPointerCapture(e.pointerId);
        await renderer.handle_mousedown(x, y);
    });

    canvas.addEventListener('pointermove', (e) => {
        if (renderer) {
            const rect = canvas.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;
            renderer.handle_mousemove(x, y);
        }
    });

    canvas.addEventListener('pointerup', async (e) => {
        if (!renderer) return;
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        await renderer.handle_mouseup(x, y);
        if (canvas.hasPointerCapture(e.pointerId)) {
            canvas.releasePointerCapture(e.pointerId);
        }
    });

    canvas.addEventListener('pointercancel', async (e) => {
        if (!renderer) return;
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        await renderer.handle_mouseup(x, y);
        if (canvas.hasPointerCapture(e.pointerId)) {
            canvas.releasePointerCapture(e.pointerId);
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
