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
        let node_1 = div().class("demo-root").build(engine.clone(), parent);
        {
            let node_2 = div()
                .class("demo-header")
                .build(engine.clone(), Some(node_1));
            {
                let node_3 = div()
                    .class("demo-title")
                    .text("CSS Demo")
                    .build(engine.clone(), Some(node_2));
                {}
                node_3;
                let node_4 = div()
                    .class("demo-subtitle")
                    .text("Shorthand + layout + typography")
                    .build(engine.clone(), Some(node_2));
                {}
                node_4;
            }
            node_2;
            let node_5 = div().class("demo-grid").build(engine.clone(), Some(node_1));
            {
                let node_6 = div()
                    .class("demo-card card-a")
                    .build(engine.clone(), Some(node_5));
                {
                    let node_7 = div()
                        .class("card-title")
                        .text("Borders + Outline")
                        .build(engine.clone(), Some(node_6));
                    {}
                    node_7;
                    let node_8 = div().class("chip-row").build(engine.clone(), Some(node_6));
                    {
                        let node_9 = div()
                            .class("chip chip-red")
                            .text("red")
                            .build(engine.clone(), Some(node_8));
                        {}
                        node_9;
                        let node_10 = div()
                            .class("chip chip-green")
                            .text("green")
                            .build(engine.clone(), Some(node_8));
                        {}
                        node_10;
                        let node_11 = div()
                            .class("chip chip-blue")
                            .text("blue")
                            .build(engine.clone(), Some(node_8));
                        {}
                        node_11;
                    }
                    node_8;
                }
                node_6;
                let node_12 = div()
                    .class("demo-card card-b")
                    .build(engine.clone(), Some(node_5));
                {
                    let node_13 = div()
                        .class("card-title")
                        .text("Spacing + Shadow")
                        .build(engine.clone(), Some(node_12));
                    {}
                    node_13;
                    let node_14 = div().class("stack").build(engine.clone(), Some(node_12));
                    {
                        let node_15 = div()
                            .class("stack-item")
                            .text("padding")
                            .build(engine.clone(), Some(node_14));
                        {}
                        node_15;
                        let node_16 = div()
                            .class("stack-item")
                            .text("margin")
                            .build(engine.clone(), Some(node_14));
                        {}
                        node_16;
                        let node_17 = div()
                            .class("stack-item")
                            .text("box-shadow")
                            .build(engine.clone(), Some(node_14));
                        {}
                        node_17;
                    }
                    node_14;
                }
                node_12;
                let node_18 = div()
                    .class("demo-card card-c")
                    .build(engine.clone(), Some(node_5));
                {
                    let node_19 = div()
                        .class("card-title")
                        .text("Position + Z Index")
                        .build(engine.clone(), Some(node_18));
                    {}
                    node_19;
                    let node_20 = div()
                        .class("position-stage")
                        .build(engine.clone(), Some(node_18));
                    {
                        let node_21 = div()
                            .class("square back")
                            .text("Back")
                            .build(engine.clone(), Some(node_20));
                        {}
                        node_21;
                        let node_22 = div()
                            .class("square front")
                            .text("Front")
                            .build(engine.clone(), Some(node_20));
                        {}
                        node_22;
                    }
                    node_20;
                }
                node_18;
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
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.078431375f32, 0.078431375f32, 0.078431375f32, 1f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        e.add_style_rule(".demo-root".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        e.add_style_rule(".demo-header".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(30f32),
        );
        e.add_style_rule(".demo-title".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.61960787f32, 0.61960787f32, 0.61960787f32, 1f32),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        e.add_style_rule(".demo-subtitle".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("space-between".to_string()),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("flex-start".to_string()),
        );
        e.add_style_rule(".demo-grid".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.1764706f32, 0.1764706f32, 1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.12156863f32, 0.12156863f32, 0.12156863f32, 1f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.1764706f32, 0.1764706f32, 1f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.1764706f32, 0.1764706f32, 1f32),
        );
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.1764706f32, 0.1764706f32, 1f32),
        );
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.35f32),
        );
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(32f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(240f32));
        e.add_style_rule(".demo-card".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".demo-card:last-child".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.9490196f32, 0.9490196f32, 0.9490196f32, 1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        e.add_style_rule(".card-title".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(1f32, 0.48235294f32, 0.48235294f32, 1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "outline-color-top".to_string(),
            renderer_core::StyleValue::Color(0f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "outline-offset".to_string(),
            renderer_core::StyleValue::Px(3f32),
        );
        decls.insert(
            "outline-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(3f32),
        );
        decls.insert(
            "outline-color-left".to_string(),
            renderer_core::StyleValue::Color(0f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "outline-color-right".to_string(),
            renderer_core::StyleValue::Color(0f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.48235294f32, 1f32, 0.62352943f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.48235294f32, 0.7176471f32, 1f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(1f32, 0.8862745f32, 0.48235294f32, 1f32),
        );
        decls.insert(
            "outline-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0f32, 1f32, 1f32, 1f32),
        );
        e.add_style_rule(".card-a".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("space-evenly".to_string()),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        e.add_style_rule(".chip-row".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.22745098f32, 0.22745098f32, 0.22745098f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.22745098f32, 0.22745098f32, 0.22745098f32, 1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(10f32),
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
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.22745098f32, 0.22745098f32, 0.22745098f32, 1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.16862746f32, 0.16862746f32, 1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.22745098f32, 0.22745098f32, 0.22745098f32, 1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        e.add_style_rule(".chip".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 0f32, 0f32, 1f32),
        );
        e.add_style_rule(".chip-red".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0.5019608f32, 0f32, 1f32),
        );
        e.add_style_rule(".chip-green".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 1f32, 1f32),
        );
        e.add_style_rule(".chip-blue".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(3f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.827451f32, 0.32941177f32, 0f32, 1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.94509804f32, 0.76862746f32, 0.05882353f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.9529412f32, 0.6117647f32, 0.07058824f32, 1f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(3f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.9019608f32, 0.49411765f32, 0.13333334f32, 1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        e.add_style_rule(".card-b".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        e.add_style_rule(".stack".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16470589f32, 0.16470589f32, 0.16470589f32, 1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.2509804f32, 0.2509804f32, 0.2509804f32, 1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.2509804f32, 0.2509804f32, 0.2509804f32, 1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.2509804f32, 0.2509804f32, 0.2509804f32, 1f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.2509804f32, 0.2509804f32, 0.2509804f32, 1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8980392f32, 0.8980392f32, 0.8980392f32, 1f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        e.add_style_rule(".stack-item".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.18431373f32, 0.18431373f32, 0.18431373f32, 1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
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
            renderer_core::StyleValue::Color(0.18431373f32, 0.18431373f32, 0.18431373f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.18431373f32, 0.18431373f32, 0.18431373f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.18431373f32, 0.18431373f32, 0.18431373f32, 1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        e.add_style_rule(".card-c".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.09019608f32, 0.09019608f32, 0.09019608f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.16862746f32, 0.16862746f32, 1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.16862746f32, 0.16862746f32, 1f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(160f32));
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.16862746f32, 0.16862746f32, 1f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.16862746f32, 0.16862746f32, 1f32),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        e.add_style_rule(".position-stage".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(13f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(100f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(70f32));
        e.add_style_rule(".square".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(32f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16078432f32, 0.5019608f32, 0.7254902f32, 1f32),
        );
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(44f32));
        decls.insert("z-index".to_string(), renderer_core::StyleValue::Px(1f32));
        e.add_style_rule(".back".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("z-index".to_string(), renderer_core::StyleValue::Px(5f32));
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(70f32));
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.90588236f32, 0.29803923f32, 0.23529412f32, 1f32),
        );
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.4f32),
        );
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(70f32));
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        e.add_style_rule(".front".to_string(), decls);
    }
}
