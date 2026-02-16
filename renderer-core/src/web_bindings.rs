#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
use js_sys::{Object, Promise, Reflect};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(
    inline_js = "export function get_window() { return typeof window !== 'undefined' ? window : undefined; }"
)]
extern "C" {
    pub fn get_window() -> Option<Window>;
}

#[wasm_bindgen(inline_js = "
export async function download_image(url) {
    const response = await fetch(url);
    const blob = await response.blob();
    const arrayBuffer = await blob.arrayBuffer();
    return new Uint8Array(arrayBuffer);
}
")]
extern "C" {
    pub fn download_image(url: &str) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    // Window & Canvas
    pub type Window;

    #[wasm_bindgen(method, getter, js_name = devicePixelRatio)]
    pub fn device_pixel_ratio(this: &Window) -> f64;

    #[wasm_bindgen(method, js_name = setInterval)]
    pub fn set_interval(this: &Window, callback: &JsValue, timeout: i32) -> JsValue;

    // Call back to JS to request a frame
    #[wasm_bindgen(js_namespace = window, js_name = requestRenderFrame)]
    pub fn request_render_frame();

    pub type HtmlCanvasElement;
    #[wasm_bindgen(method, getter)]
    pub fn width(this: &HtmlCanvasElement) -> u32;
    #[wasm_bindgen(method, getter)]
    pub fn height(this: &HtmlCanvasElement) -> u32;

    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuDevice;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuQueue;

    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuBuffer;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuTexture;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuTextureView;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuExternalTexture;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuSampler;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuShaderModule;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuBindGroupLayout;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuBindGroup;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuPipelineLayout;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuComputePipeline;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuRenderPipeline;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuCommandEncoder;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuComputePassEncoder;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuRenderPassEncoder;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuCommandBuffer;
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    pub type GpuCanvasContext;

    // GpuDevice methods
    #[wasm_bindgen(method, getter)]
    pub fn queue(this: &GpuDevice) -> GpuQueue;

    #[wasm_bindgen(method, js_name = createBuffer)]
    pub fn create_buffer(this: &GpuDevice, descriptor: &Object) -> GpuBuffer;

    #[wasm_bindgen(method, js_name = createTexture)]
    pub fn create_texture(this: &GpuDevice, descriptor: &Object) -> GpuTexture;

    #[wasm_bindgen(method, js_name = createSampler)]
    pub fn create_sampler(this: &GpuDevice, descriptor: Option<Object>) -> GpuSampler;

    #[wasm_bindgen(method, js_name = createShaderModule)]
    pub fn create_shader_module(this: &GpuDevice, descriptor: &Object) -> GpuShaderModule;

    #[wasm_bindgen(method, js_name = createBindGroupLayout)]
    pub fn create_bind_group_layout(this: &GpuDevice, descriptor: &Object) -> GpuBindGroupLayout;

    #[wasm_bindgen(method, js_name = createBindGroup)]
    pub fn create_bind_group(this: &GpuDevice, descriptor: &Object) -> GpuBindGroup;

    #[wasm_bindgen(method, js_name = createPipelineLayout)]
    pub fn create_pipeline_layout(this: &GpuDevice, descriptor: &Object) -> GpuPipelineLayout;

    #[wasm_bindgen(method, js_name = createComputePipeline)]
    pub fn create_compute_pipeline(this: &GpuDevice, descriptor: &Object) -> GpuComputePipeline;

    #[wasm_bindgen(method, js_name = createRenderPipeline)]
    pub fn create_render_pipeline(this: &GpuDevice, descriptor: &Object) -> GpuRenderPipeline;

    #[wasm_bindgen(method, js_name = createCommandEncoder)]
    pub fn create_command_encoder(this: &GpuDevice) -> GpuCommandEncoder;

    // GpuQueue methods
    #[wasm_bindgen(method, js_name = writeBuffer)]
    pub fn write_buffer_with_f64_and_js_value(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &JsValue,
    );

    #[wasm_bindgen(method, js_name = writeBuffer)]
    pub fn write_buffer_with_u8_array(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &[u8],
    );

    #[wasm_bindgen(method, js_name = writeTexture)]
    pub fn write_texture_with_u8_array(
        this: &GpuQueue,
        destination: &Object,
        data: &[u8],
        data_layout: &Object,
        size: &Object,
    );

    #[wasm_bindgen(method)]
    pub fn submit(this: &GpuQueue, command_buffers: &js_sys::Array);

    // GpuBuffer methods
    #[wasm_bindgen(method, js_name = mapAsync)]
    pub fn map_async(this: &GpuBuffer, mode: u32) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = getMappedRange)]
    pub fn get_mapped_range(this: &GpuBuffer) -> js_sys::ArrayBuffer;

    #[wasm_bindgen(method)]
    pub fn unmap(this: &GpuBuffer);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &GpuBuffer);

    // GpuCommandEncoder methods
    #[wasm_bindgen(method, js_name = beginComputePass)]
    pub fn begin_compute_pass(this: &GpuCommandEncoder) -> GpuComputePassEncoder;

    #[wasm_bindgen(method, js_name = beginComputePass)]
    pub fn begin_compute_pass_with_descriptor(
        this: &GpuCommandEncoder,
        descriptor: &Object,
    ) -> GpuComputePassEncoder;

    #[wasm_bindgen(method, js_name = beginRenderPass)]
    pub fn begin_render_pass(this: &GpuCommandEncoder, descriptor: &Object)
        -> GpuRenderPassEncoder;

    #[wasm_bindgen(method, js_name = copyBufferToBuffer)]
    pub fn copy_buffer_to_buffer(
        this: &GpuCommandEncoder,
        src: &GpuBuffer,
        src_offset: f64,
        dst: &GpuBuffer,
        dst_offset: f64,
        size: f64,
    );

    #[wasm_bindgen(method)]
    pub fn finish(this: &GpuCommandEncoder) -> GpuCommandBuffer;

    // GpuComputePassEncoder methods
    #[wasm_bindgen(method, js_name = setPipeline)]
    pub fn set_pipeline_compute(this: &GpuComputePassEncoder, pipeline: &GpuComputePipeline);

    #[wasm_bindgen(method, js_name = setBindGroup)]
    pub fn set_bind_group_compute(
        this: &GpuComputePassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
    );

    #[wasm_bindgen(method)]
    pub fn dispatchWorkgroups(this: &GpuComputePassEncoder, x: u32, y: u32, z: u32);

    #[wasm_bindgen(method, js_name = end)]
    pub fn end_compute(this: &GpuComputePassEncoder);

    // GpuRenderPassEncoder methods
    #[wasm_bindgen(method, js_name = setPipeline)]
    pub fn set_pipeline_render(this: &GpuRenderPassEncoder, pipeline: &GpuRenderPipeline);

    #[wasm_bindgen(method, js_name = setBindGroup)]
    pub fn set_bind_group_render(
        this: &GpuRenderPassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
    );

    #[wasm_bindgen(method, js_name = draw)]
    pub fn draw_with_instance_count(
        this: &GpuRenderPassEncoder,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );

    #[wasm_bindgen(method, js_name = end)]
    pub fn end_render(this: &GpuRenderPassEncoder);

    // GpuCanvasContext methods
    #[wasm_bindgen(method, js_name = getCurrentTexture)]
    pub fn get_current_texture(this: &GpuCanvasContext) -> GpuTexture;

    #[wasm_bindgen(method, getter)]
    pub fn canvas(this: &GpuCanvasContext) -> HtmlCanvasElement;

    #[wasm_bindgen(method)]
    pub fn configure(this: &GpuCanvasContext, descriptor: &Object);

    // GpuTexture methods
    #[wasm_bindgen(method, js_name = createView)]
    pub fn create_view(this: &GpuTexture) -> GpuTextureView;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &GpuTexture);
}

