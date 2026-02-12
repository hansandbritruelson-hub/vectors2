#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
use renderer_core::signals::{
    create_effect, create_memo, create_signal, ReadSignal, ToBool, ToReactiveString,
};
use renderer_core::ui::{div, mount_if, mount_list, text, Element};
use renderer_core::FlexEngine;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
pub struct Props {
    pub text: String,
}
#[allow(unused_variables)]
pub fn build(engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, props: Props) -> u32 {
    register_styles(engine.clone());
    let root_id = {
        let node_1 = div()
            .style("height", renderer_core::StyleValue::Px(100f32))
            .style(
                "flex-direction",
                renderer_core::StyleValue::Ident("column".to_string()),
            )
            .style(
                "background-color",
                renderer_core::StyleValue::Color(0.53333336f32, 0.26666668f32, 0.26666668f32, 1f32),
            )
            .style("width", renderer_core::StyleValue::Px(200f32))
            .build(engine.clone(), parent);
        {
            let node_2 = text("")
                .text("I am a component!")
                .style(
                    "color",
                    renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
                )
                .build(engine.clone(), Some(node_1));
            {}
            node_2;
            let node_3 = text("")
                .bind_text(create_memo({
                    let val = props.text.clone();
                    move || val.to_reactive_string()
                }))
                .style(
                    "color",
                    renderer_core::StyleValue::Color(1f32, 1f32, 0f32, 1f32),
                )
                .build(engine.clone(), Some(node_1));
            {}
            node_3;
        }
        node_1
    };
    root_id
}
fn register_styles(engine: Rc<RefCell<FlexEngine>>) {
    #[allow(unused_mut)]
    let mut e = engine.borrow_mut();
}
