use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::web_bindings::{
    GpuDevice, GpuCanvasContext, GpuBuffer, GpuBufferDescriptor,
    GpuBindGroup, GpuBindGroupDescriptor, GpuBindGroupEntry,
    GpuBindGroupLayoutDescriptor, GpuBindGroupLayoutEntry,
    GpuComputePipeline, GpuRenderPipeline,
    GpuPipelineLayoutDescriptor, GpuShaderModuleDescriptor,
    GpuBufferBindingLayout, GpuBufferBindingType,
    GpuColorTargetState, GpuFragmentState,
    GpuRenderPassDescriptor, GpuRenderPassColorAttachment,
    GpuComputePassEncoder, GpuRenderPassEncoder,
    GpuTexture, GpuTextureDescriptor, GpuTextureFormat,
    GpuDepthStencilState, GpuRenderPassDepthStencilAttachment,
    GpuBufferUsage, GpuTextureUsage, GpuShaderStage, GpuMapMode,
    get_window, Window, HtmlCanvasElement, GpuSampler
};
use crate::{FlexEngine, log};
use js_sys::{Object, Reflect, Promise};

// No local log/warn - use crate::log

const SHADER_COMPUTE: &str = include_str!("shaders_compute.wgsl");
const SHADER_VISUAL: &str = include_str!("shaders_visual.wgsl");

#[wasm_bindgen]
pub struct FlexRenderer {
    engine: FlexEngine,
    device: GpuDevice,
    context: GpuCanvasContext,
    
    // Textures
    depth_texture: Option<GpuTexture>,
    depth_texture_width: u32,
    depth_texture_height: u32,

    // Buffers
    nodes_buffer: Option<GpuBuffer>,
    characters_buffer: Option<GpuBuffer>,
    glyph_buffer: Option<GpuBuffer>,
    uniform_buffer: Option<GpuBuffer>,
    curve_buffer: Option<GpuBuffer>,
    glyph_info_buffer: Option<GpuBuffer>,

    // Pipelines
    pipeline_reset_signals: Option<GpuComputePipeline>,
    pipeline_bottom_up: Option<GpuComputePipeline>,
    pipeline_top_down: Option<GpuComputePipeline>,
    pipeline_height_bottom_up: Option<GpuComputePipeline>,
    pipeline_final_layout: Option<GpuComputePipeline>,
    pipeline_render: Option<GpuRenderPipeline>,
    pipeline_render_text: Option<GpuRenderPipeline>,

    // BindGroups
    bind_group_compute: Option<GpuBindGroup>,
    bind_group_render: Option<GpuBindGroup>,
}