// Helper types for descriptors
// Since WebGPU descriptors are dictionaries, we use js_sys::Object or wasm-bindgen descriptors.
// For simplicity and to match the current renderer.rs usage, we'll use helpers that return Object.

pub struct GpuBufferDescriptor;
impl GpuBufferDescriptor {
    pub fn new(size: f64, usage: u32) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"size".into(), &size.into()).unwrap();
        Reflect::set(&obj, &"usage".into(), &usage.into()).unwrap();
        obj
    }
}

pub struct GpuTextureDescriptor;
impl GpuTextureDescriptor {
    pub fn new(size: &js_sys::Array, format: &str, usage: u32) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"size".into(), size).unwrap();
        Reflect::set(&obj, &"format".into(), &format.into()).unwrap();
        Reflect::set(&obj, &"usage".into(), &usage.into()).unwrap();
        obj
    }
}

pub struct GpuShaderModuleDescriptor;
impl GpuShaderModuleDescriptor {
    pub fn new(code: &str) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"code".into(), &code.into()).unwrap();
        obj
    }
}

pub struct GpuBindGroupLayoutDescriptor;
impl GpuBindGroupLayoutDescriptor {
    pub fn new(entries: &js_sys::Array) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"entries".into(), entries).unwrap();
        obj
    }
}

