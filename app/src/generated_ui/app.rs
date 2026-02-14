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
pub mod Image;
#[allow(non_snake_case)]
pub mod Video;
#[allow(dead_code)]
#[derive(Clone)]
pub struct Props {}
#[allow(unused_variables)]
pub fn build(engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, _props: Props) -> u32 {
    register_styles(engine.clone());
    let (view, set_view) = crate::signals::create_signal("image".to_string());
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
                    let node_5 = div()
                        .class("logo-text")
                        .text("CREATOR v2")
                        .build(engine.clone(), Some(node_3));
                    {}
                    node_5;
                }
                node_3;
                let node_6 = div().class("main-nav").build(engine.clone(), Some(node_2));
                {
                    let node_7 = div()
                        .class("nav-tab")
                        .on_click(move || set_view.set("image"))
                        .text("Vector Editor\n        ")
                        .build(engine.clone(), Some(node_6));
                    {}
                    node_7;
                    let node_8 = div()
                        .class("nav-tab")
                        .on_click(move || set_view.set("video"))
                        .text("Video Editor\n        ")
                        .build(engine.clone(), Some(node_6));
                    {}
                    node_8;
                }
                node_6;
                let node_9 = div().class("spacer").build(engine.clone(), Some(node_2));
                {}
                node_9;
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
                        .text("GPU ACTIVE")
                        .build(engine.clone(), Some(node_10));
                    {}
                    node_12;
                }
                node_10;
            }
            node_2;
            let node_13 = div()
                .class("view-content")
                .build(engine.clone(), Some(node_1));
            {
                let node_14 = div()
                    .class("font-size-test")
                    .text("THIS IS FONT SIZE 100PX TEST")
                    .build(engine.clone(), Some(node_13));
                {}
                node_14;
                {
                    let engine_c = engine.clone();
                    mount_if(
                        engine.clone(),
                        node_13,
                        create_memo(move || (view == "image").to_bool()),
                        move || {
                            let engine = engine_c.clone();
                            self::Image::build(engine.clone(), None, self::Image::Props {})
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
                            self::Video::build(engine.clone(), None, self::Video::Props {})
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
            "width".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.047058824f32, 0.047058824f32, 0.047058824f32, 1f32),
        );
        decls.insert(
            "font-family".to_string(),
            renderer_core::StyleValue::Ident("'Inter',".to_string()),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        e.add_style_rule(".app-root".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-bottom".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(56f32));
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".app-header".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(40f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        e.add_style_rule(".app-logo".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(24f32));
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(24f32));
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 1f32),
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
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "letter-spacing".to_string(),
            renderer_core::StyleValue::Em(0.1f32),
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
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(24f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
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
            "color".to_string(),
            renderer_core::StyleValue::Color(0.53333336f32, 0.53333336f32, 0.53333336f32, 1f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(13f32),
        );
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(500f32),
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
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 1f32),
        );
        decls.insert("bottom".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert(
            "content".to_string(),
            renderer_core::StyleValue::Ident("\"\"".to_string()),
        );
        decls.insert("right".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(2f32));
        e.add_style_rule(".nav-tab.active::after".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.14509805f32, 0.14509805f32, 0.14509805f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        e.add_style_rule(".system-status".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(8f32));
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0.18039216f32, 0.8f32, 0.44313726f32, 0.5f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(8f32));
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.18039216f32, 0.8f32, 0.44313726f32, 1f32),
        );
        e.add_style_rule(".status-dot".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.18039216f32, 0.8f32, 0.44313726f32, 1f32),
        );
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(700f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        e.add_style_rule(".status-text".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        e.add_style_rule(".view-content".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(100f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.94509804f32, 0.76862746f32, 0.05882353f32, 1f32),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        e.add_style_rule(".font-size-test".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        e.add_style_rule(".spacer".to_string(), decls);
    }
}
