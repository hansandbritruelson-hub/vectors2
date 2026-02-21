#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
use renderer_core::signals::{
    create_effect, create_memo, create_signal, ReadSignal, ToBool, ToReactiveString,
};
use renderer_core::ui::{div, img, input, mount_if, mount_list, text, Element};
use renderer_core::FlexEngine;
use std::cell::RefCell;
use std::rc::Rc;
#[allow(non_snake_case)]
pub mod CssDemo;
#[allow(non_snake_case)]
pub mod Image;
#[allow(non_snake_case)]
pub mod Video;
use crate::design::VectorFile;
#[allow(dead_code)]
#[derive(Clone)]
pub struct Props {}
pub type SlotRenderFn = Rc<dyn Fn(Rc<RefCell<FlexEngine>>, u32, SlotScope)>;
#[derive(Clone, Default)]
pub struct SlotScope {
    values: std::collections::HashMap<String, String>,
}
impl SlotScope {
    pub fn new(values: std::collections::HashMap<String, String>) -> Self {
        Self { values }
    }
    pub fn get(&self, key: &str) -> String {
        self.values.get(key).cloned().unwrap_or_default()
    }
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
}
impl std::ops::Index<&str> for SlotScope {
    type Output = str;
    fn index(&self, key: &str) -> &Self::Output {
        self.values
            .get(key)
            .map(|value| value.as_str())
            .unwrap_or("")
    }
}
#[derive(Clone, Default)]
pub struct Slots {
    pub default: Option<SlotRenderFn>,
    pub named: std::collections::HashMap<String, SlotRenderFn>,
}
impl Slots {
    pub fn get(&self, name: &str) -> Option<SlotRenderFn> {
        self.named.get(name).cloned()
    }
}
pub fn build(engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, _props: Props) -> u32 {
    build_with_slots(engine, parent, _props, Slots::default())
}
#[allow(unused_variables)]
pub fn build_with_slots(
    engine: Rc<RefCell<FlexEngine>>,
    parent: Option<u32>,
    _props: Props,
    slots: Slots,
) -> u32 {
    register_styles(engine.clone());
    let (view, set_view) = crate::signals::create_signal("image".to_string());
    let open_file = Rc::new(RefCell::new(VectorFile {
        path: "assets/project.gemini".to_string(),
        objects: vec![],
    }));
    let root_id = {
        let node_1 = div().class("app-root").build(engine.clone(), parent);
        {
            let node_2 = div()
                .class("app-header")
                .build(engine.clone(), Some(node_1));
            {
                let node_3 = div().class("app-logo").build(engine.clone(), Some(node_2));
                {
                    let node_4 = div().class("logo-box").build(engine.clone(), Some(node_3));
                    {}
                    node_4;
                    let node_5 = div().class("logo-text").build(engine.clone(), Some(node_3));
                    {
                        div().text("CREATOR v2").build(engine.clone(), Some(node_5));
                    }
                    node_5;
                }
                node_3;
                let node_6 = div().class("main-nav").build(engine.clone(), Some(node_2));
                {
                    let node_7 = div()
                        .class("nav-tab")
                        .on_click(move |event: renderer_core::UiEvent| {
                            set_view.set("image");
                        })
                        .build(engine.clone(), Some(node_6));
                    {
                        div()
                            .text("Vector Editor")
                            .build(engine.clone(), Some(node_7));
                    }
                    node_7;
                    let node_8 = div()
                        .class("nav-tab")
                        .on_click(move |event: renderer_core::UiEvent| {
                            set_view.set("video");
                        })
                        .build(engine.clone(), Some(node_6));
                    {
                        div()
                            .text("Video Editor")
                            .build(engine.clone(), Some(node_8));
                    }
                    node_8;
                    let node_9 = div()
                        .class("nav-tab")
                        .on_click(move |event: renderer_core::UiEvent| {
                            set_view.set("css-demo");
                        })
                        .build(engine.clone(), Some(node_6));
                    {
                        div().text("CSS Demo").build(engine.clone(), Some(node_9));
                    }
                    node_9;
                }
                node_6;
                let node_10 = div()
                    .class("system-status")
                    .build(engine.clone(), Some(node_2));
                {
                    let node_11 = div()
                        .class("status-dot")
                        .build(engine.clone(), Some(node_10));
                    {}
                    node_11;
                    let node_12 = div()
                        .class("status-text")
                        .build(engine.clone(), Some(node_10));
                    {
                        div().text("GPU Ready").build(engine.clone(), Some(node_12));
                    }
                    node_12;
                }
                node_10;
            }
            node_2;
            let node_13 = div()
                .class("view-content")
                .build(engine.clone(), Some(node_1));
            {
                {
                    let engine_c = engine.clone();
                    mount_if(
                        engine.clone(),
                        node_13,
                        create_memo(move || (view == "image").to_bool()),
                        move || {
                            let engine = engine_c.clone();
                            let __mount_if_parent = node_13;
                            self::Image::build_with_slots(
                                engine.clone(),
                                Some(__mount_if_parent),
                                self::Image::Props {
                                    open_file: open_file.clone(),
                                },
                                self::Image::Slots::default(),
                            )
                        },
                    );
                    0
                };
                {
                    let engine_c = engine.clone();
                    mount_if(
                        engine.clone(),
                        node_13,
                        create_memo(move || (view == "video").to_bool()),
                        move || {
                            let engine = engine_c.clone();
                            let __mount_if_parent = node_13;
                            self::Video::build_with_slots(
                                engine.clone(),
                                Some(__mount_if_parent),
                                self::Video::Props {},
                                self::Video::Slots::default(),
                            )
                        },
                    );
                    0
                };
                {
                    let engine_c = engine.clone();
                    mount_if(
                        engine.clone(),
                        node_13,
                        create_memo(move || (view == "css-demo").to_bool()),
                        move || {
                            let engine = engine_c.clone();
                            let __mount_if_parent = node_13;
                            self::CssDemo::build_with_slots(
                                engine.clone(),
                                Some(__mount_if_parent),
                                self::CssDemo::Props {},
                                self::CssDemo::Slots::default(),
                            )
                        },
                    );
                    0
                };
            }
            node_13;
        }
        node_1
    };
    root_id
}
fn register_styles(engine: Rc<RefCell<FlexEngine>>) {
    #[allow(unused_mut)]
    let mut e = engine.borrow_mut();
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Vw(100f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Vh(100f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.047058824f32, 0.047058824f32, 0.047058824f32, 1f32),
        );
        e.add_style_rule(".app-root".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(56f32));
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert("right".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert("z-index".to_string(), renderer_core::StyleValue::Px(10f32));
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        e.add_style_rule(".app-header".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(40f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        e.add_style_rule(".app-logo".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(24f32));
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(24f32));
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 0.5f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 1f32),
        );
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".logo-box".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(700f32),
        );
        decls.insert(
            "letter-spacing".to_string(),
            renderer_core::StyleValue::Em(0.1f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        e.add_style_rule(".logo-text".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        e.add_style_rule(".main-nav".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(24f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.53333336f32, 0.53333336f32, 0.53333336f32, 1f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(500f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(24f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(13f32),
        );
        e.add_style_rule(".nav-tab".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8f32, 0.8f32, 0.8f32, 1f32),
        );
        e.add_style_rule(".nav-tab:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        e.add_style_rule(".nav-tab.active".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(2f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 1f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(0f32));
        e.add_style_rule(".nav-tab.active::after".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.22745098f32, 0.22745098f32, 0.22745098f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.15686275f32, 0.15686275f32, 0.15686275f32, 1f32),
        );
        decls.insert(
            "outline-offset".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "outline-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0f32, 0.5019608f32, 0f32, 1f32),
        );
        decls.insert(
            "outline-color-top".to_string(),
            renderer_core::StyleValue::Color(0f32, 0.5019608f32, 0f32, 1f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "outline-color-right".to_string(),
            renderer_core::StyleValue::Color(0f32, 0.5019608f32, 0f32, 1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "outline-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.14509805f32, 0.14509805f32, 0.14509805f32, 1f32),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        decls.insert(
            "outline-color-left".to_string(),
            renderer_core::StyleValue::Color(0f32, 0.5019608f32, 0f32, 1f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.27058825f32, 0.27058825f32, 0.27058825f32, 1f32),
        );
        e.add_style_rule(".system-status".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(8f32));
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0.18039216f32, 0.8f32, 0.44313726f32, 0.5f32),
        );
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.18039216f32, 0.8f32, 0.44313726f32, 1f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(8f32));
        e.add_style_rule(".status-dot".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(700f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.18039216f32, 0.8f32, 0.44313726f32, 1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        e.add_style_rule(".status-text".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("bottom".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(56f32));
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert("right".to_string(), renderer_core::StyleValue::Px(0f32));
        e.add_style_rule(".view-content".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(100f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.94509804f32, 0.76862746f32, 0.05882353f32, 1f32),
        );
        e.add_style_rule(".font-size-test".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        e.add_style_rule(".spacer".to_string(), decls);
    }
}