pub struct GpuBindGroupLayoutEntry;
impl GpuBindGroupLayoutEntry {
    pub fn new(binding: u32, visibility: u32) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"binding".into(), &binding.into()).unwrap();
        Reflect::set(&obj, &"visibility".into(), &visibility.into()).unwrap();
        obj
    }

    pub fn new_texture(binding: u32, visibility: u32) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"binding".into(), &binding.into()).unwrap();
        Reflect::set(&obj, &"visibility".into(), &visibility.into()).unwrap();

        let texture = Object::new();
        // viewDimension defaults to "2d", sampleType defaults to "float"
        Reflect::set(&obj, &"texture".into(), &texture).unwrap();
        obj
    }

    pub fn new_sampler(binding: u32, visibility: u32) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"binding".into(), &binding.into()).unwrap();
        Reflect::set(&obj, &"visibility".into(), &visibility.into()).unwrap();

        let sampler = Object::new();
        Reflect::set(&sampler, &"type".into(), &"filtering".into()).unwrap();
        Reflect::set(&obj, &"sampler".into(), &sampler).unwrap();
        obj
    }
}

pub struct GpuBufferBindingLayout;
impl GpuBufferBindingLayout {
    pub fn new() -> Object {
        Object::new()
    }
}

pub struct GpuBindGroupDescriptor;
impl GpuBindGroupDescriptor {
    pub fn new(entries: &js_sys::Array, layout: &GpuBindGroupLayout) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"entries".into(), entries).unwrap();
        Reflect::set(&obj, &"layout".into(), layout).unwrap();
        obj
    }
}

pub struct GpuBindGroupEntry;
impl GpuBindGroupEntry {
    pub fn new(binding: u32, resource: &Object) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"binding".into(), &binding.into()).unwrap();
        Reflect::set(&obj, &"resource".into(), resource).unwrap();
        obj
    }
}

pub struct GpuPipelineLayoutDescriptor;
impl GpuPipelineLayoutDescriptor {
    pub fn new(bind_group_layouts: &js_sys::Array) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"bindGroupLayouts".into(), bind_group_layouts).unwrap();
        obj
    }
}

pub struct GpuColorTargetState;
impl GpuColorTargetState {
    pub fn new(format: &str) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"format".into(), &format.into()).unwrap();
        obj
    }
}

pub struct GpuFragmentState;
impl GpuFragmentState {
    pub fn new(entry_point: &str, module: &GpuShaderModule, targets: &js_sys::Array) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"entryPoint".into(), &entry_point.into()).unwrap();
        Reflect::set(&obj, &"module".into(), module).unwrap();
        Reflect::set(&obj, &"targets".into(), targets).unwrap();
        obj
    }
}

pub struct GpuRenderPassDescriptor;
impl GpuRenderPassDescriptor {
    pub fn new(color_attachments: &js_sys::Array) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"colorAttachments".into(), color_attachments).unwrap();
        obj
    }
}

pub struct GpuRenderPassColorAttachment;
impl GpuRenderPassColorAttachment {
    pub fn new(view: &GpuTextureView) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"view".into(), view).unwrap();
        Reflect::set(&obj, &"loadOp".into(), &"clear".into()).unwrap();
        Reflect::set(&obj, &"storeOp".into(), &"store".into()).unwrap();
        obj
    }
}

