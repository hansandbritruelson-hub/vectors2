import { FlexEngine } from 'renderer-core';
import shaderCompute from './shaders_compute.wgsl?raw';
import shaderVisual from './shaders_visual.wgsl?raw';

export class FlexRenderer {
    device: GPUDevice;
    context: GPUCanvasContext;
    engine: FlexEngine;

    nodesBuffer: GPUBuffer | null = null;
    charactersBuffer: GPUBuffer | null = null;
    pipelineBottomUp: GPUComputePipeline | null = null;
    pipelineTopDown: GPUComputePipeline | null = null;
    pipelineHeightBottomUp: GPUComputePipeline | null = null;
    pipelineFinalLayout: GPUComputePipeline | null = null;
    pipelineRender: GPURenderPipeline | null = null;
    pipelineRenderText: GPURenderPipeline | null = null;
    
    bindGroupCompute: GPUBindGroup | null = null;
    bindGroupRender: GPUBindGroup | null = null;
    uniformBuffer: GPUBuffer | null = null;
    
    fontTexture: GPUTexture | null = null;
    fontSampler: GPUSampler | null = null;
    fontAtlasWidth = 512;
    fontAtlasHeight = 512;
    fontCharWidth = 32;
    fontCharHeight = 64;

    constructor(device: GPUDevice, context: GPUCanvasContext) {
        this.device = device;
        this.context = context;
        this.engine = new FlexEngine();
    }

    createFontAtlas() {
        // Create an offscreen canvas to draw characters
        const canvas = document.createElement('canvas');
        canvas.width = this.fontAtlasWidth;
        canvas.height = this.fontAtlasHeight;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        // Background
        ctx.fillStyle = 'rgba(0,0,0,0)'; // Transparent
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Font Config
        ctx.font = '48px monospace';
        ctx.fillStyle = 'white';
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';

        const cols = this.fontAtlasWidth / this.fontCharWidth;
        const rows = this.fontAtlasHeight / this.fontCharHeight;

        // ASCII 32 to 126
        for (let i = 32; i < 127; i++) {
            const index = i - 32;
            const col = index % cols;
            const row = Math.floor(index / cols);
            const x = col * this.fontCharWidth;
            const y = row * this.fontCharHeight;
            
            // Draw char centered in the cell
            const char = String.fromCharCode(i);
            ctx.fillText(char, x + this.fontCharWidth/2, y + this.fontCharHeight/2);
            
            // Debug border (optional, comment out for cleaner look)
            // ctx.strokeStyle = 'red';
            // ctx.strokeRect(x, y, this.fontCharWidth, this.fontCharHeight);
        }

        // Create GPU Texture
        this.fontTexture = this.device.createTexture({
            size: [this.fontAtlasWidth, this.fontAtlasHeight, 1],
            format: 'rgba8unorm',
            usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.RENDER_ATTACHMENT
        });

        this.device.queue.copyExternalImageToTexture(
            { source: canvas },
            { texture: this.fontTexture },
            [this.fontAtlasWidth, this.fontAtlasHeight]
        );

        this.fontSampler = this.device.createSampler({
            magFilter: 'linear',
            minFilter: 'linear',
        });
        
        console.log("Font Atlas Created and Uploaded");
    }