#[wasm_bindgen]
impl FlexRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(device: GpuDevice, context: GpuCanvasContext) -> FlexRenderer {
        FlexRenderer {
            engine: FlexEngine::new(),
            device,
            context,
            depth_texture: None,
            depth_texture_width: 0,
            depth_texture_height: 0,
            nodes_buffer: None,
            characters_buffer: None,
            glyph_buffer: None,
            uniform_buffer: None,
            curve_buffer: None,
            glyph_info_buffer: None,
            pipeline_reset_signals: None,
            pipeline_bottom_up: None,
            pipeline_top_down: None,
            pipeline_height_bottom_up: None,
            pipeline_final_layout: None,
            pipeline_render: None,
            pipeline_render_text: None,
            bind_group_compute: None,
            bind_group_render: None,
        }
    }

    pub fn init(&mut self) -> Result<(), wasm_bindgen::JsValue> {
        log("Rust Renderer Initializing...");

        // 1. Setup Scene
        crate::ui::build_ui(&mut self.engine);

        // Constants for Buffer Usage
        const USAGE_STORAGE: u32 = 0x0080;
        const USAGE_COPY_DST: u32 = 0x0008;
        const USAGE_COPY_SRC: u32 = 0x0004;
        const USAGE_UNIFORM: u32 = 0x0040;

        // 2. Buffers
        let node_count = self.engine.get_node_count() as u32;
        let node_size = self.engine.get_node_size() as u32;
        let nodes_byte_length = node_count * node_size;

        self.nodes_buffer = Some(self.device.create_buffer(&GpuBufferDescriptor::new(
             nodes_byte_length as f64,
             USAGE_STORAGE | USAGE_COPY_DST | USAGE_COPY_SRC
        )));

        let char_count = self.engine.get_character_count() as u32;
        let char_size = self.engine.get_character_size() as u32;
        let chars_byte_length = char_count * char_size;
        let chars_alloc_size = if chars_byte_length == 0 { 4 } else { chars_byte_length };

        let chars_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            chars_alloc_size as f64,
             USAGE_STORAGE | USAGE_COPY_DST | USAGE_COPY_SRC
        ));

        if char_count > 0 {
             let char_data = self.engine.get_characters_buffer();
             let char_vec = char_data.to_vec();
             self.device.queue().write_buffer_with_f64_and_u8_array(&chars_buf, 0.0, &char_vec);
        }
        self.characters_buffer = Some(chars_buf);

        let glyph_count = self.engine.get_glyph_data_count() as u32;
        let glyph_size = self.engine.get_glyph_data_size() as u32;
        let glyph_byte_length = glyph_count * glyph_size;
        let glyph_alloc_size = if glyph_byte_length == 0 { 4 } else { glyph_byte_length };

        let glyph_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            glyph_alloc_size as f64,
             USAGE_STORAGE | USAGE_COPY_DST
        ));

        if glyph_count > 0 {
            let glyph_data = self.engine.get_glyph_data_buffer();
            let glyph_vec = glyph_data.to_vec();
            self.device.queue().write_buffer_with_f64_and_u8_array(&glyph_buf, 0.0, &glyph_vec);
        }
        self.glyph_buffer = Some(glyph_buf);
        
        self.uniform_buffer = Some(self.device.create_buffer(&GpuBufferDescriptor::new(
            16.0,
            USAGE_UNIFORM | USAGE_COPY_DST
        )));

        // Vectors
        let curve_data = self.engine.get_curve_buffer();
        let curve_alloc_size = if curve_data.byte_length() == 0 { 4 } else { curve_data.byte_length() };
        
        let curve_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            curve_alloc_size as f64,
            USAGE_STORAGE | USAGE_COPY_DST | USAGE_COPY_SRC
        ));
        
        if curve_data.byte_length() > 0 {
             let curve_vec = curve_data.to_vec();
             self.device.queue().write_buffer_with_f64_and_u8_array(&curve_buf, 0.0, &curve_vec);
        }
        self.curve_buffer = Some(curve_buf);

        let glyph_info_data = self.engine.get_glyph_info_buffer();
        let info_alloc_size = if glyph_info_data.byte_length() == 0 { 4 } else { glyph_info_data.byte_length() };
        
        let info_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
             info_alloc_size as f64,
             USAGE_STORAGE | USAGE_COPY_DST | USAGE_COPY_SRC
        ));
        
        if glyph_info_data.byte_length() > 0 {
             let info_vec = glyph_info_data.to_vec();
             self.device.queue().write_buffer_with_f64_and_u8_array(&info_buf, 0.0, &info_vec);
        }
        self.glyph_info_buffer = Some(info_buf);

        // 3. Pipelines
        let module_compute = self.device.create_shader_module(&GpuShaderModuleDescriptor::new(SHADER_COMPUTE));
        let module_visual = self.device.create_shader_module(&GpuShaderModuleDescriptor::new(SHADER_VISUAL));

        // Generate Bind Group Layouts
        let make_layout_entry = |binding: u32, visibility: u32, type_: GpuBufferBindingType| -> js_sys::Object {
             let mut layout_entry = GpuBindGroupLayoutEntry::new(binding, visibility);
             let mut buffer_layout = GpuBufferBindingLayout::new();
             Reflect::set(&buffer_layout, &"type".into(), &type_.as_str().into()).unwrap();
             Reflect::set(&layout_entry, &"buffer".into(), &buffer_layout).unwrap();
             layout_entry
        };

        let bind_group_layout_compute = self.device.create_bind_group_layout(&GpuBindGroupLayoutDescriptor::new(&js_sys::Array::of4(
            &make_layout_entry(0, GpuShaderStage::COMPUTE, GpuBufferBindingType::Storage),
            &make_layout_entry(1, GpuShaderStage::COMPUTE, GpuBufferBindingType::Uniform),
            &make_layout_entry(2, GpuShaderStage::COMPUTE, GpuBufferBindingType::Storage),
            &make_layout_entry(3, GpuShaderStage::COMPUTE, GpuBufferBindingType::ReadOnlyStorage),
        )));

        let bind_group_layout_render = self.device.create_bind_group_layout(&GpuBindGroupLayoutDescriptor::new(&js_sys::Array::of5(
            &make_layout_entry(0, GpuShaderStage::VERTEX, GpuBufferBindingType::ReadOnlyStorage), // Vertex
            &make_layout_entry(1, GpuShaderStage::VERTEX, GpuBufferBindingType::Uniform), // Vertex
            &make_layout_entry(2, GpuShaderStage::VERTEX, GpuBufferBindingType::ReadOnlyStorage), // Vertex
            &make_layout_entry(3, GpuShaderStage::FRAGMENT, GpuBufferBindingType::ReadOnlyStorage), // Fragment
            &make_layout_entry(4, GpuShaderStage::FRAGMENT, GpuBufferBindingType::ReadOnlyStorage), // Fragment
        )));

        let layout_compute = self.device.create_pipeline_layout(&GpuPipelineLayoutDescriptor::new(&js_sys::Array::of1(&bind_group_layout_compute)));
        let layout_render = self.device.create_pipeline_layout(&GpuPipelineLayoutDescriptor::new(&js_sys::Array::of1(&bind_group_layout_render)));

        // Create Pipelines
        // Note: GpuComputePipelineDescriptor::new might not exist explicitly if web-sys generated just dictionary
        // But let's check error: "function new is implemented on Object".
        // It implies we should cast object.
        
        let create_compute = |entry: &str| -> GpuComputePipeline {
             let stage = js_sys::Object::new();
             Reflect::set(&stage, &"module".into(), &module_compute).unwrap();
             Reflect::set(&stage, &"entryPoint".into(), &entry.into()).unwrap();
             
             // Construct descriptor manually object
             let desc = js_sys::Object::new();
             Reflect::set(&desc, &"layout".into(), &layout_compute).unwrap();
             Reflect::set(&desc, &"compute".into(), &stage).unwrap();

             self.device.create_compute_pipeline(&desc.unchecked_into())
        };

        self.pipeline_reset_signals = Some(create_compute("reset_signals"));
        self.pipeline_bottom_up = Some(create_compute("width_bottom_up"));
        self.pipeline_top_down = Some(create_compute("width_top_down"));
        self.pipeline_height_bottom_up = Some(create_compute("height_bottom_up"));
        self.pipeline_final_layout = Some(create_compute("final_layout"));

        // Render Pipelines
        let create_render = |vs_entry: &str, fs_entry: &str| -> GpuRenderPipeline {
            let vs_state_obj = js_sys::Object::new();
            Reflect::set(&vs_state_obj, &"module".into(), &module_visual).unwrap();
            Reflect::set(&vs_state_obj, &"entryPoint".into(), &vs_entry.into()).unwrap();

            let target = GpuColorTargetState::new(GpuTextureFormat::Bgra8unorm); 
            // Blend
            let blend = js_sys::Object::new();
            let color_blend = js_sys::Object::new();
            Reflect::set(&color_blend, &"srcFactor".into(), &"src-alpha".into()).unwrap();
            Reflect::set(&color_blend, &"dstFactor".into(), &"one-minus-src-alpha".into()).unwrap();
            Reflect::set(&color_blend, &"operation".into(), &"add".into()).unwrap();
            
            let alpha_blend = js_sys::Object::new();
            Reflect::set(&alpha_blend, &"srcFactor".into(), &"one".into()).unwrap();
            Reflect::set(&alpha_blend, &"dstFactor".into(), &"one-minus-src-alpha".into()).unwrap();
            Reflect::set(&alpha_blend, &"operation".into(), &"add".into()).unwrap();

            Reflect::set(&blend, &"color".into(), &color_blend).unwrap();
            Reflect::set(&blend, &"alpha".into(), &alpha_blend).unwrap();
            
            // Use Reflect to set blend since generated method might be missing or expect strict type
            Reflect::set(&target, &"blend".into(), &blend).unwrap();

            let fs_state = GpuFragmentState::new(fs_entry, &module_visual, &js_sys::Array::of1(&target));
            
            // Depth Stencil State
            let depth_state = js_sys::Object::new();
            Reflect::set(&depth_state, &"format".into(), &"depth24plus".into()).unwrap();
            Reflect::set(&depth_state, &"depthWriteEnabled".into(), &true.into()).unwrap();
            Reflect::set(&depth_state, &"depthCompare".into(), &"less-equal".into()).unwrap();
            
            // GpuRenderPipelineDescriptor
            let desc = js_sys::Object::new();
            Reflect::set(&desc, &"layout".into(), &layout_render).unwrap();
            Reflect::set(&desc, &"vertex".into(), &vs_state_obj).unwrap();
            Reflect::set(&desc, &"fragment".into(), &fs_state).unwrap();
            Reflect::set(&desc, &"depthStencil".into(), &depth_state).unwrap(); // Add depth stencil
            
            // primitive
            let prim = js_sys::Object::new();
            Reflect::set(&prim, &"topology".into(), &"triangle-list".into()).unwrap();
            Reflect::set(&desc, &"primitive".into(), &prim).unwrap();

            self.device.create_render_pipeline(&desc.unchecked_into())
        };

        self.pipeline_render = Some(create_render("vs_main", "fs_main"));
        self.pipeline_render_text = Some(create_render("vs_text", "fs_text"));

        // 4. Bind Groups
        let make_buffer_binding = |buffer: &GpuBuffer| -> Object {
             let obj = Object::new();
             Reflect::set(&obj, &"buffer".into(), buffer).unwrap();
             obj
        };
        
        self.bind_group_compute = Some(self.device.create_bind_group(&GpuBindGroupDescriptor::new(
            &js_sys::Array::of4(
                &GpuBindGroupEntry::new(0, &make_buffer_binding(self.nodes_buffer.as_ref().unwrap())),
                &GpuBindGroupEntry::new(1, &make_buffer_binding(self.uniform_buffer.as_ref().unwrap())),
                &GpuBindGroupEntry::new(2, &make_buffer_binding(self.characters_buffer.as_ref().unwrap())),
                &GpuBindGroupEntry::new(3, &make_buffer_binding(self.glyph_buffer.as_ref().unwrap())),
            ),
            &bind_group_layout_compute,
        )));

        self.bind_group_render = Some(self.device.create_bind_group(&GpuBindGroupDescriptor::new(
            &js_sys::Array::of5(
                &GpuBindGroupEntry::new(0, &make_buffer_binding(self.nodes_buffer.as_ref().unwrap())),
                &GpuBindGroupEntry::new(1, &make_buffer_binding(self.uniform_buffer.as_ref().unwrap())),
                &GpuBindGroupEntry::new(2, &make_buffer_binding(self.characters_buffer.as_ref().unwrap())),
                &GpuBindGroupEntry::new(3, &make_buffer_binding(self.curve_buffer.as_ref().unwrap())),
                &GpuBindGroupEntry::new(4, &make_buffer_binding(self.glyph_info_buffer.as_ref().unwrap())),
            ),
            &bind_group_layout_render,
        )));

        log("Rust Renderer Init Complete");
        Ok(())
    }

    pub fn render(&mut self) {
        log("Render Start");
        if self.nodes_buffer.is_none() { 
            log("Render: skipping, no buffers");
            return; 
        }

        let windowValue = get_window().unwrap();
        let dpr = windowValue.device_pixel_ratio();
        let canvas: HtmlCanvasElement = self.context.canvas();
        let width = canvas.width() as f32 / dpr as f32;
        let height = canvas.height() as f32 / dpr as f32;

        let ascender = self.engine.get_ascender();
        let _descender = self.engine.get_descender();
        let line_gap = self.engine.get_line_gap();
        let line_height = ascender - _descender + line_gap;

        let uniform_data = vec![width, height, ascender, line_height];
        let uniform_bytes: &[u8] = unsafe {
             std::slice::from_raw_parts(uniform_data.as_ptr() as *const u8, uniform_data.len() * 4)
        };

        self.device.queue().write_buffer_with_f64_and_u8_array(
            self.uniform_buffer.as_ref().unwrap(), 
            0.0, 
            uniform_bytes
        );

        // Update Nodes
        let nodes_data = self.engine.get_nodes_buffer();
        let nodes_vec = nodes_data.to_vec();
        self.device.queue().write_buffer_with_f64_and_u8_array(
            self.nodes_buffer.as_ref().unwrap(),
            0.0,
            &nodes_vec
        );

        let command_encoder = self.device.create_command_encoder(); 

        let node_count = self.engine.get_node_count() as u32;
        let workgroups = (node_count as f32 / 64.0).ceil() as u32;
        let dispatch = |pass: &GpuComputePassEncoder, x: u32| {
             pass.dispatchWorkgroups(x, 1, 1);
        };
        
        let end_compute = |pass: &GpuComputePassEncoder| {
             pass.end_compute();
        };

        // Pass 1: Width Bottom-Up
        log("Render: Pass 1");
        {
            let pass = command_encoder.begin_compute_pass();
            if self.pipeline_bottom_up.is_none() { log("Pipeline bottom_up is None!"); }
            pass.set_pipeline_compute(self.pipeline_bottom_up.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }

        // Pass Reset
        log("Render: Pass 1 Reset");
        {
            let pass = command_encoder.begin_compute_pass();
             if self.pipeline_reset_signals.is_none() { log("Pipeline reset is None!"); }
            pass.set_pipeline_compute(self.pipeline_reset_signals.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }

        // Pass 2: Width Top-Down (8 iterations)
        log("Render: Pass 2");
        for _ in 0..8 {
            let pass = command_encoder.begin_compute_pass();
             if self.pipeline_top_down.is_none() { log("Pipeline top_down is None!"); }
            pass.set_pipeline_compute(self.pipeline_top_down.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }

        // Pass Reset
        log("Render: Pass 2 Reset");
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_reset_signals.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }

         // Pass 3: Height Bottom-Up
         log("Render: Pass 3");
        {
            let pass = command_encoder.begin_compute_pass();
            if self.pipeline_height_bottom_up.is_none() { log("Pipeline height_bottom_up is None!"); }
            pass.set_pipeline_compute(self.pipeline_height_bottom_up.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }
        
        // Pass Reset
        log("Render: Pass 3 Reset");
        {
             let pass = command_encoder.begin_compute_pass();
             pass.set_pipeline_compute(self.pipeline_reset_signals.as_ref().unwrap());
             pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
             dispatch(&pass, workgroups);
             end_compute(&pass);
        }

         // Pass 4: Final Layout (8 iterations)
         log("Render: Pass 4");
        for _ in 0..8 {
            let pass = command_encoder.begin_compute_pass();
            if self.pipeline_final_layout.is_none() { log("Pipeline final_layout is None!"); }
            pass.set_pipeline_compute(self.pipeline_final_layout.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }

        // Render Pass
        log("Render: Render Pass Setup");
        let texture_view = self.context.get_current_texture().create_view();
        
        // Depth Texture Management
        let canvas_width = canvas.width();
        let canvas_height = canvas.height();
        
        if self.depth_texture.is_none() || self.depth_texture_width != canvas_width || self.depth_texture_height != canvas_height {
            if let Some(texture) = &self.depth_texture {
                texture.destroy();
            }
            
            let depth_desc = js_sys::Object::new();
            let size = js_sys::Array::of2(&canvas_width.into(), &canvas_height.into());
            Reflect::set(&depth_desc, &"size".into(), &size).unwrap();
            Reflect::set(&depth_desc, &"format".into(), &"depth24plus".into()).unwrap();
            Reflect::set(&depth_desc, &"usage".into(), &GpuTextureUsage::RENDER_ATTACHMENT.into()).unwrap();
            
            self.depth_texture = Some(self.device.create_texture(&depth_desc));
            self.depth_texture_width = canvas_width;
            self.depth_texture_height = canvas_height;
            log(&format!("Created Depth Texture: {}x{}", canvas_width, canvas_height));
        }
        
        let depth_view = self.depth_texture.as_ref().unwrap().create_view();
        let depth_attachment_obj = js_sys::Object::new();
        Reflect::set(&depth_attachment_obj, &"view".into(), &depth_view).unwrap();
        Reflect::set(&depth_attachment_obj, &"depthLoadOp".into(), &"clear".into()).unwrap();
        Reflect::set(&depth_attachment_obj, &"depthStoreOp".into(), &"store".into()).unwrap();
        Reflect::set(&depth_attachment_obj, &"depthClearValue".into(), &1.0.into()).unwrap();

        // Manual creation of descriptor object to ensure keys are correct
        let color_attachment_obj = js_sys::Object::new();
        Reflect::set(&color_attachment_obj, &"view".into(), &texture_view).unwrap();
        Reflect::set(&color_attachment_obj, &"loadOp".into(), &"clear".into()).unwrap();
        Reflect::set(&color_attachment_obj, &"storeOp".into(), &"store".into()).unwrap();
        
        let clear_val = js_sys::Object::new();
        Reflect::set(&clear_val, &"r".into(), &0.1.into()).unwrap();
        Reflect::set(&clear_val, &"g".into(), &0.1.into()).unwrap();
        Reflect::set(&clear_val, &"b".into(), &0.1.into()).unwrap();
        Reflect::set(&clear_val, &"a".into(), &1.0.into()).unwrap();
        
        Reflect::set(&color_attachment_obj, &"clearValue".into(), &clear_val).unwrap();

        log("Render: Pass Descriptor");
        // Cast object to GpuRenderPassColorAttachment for type safety in array (unchecked is fine)
        let render_pass_desc = GpuRenderPassDescriptor::new(&js_sys::Array::of1(&color_attachment_obj));
        
        // Attach Depth Stencil
        Reflect::set(&render_pass_desc, &"depthStencilAttachment".into(), &depth_attachment_obj).unwrap();
        
        log("Render: Begin Pass");
        let render_pass = command_encoder.begin_render_pass(&render_pass_desc);
        
        log("Render: Set Pipeline");
        if self.pipeline_render.is_none() { log("CRITICAL: pipeline_render is None"); }
        render_pass.set_pipeline_render(self.pipeline_render.as_ref().unwrap());

        log("Render: Set Bind Group");
        if self.bind_group_render.is_none() { log("CRITICAL: bind_group_render is None"); }
        render_pass.set_bind_group_render(0, self.bind_group_render.as_ref().unwrap());
        
        log("Render: Draw");
        render_pass.draw_with_instance_count(6, node_count, 0, 0);

        // Draw Text
        let char_count = self.engine.get_character_count() as u32;
        if char_count > 0 {
            log("Render: Draw Text");
            if self.pipeline_render_text.is_none() { log("CRITICAL: pipeline_render_text is None"); }
            render_pass.set_pipeline_render(self.pipeline_render_text.as_ref().unwrap());
            render_pass.set_bind_group_render(0, self.bind_group_render.as_ref().unwrap());
            render_pass.draw_with_instance_count(6, char_count, 0, 0);
        }

        log("Render: End Pass");
        let end_render = |pass: &GpuRenderPassEncoder| {
             pass.end_render();
        };
        end_render(&render_pass);

        self.device.queue().submit(&js_sys::Array::of1(&command_encoder.finish()));
    }

    pub fn debug(&self) -> Promise {
        let device = self.device.unchecked_ref::<Object>().clone().unchecked_into::<GpuDevice>();
        let nodes_buffer = self.nodes_buffer.as_ref().map(|b| b.unchecked_ref::<Object>().clone().unchecked_into::<GpuBuffer>());
        let characters_buffer = self.characters_buffer.as_ref().map(|b| b.unchecked_ref::<Object>().clone().unchecked_into::<GpuBuffer>());
        let curve_buffer = self.curve_buffer.as_ref().map(|b| b.unchecked_ref::<Object>().clone().unchecked_into::<GpuBuffer>());
        let glyph_info_buffer = self.glyph_info_buffer.as_ref().map(|b| b.unchecked_ref::<Object>().clone().unchecked_into::<GpuBuffer>());

        wasm_bindgen_futures::future_to_promise(async move {
            if nodes_buffer.is_none() || characters_buffer.is_none() || curve_buffer.is_none() || glyph_info_buffer.is_none() {
                return Ok(wasm_bindgen::JsValue::UNDEFINED);
            }

            let nodes_buffer = nodes_buffer.unwrap();
            let characters_buffer = characters_buffer.unwrap();
            let curve_buffer = curve_buffer.unwrap();
            let glyph_info_buffer = glyph_info_buffer.unwrap();

            let command_encoder = device.create_command_encoder();

            // Safe helper for size using Reflect
            let get_size = |buffer: &GpuBuffer| -> f64 {
                 let val = Reflect::get(buffer, &"size".into()).unwrap();
                 val.as_f64().unwrap_or(0.0)
            };

            let create_read_buffer = |size: f64| -> GpuBuffer {
                 device.create_buffer(&GpuBufferDescriptor::new(
                    size,
                    GpuBufferUsage::COPY_DST | GpuBufferUsage::MAP_READ
                ))
            };
            
            let nodes_read = create_read_buffer(get_size(&nodes_buffer));
            let chars_read = create_read_buffer(get_size(&characters_buffer));
            let curve_read = create_read_buffer(get_size(&curve_buffer));
            let info_read = create_read_buffer(get_size(&glyph_info_buffer));

            command_encoder.copy_buffer_to_buffer(&nodes_buffer, 0.0, &nodes_read, 0.0, get_size(&nodes_buffer));
            command_encoder.copy_buffer_to_buffer(&characters_buffer, 0.0, &chars_read, 0.0, get_size(&characters_buffer));
            command_encoder.copy_buffer_to_buffer(&curve_buffer, 0.0, &curve_read, 0.0, get_size(&curve_buffer));
            command_encoder.copy_buffer_to_buffer(&glyph_info_buffer, 0.0, &info_read, 0.0, get_size(&glyph_info_buffer));

            device.queue().submit(&js_sys::Array::of1(&command_encoder.finish()));

            // Map Async
            let map_read = GpuMapMode::READ;
            let map = |buffer: &GpuBuffer| -> Promise {
                 buffer.map_async(map_read)
            };

            let _ = wasm_bindgen_futures::JsFuture::from(map(&nodes_read)).await;
            let _ = wasm_bindgen_futures::JsFuture::from(map(&chars_read)).await;
            let _ = wasm_bindgen_futures::JsFuture::from(map(&curve_read)).await;
            let _ = wasm_bindgen_futures::JsFuture::from(map(&info_read)).await;

            let read_f32 = |buffer: &GpuBuffer| -> Vec<f32> {
                 let range = buffer.get_mapped_range();
                 let f32_array = js_sys::Float32Array::new(&range);
                 let vec = f32_array.to_vec();
                 buffer.unmap();
                 vec
            };

            let read_u32 = |buffer: &GpuBuffer| -> Vec<u32> {
                 let range = buffer.get_mapped_range();
                 let u32_array = js_sys::Uint32Array::new(&range);
                 let vec = u32_array.to_vec();
                 buffer.unmap();
                 vec
            };

            log("--- Debug GPU Buffers ---");
            
            let nodes = read_f32(&nodes_read);
            log(&format!("Nodes (first 10): {:?}", nodes.iter().take(40).collect::<Vec<_>>())); 

            let chars_u32 = read_u32(&chars_read);
            log(&format!("Chars (first 10): {:?}", chars_u32.iter().take(10).collect::<Vec<_>>()));

            let curves = read_f32(&curve_read);
            log(&format!("Curves (first 10): {:?}", curves.iter().take(20).collect::<Vec<_>>()));

            let info = read_u32(&info_read);
            log(&format!("Glyph Info (first 10): {:?}", info.iter().take(10).collect::<Vec<_>>()));

            Ok(wasm_bindgen::JsValue::UNDEFINED)
        })
    }
}
