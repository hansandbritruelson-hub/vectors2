#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
use renderer_core::signals::{
    create_effect, create_memo, create_signal, ReadSignal, ToBool, ToReactiveString,
};
use renderer_core::ui::{div, input, mount_if, mount_list, text, Element};
use renderer_core::FlexEngine;
use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
#[derive(Clone)]
pub struct Props {}
#[allow(unused_variables)]
pub fn build(engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, _props: Props) -> u32 {
    register_styles(engine.clone());
    let root_id = {
        let node_1 = div().class("video-editor").build(engine.clone(), parent);
        {
            let node_2 = div().class("header").build(engine.clone(), Some(node_1));
            {
                text("Video Editor").build(engine.clone(), Some(node_2));
            }
            node_2;
            let node_3 = div()
                .class("player-placeholder")
                .build(engine.clone(), Some(node_1));
            {
                let node_4 = div().class("label").build(engine.clone(), Some(node_3));
                {
                    text("Video Player Placeholder").build(engine.clone(), Some(node_4));
                }
                node_4;
            }
            node_3;
            let node_5 = div().class("timeline").build(engine.clone(), Some(node_1));
            {
                let node_6 = div().class("label").build(engine.clone(), Some(node_5));
                {
                    text("Timeline").build(engine.clone(), Some(node_6));
                }
                node_6;
            }
            node_5;
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
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.101960786f32, 0.101960786f32, 0.101960786f32, 1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        e.add_style_rule(".video-editor".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(24f32),
        );
        e.add_style_rule(".header".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(300f32));
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16470589f32, 0.16470589f32, 0.16470589f32, 1f32),
        );
        e.add_style_rule(".player-placeholder".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(100f32));
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        e.add_style_rule(".timeline".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.4f32, 0.4f32, 0.4f32, 1f32),
        );
        e.add_style_rule(".label".to_string(), decls);
    }
}