    async init() {
        console.log("Renderer Initialized");
        
        // 1. Setup Scene with 4 nodes containing sentences
        // Using a wider basis (800.0) so text doesn't wrap immediately at 5 chars (5 * 10px = 50px < 100px)
        const root = this.engine.add_node(800.0); // Root (0)
        
        const sentences = [
            "This is the first sentence that will be rendered in a div.",
            "Here is another sentence, slightly longer than the first one to test wrapping.",
            "Short sentence.",
            "Building a GPU-accelerated flexbox renderer is quite an interesting challenge for WebGPU and Rust."
        ];

        for (let i = 0; i < sentences.length; i++) {
            const childIdx = this.engine.add_node(0.0);
            this.engine.set_text(childIdx, sentences[i]);
            this.engine.set_parent(childIdx, root);
        }
        
        this.engine.set_child_start(root, 1);
        
        // Generate Atlas
        this.createFontAtlas();

        // 2. Buffers
        const nodeCount = this.engine.get_node_count();
        const nodeSize = this.engine.get_node_size();
        
        this.nodesBuffer = this.device.createBuffer({
            size: nodeCount * nodeSize,
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
        });

        const charCount = this.engine.get_character_count();
        const charSize = this.engine.get_character_size();
        
        this.charactersBuffer = this.device.createBuffer({
            size: Math.max(charCount * charSize, 4), 
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
        });
        
        // Initial upload of character data
        if (charCount > 0) {
            const charData = this.engine.get_characters_buffer();
            this.device.queue.writeBuffer(this.charactersBuffer, 0, charData.buffer, charData.byteOffset, charData.byteLength);
        }
        
        this.uniformBuffer = this.device.createBuffer({
            size: 16, 
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
        });

        // 3. Pipelines
        const moduleCompute = this.device.createShaderModule({ code: shaderCompute });
        const moduleVisual = this.device.createShaderModule({ code: shaderVisual });
        
        const bindGroupLayoutCompute = this.device.createBindGroupLayout({
            entries: [
                {
                    binding: 0,
                    visibility: GPUShaderStage.COMPUTE,
                    buffer: { type: 'storage' }
                },
                {
                    binding: 1,
                    visibility: GPUShaderStage.COMPUTE,
                    buffer: { type: 'uniform' }
                },
                {
                    binding: 2,
                    visibility: GPUShaderStage.COMPUTE,
                    buffer: { type: 'storage' }
                }
            ]
        });

        const bindGroupLayoutRender = this.device.createBindGroupLayout({
            entries: [
                {
                    binding: 0,
                    visibility: GPUShaderStage.VERTEX,
                    buffer: { type: 'read-only-storage' }
                },
                {
                    binding: 1,
                    visibility: GPUShaderStage.VERTEX,
                    buffer: { type: 'uniform' }
                },
                {
                    binding: 2,
                    visibility: GPUShaderStage.VERTEX,
                    buffer: { type: 'read-only-storage' }
                },
                {
                    binding: 3,
                    visibility: GPUShaderStage.FRAGMENT,
                    texture: {}
                },
                {
                    binding: 4,
                    visibility: GPUShaderStage.FRAGMENT,
                    sampler: {}
                }
            ]
        });

        const pipelineLayoutCompute = this.device.createPipelineLayout({
            bindGroupLayouts: [bindGroupLayoutCompute]
        });

        const pipelineLayoutRender = this.device.createPipelineLayout({
            bindGroupLayouts: [bindGroupLayoutRender]
        });

        this.pipelineBottomUp = this.device.createComputePipeline({
            layout: pipelineLayoutCompute,
            compute: { module: moduleCompute, entryPoint: 'width_bottom_up' }
        });

        this.pipelineTopDown = this.device.createComputePipeline({
            layout: pipelineLayoutCompute,
            compute: { module: moduleCompute, entryPoint: 'width_top_down' }
        });

        this.pipelineHeightBottomUp = this.device.createComputePipeline({
            layout: pipelineLayoutCompute,
            compute: { module: moduleCompute, entryPoint: 'height_bottom_up' }
        });

        this.pipelineFinalLayout = this.device.createComputePipeline({
            layout: pipelineLayoutCompute,
            compute: { module: moduleCompute, entryPoint: 'final_layout' }
        });

        this.pipelineRender = this.device.createRenderPipeline({
            layout: pipelineLayoutRender,
            vertex: {
                module: moduleVisual,
                entryPoint: 'vs_main',
            },
            fragment: {
                module: moduleVisual,
                entryPoint: 'fs_main',
                targets: [{ format: navigator.gpu.getPreferredCanvasFormat() }],
            },
            primitive: {
                topology: 'triangle-list',
            },
        });

        this.pipelineRenderText = this.device.createRenderPipeline({
            layout: pipelineLayoutRender,
            vertex: {
                module: moduleVisual,
                entryPoint: 'vs_text',
            },
            fragment: {
                module: moduleVisual,
                entryPoint: 'fs_text',
                targets: [{ format: navigator.gpu.getPreferredCanvasFormat() }],
            },
            primitive: {
                topology: 'triangle-list',
            },
        });

        // 4. Bind Groups
        this.bindGroupCompute = this.device.createBindGroup({
            layout: bindGroupLayoutCompute,
            entries: [
                { binding: 0, resource: { buffer: this.nodesBuffer } },
                { binding: 1, resource: { buffer: this.uniformBuffer } },
                { binding: 2, resource: { buffer: this.charactersBuffer } }
            ]
        });

        this.bindGroupRender = this.device.createBindGroup({
            layout: bindGroupLayoutRender,
            entries: [
                { binding: 0, resource: { buffer: this.nodesBuffer } },
                { binding: 1, resource: { buffer: this.uniformBuffer } },
                { binding: 2, resource: { buffer: this.charactersBuffer } },
                { binding: 3, resource: this.fontTexture!.createView() },
                { binding: 4, resource: this.fontSampler! }
            ]
        });
    }

