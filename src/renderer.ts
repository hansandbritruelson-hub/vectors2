import { FlexEngine } from 'renderer-core';
import shaderCompute from './shaders_compute.wgsl?raw';
import shaderVisual from './shaders_visual.wgsl?raw';

export class FlexRenderer {
    device: GPUDevice;
    context: GPUCanvasContext;
    engine: FlexEngine;

    nodesBuffer: GPUBuffer | null = null;
    pipelineBottomUp: GPUComputePipeline | null = null;
    pipelineTopDown: GPUComputePipeline | null = null;
    pipelineHeightBottomUp: GPUComputePipeline | null = null;
    pipelineFinalLayout: GPUComputePipeline | null = null;
    pipelineRender: GPURenderPipeline | null = null;
    
    bindGroupCompute: GPUBindGroup | null = null;
    bindGroupRender: GPUBindGroup | null = null;
    uniformBuffer: GPUBuffer | null = null;

    constructor(device: GPUDevice, context: GPUCanvasContext) {
        this.device = device;
        this.context = context;
        this.engine = new FlexEngine();
    }

    async init() {
        console.log("Renderer Initialized");
        
        // 1. Setup Scene with 4 nodes containing sentences
        const root = this.engine.add_node(100.0); // Root (0)
        
        const sentences = [
            "This is the first sentence that will be rendered in a div.",
            "Here is another sentence, slightly longer than the first one to test wrapping.",
            "Short sentence.",
            "Building a GPU-accelerated flexbox renderer is quite an interesting challenge for WebGPU and Rust."
        ];

        for (let i = 0; i < sentences.length; i++) {
            const childIdx = this.engine.add_node(0.0);
            this.engine.set_text_length(childIdx, sentences[i].length);
            this.engine.set_parent(childIdx, root);
        }
        
        this.engine.set_child_start(root, 1);

        // 2. Buffers
        const nodeCount = this.engine.get_node_count();
        const nodeSize = this.engine.get_node_size();
        
        this.nodesBuffer = this.device.createBuffer({
            size: nodeCount * nodeSize,
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
        });
        
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

        // 4. Bind Groups
        this.bindGroupCompute = this.device.createBindGroup({
            layout: bindGroupLayoutCompute,
            entries: [
                { binding: 0, resource: { buffer: this.nodesBuffer } },
                { binding: 1, resource: { buffer: this.uniformBuffer } }
            ]
        });

        this.bindGroupRender = this.device.createBindGroup({
            layout: bindGroupLayoutRender,
            entries: [
                { binding: 0, resource: { buffer: this.nodesBuffer } },
                { binding: 1, resource: { buffer: this.uniformBuffer } }
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
        renderPass.end();

        this.device.queue.submit([commandEncoder.finish()]);
    }

    async debug() {
        if (!this.nodesBuffer) return;
        
        const readBuffer = this.device.createBuffer({
            size: this.nodesBuffer.size,
            usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        });

        const commandEncoder = this.device.createCommandEncoder();
        commandEncoder.copyBufferToBuffer(this.nodesBuffer, 0, readBuffer, 0, readBuffer.size);
        this.device.queue.submit([commandEncoder.finish()]);

        await readBuffer.mapAsync(GPUMapMode.READ);
        const arrayBuffer = readBuffer.getMappedRange();
        console.log("GPU Node Data:", new Float32Array(arrayBuffer));
        readBuffer.unmap();
    }
}
