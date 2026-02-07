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
    glyphBuffer: GPUBuffer | null = null;
    
    // Vector Graphics Buffers
    curveBuffer: GPUBuffer | null = null;
    glyphInfoBuffer: GPUBuffer | null = null;

    constructor(device: GPUDevice, context: GPUCanvasContext) {
        this.device = device;
        this.context = context;
        this.engine = new FlexEngine();
    }

    async init() {
        console.log("Renderer Initialized");
        
        // 1. Setup Scene
        // Hierarchy:
        // Root (Column)
        //  -> Row 1 (Row)
        //      -> Text 1
        //      -> Text 2
        //  -> Row 2 (Row)
        //      -> Text 3
        //      -> Text 4

        // Root Node (Index 0) - Column Direction
        const root = this.engine.add_node(800.0); 
        this.engine.set_flex_direction(root, 1); // 1 = Column

        // Row 1 (Index 1) - Row Direction
        const row1 = this.engine.add_node(0.0);
        this.engine.set_parent(row1, root);

        // Row 2 (Index 2) - Row Direction
        const row2 = this.engine.add_node(0.0);
        this.engine.set_parent(row2, root);

        // Set Root Children (Row 1, Row 2)
        this.engine.set_child_start(root, 1); // Children start at index 1

        // Content for Row 1 (Indices 3, 4)
        const t1 = this.engine.add_node(0.0);
        this.engine.set_text(t1, "Row 1 - Item A: This is a much longer sentence designed to test the wrapping capabilities of our GPU renderer. It should span multiple lines if everything is working correctly.");
        this.engine.set_parent(t1, row1);

        const t2 = this.engine.add_node(0.0);
        this.engine.set_text(t2, "Row 1 - Item B: This is also a significant amount of text to ensure that we have proper distribution of space between these two items in the first row.");
        this.engine.set_parent(t2, row1);
        
        this.engine.set_child_start(row1, 3); // Children start at index 3

        // Content for Row 2 (Indices 5, 6)
        const t3 = this.engine.add_node(0.0);
        this.engine.set_text(t3, "Row 2 - Item C: This third block of text is in the second row, which should appear below the first row. It also needs to be long enough to wrap.");
        this.engine.set_parent(t3, row2);

        const t4 = this.engine.add_node(0.0);
        this.engine.set_text(t4, "Row 2 - Item D: Finally, this is the last block of text. By making all of these sentences longer, we stress test the line breaking algorithms in the compute shader.");
        this.engine.set_parent(t4, row2);
        
        this.engine.set_child_start(row2, 5); // Children start at index 5
        
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

        const glyphCount = this.engine.get_glyph_data_count();
        const glyphSize = this.engine.get_glyph_data_size();
        
        this.glyphBuffer = this.device.createBuffer({
            size: Math.max(glyphCount * glyphSize, 4),
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
        });

        if (glyphCount > 0) {
            const glyphData = this.engine.get_glyph_data_buffer();
            this.device.queue.writeBuffer(this.glyphBuffer, 0, glyphData.buffer, glyphData.byteOffset, glyphData.byteLength);
        }
        
        this.uniformBuffer = this.device.createBuffer({
            size: 16, 
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
        });

        // Vectors
        const curveData = this.engine.get_curve_buffer();
        console.log(`[JS] Curve Data Size: ${curveData.byteLength} bytes`);
        this.curveBuffer = this.device.createBuffer({
            size: Math.max(curveData.byteLength, 4), // 8 floats = 32 bytes per curve
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
        });
        if (curveData.byteLength > 0) {
           this.device.queue.writeBuffer(this.curveBuffer, 0, curveData.buffer, curveData.byteOffset, curveData.byteLength);
        } else {
           console.warn("[JS] Curve buffer is empty!");
        }

        const glyphInfoData = this.engine.get_glyph_info_buffer();
        console.log(`[JS] Glyph Info Size: ${glyphInfoData.byteLength} bytes`);
        this.glyphInfoBuffer = this.device.createBuffer({
            size: Math.max(glyphInfoData.byteLength, 4), // 4 u32s = 16 bytes per glyph
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
        });
        if (glyphInfoData.byteLength > 0) {
            this.device.queue.writeBuffer(this.glyphInfoBuffer, 0, glyphInfoData.buffer, glyphInfoData.byteOffset, glyphInfoData.byteLength);
        } else {
            console.warn("[JS] Glyph Info buffer is empty!");
        }

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
                },
                {
                    binding: 3,
                    visibility: GPUShaderStage.COMPUTE,
                    buffer: { type: 'read-only-storage' }
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
                    buffer: { type: 'read-only-storage' } // characters
                },
                {
                    binding: 3,
                    visibility: GPUShaderStage.FRAGMENT,
                    buffer: { type: 'read-only-storage' } // curves
                },
                {
                    binding: 4,
                    visibility: GPUShaderStage.FRAGMENT,
                    buffer: { type: 'read-only-storage' } // glyph_infos
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
                { binding: 2, resource: { buffer: this.charactersBuffer } },
                { binding: 3, resource: { buffer: this.glyphBuffer } }
            ]
        });

        this.bindGroupRender = this.device.createBindGroup({
            layout: bindGroupLayoutRender,
            entries: [
                { binding: 0, resource: { buffer: this.nodesBuffer } },
                { binding: 1, resource: { buffer: this.uniformBuffer } },
                { binding: 2, resource: { buffer: this.charactersBuffer } },
                { binding: 3, resource: { buffer: this.curveBuffer! } },
                { binding: 4, resource: { buffer: this.glyphInfoBuffer! } }
            ]
        });
    }

    render() {
        if (!this.nodesBuffer || !this.uniformBuffer || !this.bindGroupCompute || !this.bindGroupRender) return;

        // 1. Update Uniforms
        const canvas = this.context.canvas as HTMLCanvasElement;
        // The shader expects "Screen CSS Pixels" if we want to map layout units (CSS px) to NDC directly.
        // But the viewport is set to the full physical texture size.
        // We pass the logical size so that 1 layout unit = 1 pixel on screen (roughly).
        // Actually, let's pass the physical size, and rely on the fact that our layouts 
        // need to be scaled by DPR for crisp rendering, OR we pass logical size.
        // If we pass Logical Size (e.g. 800) and Vertex is at 400. 400/800 = 0.5 (NDC 0.0 center).
        // Viewport is 0..1600. Center is 800.
        // So the object appears at pixel 800.
        // In CSS space, 400 is center. In Physical space, 800 is center.
        // So it matches!
        const dpr = window.devicePixelRatio || 1;
        const ascender = this.engine.get_ascender();
        const descender = this.engine.get_descender();
        const lineGap = this.engine.get_line_gap();
        const lineHeight = ascender - descender + lineGap;
        
        const uniformData = new Float32Array([
            canvas.width / dpr, 
            canvas.height / dpr, 
            ascender,
            lineHeight
        ]);
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
        if (!this.nodesBuffer || !this.charactersBuffer || !this.curveBuffer || !this.glyphInfoBuffer) return;
        
        const readBufferNodes = this.device.createBuffer({
            size: this.nodesBuffer.size,
            usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        });

        const readBufferChars = this.device.createBuffer({
            size: this.charactersBuffer.size,
            usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        });

        // Debug Vector Buffers
        const readBufferCurves = this.device.createBuffer({
            size: this.curveBuffer.size,
            usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        });
        
        const readBufferInfos = this.device.createBuffer({
            size: this.glyphInfoBuffer.size,
            usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        });

        const commandEncoder = this.device.createCommandEncoder();
        commandEncoder.copyBufferToBuffer(this.nodesBuffer, 0, readBufferNodes, 0, readBufferNodes.size);
        commandEncoder.copyBufferToBuffer(this.charactersBuffer, 0, readBufferChars, 0, readBufferChars.size);
        commandEncoder.copyBufferToBuffer(this.curveBuffer, 0, readBufferCurves, 0, readBufferCurves.size);
        commandEncoder.copyBufferToBuffer(this.glyphInfoBuffer, 0, readBufferInfos, 0, readBufferInfos.size);
        
        this.device.queue.submit([commandEncoder.finish()]);

        await Promise.all([
            readBufferNodes.mapAsync(GPUMapMode.READ),
            readBufferChars.mapAsync(GPUMapMode.READ),
            readBufferCurves.mapAsync(GPUMapMode.READ),
            readBufferInfos.mapAsync(GPUMapMode.READ)
        ]);

        const nodesAB = readBufferNodes.getMappedRange();
        const nodesData = new Float32Array(nodesAB);
        const nodesU32 = new Uint32Array(nodesAB);
        
        const charsAB = readBufferChars.getMappedRange();
        const charsData = new Float32Array(charsAB);
        const charsU32 = new Uint32Array(charsAB);
        
        const curvesAB = readBufferCurves.getMappedRange();
        const curvesData = new Float32Array(curvesAB);
        
        const infosAB = readBufferInfos.getMappedRange();
        const infosU32 = new Uint32Array(infosAB);

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
                
                // Inspect First Char
                const firstBase = textStart * 8;
                const glyphIndex = charsU32[firstBase + 1];
                console.log(`  First Char Info: GlyphID ${glyphIndex}, Width ${charsData[firstBase+6]}, BearingY ${charsData[firstBase+5]}`);
                
                // Lookup GlyphInfo
                const infoBase = glyphIndex * 4; // 4 u32s
                const startCurve = infosU32[infoBase + 0];
                const countCurve = infosU32[infoBase + 1];
                console.log(`  Glyph Info: Curves Start ${startCurve}, Count ${countCurve}`);
                
                if (countCurve > 0) {
                    // Dump first curve
                    const curveBase = startCurve * 8; // 8 floats
                    const p0x = curvesData[curveBase + 0];
                    const p0y = curvesData[curveBase + 1];
                    const p1x = curvesData[curveBase + 2];
                    const p1y = curvesData[curveBase + 3];
                    const p2x = curvesData[curveBase + 4];
                    const p2y = curvesData[curveBase + 5];
                    console.log(`    Curve 0: (${p0x.toFixed(2)},${p0y.toFixed(2)}) -> (${p1x.toFixed(2)},${p1y.toFixed(2)}) -> (${p2x.toFixed(2)},${p2y.toFixed(2)})`);
                }
            }
        }

        readBufferNodes.unmap();
        readBufferChars.unmap();
        readBufferCurves.unmap();
        readBufferInfos.unmap();
    }
}
