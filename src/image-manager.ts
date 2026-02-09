import { render_svg } from 'renderer-core';

interface ImageState {
    svgContent: string;
    texture: GPUTexture;
    bindGroup: GPUBindGroup | null;
    width: number;
    height: number;
    dirty: boolean;
}

export class ImageManager {
    device: GPUDevice;
    images: Map<number, ImageState> = new Map();
    bindGroupLayout: GPUBindGroupLayout;

    constructor(device: GPUDevice) {
        this.device = device;

        // Create a bind group layout for images (Texture + Sampler)
        this.bindGroupLayout = device.createBindGroupLayout({
            entries: [
                {
                    binding: 0,
                    visibility: GPUShaderStage.FRAGMENT,
                    texture: { sampleType: 'float' } // defaults to 'float' which supports 'rgba8unorm'
                },
                {
                    binding: 1,
                    visibility: GPUShaderStage.FRAGMENT,
                    sampler: {}
                }
            ]
        });
    }

    addImage(nodeIndex: number, svgContent: string) {
        // Initialize with a 1x1 placeholder until first layout
        const texture = this.createTexture(1, 1, new Uint8Array([0, 0, 0, 0]));

        this.images.set(nodeIndex, {
            svgContent,
            texture,
            bindGroup: this.createBindGroup(texture),
            width: 1,
            height: 1,
            dirty: true
        });
    }

    // Check for layout changes and update textures
    update(nodesBufferParam: Float32Array) {
        // nodesBuffer is the Float32Array view of the nodes buffer
        // Node struct size is 16 floats (64 bytes)
        // final_width is at index +3
        // final_height is at index +5

        const NODE_SIZE_FLOATS = 16;
        // const sampler = ... (removed)

        for (const [nodeIndex, state] of this.images.entries()) {
            const baseIndex = nodeIndex * NODE_SIZE_FLOATS;
            if (baseIndex + 5 >= nodesBufferParam.length) continue;

            const newWidth = Math.max(1, Math.round(nodesBufferParam[baseIndex + 3]));
            const newHeight = Math.max(1, Math.round(nodesBufferParam[baseIndex + 5]));

            if (newWidth !== state.width || newHeight !== state.height || state.dirty) {
                // Layout changed, re-rasterize
                console.log(`[ImageManager] Rasterizing Node ${nodeIndex} at ${newWidth}x${newHeight}`);

                const pixelData = render_svg(state.svgContent, newWidth, newHeight);

                if (pixelData.length > 0) {
                    // Destroy old texture
                    state.texture.destroy();

                    // Create new texture
                    const newTexture = this.createTexture(newWidth, newHeight, pixelData);

                    state.texture = newTexture;
                    state.bindGroup = this.createBindGroup(newTexture);
                    state.width = newWidth;
                    state.height = newHeight;
                    state.dirty = false;
                }
            }
        }
    }

    private createTexture(width: number, height: number, data: Uint8Array): GPUTexture {
        const texture = this.device.createTexture({
            size: [width, height, 1],
            format: 'rgba8unorm',
            usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.RENDER_ATTACHMENT
        });

        this.device.queue.writeTexture(
            { texture },
            data as any,
            { bytesPerRow: width * 4 },
            { width, height }
        );

        return texture;
    }

    private createBindGroup(texture: GPUTexture): GPUBindGroup {
        const sampler = this.device.createSampler({
            magFilter: 'linear',
            minFilter: 'linear',
        });

        return this.device.createBindGroup({
            layout: this.bindGroupLayout,
            entries: [
                { binding: 0, resource: texture.createView() },
                { binding: 1, resource: sampler }
            ]
        });
    }

    getBindGroup(nodeIndex: number): GPUBindGroup | null {
        return this.images.get(nodeIndex)?.bindGroup || null;
    }

    getImageNodes(): number[] {
        return Array.from(this.images.keys());
    }
}
