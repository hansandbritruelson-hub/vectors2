use crate::web_bindings::{
    get_window, GpuBindGroup, GpuBindGroupDescriptor, GpuBindGroupEntry, GpuBindGroupLayout,
    GpuBindGroupLayoutDescriptor, GpuBindGroupLayoutEntry, GpuBuffer, GpuBufferBindingLayout,
    GpuBufferBindingType, GpuBufferDescriptor, GpuCanvasContext, GpuColorTargetState,
    GpuComputePassEncoder, GpuComputePipeline, GpuDevice, GpuExtent3D, GpuFragmentState,
    GpuImageCopyTexture, GpuImageDataLayout, GpuPipelineLayoutDescriptor, GpuRenderPassDescriptor,
    GpuRenderPipeline, GpuSampler, GpuSamplerDescriptor, GpuShaderModuleDescriptor, GpuShaderStage,
    GpuTexture, GpuTextureDescriptor, GpuTextureFormat, GpuTextureUsage, HtmlCanvasElement,
};
use crate::{log, FlexEngine};
use js_sys::{Object, Promise, Reflect};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const SHADER_COMPUTE: &str = include_str!("shaders_compute.wgsl");
const SHADER_VISUAL: &str = include_str!("shaders_visual.wgsl");
const SHADER_STYLE_CONSTANTS: &str =
    include_str!(concat!(env!("OUT_DIR"), "/style_constants.wgsl"));

#[wasm_bindgen]
pub struct FlexRenderer {
    engine: Rc<RefCell<FlexEngine>>,
    device: GpuDevice,
    context: GpuCanvasContext,

    depth_texture: Option<GpuTexture>,
    depth_texture_width: u32,
    depth_texture_height: u32,

    nodes_buffer: Option<GpuBuffer>,
    characters_buffer: Option<GpuBuffer>,
    glyph_buffer: Option<GpuBuffer>,
    uniform_buffer: Option<GpuBuffer>,
    curve_buffer: Option<GpuBuffer>,
    glyph_info_buffer: Option<GpuBuffer>,
    class_defs_buffer: Option<GpuBuffer>,
    node_class_list_and_inline_styles_buffer: Option<GpuBuffer>,
    leaf_nodes_buffer: Option<GpuBuffer>,
    panic_buffer: Option<GpuBuffer>,
    panic_readback_buffer: Option<GpuBuffer>,

    // Image Resources
    atlas_texture: Option<GpuTexture>,
    atlas_width: u32,
    atlas_height: u32,
    sampler: Option<GpuSampler>,

    pipeline_reset_signals: Option<GpuComputePipeline>,
    pipeline_bottom_up: Option<GpuComputePipeline>,
    pipeline_top_down: Option<GpuComputePipeline>,
    pipeline_height_bottom_up: Option<GpuComputePipeline>,
    pipeline_final_layout: Option<GpuComputePipeline>,
    pipeline_render: Option<GpuRenderPipeline>,
    pipeline_render_text: Option<GpuRenderPipeline>,
    pipeline_resolve_styles: Option<GpuComputePipeline>,
    pipeline_inherit_styles: Option<GpuComputePipeline>,

    bind_group_compute: Option<GpuBindGroup>,
    bind_group_render: Option<GpuBindGroup>,

    bind_group_layout_compute: Option<GpuBindGroupLayout>,
    bind_group_layout_render: Option<GpuBindGroupLayout>,

    readback_in_progress: Rc<std::cell::Cell<bool>>,
    panic_readback_in_progress: Rc<std::cell::Cell<bool>>,
}

#[wasm_bindgen]
impl FlexRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(device: GpuDevice, context: GpuCanvasContext, engine: FlexEngine) -> FlexRenderer {
        let engine = Rc::new(RefCell::new(engine));
        Self::new_with_ref(device, context, engine)
    }
}

impl FlexRenderer {
    pub fn new_with_ref(
        device: GpuDevice,
        context: GpuCanvasContext,
        engine: Rc<RefCell<FlexEngine>>,
    ) -> FlexRenderer {
        log(&format!("--- RENDERER STARTUP (Shared Engine) ---"));
        log(&format!(
            "GpuNode size: {} bytes",
            std::mem::size_of::<crate::GpuNode>()
        ));
        log(&format!(
            "Character size: {} bytes",
            std::mem::size_of::<crate::Character>()
        ));
        log(&format!(
            "GlyphData size: {} bytes",
            std::mem::size_of::<crate::GlyphData>()
        ));

        FlexRenderer {
            device,
            context,
            engine,
            depth_texture: None,
            depth_texture_width: 0,
            depth_texture_height: 0,
            nodes_buffer: None,
            characters_buffer: None,
            glyph_buffer: None,
            uniform_buffer: None,
            curve_buffer: None,
            glyph_info_buffer: None,
            class_defs_buffer: None,
            node_class_list_and_inline_styles_buffer: None,
            leaf_nodes_buffer: None,
            panic_buffer: None,
            panic_readback_buffer: None,
            pipeline_reset_signals: None,
            pipeline_bottom_up: None,
            pipeline_top_down: None,
            pipeline_height_bottom_up: None,
            pipeline_final_layout: None,
            pipeline_render: None,
            pipeline_render_text: None,
            pipeline_resolve_styles: None,
            pipeline_inherit_styles: None,

            bind_group_compute: None,
            bind_group_render: None,
            bind_group_layout_compute: None,
            bind_group_layout_render: None,
            atlas_texture: None,
            atlas_width: 0,
            atlas_height: 0,
            sampler: None,
            readback_in_progress: Rc::new(std::cell::Cell::new(false)),
            panic_readback_in_progress: Rc::new(std::cell::Cell::new(false)),
        }
    }
}