pub struct GpuDepthStencilState;
impl GpuDepthStencilState {
    pub fn new(format: &str) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"format".into(), &format.into()).unwrap();
        obj
    }
}

pub struct GpuRenderPassDepthStencilAttachment;
impl GpuRenderPassDepthStencilAttachment {
    pub fn new(view: &GpuTextureView) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"view".into(), view).unwrap();
        Reflect::set(&obj, &"depthLoadOp".into(), &"clear".into()).unwrap();
        Reflect::set(&obj, &"depthStoreOp".into(), &"store".into()).unwrap();
        Reflect::set(&obj, &"depthClearValue".into(), &1.0.into()).unwrap();
        obj
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    pub fn warn(s: &str);
}

// Constants
pub mod GpuBufferUsage {
    pub const MAP_READ: u32 = 0x0001;
    pub const MAP_WRITE: u32 = 0x0002;
    pub const COPY_SRC: u32 = 0x0004;
    pub const COPY_DST: u32 = 0x0008;
    pub const INDEX: u32 = 0x0010;
    pub const VERTEX: u32 = 0x0020;
    pub const UNIFORM: u32 = 0x0040;
    pub const STORAGE: u32 = 0x0080;
    pub const INDIRECT: u32 = 0x0100;
    pub const QUERY_RESOLVE: u32 = 0x0200;
}

pub mod GpuTextureUsage {
    pub const COPY_SRC: u32 = 0x0001;
    pub const COPY_DST: u32 = 0x0002;
    pub const TEXTURE_BINDING: u32 = 0x0004;
    pub const STORAGE_BINDING: u32 = 0x0008;
    pub const RENDER_ATTACHMENT: u32 = 0x0010;
}

pub mod GpuShaderStage {
    pub const VERTEX: u32 = 0x0001;
    pub const FRAGMENT: u32 = 0x0002;
    pub const COMPUTE: u32 = 0x0004;
}

pub mod GpuMapMode {
    pub const READ: u32 = 0x0001;
    pub const WRITE: u32 = 0x0002;
}

pub struct GpuTextureFormat;
impl GpuTextureFormat {
    pub const Bgra8unorm: &'static str = "bgra8unorm";
}

pub enum GpuBufferBindingType {
    Uniform,
    Storage,
    ReadOnlyStorage,
}

impl GpuBufferBindingType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Uniform => "uniform",
            Self::Storage => "storage",
            Self::ReadOnlyStorage => "read-only-storage",
        }
    }
}

pub struct GpuImageCopyTexture;
impl GpuImageCopyTexture {
    pub fn new(texture: &GpuTexture) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"texture".into(), texture).unwrap();
        obj
    }
}

pub struct GpuImageDataLayout;
impl GpuImageDataLayout {
    pub fn new(bytes_per_row: u32, rows_per_image: u32) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"bytesPerRow".into(), &bytes_per_row.into()).unwrap();
        Reflect::set(&obj, &"rowsPerImage".into(), &rows_per_image.into()).unwrap();
        obj
    }
}

pub struct GpuExtent3D;
impl GpuExtent3D {
    pub fn new(width: u32, height: u32) -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"width".into(), &width.into()).unwrap();
        Reflect::set(&obj, &"height".into(), &height.into()).unwrap();
        Reflect::set(&obj, &"depthOrArrayLayers".into(), &1u32.into()).unwrap();
        obj
    }
}

pub struct GpuSamplerBindingType;
impl GpuSamplerBindingType {
    pub const Filtering: &'static str = "filtering";
}

pub struct GpuTextureSampleType;
impl GpuTextureSampleType {
    pub const Float: &'static str = "float";
}

pub struct GpuTextureViewDimension;
impl GpuTextureViewDimension {
    pub const D2: &'static str = "2d";
}

pub struct GpuSamplerDescriptor;
impl GpuSamplerDescriptor {
    pub fn new() -> Object {
        let obj = Object::new();
        Reflect::set(&obj, &"magFilter".into(), &"linear".into()).unwrap();
        Reflect::set(&obj, &"minFilter".into(), &"linear".into()).unwrap();
        obj
    }
}
