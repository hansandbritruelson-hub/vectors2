use wasm_bindgen::prelude::*;
use web_sys::{
    GpuDevice, GpuCanvasContext, GpuBuffer, GpuComputePipeline, GpuRenderPipeline, 
    GpuBindGroup, GpuShaderModule,
    GpuBufferDescriptor,
    GpuCommandEncoderDescriptor, GpuComputePassDescriptor,
    GpuRenderPassDescriptor, GpuRenderPassColorAttachment, GpuLoadOp, GpuStoreOp,
    GpuColorTargetState, GpuFragmentState, GpuVertexState, GpuPrimitiveState,
    GpuBindGroupDescriptor, GpuBindGroupEntry, GpuBindGroupLayoutDescriptor, GpuBindGroupLayoutEntry,
    GpuPipelineLayoutDescriptor, GpuShaderModuleDescriptor,
    GpuRenderPipelineDescriptor, GpuComputePipelineDescriptor,
    GpuBufferBinding, GpuBufferBindingLayout, GpuBufferBindingType,
};
use crate::FlexEngine;
use js_sys::{Object, Reflect};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    fn warn(s: &str);
}

const SHADER_COMPUTE: &str = include_str!("shaders_compute.wgsl");
const SHADER_VISUAL: &str = include_str!("shaders_visual.wgsl");

#[wasm_bindgen]
pub struct FlexRenderer {
    engine: FlexEngine,
    device: GpuDevice,
    context: GpuCanvasContext,

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

