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
        let node_1 = div().class("container").build(engine.clone(), parent);
        {
            let node_2 = div().class("nav").build(engine.clone(), Some(node_1));
            {
                let node_3 = div()
                    .class("nav-item")
                    .on_click(move || set_view.set("image"))
                    .build(engine.clone(), Some(node_2));
                {
                    text("Image").build(engine.clone(), Some(node_3));
                }
                node_3;
                let node_4 = div()
                    .class("nav-item")
                    .on_click(move || set_view.set("video"))
                    .build(engine.clone(), Some(node_2));
                {
                    text("Video").build(engine.clone(), Some(node_4));
                }
                node_4;
            }
            node_2;
            let node_5 = div().class("content").build(engine.clone(), Some(node_1));
            {
                {
                    let engine_c = engine.clone();
                    mount_if(
                        engine.clone(),
                        node_5,
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
                        node_5,
                        create_memo(move || (view == "video").to_bool()),
                        move || {
                            let engine = engine_c.clone();
                            self::Video::build(engine.clone(), None, self::Video::Props {})
                        },
                    );
                    0
                };
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
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.07058824f32, 0.07058824f32, 0.07058824f32, 1f32),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        e.add_style_rule(".container".to_string(), decls);
    }
}
