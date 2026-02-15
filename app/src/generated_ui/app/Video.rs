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
        let node_1 = div().class("video-container").build(engine.clone(), parent);
        {
            let node_2 = div().class("workspace").build(engine.clone(), Some(node_1));
            {
                let node_3 = div()
                    .class("preview-area")
                    .build(engine.clone(), Some(node_2));
                {
                    let node_4 = div()
                        .class("player-container")
                        .build(engine.clone(), Some(node_3));
                    {
                        let node_5 = div()
                            .class("video-mock")
                            .build(engine.clone(), Some(node_4));
                        {
                            let node_6 = div()
                                .class("play-overlay")
                                .build(engine.clone(), Some(node_5));
                            {
                                let node_7 = img()
                                    .image("asset://phosphor/play.svg")
                                    .class("icon-play-large")
                                    .build(engine.clone(), Some(node_6));
                                {}
                                node_7;
                            }
                            node_6;
                        }
                        node_5;
                        let node_8 = div()
                            .class("player-controls")
                            .build(engine.clone(), Some(node_4));
                        {
                            let node_9 = div()
                                .class("control-btn")
                                .build(engine.clone(), Some(node_8));
                            {
                                let node_10 = img()
                                    .image("asset://phosphor/skip-back.svg")
                                    .class("icon-control")
                                    .build(engine.clone(), Some(node_9));
                                {}
                                node_10;
                            }
                            node_9;
                            let node_11 = div()
                                .class("control-btn active")
                                .build(engine.clone(), Some(node_8));
                            {
                                let node_12 = img()
                                    .image("asset://phosphor/play.svg")
                                    .class("icon-control")
                                    .build(engine.clone(), Some(node_11));
                                {}
                                node_12;
                            }
                            node_11;
                            let node_13 = div()
                                .class("control-btn")
                                .build(engine.clone(), Some(node_8));
                            {
                                let node_14 = img()
                                    .image("asset://phosphor/skip-forward.svg")
                                    .class("icon-control")
                                    .build(engine.clone(), Some(node_13));
                                {}
                                node_14;
                            }
                            node_13;
                            let node_15 = div()
                                .class("time-display")
                                .text("00:04:12 / 00:15:00")
                                .build(engine.clone(), Some(node_8));
                            {}
                            node_15;
                            let node_16 = div().class("spacer").build(engine.clone(), Some(node_8));
                            {}
                            node_16;
                            let node_17 = div()
                                .class("control-btn")
                                .build(engine.clone(), Some(node_8));
                            {
                                let node_18 = img()
                                    .image("asset://phosphor/settings.svg")
                                    .class("icon-control")
                                    .build(engine.clone(), Some(node_17));
                                {}
                                node_18;
                            }
                            node_17;
                        }
                        node_8;
                    }
                    node_4;
                }
                node_3;
                let node_19 = div()
                    .class("timeline-area")
                    .build(engine.clone(), Some(node_2));
                {
                    let node_20 = div()
                        .class("timeline-header")
                        .build(engine.clone(), Some(node_19));
                    {
                        let node_21 = div()
                            .class("timeline-tool")
                            .build(engine.clone(), Some(node_20));
                        {
                            let node_22 = img()
                                .image("asset://phosphor/selection.svg")
                                .class("icon-mini")
                                .build(engine.clone(), Some(node_21));
                            {}
                            node_22;
                        }
                        node_21;
                        let node_23 = div()
                            .class("timeline-tool")
                            .build(engine.clone(), Some(node_20));
                        {
                            let node_24 = img()
                                .image("asset://phosphor/cut.svg")
                                .class("icon-mini")
                                .build(engine.clone(), Some(node_23));
                            {}
                            node_24;
                        }
                        node_23;
                        let node_25 = div().class("spacer").build(engine.clone(), Some(node_20));
                        {}
                        node_25;
                        let node_26 = div()
                            .class("zoom-controls")
                            .build(engine.clone(), Some(node_20));
                        {
                            let node_27 = img()
                                .image("asset://phosphor/zoom-in.svg")
                                .class("icon-mini")
                                .build(engine.clone(), Some(node_26));
                            {}
                            node_27;
                        }
                        node_26;
                    }
                    node_20;
                    let node_28 = div()
                        .class("timeline-ruler")
                        .build(engine.clone(), Some(node_19));
                    {
                        let node_29 = div()
                            .class("ruler-mark")
                            .text("0s")
                            .build(engine.clone(), Some(node_28));
                        {}
                        node_29;
                        let node_30 = div()
                            .class("ruler-mark")
                            .text("5s")
                            .build(engine.clone(), Some(node_28));
                        {}
                        node_30;
                        let node_31 = div()
                            .class("ruler-mark")
                            .text("10s")
                            .build(engine.clone(), Some(node_28));
                        {}
                        node_31;
                        let node_32 = div()
                            .class("ruler-mark")
                            .text("15s")
                            .build(engine.clone(), Some(node_28));
                        {}
                        node_32;
                    }
                    node_28;
                    let node_33 = div().class("tracks").build(engine.clone(), Some(node_19));
                    {
                        let node_34 = div().class("track").build(engine.clone(), Some(node_33));
                        {
                            let node_35 = div()
                                .class("track-label")
                                .text("Video 1")
                                .build(engine.clone(), Some(node_34));
                            {}
                            node_35;
                            let node_36 = div()
                                .class("track-content")
                                .build(engine.clone(), Some(node_34));
                            {
                                let node_37 = div()
                                    .class("clip orange")
                                    .style("width", renderer_core::StyleValue::Px(200f32))
                                    .style("left", renderer_core::StyleValue::Px(50f32))
                                    .text("Clip_01.mp4")
                                    .build(engine.clone(), Some(node_36));
                                {}
                                node_37;
                                let node_38 = div()
                                    .class("clip orange")
                                    .style("left", renderer_core::StyleValue::Px(300f32))
                                    .style("width", renderer_core::StyleValue::Px(150f32))
                                    .text("Clip_02.mp4")
                                    .build(engine.clone(), Some(node_36));
                                {}
                                node_38;
                            }
                            node_36;
                        }
                        node_34;
                        let node_39 = div().class("track").build(engine.clone(), Some(node_33));
                        {
                            let node_40 = div()
                                .class("track-label")
                                .text("Audio 1")
                                .build(engine.clone(), Some(node_39));
                            {}
                            node_40;
                            let node_41 = div()
                                .class("track-content")
                                .build(engine.clone(), Some(node_39));
                            {
                                let node_42 = div()
                                    .class("clip blue")
                                    .style("width", renderer_core::StyleValue::Px(400f32))
                                    .style("left", renderer_core::StyleValue::Px(50f32))
                                    .text("Background_Music.wav")
                                    .build(engine.clone(), Some(node_41));
                                {}
                                node_42;
                            }
                            node_41;
                        }
                        node_39;
                    }
                    node_33;
                    let node_43 = div().class("playhead").build(engine.clone(), Some(node_19));
                    {}
                    node_43;
                }
                node_19;
            }
            node_2;
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
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.07058824f32, 0.07058824f32, 0.07058824f32, 1f32),
        );
        e.add_style_rule(".video-container".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        e.add_style_rule(".workspace".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(24f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(24f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.047058824f32, 0.047058824f32, 0.047058824f32, 1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(24f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(24f32),
        );
        e.add_style_rule("/* Preview Area */\n.preview-area".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "aspect-ratio".to_string(),
            renderer_core::StyleValue::Px(1.777f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert("/* 16".to_string(), renderer_core::StyleValue::Px(9f32));
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(50f32),
        );
        decls.insert(
            "overflow".to_string(),
            renderer_core::StyleValue::Ident("hidden".to_string()),
        );
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.6f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(80f32),
        );
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".player-container".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 1f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        e.add_style_rule(".video-mock".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 0.2f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(40f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 0.2f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 0.2f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 0.1f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(80f32));
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(80f32));
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 0.2f32),
        );
        e.add_style_rule(".play-overlay".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert("opacity".to_string(), renderer_core::StyleValue::Px(0.8f32));
        e.add_style_rule(".icon-play-large".to_string(), decls);
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
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "border-top".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(60f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".player-controls".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".control-btn".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        e.add_style_rule(".control-btn:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 1f32),
        );
        e.add_style_rule(".control-btn.active".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(20f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(20f32));
        e.add_style_rule(".icon-control".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "font-family".to_string(),
            renderer_core::StyleValue::Ident("monospace".to_string()),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.53333336f32, 0.53333336f32, 0.53333336f32, 1f32),
        );
        e.add_style_rule(".time-display".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-top".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(300f32));
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        e.add_style_rule("/* Timeline Area */\n.timeline-area".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.14509805f32, 0.14509805f32, 0.14509805f32, 1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "border-bottom".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        e.add_style_rule(".timeline-header".to_string(), decls);
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
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(28f32));
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(28f32));
        e.add_style_rule(".timeline-tool".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.21960784f32, 0.21960784f32, 0.21960784f32, 1f32),
        );
        e.add_style_rule(".timeline-tool:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(100f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(24f32));
        decls.insert(
            "border-bottom".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.101960786f32, 0.101960786f32, 0.101960786f32, 1f32),
        );
        e.add_style_rule(".timeline-ruler".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "border-left".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(100f32));
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        e.add_style_rule(".ruler-mark".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "overflow-y".to_string(),
            renderer_core::StyleValue::Ident("scroll".to_string()),
        );
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        e.add_style_rule(".tracks".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "border-bottom".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(60f32));
        e.add_style_rule(".track".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "border-right".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.13333334f32, 0.13333334f32, 0.13333334f32, 1f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.53333336f32, 0.53333336f32, 0.53333336f32, 1f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(100f32));
        e.add_style_rule(".track-label".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.08627451f32, 0.08627451f32, 0.08627451f32, 1f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
        );
        e.add_style_rule(".track-content".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("bottom".to_string(), renderer_core::StyleValue::Px(10f32));
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(10f32));
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(11f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.2f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.2f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.2f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 0.8f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.2f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        e.add_style_rule(".clip".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.827451f32, 0.32941177f32, 0f32, 1f32),
        );
        e.add_style_rule(".clip.orange".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16078432f32, 0.5019608f32, 0.7254902f32, 1f32),
        );
        e.add_style_rule(".clip.blue".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.90588236f32, 0.29803923f32, 0.23529412f32, 1f32),
        );
        decls.insert("bottom".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(350f32));
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(2f32));
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert("z-index".to_string(), renderer_core::StyleValue::Px(10f32));
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(40f32));
        e.add_style_rule(".playhead".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-right".to_string(),
            renderer_core::StyleValue::Px(5f32),
        );
        decls.insert(
            "border-left".to_string(),
            renderer_core::StyleValue::Px(5f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert(
            "border-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert(
            "content".to_string(),
            renderer_core::StyleValue::Ident("\"\"".to_string()),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(-4f32));
        e.add_style_rule(".playhead::after".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(16f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(16f32));
        e.add_style_rule(".icon-mini".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("flex-grow".to_string(), renderer_core::StyleValue::Px(1f32));
        e.add_style_rule(".spacer".to_string(), decls);
    }
}
