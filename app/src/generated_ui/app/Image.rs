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
        let node_1 = div()
            .class("editor-container")
            .build(engine.clone(), parent);
        {
            let node_2 = div().class("sidebar").build(engine.clone(), Some(node_1));
            {
                let node_3 = div()
                    .class("tool-icon active")
                    .build(engine.clone(), Some(node_2));
                {
                    let node_4 = img()
                        .image("asset://phosphor/selection.svg")
                        .class("icon-img")
                        .build(engine.clone(), Some(node_3));
                    {}
                    node_4;
                }
                node_3;
                let node_5 = div().class("tool-icon").build(engine.clone(), Some(node_2));
                {
                    let node_6 = img()
                        .image("asset://phosphor/cursor.svg")
                        .class("icon-img")
                        .build(engine.clone(), Some(node_5));
                    {}
                    node_6;
                }
                node_5;
                let node_7 = div().class("tool-icon").build(engine.clone(), Some(node_2));
                {
                    let node_8 = img()
                        .image("asset://phosphor/pencil.svg")
                        .class("icon-img")
                        .build(engine.clone(), Some(node_7));
                    {}
                    node_8;
                }
                node_7;
                let node_9 = div().class("tool-icon").build(engine.clone(), Some(node_2));
                {
                    let node_10 = img()
                        .image("asset://phosphor/square.svg")
                        .class("icon-img")
                        .build(engine.clone(), Some(node_9));
                    {}
                    node_10;
                }
                node_9;
                let node_11 = div().class("tool-icon").build(engine.clone(), Some(node_2));
                {
                    let node_12 = img()
                        .image("asset://phosphor/circle.svg")
                        .class("icon-img")
                        .build(engine.clone(), Some(node_11));
                    {}
                    node_12;
                }
                node_11;
                let node_13 = div().class("tool-icon").build(engine.clone(), Some(node_2));
                {
                    let node_14 = img()
                        .image("asset://phosphor/hand-grabbing.svg")
                        .class("icon-img")
                        .build(engine.clone(), Some(node_13));
                    {}
                    node_14;
                }
                node_13;
                let node_15 = div().class("spacer").build(engine.clone(), Some(node_2));
                {}
                node_15;
                let node_16 = div().class("tool-icon").build(engine.clone(), Some(node_2));
                {
                    let node_17 = img()
                        .image("asset://phosphor/settings.svg")
                        .class("icon-img")
                        .build(engine.clone(), Some(node_16));
                    {}
                    node_17;
                }
                node_16;
            }
            node_2;
            let node_18 = div()
                .class("main-content")
                .build(engine.clone(), Some(node_1));
            {
                let node_19 = div().class("top-bar").build(engine.clone(), Some(node_18));
                {
                    let node_20 = div()
                        .class("menu-item")
                        .build(engine.clone(), Some(node_19));
                    {
                        text("File").build(engine.clone(), Some(node_20));
                    }
                    node_20;
                    let node_21 = div()
                        .class("menu-item")
                        .build(engine.clone(), Some(node_19));
                    {
                        text("Edit").build(engine.clone(), Some(node_21));
                    }
                    node_21;
                    let node_22 = div()
                        .class("menu-item")
                        .build(engine.clone(), Some(node_19));
                    {
                        text("View").build(engine.clone(), Some(node_22));
                    }
                    node_22;
                    let node_23 = div()
                        .class("menu-item")
                        .build(engine.clone(), Some(node_19));
                    {
                        text("Object").build(engine.clone(), Some(node_23));
                    }
                    node_23;
                    let node_24 = div().class("spacer").build(engine.clone(), Some(node_19));
                    {}
                    node_24;
                    let node_25 = div()
                        .class("project-title")
                        .build(engine.clone(), Some(node_19));
                    {
                        text("Untitled Vector Project").build(engine.clone(), Some(node_25));
                    }
                    node_25;
                    let node_26 = div().class("spacer").build(engine.clone(), Some(node_19));
                    {}
                    node_26;
                    let node_27 = div()
                        .class("user-profile")
                        .build(engine.clone(), Some(node_19));
                    {
                        text("HB").build(engine.clone(), Some(node_27));
                    }
                    node_27;
                }
                node_19;
                let node_28 = div()
                    .class("context-bar")
                    .build(engine.clone(), Some(node_18));
                {
                    let node_29 = div()
                        .class("context-tools")
                        .build(engine.clone(), Some(node_28));
                    {
                        let node_30 = div()
                            .class("tool-icon-small")
                            .build(engine.clone(), Some(node_29));
                        {
                            let node_31 = img()
                                .image("asset://phosphor/undo.svg")
                                .class("icon-img-small")
                                .build(engine.clone(), Some(node_30));
                            {}
                            node_31;
                        }
                        node_30;
                        let node_32 = div()
                            .class("tool-icon-small")
                            .build(engine.clone(), Some(node_29));
                        {
                            let node_33 = img()
                                .image("asset://phosphor/redo.svg")
                                .class("icon-img-small")
                                .build(engine.clone(), Some(node_32));
                            {}
                            node_33;
                        }
                        node_32;
                        let node_34 = div().class("divider").build(engine.clone(), Some(node_29));
                        {}
                        node_34;
                        let node_35 = div()
                            .class("tool-icon-small")
                            .build(engine.clone(), Some(node_29));
                        {
                            let node_36 = img()
                                .image("asset://phosphor/copy.svg")
                                .class("icon-img-small")
                                .build(engine.clone(), Some(node_35));
                            {}
                            node_36;
                        }
                        node_35;
                        let node_37 = div()
                            .class("tool-icon-small")
                            .build(engine.clone(), Some(node_29));
                        {
                            let node_38 = img()
                                .image("asset://phosphor/paste.svg")
                                .class("icon-img-small")
                                .build(engine.clone(), Some(node_37));
                            {}
                            node_38;
                        }
                        node_37;
                        let node_39 = div()
                            .class("tool-icon-small")
                            .build(engine.clone(), Some(node_29));
                        {
                            let node_40 = img()
                                .image("asset://phosphor/delete.svg")
                                .class("icon-img-small")
                                .build(engine.clone(), Some(node_39));
                            {}
                            node_40;
                        }
                        node_39;
                    }
                    node_29;
                    let node_41 = div().class("spacer").build(engine.clone(), Some(node_28));
                    {}
                    node_41;
                    let node_42 = div()
                        .class("snap-tools")
                        .build(engine.clone(), Some(node_28));
                    {
                        let node_43 = div()
                            .class("tool-icon-small active")
                            .build(engine.clone(), Some(node_42));
                        {
                            let node_44 = img()
                                .image("asset://phosphor/snapping.svg")
                                .class("icon-img-small")
                                .build(engine.clone(), Some(node_43));
                            {}
                            node_44;
                        }
                        node_43;
                        let node_45 = div()
                            .class("label-small")
                            .build(engine.clone(), Some(node_42));
                        {
                            text("Snapping").build(engine.clone(), Some(node_45));
                        }
                        node_45;
                    }
                    node_42;
                }
                node_28;
                let node_46 = div()
                    .class("editor-body")
                    .build(engine.clone(), Some(node_18));
                {
                    let node_47 = div()
                        .class("canvas-area")
                        .build(engine.clone(), Some(node_46));
                    {
                        let node_48 = div()
                            .class("canvas-mock")
                            .build(engine.clone(), Some(node_47));
                        {
                            let node_49 = div()
                                .class("rect-shape")
                                .build(engine.clone(), Some(node_48));
                            {}
                            node_49;
                            let node_50 = div()
                                .class("circle-shape")
                                .build(engine.clone(), Some(node_48));
                            {}
                            node_50;
                        }
                        node_48;
                    }
                    node_47;
                    let node_51 = div()
                        .class("right-panel")
                        .build(engine.clone(), Some(node_46));
                    {
                        let node_52 = div()
                            .class("panel-header")
                            .build(engine.clone(), Some(node_51));
                        {
                            let node_53 = img()
                                .image("asset://phosphor/layers.svg")
                                .class("icon-img-mini")
                                .build(engine.clone(), Some(node_52));
                            {}
                            node_53;
                            let node_54 = div()
                                .class("panel-title")
                                .build(engine.clone(), Some(node_52));
                            {
                                text("Layers").build(engine.clone(), Some(node_54));
                            }
                            node_54;
                        }
                        node_52;
                        let node_55 = div()
                            .class("layer-list")
                            .build(engine.clone(), Some(node_51));
                        {
                            let node_56 = div()
                                .class("layer-item active")
                                .build(engine.clone(), Some(node_55));
                            {
                                let node_57 = img()
                                    .image("asset://phosphor/circle.svg")
                                    .class("icon-img-mini")
                                    .build(engine.clone(), Some(node_56));
                                {}
                                node_57;
                                let node_58 = div()
                                    .class("layer-name")
                                    .build(engine.clone(), Some(node_56));
                                {
                                    text("Circle 1").build(engine.clone(), Some(node_58));
                                }
                                node_58;
                            }
                            node_56;
                            let node_59 = div()
                                .class("layer-item")
                                .build(engine.clone(), Some(node_55));
                            {
                                let node_60 = img()
                                    .image("asset://phosphor/square.svg")
                                    .class("icon-img-mini")
                                    .build(engine.clone(), Some(node_59));
                                {}
                                node_60;
                                let node_61 = div()
                                    .class("layer-name")
                                    .build(engine.clone(), Some(node_59));
                                {
                                    text("Rectangle 1").build(engine.clone(), Some(node_61));
                                }
                                node_61;
                            }
                            node_59;
                            let node_62 = div()
                                .class("layer-item")
                                .build(engine.clone(), Some(node_55));
                            {
                                let node_63 = img()
                                    .image("asset://phosphor/pencil.svg")
                                    .class("icon-img-mini")
                                    .build(engine.clone(), Some(node_62));
                                {}
                                node_63;
                                let node_64 = div()
                                    .class("layer-name")
                                    .build(engine.clone(), Some(node_62));
                                {
                                    text("Path 1").build(engine.clone(), Some(node_64));
                                }
                                node_64;
                            }
                            node_62;
                        }
                        node_55;
                    }
                    node_51;
                }
                node_46;
            }
            node_18;
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
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8784314f32, 0.8784314f32, 0.8784314f32, 1f32),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.07058824f32, 0.07058824f32, 0.07058824f32, 1f32),
        );
        e.add_style_rule(".editor-container".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(60f32));
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "border-right".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        e.add_style_rule("/* Sidebar */\n.sidebar".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        e.add_style_rule(".tool-icon".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        e.add_style_rule(".tool-icon:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        e.add_style_rule(".tool-icon.active".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(24f32));
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(24f32));
        e.add_style_rule(".icon-img".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        e.add_style_rule("/* Main Content */\n.main-content".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(44f32));
        decls.insert(
            "border-bottom".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        e.add_style_rule("/* Top Bar */\n.top-bar".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(13f32),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.73333335f32, 0.73333335f32, 0.73333335f32, 1f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".menu-item".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        e.add_style_rule(".menu-item:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(500f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.53333336f32, 0.53333336f32, 0.53333336f32, 1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(13f32),
        );
        e.add_style_rule(".project-title".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(48f32));
        decls.insert(
            "border-bottom".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.14509805f32, 0.14509805f32, 0.14509805f32, 1f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        e.add_style_rule("/* Context Bar */\n.context-bar".to_string(), decls);
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
        e.add_style_rule(".context-tools".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(32f32));
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(32f32));
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        e.add_style_rule(".tool-icon-small".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.21960784f32, 0.21960784f32, 0.21960784f32, 1f32),
        );
        e.add_style_rule(".tool-icon-small:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        e.add_style_rule(".tool-icon-small.active".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(18f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(18f32));
        e.add_style_rule(".icon-img-small".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(20f32));
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        e.add_style_rule(".divider".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        e.add_style_rule(".snap-tools".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(11f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8f32, 0.8f32, 0.8f32, 1f32),
        );
        e.add_style_rule(".label-small".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        e.add_style_rule("/* Editor Body */\n.editor-body".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(40f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(40f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(40f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.07058824f32, 0.07058824f32, 0.07058824f32, 1f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(40f32),
        );
        e.add_style_rule(".canvas-area".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
        );
        decls.insert(
            "overflow".to_string(),
            renderer_core::StyleValue::Ident("hidden".to_string()),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(600f32));
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(400f32));
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(30f32),
        );
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.5f32),
        );
        e.add_style_rule(".canvas-mock".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.16078432f32, 0.5019608f32, 0.7254902f32, 1f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(150f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(100f32));
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(50f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.16078432f32, 0.5019608f32, 0.7254902f32, 1f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.16078432f32, 0.5019608f32, 0.7254902f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.16078432f32, 0.5019608f32, 0.7254902f32, 1f32),
        );
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(50f32));
        e.add_style_rule(".rect-shape".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.7529412f32, 0.22352941f32, 0.16862746f32, 1f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(300f32));
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.7529412f32, 0.22352941f32, 0.16862746f32, 1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(120f32));
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(60f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.90588236f32, 0.29803923f32, 0.23529412f32, 1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.7529412f32, 0.22352941f32, 0.16862746f32, 1f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(120f32));
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(180f32));
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.7529412f32, 0.22352941f32, 0.16862746f32, 1f32),
        );
        e.add_style_rule(".circle-shape".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(240f32));
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "border-left".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        e.add_style_rule("/* Right Panel */\n.right-panel".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.14509805f32, 0.14509805f32, 0.14509805f32, 1f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "border-bottom".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
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
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        e.add_style_rule(".panel-header".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "text-transform".to_string(),
            renderer_core::StyleValue::Ident("uppercase".to_string()),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(11f32),
        );
        decls.insert(
            "letter-spacing".to_string(),
            renderer_core::StyleValue::Em(0.05f32),
        );
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(600f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.53333336f32, 0.53333336f32, 0.53333336f32, 1f32),
        );
        e.add_style_rule(".panel-title".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        e.add_style_rule(".layer-list".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(32f32));
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".layer-item".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16470589f32, 0.16470589f32, 0.16470589f32, 1f32),
        );
        e.add_style_rule(".layer-item:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
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
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        e.add_style_rule(".layer-item.active".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8f32, 0.8f32, 0.8f32, 1f32),
        );
        e.add_style_rule(".layer-name".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(14f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(14f32));
        e.add_style_rule(".icon-img-mini".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        e.add_style_rule(".spacer".to_string(), decls);
    }
}