#[wasm_bindgen]
impl FlexRenderer {
    pub fn init(&mut self) -> Result<(), wasm_bindgen::JsValue> {
        // log("Rust Renderer Initializing...");

        // UI building is now handled externally before passing engine
        // crate::ui::build_ui(self.engine.clone());

        let engine = self.engine.borrow();
        const USAGE_STORAGE: u32 = 0x0080;
        const USAGE_COPY_DST: u32 = 0x0008;
        const USAGE_COPY_SRC: u32 = 0x0004;
        const USAGE_UNIFORM: u32 = 0x0040;
        const USAGE_MAP_READ: u32 = 0x0001;

        let node_count = engine.get_node_count() as u32;
        let node_size = engine.get_node_size() as u32;

        log(&format!(
            "Renderer init: node_count={}, node_size={}",
            node_count, node_size
        ));
        let nodes_byte_length = if node_count == 0 {
            4
        } else {
            node_count * node_size
        };

        self.nodes_buffer = Some(self.device.create_buffer(&GpuBufferDescriptor::new(
            nodes_byte_length as f64,
            USAGE_STORAGE | USAGE_COPY_DST | USAGE_COPY_SRC,
        )));

        let char_count = engine.get_character_count() as u32;
        let char_size = engine.get_character_size() as u32;
        let chars_byte_length = char_count * char_size;
        let chars_alloc_size = if chars_byte_length == 0 {
            4
        } else {
            chars_byte_length
        };

        let chars_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            chars_alloc_size as f64,
            USAGE_STORAGE | USAGE_COPY_DST | USAGE_COPY_SRC,
        ));

        if char_count > 0 {
            let engine = self.engine.borrow();
            let char_data = unsafe {
                std::slice::from_raw_parts(
                    engine.characters.as_ptr() as *const u8,
                    engine.characters.len() * std::mem::size_of::<crate::Character>(),
                )
            };
            self.device
                .queue()
                .write_buffer_with_u8_array(&chars_buf, 0.0, char_data);
        }
        self.characters_buffer = Some(chars_buf);

        let glyph_count = engine.get_glyph_data_count() as u32;
        let glyph_size = engine.get_glyph_data_size() as u32;
        let glyph_byte_length = glyph_count * glyph_size;
        let glyph_alloc_size = if glyph_byte_length == 0 {
            4
        } else {
            glyph_byte_length
        };

        let glyph_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            glyph_alloc_size as f64,
            USAGE_STORAGE | USAGE_COPY_DST,
        ));

        if glyph_count > 0 {
            let engine = self.engine.borrow();
            let glyph_data = unsafe {
                std::slice::from_raw_parts(
                    engine.glyph_data.as_ptr() as *const u8,
                    engine.glyph_data.len() * std::mem::size_of::<crate::GlyphData>(),
                )
            };
            self.device
                .queue()
                .write_buffer_with_u8_array(&glyph_buf, 0.0, glyph_data);
        }
        self.glyph_buffer = Some(glyph_buf);

        self.uniform_buffer = Some(self.device.create_buffer(&GpuBufferDescriptor::new(
            32.0, // Increased to 32 bytes for alignment (was 16)
            USAGE_UNIFORM | USAGE_COPY_DST,
        )));

        let curve_data = engine.get_curve_buffer();
        let curve_alloc_size = if curve_data.byte_length() == 0 {
            4
        } else {
            curve_data.byte_length()
        };
        let curve_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            curve_alloc_size as f64,
            USAGE_STORAGE | USAGE_COPY_DST | USAGE_COPY_SRC,
        ));
        if curve_data.byte_length() > 0 {
            let engine = self.engine.borrow();
            let curve_slice = unsafe {
                std::slice::from_raw_parts(
                    engine.curves.as_ptr() as *const u8,
                    engine.curves.len() * std::mem::size_of::<crate::GpuCurve>(),
                )
            };
            self.device
                .queue()
                .write_buffer_with_u8_array(&curve_buf, 0.0, curve_slice);
        }
        self.curve_buffer = Some(curve_buf);

        let glyph_info_data = engine.get_glyph_info_buffer();
        let info_alloc_size = if glyph_info_data.byte_length() == 0 {
            4
        } else {
            glyph_info_data.byte_length()
        };
        let info_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            info_alloc_size as f64,
            USAGE_STORAGE | USAGE_COPY_DST | USAGE_COPY_SRC,
        ));
        if glyph_info_data.byte_length() > 0 {
            let engine = self.engine.borrow();
            let info_slice = unsafe {
                std::slice::from_raw_parts(
                    engine.glyph_infos.as_ptr() as *const u8,
                    engine.glyph_infos.len() * std::mem::size_of::<crate::GpuGlyphInfo>(),
                )
            };
            self.device
                .queue()
                .write_buffer_with_u8_array(&info_buf, 0.0, info_slice);
        }
        self.glyph_info_buffer = Some(info_buf);

        // Class Defs Buffer
        let class_defs_data = engine.get_class_defs_buffer();
        let class_defs_alloc = if class_defs_data.byte_length() == 0 {
            4
        } else {
            class_defs_data.byte_length()
        };
        let class_defs_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            class_defs_alloc as f64,
            USAGE_STORAGE | USAGE_COPY_DST,
        ));
        if class_defs_data.byte_length() > 0 {
            let engine = self.engine.borrow();
            let class_defs_slice = unsafe {
                std::slice::from_raw_parts(
                    engine.class_defs.as_ptr() as *const u8,
                    engine.class_defs.len() * std::mem::size_of::<u32>(),
                )
            };
            self.device
                .queue()
                .write_buffer_with_u8_array(&class_defs_buf, 0.0, class_defs_slice);
        }
        self.class_defs_buffer = Some(class_defs_buf);

        // Node Class List and Inline Styles Buffer
        let ncl_data = engine.get_node_class_list_and_inline_styles_buffer();
        let ncl_alloc = if ncl_data.byte_length() == 0 {
            4
        } else {
            ncl_data.byte_length()
        };
        let ncl_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            ncl_alloc as f64,
            USAGE_STORAGE | USAGE_COPY_DST,
        ));
        if ncl_data.byte_length() > 0 {
            let engine = self.engine.borrow();
            let ncl_slice = unsafe {
                std::slice::from_raw_parts(
                    engine.node_class_list_and_inline_styles.as_ptr() as *const u8,
                    engine.node_class_list_and_inline_styles.len() * std::mem::size_of::<u32>(),
                )
            };
            self.device
                .queue()
                .write_buffer_with_u8_array(&ncl_buf, 0.0, ncl_slice);
        }
        self.node_class_list_and_inline_styles_buffer = Some(ncl_buf);

        // Panic Buffer (1 * u32)
        let panic_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            4.0,
            USAGE_STORAGE | USAGE_COPY_SRC | USAGE_COPY_DST,
        ));
        self.panic_buffer = Some(panic_buf);

        let panic_read_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            4.0,
            USAGE_MAP_READ | USAGE_COPY_DST,
        ));
        self.panic_readback_buffer = Some(panic_read_buf);

        // Leaf Nodes Buffer
        let leaf_count = engine.leaf_nodes.len() as u32;
        let leaf_alloc = if leaf_count == 0 { 4 } else { leaf_count * 4 };
        let leaf_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
            leaf_alloc as f64,
            USAGE_STORAGE | USAGE_COPY_DST,
        ));
        if leaf_count > 0 {
            let leaf_slice = unsafe {
                std::slice::from_raw_parts(
                    engine.leaf_nodes.as_ptr() as *const u8,
                    engine.leaf_nodes.len() * 4,
                )
            };
            self.device
                .queue()
                .write_buffer_with_u8_array(&leaf_buf, 0.0, leaf_slice);
        }
        self.leaf_nodes_buffer = Some(leaf_buf);

        // Prepend generated WGSL constants to compute shader
        let full_compute_source = format!("{}\n{}", SHADER_STYLE_CONSTANTS, SHADER_COMPUTE);
        let module_compute = self
            .device
            .create_shader_module(&GpuShaderModuleDescriptor::new(&full_compute_source));
        let module_visual = self
            .device
            .create_shader_module(&GpuShaderModuleDescriptor::new(SHADER_VISUAL));

        let make_layout_entry =
            |binding: u32, visibility: u32, type_: GpuBufferBindingType| -> js_sys::Object {
                let layout_entry = GpuBindGroupLayoutEntry::new(binding, visibility);
                let buffer_layout = GpuBufferBindingLayout::new();
                Reflect::set(&buffer_layout, &"type".into(), &type_.as_str().into()).unwrap();
                Reflect::set(&layout_entry, &"buffer".into(), &buffer_layout).unwrap();
                layout_entry
            };

        let entries_compute = js_sys::Array::new();
        entries_compute.push(&make_layout_entry(
            0,
            GpuShaderStage::COMPUTE,
            GpuBufferBindingType::Storage,
        ));
        entries_compute.push(&make_layout_entry(
            1,
            GpuShaderStage::COMPUTE,
            GpuBufferBindingType::Uniform,
        ));
        entries_compute.push(&make_layout_entry(
            2,
            GpuShaderStage::COMPUTE,
            GpuBufferBindingType::Storage,
        ));
        entries_compute.push(&make_layout_entry(
            3,
            GpuShaderStage::COMPUTE,
            GpuBufferBindingType::ReadOnlyStorage,
        ));
        entries_compute.push(&make_layout_entry(
            4,
            GpuShaderStage::COMPUTE,
            GpuBufferBindingType::ReadOnlyStorage,
        ));
        entries_compute.push(&make_layout_entry(
            5,
            GpuShaderStage::COMPUTE,
            GpuBufferBindingType::ReadOnlyStorage,
        ));
        entries_compute.push(&make_layout_entry(
            6,
            GpuShaderStage::COMPUTE,
            GpuBufferBindingType::Storage,
        )); // Panic
        entries_compute.push(&make_layout_entry(
            7,
            GpuShaderStage::COMPUTE,
            GpuBufferBindingType::ReadOnlyStorage,
        )); // Leaf Nodes
        self.bind_group_layout_compute = Some(
            self.device
                .create_bind_group_layout(&GpuBindGroupLayoutDescriptor::new(&entries_compute)),
        );

        let layout_entries = js_sys::Array::new();
        layout_entries.push(&make_layout_entry(
            0,
            GpuShaderStage::VERTEX | GpuShaderStage::FRAGMENT,
            GpuBufferBindingType::ReadOnlyStorage,
        ));
        layout_entries.push(&make_layout_entry(
            1,
            GpuShaderStage::VERTEX,
            GpuBufferBindingType::Uniform,
        ));
        layout_entries.push(&make_layout_entry(
            2,
            GpuShaderStage::VERTEX,
            GpuBufferBindingType::ReadOnlyStorage,
        ));
        layout_entries.push(&make_layout_entry(
            3,
            GpuShaderStage::FRAGMENT,
            GpuBufferBindingType::ReadOnlyStorage,
        ));
        layout_entries.push(&make_layout_entry(
            4,
            GpuShaderStage::FRAGMENT,
            GpuBufferBindingType::ReadOnlyStorage,
        ));
        layout_entries.push(&GpuBindGroupLayoutEntry::new_texture(
            5,
            GpuShaderStage::FRAGMENT,
        )); // Texture
        layout_entries.push(&GpuBindGroupLayoutEntry::new_sampler(
            6,
            GpuShaderStage::FRAGMENT,
        )); // Sampler

        self.bind_group_layout_render = Some(
            self.device
                .create_bind_group_layout(&GpuBindGroupLayoutDescriptor::new(&layout_entries)),
        );

        let layout_compute = self
            .device
            .create_pipeline_layout(&GpuPipelineLayoutDescriptor::new(&js_sys::Array::of1(
                self.bind_group_layout_compute.as_ref().unwrap(),
            )));
        let layout_render = self
            .device
            .create_pipeline_layout(&GpuPipelineLayoutDescriptor::new(&js_sys::Array::of1(
                self.bind_group_layout_render.as_ref().unwrap(),
            )));

        let create_compute = |entry: &str| -> GpuComputePipeline {
            let stage = js_sys::Object::new();
            Reflect::set(&stage, &"module".into(), &module_compute).unwrap();
            Reflect::set(&stage, &"entryPoint".into(), &entry.into()).unwrap();
            let desc = js_sys::Object::new();
            Reflect::set(&desc, &"layout".into(), &layout_compute).unwrap();
            Reflect::set(&desc, &"compute".into(), &stage).unwrap();
            self.device.create_compute_pipeline(&desc.unchecked_into())
        };

        self.pipeline_reset_signals = Some(create_compute("reset_signals"));
        self.pipeline_resolve_styles = Some(create_compute("resolve_styles"));
        self.pipeline_inherit_styles = Some(create_compute("inherit_styles"));
        self.pipeline_bottom_up = Some(create_compute("width_bottom_up"));
        self.pipeline_top_down = Some(create_compute("width_top_down"));
        self.pipeline_height_bottom_up = Some(create_compute("height_bottom_up"));
        self.pipeline_final_layout = Some(create_compute("final_layout"));

        let create_render = |vs_entry: &str, fs_entry: &str| -> GpuRenderPipeline {
            let vs_state_obj = js_sys::Object::new();
            Reflect::set(&vs_state_obj, &"module".into(), &module_visual).unwrap();
            Reflect::set(&vs_state_obj, &"entryPoint".into(), &vs_entry.into()).unwrap();
            let target = GpuColorTargetState::new(GpuTextureFormat::Bgra8unorm);
            let blend = js_sys::Object::new();
            let color_blend = js_sys::Object::new();
            Reflect::set(&color_blend, &"srcFactor".into(), &"src-alpha".into()).unwrap();
            Reflect::set(
                &color_blend,
                &"dstFactor".into(),
                &"one-minus-src-alpha".into(),
            )
            .unwrap();
            Reflect::set(&color_blend, &"operation".into(), &"add".into()).unwrap();
            let alpha_blend = js_sys::Object::new();
            Reflect::set(&alpha_blend, &"srcFactor".into(), &"one".into()).unwrap();
            Reflect::set(
                &alpha_blend,
                &"dstFactor".into(),
                &"one-minus-src-alpha".into(),
            )
            .unwrap();
            Reflect::set(&alpha_blend, &"operation".into(), &"add".into()).unwrap();
            Reflect::set(&blend, &"color".into(), &color_blend).unwrap();
            Reflect::set(&blend, &"alpha".into(), &alpha_blend).unwrap();
            Reflect::set(&target, &"blend".into(), &blend).unwrap();
            let fs_state =
                GpuFragmentState::new(fs_entry, &module_visual, &js_sys::Array::of1(&target));
            let depth_state = js_sys::Object::new();
            Reflect::set(&depth_state, &"format".into(), &"depth24plus".into()).unwrap();
            Reflect::set(&depth_state, &"depthWriteEnabled".into(), &true.into()).unwrap();
            Reflect::set(&depth_state, &"depthCompare".into(), &"less-equal".into()).unwrap();
            let desc = js_sys::Object::new();
            Reflect::set(&desc, &"layout".into(), &layout_render).unwrap();
            Reflect::set(&desc, &"vertex".into(), &vs_state_obj).unwrap();
            Reflect::set(&desc, &"fragment".into(), &fs_state).unwrap();
            Reflect::set(&desc, &"depthStencil".into(), &depth_state).unwrap();
            let prim = js_sys::Object::new();
            Reflect::set(&prim, &"topology".into(), &"triangle-list".into()).unwrap();
            Reflect::set(&desc, &"primitive".into(), &prim).unwrap();
            self.device.create_render_pipeline(&desc.unchecked_into())
        };

        self.pipeline_render = Some(create_render("vs_main", "fs_main"));
        self.pipeline_render_text = Some(create_render("vs_text", "fs_text"));

        // Texture & Sampler Setup
        self.sampler = Some(
            self.device
                .create_sampler(Some(GpuSamplerDescriptor::new())),
        );

        // create placeholder texture
        let size = js_sys::Array::of2(&1u32.into(), &1u32.into());
        let tex_desc = GpuTextureDescriptor::new(
            &size,
            "rgba8unorm",
            GpuTextureUsage::TEXTURE_BINDING
                | GpuTextureUsage::COPY_DST
                | GpuTextureUsage::RENDER_ATTACHMENT,
        );
        let placeholder_tex = self.device.create_texture(&tex_desc);

        // Upload white pixel
        let white_pixel = [255u8, 255u8, 255u8, 255u8];
        let layout = GpuImageDataLayout::new(4, 1);
        let extent = GpuExtent3D::new(1, 1);
        let dest = GpuImageCopyTexture::new(&placeholder_tex);
        self.device
            .queue()
            .write_texture_with_u8_array(&dest, &white_pixel, &layout, &extent);

        self.atlas_texture = Some(placeholder_tex);
        self.atlas_width = 1;
        self.atlas_height = 1;

        let make_buffer_binding = |buffer: &GpuBuffer| -> Object {
            let obj = Object::new();
            Reflect::set(&obj, &"buffer".into(), buffer).unwrap();
            obj
        };

        self.bind_group_compute =
            Some(self.device.create_bind_group(&GpuBindGroupDescriptor::new(
                &{
                    let entries = js_sys::Array::new();
                    entries.push(&GpuBindGroupEntry::new(
                        0,
                        &make_buffer_binding(self.nodes_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        1,
                        &make_buffer_binding(self.uniform_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        2,
                        &make_buffer_binding(self.characters_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        3,
                        &make_buffer_binding(self.glyph_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        4,
                        &make_buffer_binding(self.class_defs_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        5,
                        &make_buffer_binding(
                            self.node_class_list_and_inline_styles_buffer
                                .as_ref()
                                .unwrap(),
                        ),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        6,
                        &make_buffer_binding(self.panic_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        7,
                        &make_buffer_binding(self.leaf_nodes_buffer.as_ref().unwrap()),
                    ));
                    entries
                },
                self.bind_group_layout_compute.as_ref().unwrap(),
            )));

        let entries = js_sys::Array::new();
        entries.push(&GpuBindGroupEntry::new(
            0,
            &make_buffer_binding(self.nodes_buffer.as_ref().unwrap()),
        ));
        entries.push(&GpuBindGroupEntry::new(
            1,
            &make_buffer_binding(self.uniform_buffer.as_ref().unwrap()),
        ));
        entries.push(&GpuBindGroupEntry::new(
            2,
            &make_buffer_binding(self.characters_buffer.as_ref().unwrap()),
        ));
        entries.push(&GpuBindGroupEntry::new(
            3,
            &make_buffer_binding(self.curve_buffer.as_ref().unwrap()),
        ));
        entries.push(&GpuBindGroupEntry::new(
            4,
            &make_buffer_binding(self.glyph_info_buffer.as_ref().unwrap()),
        ));

        let tex_view: Object = self.atlas_texture.as_ref().unwrap().create_view().into();
        entries.push(&GpuBindGroupEntry::new(5, &tex_view));

        let sampler: Object = self.sampler.as_ref().unwrap().clone().into();
        entries.push(&GpuBindGroupEntry::new(6, &sampler));

        self.bind_group_render = Some(self.device.create_bind_group(&GpuBindGroupDescriptor::new(
            &entries,
            self.bind_group_layout_render.as_ref().unwrap(),
        )));

        // log("Rust Renderer Init Complete");
        drop(engine);
        self.engine.borrow_mut().mark_dirty();
        Ok(())
    }

    pub fn render(&mut self) {
        let window_value = get_window().unwrap();
        let dpr = window_value.device_pixel_ratio();

        {
            let mut engine = self.engine.borrow_mut();
            engine.device_pixel_ratio = dpr as f32;
        }

        if !self.engine.borrow().is_dirty() {
            // log("Render: skipping, no dirty");
            return;
        }
        // crate::log("--- FRAME START ---");

        self.engine.borrow_mut().render();

        if self.nodes_buffer.is_none() {
            // log("Render: skipping, no buffers");
            return;
        }

        let canvas: HtmlCanvasElement = self.context.canvas();
        let width = canvas.width() as f32 / dpr as f32;
        let height = canvas.height() as f32 / dpr as f32;

        let uniform_bytes: Vec<u8> = {
            let engine = self.engine.borrow();
            let ascender = engine.get_ascender();
            let line_gap = engine.get_line_gap();
            let node_count = engine.get_node_count() as f32;
            let _descender = engine.get_descender();
            let line_height = ascender - _descender + line_gap;
            let leaf_count = engine.leaf_nodes.len() as f32;

            // Pad to 32 bytes (8 floats)
            let uniform_data = vec![
                width,
                height,
                ascender,
                line_height,
                node_count,
                leaf_count,
                0.0,
                0.0,
            ];
            unsafe {
                std::slice::from_raw_parts(
                    uniform_data.as_ptr() as *const u8,
                    uniform_data.len() * 4,
                )
                .to_vec()
            }
        };

        self.device.queue().write_buffer_with_u8_array(
            self.uniform_buffer.as_ref().unwrap(),
            0.0,
            &uniform_bytes,
        );

        let node_count: u32;
        let char_count: u32;

        // Perform potentially destructive buffer updates (resizes)
        {
            let engine = self.engine.borrow();
            node_count = engine.get_node_count() as u32;
            let node_size = engine.get_node_size() as u32;
            let nodes_byte_length = node_count * node_size;
            let current_nodes_buffer = self.nodes_buffer.as_ref().unwrap();
            let nodes_buffer_size = Reflect::get(current_nodes_buffer, &"size".into())
                .unwrap()
                .as_f64()
                .unwrap_or(0.0) as u32;

            if nodes_byte_length > nodes_buffer_size {
                // log(&format!("Resizing nodes_buffer from {} to {}", nodes_buffer_size, nodes_byte_length));
                if let Some(old) = &self.nodes_buffer {
                    old.destroy();
                }
                let new_nodes_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
                    nodes_byte_length as f64,
                    0x0080 | 0x0008 | 0x0004,
                ));
                self.nodes_buffer = Some(new_nodes_buf);
                drop(engine);
                self.rebind_all();
            }
        }

        // Write Node data
        {
            let engine = self.engine.borrow();
            let nodes_slice = unsafe {
                std::slice::from_raw_parts(
                    engine.gpu_nodes.as_ptr() as *const u8,
                    engine.gpu_nodes.len() * std::mem::size_of::<crate::GpuNode>(),
                )
            };
            self.device.queue().write_buffer_with_u8_array(
                self.nodes_buffer.as_ref().unwrap(),
                0.0,
                nodes_slice,
            );
        }

        // Update Characters
        {
            let engine = self.engine.borrow();
            char_count = engine.get_character_count() as u32;
            let char_size = engine.get_character_size() as u32;
            let chars_byte_length = char_count * char_size;

            if char_count > 0 {
                let chars_slice = unsafe {
                    std::slice::from_raw_parts(
                        engine.characters.as_ptr() as *const u8,
                        engine.characters.len() * std::mem::size_of::<crate::Character>(),
                    )
                };
                if let Some(old) = &self.characters_buffer {
                    old.destroy();
                }
                let new_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
                    (if chars_byte_length == 0 {
                        4
                    } else {
                        chars_byte_length
                    }) as f64,
                    0x0080 | 0x0008 | 0x0004,
                ));
                self.device
                    .queue()
                    .write_buffer_with_u8_array(&new_buf, 0.0, chars_slice);
                self.characters_buffer = Some(new_buf);
                drop(engine);
                self.rebind_all();
            }
        }

        // Update Leaf Nodes
        let leaf_count: u32;
        {
            let engine = self.engine.borrow();
            leaf_count = engine.leaf_nodes.len() as u32;
            let leaf_byte_length = leaf_count * 4;

            if leaf_count > 0 {
                let leaf_slice = unsafe {
                    std::slice::from_raw_parts(
                        engine.leaf_nodes.as_ptr() as *const u8,
                        engine.leaf_nodes.len() * 4,
                    )
                };

                let current_leaf_buffer = self.leaf_nodes_buffer.as_ref().unwrap();
                let current_size = Reflect::get(current_leaf_buffer, &"size".into())
                    .unwrap()
                    .as_f64()
                    .unwrap_or(0.0) as u32;

                if leaf_byte_length > current_size {
                    if let Some(old) = &self.leaf_nodes_buffer {
                        old.destroy();
                    }
                    let new_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
                        leaf_byte_length as f64,
                        0x0080 | 0x0008 | 0x0004,
                    ));
                    self.leaf_nodes_buffer = Some(new_buf);
                    drop(engine);
                    self.rebind_all();
                }

                let engine = self.engine.borrow();
                self.device.queue().write_buffer_with_u8_array(
                    self.leaf_nodes_buffer.as_ref().unwrap(),
                    0.0,
                    leaf_slice,
                );
            }
        }

        // Zero out panic buffer
        let zero = [0u8; 4];
        self.device.queue().write_buffer_with_u8_array(
            self.panic_buffer.as_ref().unwrap(),
            0.0,
            &zero,
        );

        // Update Curves
        {
            let engine = self.engine.borrow();
            let curve_data = engine.get_curve_buffer();
            let curve_byte_length = curve_data.byte_length();

            if curve_byte_length > 0 {
                let current_curve_buffer = self.curve_buffer.as_ref().unwrap();
                let current_size = Reflect::get(current_curve_buffer, &"size".into())
                    .unwrap()
                    .as_f64()
                    .unwrap_or(0.0) as u32;

                if curve_byte_length > current_size {
                    if let Some(old) = &self.curve_buffer {
                        old.destroy();
                    }
                    let new_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
                        curve_byte_length as f64,
                        0x0080 | 0x0008 | 0x0004,
                    ));
                    self.curve_buffer = Some(new_buf);
                    drop(engine);
                    self.rebind_all();
                } else {
                    drop(engine);
                }

                let engine = self.engine.borrow();
                let curve_slice = unsafe {
                    std::slice::from_raw_parts(
                        engine.curves.as_ptr() as *const u8,
                        engine.curves.len() * std::mem::size_of::<crate::GpuCurve>(),
                    )
                };
                self.device.queue().write_buffer_with_u8_array(
                    self.curve_buffer.as_ref().unwrap(),
                    0.0,
                    curve_slice,
                );
            }
        }

        // Update Glyph Infos
        {
            let engine = self.engine.borrow();
            let info_data = engine.get_glyph_info_buffer();
            let info_byte_length = info_data.byte_length();

            if info_byte_length > 0 {
                let current_info_buffer = self.glyph_info_buffer.as_ref().unwrap();
                let current_size = Reflect::get(current_info_buffer, &"size".into())
                    .unwrap()
                    .as_f64()
                    .unwrap_or(0.0) as u32;

                if info_byte_length > current_size {
                    if let Some(old) = &self.glyph_info_buffer {
                        old.destroy();
                    }
                    let new_buf = self.device.create_buffer(&GpuBufferDescriptor::new(
                        info_byte_length as f64,
                        0x0080 | 0x0008 | 0x0004,
                    ));
                    self.glyph_info_buffer = Some(new_buf);
                    drop(engine);
                    self.rebind_all();
                } else {
                    drop(engine);
                }

                let engine = self.engine.borrow();
                let info_slice = unsafe {
                    std::slice::from_raw_parts(
                        engine.glyph_infos.as_ptr() as *const u8,
                        engine.glyph_infos.len() * std::mem::size_of::<crate::GpuGlyphInfo>(),
                    )
                };
                self.device.queue().write_buffer_with_u8_array(
                    self.glyph_info_buffer.as_ref().unwrap(),
                    0.0,
                    info_slice,
                );
            }
        }

        // Texture Updates (Texture Atlas)
        let mut rebind_needed = false;
        {
            let mut engine = self.engine.borrow_mut();
            engine.texture_atlas.process_deletions();

            if engine.texture_atlas.dirty {
                let atlas = &mut engine.texture_atlas;
                let needed_width = atlas.width;
                let needed_height = atlas.height;

                // Check if current texture matches needed size
                let mut recreate = false;
                if let Some(_tex) = &self.atlas_texture {
                    if self.atlas_width != needed_width || self.atlas_height != needed_height {
                        recreate = true;
                    }
                } else {
                    recreate = true;
                }

                if recreate {
                    if let Some(old) = &self.atlas_texture {
                        let old_tex: &GpuTexture = old;
                        old_tex.destroy();
                    }

                    let size = js_sys::Array::of2(&needed_width.into(), &needed_height.into());
                    let tex_desc = GpuTextureDescriptor::new(
                        &size,
                        "rgba8unorm",
                        GpuTextureUsage::TEXTURE_BINDING
                            | GpuTextureUsage::COPY_DST
                            | GpuTextureUsage::RENDER_ATTACHMENT,
                    );
                    self.atlas_texture = Some(self.device.create_texture(&tex_desc));
                    self.atlas_width = needed_width;
                    self.atlas_height = needed_height;
                    rebind_needed = true;
                }

                // Upload Pending Regions
                // Note: If we recreated the texture, do we need to re-upload *everything*?
                // Yes. The new texture is empty.
                // But `pending_uploads` only contains *new* stuff.
                // If we recreated, we need the *entire* atlas data.
                // `TextureAtlas` has `data` (shadow copy).
                // If `recreate` is true, we should upload the *whole* `atlas.data`.

                if recreate {
                    // We recreated the texture (it's empty).
                    // We must upload whatever is in pending_uploads.
                    // Note: If we had "old" data that wasn't pending, it is LOST here because we removed shadow copy.
                    // This is the tradeoff.
                    // However, since we only recreate on startup (or if we implemented resize), startup is fine.
                    // If we resize, we need to re-add all assets to valid regions.
                    // For now, just process pending uploads below.
                }
                // Always upload pending updates
                let tex = self.atlas_texture.as_ref().unwrap();
                for (region, pixels) in atlas.pending_uploads.drain(..) {
                    let layout = GpuImageDataLayout::new(region.width * 4, region.height);
                    let extent = GpuExtent3D::new(region.width, region.height);
                    let dest = GpuImageCopyTexture::new(tex);

                    let origin = js_sys::Array::of3(&region.x.into(), &region.y.into(), &0.into());
                    Reflect::set(&dest, &"origin".into(), &origin).unwrap();

                    self.device
                        .queue()
                        .write_texture_with_u8_array(&dest, &pixels, &layout, &extent);
                }

                atlas.dirty = false;
            }
        }

        if rebind_needed {
            self.rebind_all();
        }
        // Update Class Buffers
        {
            let engine = self.engine.borrow();
            let class_defs_data = engine.get_class_defs_buffer();
            let ncl_data = engine.get_node_class_list_and_inline_styles_buffer();

            let cd_byte_len = class_defs_data.byte_length();
            let ncl_byte_len = ncl_data.byte_length();

            let mut need_rebind = false;

            // Resize class_defs_buffer if needed
            if cd_byte_len > 0 {
                let current_size =
                    Reflect::get(self.class_defs_buffer.as_ref().unwrap(), &"size".into())
                        .unwrap()
                        .as_f64()
                        .unwrap_or(0.0) as u32;
                if cd_byte_len > current_size {
                    if let Some(old) = &self.class_defs_buffer {
                        old.destroy();
                    }
                    self.class_defs_buffer = Some(self.device.create_buffer(
                        &GpuBufferDescriptor::new(cd_byte_len as f64, 0x0080 | 0x0008),
                    ));
                    need_rebind = true;
                }
                let class_defs_slice = unsafe {
                    std::slice::from_raw_parts(
                        engine.class_defs.as_ptr() as *const u8,
                        engine.class_defs.len() * std::mem::size_of::<u32>(),
                    )
                };
                self.device.queue().write_buffer_with_u8_array(
                    self.class_defs_buffer.as_ref().unwrap(),
                    0.0,
                    class_defs_slice,
                );
            }

            // Resize/Sync node_class_list_and_inline_styles_buffer
            if ncl_byte_len > 0 {
                let current_size = Reflect::get(
                    self.node_class_list_and_inline_styles_buffer
                        .as_ref()
                        .unwrap(),
                    &"size".into(),
                )
                .unwrap()
                .as_f64()
                .unwrap_or(0.0) as u32;
                if ncl_byte_len > current_size {
                    if let Some(old) = &self.node_class_list_and_inline_styles_buffer {
                        old.destroy();
                    }
                    self.node_class_list_and_inline_styles_buffer =
                        Some(self.device.create_buffer(&GpuBufferDescriptor::new(
                            ncl_byte_len as f64,
                            0x0080 | 0x0008,
                        )));
                    need_rebind = true;
                }
                let ncl_slice = unsafe {
                    std::slice::from_raw_parts(
                        engine.node_class_list_and_inline_styles.as_ptr() as *const u8,
                        engine.node_class_list_and_inline_styles.len() * std::mem::size_of::<u32>(),
                    )
                };
                self.device.queue().write_buffer_with_u8_array(
                    self.node_class_list_and_inline_styles_buffer
                        .as_ref()
                        .unwrap(),
                    0.0,
                    ncl_slice,
                );
            }

            drop(engine);
            if need_rebind {
                self.rebind_all();
            }
        }

        let command_encoder = self.device.create_command_encoder();
        let workgroups = (node_count as f32 / 64.0).ceil() as u32;
        let leaf_workgroups = (leaf_count as f32 / 64.0).ceil() as u32;
        let dispatch = |pass: &GpuComputePassEncoder, x: u32| {
            pass.dispatchWorkgroups(x, 1, 1);
        };

        let end_compute = |pass: &GpuComputePassEncoder| {
            pass.end_compute();
        };

        // PASS 0: Resolve Styles
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_resolve_styles.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }

        // PASS 0.1: Inherit Styles
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_reset_signals.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }
        for _ in 0..16 {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_inherit_styles.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }

        // PASS 1: Width Bottom-Up
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_reset_signals.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }
        for _ in 0..64 {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_bottom_up.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, leaf_workgroups);
            end_compute(&pass);
        }

        // PASS 2: Width Top-Down
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_reset_signals.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }
        for _ in 0..64 {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_top_down.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }

        // PASS 3: Height Bottom-Up
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_reset_signals.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }
        for _ in 0..64 {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_height_bottom_up.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, leaf_workgroups);
            end_compute(&pass);
        }

        // PASS 4: Final Layout
        {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_reset_signals.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }
        for _ in 0..64 {
            let pass = command_encoder.begin_compute_pass();
            pass.set_pipeline_compute(self.pipeline_final_layout.as_ref().unwrap());
            pass.set_bind_group_compute(0, self.bind_group_compute.as_ref().unwrap());
            dispatch(&pass, workgroups);
            end_compute(&pass);
        }

        // --- Post-Layout Cleanup & Panic Check ---
        let panic_check_this_frame = !self.panic_readback_in_progress.get();
        if panic_check_this_frame {
            command_encoder.copy_buffer_to_buffer(
                self.panic_buffer.as_ref().unwrap(),
                0.0,
                self.panic_readback_buffer.as_ref().unwrap(),
                0.0,
                4.0,
            );
        }

        let canvas = self.context.canvas();
        let texture_view = self.context.get_current_texture().create_view();
        let canvas_width = canvas.width();
        let canvas_height = canvas.height();

        if self.depth_texture.is_none()
            || self.depth_texture_width != canvas_width
            || self.depth_texture_height != canvas_height
        {
            if let Some(texture) = &self.depth_texture {
                let t: &GpuTexture = texture;
                t.destroy();
            }
            let depth_desc = js_sys::Object::new();
            let size = js_sys::Array::of2(&canvas_width.into(), &canvas_height.into());
            Reflect::set(&depth_desc, &"size".into(), &size).unwrap();
            Reflect::set(&depth_desc, &"format".into(), &"depth24plus".into()).unwrap();
            Reflect::set(
                &depth_desc,
                &"usage".into(),
                &GpuTextureUsage::RENDER_ATTACHMENT.into(),
            )
            .unwrap();
            self.depth_texture = Some(self.device.create_texture(&depth_desc));
            self.depth_texture_width = canvas_width;
            self.depth_texture_height = canvas_height;
        }

        let depth_view = self.depth_texture.as_ref().unwrap().create_view();
        let depth_attachment_obj = js_sys::Object::new();
        Reflect::set(&depth_attachment_obj, &"view".into(), &depth_view).unwrap();
        Reflect::set(
            &depth_attachment_obj,
            &"depthLoadOp".into(),
            &"clear".into(),
        )
        .unwrap();
        Reflect::set(
            &depth_attachment_obj,
            &"depthStoreOp".into(),
            &"store".into(),
        )
        .unwrap();
        Reflect::set(
            &depth_attachment_obj,
            &"depthClearValue".into(),
            &1.0.into(),
        )
        .unwrap();

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

        let render_pass_desc =
            GpuRenderPassDescriptor::new(&js_sys::Array::of1(&color_attachment_obj));
        Reflect::set(
            &render_pass_desc,
            &"depthStencilAttachment".into(),
            &depth_attachment_obj,
        )
        .unwrap();

        let render_pass = command_encoder.begin_render_pass(&render_pass_desc);
        render_pass.set_pipeline_render(self.pipeline_render.as_ref().unwrap());
        render_pass.set_bind_group_render(0, self.bind_group_render.as_ref().unwrap());
        render_pass.draw_with_instance_count(6, node_count, 0, 0);

        if char_count > 0 {
            render_pass.set_pipeline_render(self.pipeline_render_text.as_ref().unwrap());
            render_pass.set_bind_group_render(0, self.bind_group_render.as_ref().unwrap());
            render_pass.draw_with_instance_count(6, char_count, 0, 0);
        }

        render_pass.end_render();
        self.device
            .queue()
            .submit(&js_sys::Array::of1(&command_encoder.finish()));
        self.engine.borrow_mut().mark_clean();

        // Trigger Panic Check (Async), but only when no panic readback is in-flight.
        if panic_check_this_frame {
            self.check_gpu_panic();
        }

        // PASS 5: Read-back Nodes for hit testing (deferred to end of frame)
        if !self.readback_in_progress.get() {
            self.readback_in_progress.set(true);
            let device = self.device.clone();
            let nodes_buffer = self.nodes_buffer.as_ref().unwrap().clone();
            let engine = self.engine.clone();
            let readback_flag = self.readback_in_progress.clone();

            let _ = wasm_bindgen_futures::future_to_promise(async move {
                let size_val = Reflect::get(&nodes_buffer, &"size".into())
                    .unwrap()
                    .as_f64()
                    .unwrap_or(0.0);
                if size_val > 0.0 {
                    let staging_buf =
                        device.create_buffer(&GpuBufferDescriptor::new(size_val, 0x0001 | 0x0008));
                    let encoder = device.create_command_encoder();
                    encoder.copy_buffer_to_buffer(&nodes_buffer, 0.0, &staging_buf, 0.0, size_val);
                    device
                        .queue()
                        .submit(&js_sys::Array::of1(&encoder.finish()));

                    let promise = staging_buf.map_async(0x0001); // MAP_READ
                    if let Ok(_) = wasm_bindgen_futures::JsFuture::from(promise).await {
                        let array_buffer = staging_buf.get_mapped_range();
                        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                        let vec = uint8_array.to_vec();
                        staging_buf.unmap();
                        staging_buf.destroy();

                        let node_size = std::mem::size_of::<crate::GpuNode>();
                        let count = vec.len() / node_size;
                        // crate::log(&format!("Readback complete: {} nodes", count));
                        let mut gpu_nodes = Vec::with_capacity(count);
                        for i in 0..count {
                            let offset = i * node_size;
                            let mut node = std::mem::MaybeUninit::<crate::GpuNode>::uninit();
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    vec.as_ptr().add(offset),
                                    node.as_mut_ptr() as *mut u8,
                                    node_size,
                                );
                            }
                            gpu_nodes.push(unsafe { node.assume_init() });
                        }
                        let hover_callbacks = {
                            let mut e = engine.borrow_mut();

                            // Request a follow-up frame when the readback snapshot changed.
                            // This keeps click-driven rendering while allowing SVG atlas keys
                            // (which depend on computed style/readback) to settle after view swaps.
                            let needs_follow_up = {
                                let prev = &e.hit_test_nodes;
                                prev.is_empty()
                                    || prev.len() != gpu_nodes.len()
                                    || prev.iter().zip(gpu_nodes.iter()).any(|(a, b)| {
                                        a.cpu_index != b.cpu_index
                                            || a.fill_color != b.fill_color
                                            || a.stroke_color != b.stroke_color
                                            || a.stroke_width.to_bits() != b.stroke_width.to_bits()
                                            || a.text_color_r.to_bits() != b.text_color_r.to_bits()
                                            || a.text_color_g.to_bits() != b.text_color_g.to_bits()
                                            || a.text_color_b.to_bits() != b.text_color_b.to_bits()
                                            || a.text_color_a.to_bits() != b.text_color_a.to_bits()
                                    })
                            };

                            e.hit_test_nodes = gpu_nodes;
                            let hover_callbacks = e.process_hover_from_latest_hit_test();

                            if needs_follow_up || !hover_callbacks.is_empty() {
                                e.mark_dirty();
                            }

                            hover_callbacks
                        };

                        for (cb, event) in hover_callbacks {
                            cb(event);
                        }
                    } else {
                        staging_buf.destroy();
                    }
                }
                readback_flag.set(false);
                Ok(JsValue::UNDEFINED)
            });
        }
    }

    pub fn debug(&self) -> Promise {
        let characters_buffer = self.characters_buffer.as_ref().unwrap().clone();
        let device = self.device.clone();
        Self::debug_internal(device, characters_buffer)
    }

    fn debug_internal(device: GpuDevice, characters_buffer: GpuBuffer) -> Promise {
        wasm_bindgen_futures::future_to_promise(async move {
            let size_val = Reflect::get(&characters_buffer, &"size".into())
                .unwrap()
                .as_f64()
                .unwrap_or(0.0);
            if size_val <= 0.0 {
                return Ok(wasm_bindgen::JsValue::UNDEFINED);
            }

            // Create staging buffer (WebGPU requires staging for MapRead on Storage)
            // 0x0001 = MAP_READ, 0x0008 = COPY_DST
            let staging_buf = device.create_buffer(&crate::web_bindings::GpuBufferDescriptor::new(
                size_val,
                0x0001 | 0x0008,
            ));

            let encoder = device.create_command_encoder();
            encoder.copy_buffer_to_buffer(&characters_buffer, 0.0, &staging_buf, 0.0, size_val);
            device
                .queue()
                .submit(&js_sys::Array::of1(&encoder.finish()));

            let promise = staging_buf.map_async(0x0001); // MAP_READ
            wasm_bindgen_futures::JsFuture::from(promise).await?;

            let array_buffer = staging_buf.get_mapped_range();
            let uint8_array = js_sys::Uint8Array::new(&array_buffer);
            let vec = uint8_array.to_vec();
            staging_buf.unmap();
            staging_buf.destroy();

            // Reconstruct Character structs from bytes
            let char_size = std::mem::size_of::<crate::Character>();
            let count = vec.len() / char_size;
            // log(&format!("--- GPU DEBUG: Character Read-back (count: {}) ---", count));

            for i in 0..count {
                let offset = i * char_size;
                if offset + char_size > vec.len() {
                    break;
                }

                // Safe alignment handling: copy to stack-aligned struct
                let mut c = std::mem::MaybeUninit::<crate::Character>::uninit();
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        vec.as_ptr().add(offset),
                        c.as_mut_ptr() as *mut u8,
                        char_size,
                    );
                }
                let c = unsafe { c.assume_init() };

                if c.value != 0 {
                    // log(&format!("--- DEBUG: Node {} text: {:?} ---", gpu_idx, cpu_node.text));
                    // log(&format!("  text_start: {}, text_length: {}", chars_start, chars_len));
                    // log(&format!("  Char[{}] val: '{}' (u32: {}), node: {}, x: {:.2}, y: {:.2}, w: {:.2}, h: {:.2}",
                    // i, std::char::from_u32(c.value).unwrap_or('?'), c.value, c.node_index, c.x, c.y, c.width, c.height));
                }
            }

            Ok(wasm_bindgen::JsValue::UNDEFINED)
        })
    }

    pub fn handle_click(&self, x: f32, y: f32) -> Promise {
        let device = self.device.clone();
        let nodes_buffer = self
            .nodes_buffer
            .as_ref()
            .expect("nodes_buffer not initialized")
            .clone();
        let engine = self.engine.clone();

        wasm_bindgen_futures::future_to_promise(async move {
            let size_val = Reflect::get(&nodes_buffer, &"size".into())
                .expect("size property missing")
                .as_f64()
                .unwrap_or(0.0);
            if size_val <= 0.0 {
                return Ok(JsValue::UNDEFINED);
            }

            let staging_buf =
                device.create_buffer(&GpuBufferDescriptor::new(size_val, 0x0001 | 0x0008));

            let encoder = device.create_command_encoder();
            encoder.copy_buffer_to_buffer(&nodes_buffer, 0.0, &staging_buf, 0.0, size_val);
            device
                .queue()
                .submit(&js_sys::Array::of1(&encoder.finish()));

            let promise = staging_buf.map_async(0x0001); // MAP_READ
            wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .map_err(|_| JsValue::from_str("Failed to map buffer"))?;

            let array_buffer = staging_buf.get_mapped_range();
            let uint8_array = js_sys::Uint8Array::new(&array_buffer);
            let vec = uint8_array.to_vec();
            staging_buf.unmap();
            staging_buf.destroy(); // Keep this line as it was in the original

            let node_size = std::mem::size_of::<crate::GpuNode>();
            let count = vec.len() / node_size;
            let mut gpu_nodes = Vec::with_capacity(count);
            for i in 0..count {
                let offset = i * node_size;

                let mut node = std::mem::MaybeUninit::<crate::GpuNode>::uninit();
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        vec.as_ptr().add(offset),
                        node.as_mut_ptr() as *mut u8,
                        node_size,
                    );
                }
                let node = unsafe { node.assume_init() };

                gpu_nodes.push(node);
            }

            let callbacks = {
                let mut e = engine.borrow_mut();
                e.hit_test_nodes = gpu_nodes;
                e.handle_click(x, y)
            };

            for (cb, event) in callbacks {
                cb(event);
            }
            Ok(JsValue::UNDEFINED)
        })
    }

    pub fn handle_mousemove(&self, x: f32, y: f32) {
        self.engine.borrow_mut().handle_mousemove(x, y);
    }

    pub fn handle_keydown(&self, key: String) {
        let result = self.engine.borrow_mut().handle_keydown(key);
        if let Some((cb, val)) = result {
            cb(val);
        }
    }

    fn rebind_all(&mut self) {
        let make_buffer_binding = |buffer: &GpuBuffer| -> Object {
            let obj = Object::new();
            Reflect::set(&obj, &"buffer".into(), buffer).unwrap();
            obj
        };

        self.bind_group_compute =
            Some(self.device.create_bind_group(&GpuBindGroupDescriptor::new(
                &{
                    let entries = js_sys::Array::new();
                    entries.push(&GpuBindGroupEntry::new(
                        0,
                        &make_buffer_binding(self.nodes_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        1,
                        &make_buffer_binding(self.uniform_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        2,
                        &make_buffer_binding(self.characters_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        3,
                        &make_buffer_binding(self.glyph_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        4,
                        &make_buffer_binding(self.class_defs_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        5,
                        &make_buffer_binding(
                            self.node_class_list_and_inline_styles_buffer
                                .as_ref()
                                .unwrap(),
                        ),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        6,
                        &make_buffer_binding(self.panic_buffer.as_ref().unwrap()),
                    ));
                    entries.push(&GpuBindGroupEntry::new(
                        7,
                        &make_buffer_binding(self.leaf_nodes_buffer.as_ref().unwrap()),
                    ));
                    entries
                },
                self.bind_group_layout_compute.as_ref().unwrap(),
            )));

        let entries = js_sys::Array::new();
        entries.push(&GpuBindGroupEntry::new(
            0,
            &make_buffer_binding(self.nodes_buffer.as_ref().unwrap()),
        ));
        entries.push(&GpuBindGroupEntry::new(
            1,
            &make_buffer_binding(self.uniform_buffer.as_ref().unwrap()),
        ));
        entries.push(&GpuBindGroupEntry::new(
            2,
            &make_buffer_binding(self.characters_buffer.as_ref().unwrap()),
        ));
        entries.push(&GpuBindGroupEntry::new(
            3,
            &make_buffer_binding(self.curve_buffer.as_ref().unwrap()),
        ));
        entries.push(&GpuBindGroupEntry::new(
            4,
            &make_buffer_binding(self.glyph_info_buffer.as_ref().unwrap()),
        ));

        let tex_view: Object = self.atlas_texture.as_ref().unwrap().create_view().into();
        entries.push(&GpuBindGroupEntry::new(5, &tex_view));

        let sampler: Object = self.sampler.as_ref().unwrap().clone().into();
        entries.push(&GpuBindGroupEntry::new(6, &sampler));

        self.bind_group_render = Some(self.device.create_bind_group(&GpuBindGroupDescriptor::new(
            &entries,
            self.bind_group_layout_render.as_ref().unwrap(),
        )));
    }

    fn check_gpu_panic(&self) {
        let panic_read_buf = self.panic_readback_buffer.as_ref().unwrap().clone();
        let panic_flag = self.panic_readback_in_progress.clone();
        panic_flag.set(true);

        let _ = wasm_bindgen_futures::future_to_promise(async move {
            let promise = panic_read_buf.map_async(0x0001); // GpuMapMode::READ
            let mapped = wasm_bindgen_futures::JsFuture::from(promise).await;
            if mapped.is_err() {
                panic_flag.set(false);
                return Ok(JsValue::UNDEFINED);
            }

            let array_buffer = panic_read_buf.get_mapped_range();
            let uint32_array = js_sys::Uint32Array::new(&array_buffer);
            let error_id = uint32_array.get_index(0);

            panic_read_buf.unmap();
            panic_flag.set(false);

            if error_id != 0 {
                crate::web_bindings::warn(&format!("--- GPU PANIC ---"));
                crate::web_bindings::warn(&format!(
                    "Unknown CSS Property ID encountered: {}",
                    error_id
                ));
                crate::web_bindings::warn(&format!("Check style_defs.toml and ensure the ID is handled in apply_style_stream (shaders_compute.wgsl)"));
                panic!("GPU Panic: Unknown CSS Property ID: {}", error_id);
            }

            Ok(JsValue::UNDEFINED)
        });
    }
}