    render() {
        if (!this.nodesBuffer || !this.uniformBuffer || !this.bindGroupCompute || !this.bindGroupRender) return;

        // 1. Update Uniforms
        const canvas = this.context.canvas as HTMLCanvasElement;
        const uniformData = new Float32Array([canvas.width, canvas.height, 0, 0]);
        this.device.queue.writeBuffer(this.uniformBuffer, 0, uniformData);

        // 2. Update Nodes
        const nodesData = this.engine.get_nodes_buffer();
        this.device.queue.writeBuffer(this.nodesBuffer, 0, nodesData.buffer, nodesData.byteOffset, nodesData.byteLength);

        const commandEncoder = this.device.createCommandEncoder();
        
        // 3. Compute Passes (Split into separate passes for implicit synchronization)
        const nodeCount = this.engine.get_node_count();
        const workgroups = Math.ceil(nodeCount / 64);

        // Pass 1: Width Bottom-Up
        const pass1 = commandEncoder.beginComputePass();
        pass1.setBindGroup(0, this.bindGroupCompute!);
        pass1.setPipeline(this.pipelineBottomUp!);
        pass1.dispatchWorkgroups(workgroups);
        pass1.end();

        // Pass 2: Width Top-Down
        const pass2 = commandEncoder.beginComputePass();
        pass2.setBindGroup(0, this.bindGroupCompute!);
        pass2.setPipeline(this.pipelineTopDown!);
        pass2.dispatchWorkgroups(workgroups);
        pass2.end();

        // Pass 3: Height Bottom-Up
        const pass3 = commandEncoder.beginComputePass();
        pass3.setBindGroup(0, this.bindGroupCompute!);
        pass3.setPipeline(this.pipelineHeightBottomUp!);
        pass3.dispatchWorkgroups(workgroups);
        pass3.end();

        // Pass 4: Final Layout
        const pass4 = commandEncoder.beginComputePass();
        pass4.setBindGroup(0, this.bindGroupCompute!);
        pass4.setPipeline(this.pipelineFinalLayout!);
        pass4.dispatchWorkgroups(workgroups);
        pass4.end();

        // 4. Render Pass
        const renderPass = commandEncoder.beginRenderPass({
            colorAttachments: [{
                view: this.context.getCurrentTexture().createView(),
                clearValue: { r: 0.1, g: 0.1, b: 0.1, a: 1.0 },
                loadOp: 'clear',
                storeOp: 'store',
            }]
        });
        renderPass.setPipeline(this.pipelineRender!);
        renderPass.setBindGroup(0, this.bindGroupRender);
        renderPass.draw(6, this.engine.get_node_count());
        
        // Draw Text
        // We draw 6 vertices per character (quad)
        // Instance count is character count
        const charCount = this.engine.get_character_count();
        if (charCount > 0) {
            renderPass.setPipeline(this.pipelineRenderText!);
            renderPass.setBindGroup(0, this.bindGroupRender); // Same bind group works
            renderPass.draw(6, charCount);
        }

        renderPass.end();

        this.device.queue.submit([commandEncoder.finish()]);
    }

    async debug() {
        if (!this.nodesBuffer || !this.charactersBuffer) return;
        
        const readBufferNodes = this.device.createBuffer({
            size: this.nodesBuffer.size,
            usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        });

        const readBufferChars = this.device.createBuffer({
            size: this.charactersBuffer.size,
            usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        });

        const commandEncoder = this.device.createCommandEncoder();
        commandEncoder.copyBufferToBuffer(this.nodesBuffer, 0, readBufferNodes, 0, readBufferNodes.size);
        commandEncoder.copyBufferToBuffer(this.charactersBuffer, 0, readBufferChars, 0, readBufferChars.size);
        this.device.queue.submit([commandEncoder.finish()]);

        await Promise.all([
            readBufferNodes.mapAsync(GPUMapMode.READ),
            readBufferChars.mapAsync(GPUMapMode.READ)
        ]);

        const nodesAB = readBufferNodes.getMappedRange();
        const nodesData = new Float32Array(nodesAB);
        const nodesU32 = new Uint32Array(nodesAB);
        
        const charsAB = readBufferChars.getMappedRange();
        const charsData = new Float32Array(charsAB);
        const charsU32 = new Uint32Array(charsAB);

        console.log("--- GPU Debug Output ---");
        const nodeCount = this.engine.get_node_count();
        const nodeFloats = 16; // 64 bytes / 4
        
        for (let i = 0; i < nodeCount; i++) {
            const base = i * nodeFloats;
            const finalW = nodesData[base + 3];
            const finalH = nodesData[base + 5];
            const x = nodesData[base + 6];
            const y = nodesData[base + 7];
            
            const textStart = nodesU32[base + 12];
            const textLen = nodesU32[base + 13];

            console.log(`Node ${i}: Pos(${x.toFixed(1)}, ${y.toFixed(1)}) Size(${finalW.toFixed(1)} x ${finalH.toFixed(1)}) text[${textStart}, ${textLen}]`);
            
            if (textLen > 0) {
                let s = "";
                for (let j = 0; j < textLen; j++) {
                    const charIdx = textStart + j;
                    const charBase = charIdx * 8; // 32 bytes / 4
                    const val = charsU32[charBase + 0];
                    s += String.fromCharCode(val);
                }
                console.log(`  Text: "${s}"`);
                
                const firstBase = textStart * 8;
                const lastBase = (textStart + textLen - 1) * 8;
                console.log(`  First Char: (${charsData[firstBase+4].toFixed(1)}, ${charsData[firstBase+5].toFixed(1)})`);
                console.log(`  Last Char:  (${charsData[lastBase+4].toFixed(1)}, ${charsData[lastBase+5].toFixed(1)})`);
            }
        }

        readBufferNodes.unmap();
        readBufferChars.unmap();
    }
}
