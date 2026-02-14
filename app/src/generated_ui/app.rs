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
#[allow(dead_code)]
#[derive(Clone)]
pub struct Props {}
#[allow(unused_variables)]
pub fn build(engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, _props: Props) -> u32 {
    register_styles(engine.clone());
    let root_id = {
        let node_1 = div().class("app-container").build(engine.clone(), parent);
        {
            let node_2 = div()
                .text("Before Image")
                .build(engine.clone(), Some(node_1));
            {}
            node_2;
            let node_3 = img()
                .image("asset://phosphor/circle.svg")
                .class("icon")
                .build(engine.clone(), Some(node_1));
            {}
            node_3;
            let node_4 = div()
                .text("After Image")
                .build(engine.clone(), Some(node_1));
            {}
            node_4;
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
            "stroke-width".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(100f32));
        decls.insert(
            "stroke".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "fill".to_string(),
            renderer_core::StyleValue::Color(0.6039216f32, 0.17254902f32, 0.17254902f32, 1f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(100f32));
        e.add_style_rule(".icon".to_string(), decls);
    }
}
