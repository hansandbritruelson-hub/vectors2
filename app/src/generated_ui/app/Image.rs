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
    let (text_val, set_text_val) = crate::signals::create_signal("Hello".to_string());
    let (float_val, set_float_val) = crate::signals::create_signal("0.0".to_string());
    let root_id = {
        let node_1 = div().class("image-editor").build(engine.clone(), parent);
        {
            let node_2 = div().class("header").build(engine.clone(), Some(node_1));
            {
                text("Image Editor").build(engine.clone(), Some(node_2));
            }
            node_2;
            let node_3 = div().class("input-row").build(engine.clone(), Some(node_1));
            {
                let node_4 = div().class("label").build(engine.clone(), Some(node_3));
                {
                    text("Text:").build(engine.clone(), Some(node_4));
                }
                node_4;
                let node_5 = input()
                    .input_type("text")
                    .value(create_memo({
                        let val = text_val.clone();
                        move || val.to_reactive_string()
                    }))
                    .on_update_model_value(move |val| set_text_val.set(val))
                    .class("my-input")
                    .build(engine.clone(), Some(node_3));
                {}
                node_5;
            }
            node_3;
            let node_6 = div().class("input-row").build(engine.clone(), Some(node_1));
            {
                let node_7 = div().class("label").build(engine.clone(), Some(node_6));
                {
                    text("Float:").build(engine.clone(), Some(node_7));
                }
                node_7;
                let node_8 = input()
                    .input_type("float64")
                    .value(create_memo({
                        let val = float_val.clone();
                        move || val.to_reactive_string()
                    }))
                    .on_update_model_value(move |val| set_float_val.set(val))
                    .class("my-input")
                    .build(engine.clone(), Some(node_6));
                {}
                node_8;
            }
            node_6;
            let node_9 = div().class("result").build(engine.clone(), Some(node_1));
            {
                text("Text Value: ").build(engine.clone(), Some(node_9));
                div()
                    .child(text("").value(create_memo({
                        let val = text_val.clone();
                        move || val.to_reactive_string()
                    })))
                    .build(engine.clone(), Some(node_9));
            }
            node_9;
            let node_10 = div().class("result").build(engine.clone(), Some(node_1));
            {
                text("Float Value: ").build(engine.clone(), Some(node_10));
                div()
                    .child(text("").value(create_memo({
                        let val = float_val.clone();
                        move || val.to_reactive_string()
                    })))
                    .build(engine.clone(), Some(node_10));
            }
            node_10;
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
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.101960786f32, 0.101960786f32, 0.101960786f32, 1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        e.add_style_rule(".image-editor".to_string(), decls);
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
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        e.add_style_rule(".input-row".to_string(), decls);
    }
}