    pub fn init(&mut self) -> Result<(), JsValue> {
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
        ))?);

        let char_count = self.engine.get_character_count() as u32;
        let char_size = self.engine.get_character_size() as u32;
        let chars_byte_length = char_count * char_size;
        let chars_alloc_size = if chars_byte_length == 0 { 4 } else { chars_byte_length };

        let chars_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            chars_alloc_size as f64,
             USAGE_STORAGE | USAGE_COPY_DST | USAGE_COPY_SRC
        ))?;

        if char_count > 0 {
             let char_data = self.engine.get_characters_buffer();
             self.device.queue().write_buffer_with_f64_and_u8_array(&chars_buf, 0.0, &char_data);
        }
        self.characters_buffer = Some(chars_buf);

        let glyph_count = self.engine.get_glyph_data_count() as u32;
        let glyph_size = self.engine.get_glyph_data_size() as u32;
        let glyph_byte_length = glyph_count * glyph_size;
        let glyph_alloc_size = if glyph_byte_length == 0 { 4 } else { glyph_byte_length };

        let glyph_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            glyph_alloc_size as f64,
             USAGE_STORAGE | USAGE_COPY_DST
        ))?;

        if glyph_count > 0 {
            let glyph_data = self.engine.get_glyph_data_buffer();
            self.device.queue().write_buffer_with_f64_and_u8_array(&glyph_buf, 0.0, &glyph_data);
        }
        self.glyph_buffer = Some(glyph_buf);
        
        self.uniform_buffer = Some(self.device.create_buffer(&GpuBufferDescriptor::new(
            16.0,
            USAGE_UNIFORM | USAGE_COPY_DST
        ))?);

        // Vectors
        let curve_data = self.engine.get_curve_buffer();
        let curve_alloc_size = if curve_data.byte_length() == 0 { 4 } else { curve_data.byte_length() };
        
        let curve_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            curve_alloc_size as f64,
            USAGE_STORAGE | USAGE_COPY_DST | USAGE_COPY_SRC
        ))?;
        
        if curve_data.byte_length() > 0 {
             self.device.queue().write_buffer_with_f64_and_u8_array(&curve_buf, 0.0, &curve_data);
        }
        self.curve_buffer = Some(curve_buf);

        let glyph_info_data = self.engine.get_glyph_info_buffer();
        let info_alloc_size = if glyph_info_data.byte_length() == 0 { 4 } else { glyph_info_data.byte_length() };
        
        let info_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
             info_alloc_size as f64,
             USAGE_STORAGE | USAGE_COPY_DST | USAGE_COPY_SRC
        ))?;
        
        if glyph_info_data.byte_length() > 0 {
             self.device.queue().write_buffer_with_f64_and_u8_array(&info_buf, 0.0, &glyph_info_data);
        }
        self.glyph_info_buffer = Some(info_buf);

        // 3. Pipelines
        let module_compute = self.device.create_shader_module(&GpuShaderModuleDescriptor::new(SHADER_COMPUTE));
        let module_visual = self.device.create_shader_module(&GpuShaderModuleDescriptor::new(SHADER_VISUAL));

        // Generate Bind Group Layouts
        let make_layout_entry = |binding: u32, visibility: u32, type_: GpuBufferBindingType| -> GpuBindGroupLayoutEntry {
             let mut layout_entry = GpuBindGroupLayoutEntry::new(binding, visibility);
             let mut buffer_layout = GpuBufferBindingLayout::new();
             buffer_layout.set_type(type_);
             layout_entry.set_buffer(&buffer_layout);
             layout_entry
        };

        let bind_group_layout_compute = self.device.create_bind_group_layout(&GpuBindGroupLayoutDescriptor::new(&js_sys::Array::of4(
            &make_layout_entry(0, 4, GpuBufferBindingType::Storage),
            &make_layout_entry(1, 4, GpuBufferBindingType::Uniform),
            &make_layout_entry(2, 4, GpuBufferBindingType::Storage),
            &make_layout_entry(3, 4, GpuBufferBindingType::ReadOnlyStorage),
        ))).unwrap();

        let bind_group_layout_render = self.device.create_bind_group_layout(&GpuBindGroupLayoutDescriptor::new(&js_sys::Array::of5(
            &make_layout_entry(0, 1, GpuBufferBindingType::ReadOnlyStorage), // Vertex
            &make_layout_entry(1, 1, GpuBufferBindingType::Uniform), // Vertex
            &make_layout_entry(2, 1, GpuBufferBindingType::ReadOnlyStorage), // Vertex
            &make_layout_entry(3, 2, GpuBufferBindingType::ReadOnlyStorage), // Fragment
            &make_layout_entry(4, 2, GpuBufferBindingType::ReadOnlyStorage), // Fragment
        ))).unwrap();

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

            let target = GpuColorTargetState::new(web_sys::GpuTextureFormat::Bgra8unorm); 
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

            let fs_state = GpuFragmentState::new(&module_visual, &js_sys::Array::of1(&target));
            fs_state.set_entry_point(fs_entry);
            
            // GpuRenderPipelineDescriptor
            let desc = js_sys::Object::new();
            Reflect::set(&desc, &"layout".into(), &layout_render).unwrap();
            Reflect::set(&desc, &"vertex".into(), &vs_state_obj).unwrap();
            Reflect::set(&desc, &"fragment".into(), &fs_state).unwrap();
            
            // primitive
            let prim = js_sys::Object::new();
            Reflect::set(&prim, &"topology".into(), &"triangle-list".into()).unwrap();
            Reflect::set(&desc, &"primitive".into(), &prim).unwrap();

            self.device.create_render_pipeline(&desc.unchecked_into()).unwrap()
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

        Ok(())
    }

    pub fn render(&self) {
        if self.nodes_buffer.is_none() { return; }

        let window = web_sys::window().unwrap();
        let dpr = window.device_pixel_ratio();
        let canvas: web_sys::HtmlCanvasElement = self.context.canvas().unchecked_into();
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

        {
             let arr = js_sys::Uint8Array::new_with_length(uniform_bytes.len() as u32);
             arr.copy_from(uniform_bytes);
             self.device.queue().write_buffer_with_f64_and_u8_array(
                 self.uniform_buffer.as_ref().unwrap(), 
                 0.0, 
                 &arr
             );
        }

        // Update Nodes
        let nodes_data = self.engine.get_nodes_buffer();
        self.device.queue().write_buffer_with_f64_and_u8_array(
            self.nodes_buffer.as_ref().unwrap(),
            0.0,
            &nodes_data
        );

        let command_encoder = self.device.create_command_encoder(); 

        let node_count = self.engine.get_node_count() as u32;
        let workgroups = (node_count as f32 / 64.0).ceil() as u32;

        // Pass 1: Width Bottom-Up
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline(self.pipeline_bottom_up.as_ref().unwrap());
            pass.set_bind_group(0, Some(self.bind_group_compute.as_ref().unwrap()));
            pass.dispatch_workgroups(workgroups);
            pass.end();
        }

        // Pass Reset
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline(self.pipeline_reset_signals.as_ref().unwrap());
            pass.set_bind_group(0, Some(self.bind_group_compute.as_ref().unwrap()));
            pass.dispatch_workgroups(workgroups);
            pass.end();
        }

        // Pass 2: Width Top-Down (8 iterations)
        for _ in 0..8 {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline(self.pipeline_top_down.as_ref().unwrap());
            pass.set_bind_group(0, Some(self.bind_group_compute.as_ref().unwrap()));
            pass.dispatch_workgroups(workgroups);
            pass.end();
        }

        // Pass Reset
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline(self.pipeline_reset_signals.as_ref().unwrap());
            pass.set_bind_group(0, Some(self.bind_group_compute.as_ref().unwrap()));
            pass.dispatch_workgroups(workgroups);
            pass.end();
        }

         // Pass 3: Height Bottom-Up
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline(self.pipeline_height_bottom_up.as_ref().unwrap());
            pass.set_bind_group(0, Some(self.bind_group_compute.as_ref().unwrap()));
            pass.dispatch_workgroups(workgroups);
            pass.end();
        }

        // Pass Reset
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline(self.pipeline_reset_signals.as_ref().unwrap());
            pass.set_bind_group(0, Some(self.bind_group_compute.as_ref().unwrap()));
            pass.dispatch_workgroups(workgroups);
            pass.end();
        }

         // Pass 4: Final Layout (8 iterations)
        for _ in 0..8 {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline(self.pipeline_final_layout.as_ref().unwrap());
            pass.set_bind_group(0, Some(self.bind_group_compute.as_ref().unwrap()));
            pass.dispatch_workgroups(workgroups);
            pass.end();
        }

        // Render Pass
        let texture_view = self.context.get_current_texture().unwrap().create_view().unwrap();
        
        let clear_val = js_sys::Object::new();
        Reflect::set(&clear_val, &"r".into(), &0.1.into()).unwrap();
        Reflect::set(&clear_val, &"g".into(), &0.1.into()).unwrap();
        Reflect::set(&clear_val, &"b".into(), &0.1.into()).unwrap();
        Reflect::set(&clear_val, &"a".into(), &1.0.into()).unwrap();

        let color_attachment = GpuRenderPassColorAttachment::new(
            GpuLoadOp::Clear, 
            GpuStoreOp::Store, 
            &texture_view
        );
        color_attachment.set_clear_value(&clear_val);

        let render_pass_desc = GpuRenderPassDescriptor::new(&js_sys::Array::of1(&color_attachment));
        let render_pass = command_encoder.begin_render_pass(&render_pass_desc).unwrap();
        
        render_pass.set_pipeline(self.pipeline_render.as_ref().unwrap());
        render_pass.set_bind_group(0, Some(self.bind_group_render.as_ref().unwrap()));
        render_pass.draw_with_instance_count(6, node_count);

        // Draw Text
        let char_count = self.engine.get_character_count() as u32;
        if char_count > 0 {
            render_pass.set_pipeline(self.pipeline_render_text.as_ref().unwrap());
            render_pass.set_bind_group(0, Some(self.bind_group_render.as_ref().unwrap()));
            render_pass.draw_with_instance_count(6, char_count);
        }

        render_pass.end(); // void return

        self.device.queue().submit(&js_sys::Array::of1(&command_encoder.finish()));
    }

    pub fn debug(&self) {
        log("Debug: Rust renderer running. (Buffer inspection not yet ported)");
    }
}
