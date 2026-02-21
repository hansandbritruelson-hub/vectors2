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
    let root_id = {
        let node_1 = div().class("demo-root").build(engine.clone(), parent);
        {
            let node_2 = div()
                .class("demo-header")
                .build(engine.clone(), Some(node_1));
            {
                let node_3 = div()
                    .class("demo-title")
                    .build(engine.clone(), Some(node_2));
                {
                    div()
                        .text("CSS Feature Demo")
                        .build(engine.clone(), Some(node_3));
                }
                node_3;
                let node_4 = div()
                    .class("demo-subtitle")
                    .build(engine.clone(), Some(node_2));
                {
                    div()
                        .text("Showcasing all currently supported style groups")
                        .build(engine.clone(), Some(node_4));
                }
                node_4;
            }
            node_2;
            let node_5 = div().class("demo-grid").build(engine.clone(), Some(node_1));
            {
                let node_6 = div().class("demo-row").build(engine.clone(), Some(node_5));
                {
                    let node_7 = div()
                        .class("demo-card card-a")
                        .build(engine.clone(), Some(node_6));
                    {
                        let node_8 = div()
                            .class("card-title")
                            .build(engine.clone(), Some(node_7));
                        {
                            div()
                                .text("Layout + Sizing")
                                .build(engine.clone(), Some(node_8));
                        }
                        node_8;
                        let node_9 = div()
                            .class("layout-stage")
                            .build(engine.clone(), Some(node_7));
                        {
                            let node_10 = div()
                                .class("layout-item one")
                                .build(engine.clone(), Some(node_9));
                            {
                                div().text("100 x 44").build(engine.clone(), Some(node_10));
                            }
                            node_10;
                            let node_11 = div()
                                .class("layout-item two")
                                .build(engine.clone(), Some(node_9));
                            {
                                div().text("130 x 36").build(engine.clone(), Some(node_11));
                            }
                            node_11;
                            let node_12 = div()
                                .class("layout-item three")
                                .build(engine.clone(), Some(node_9));
                            {
                                div().text("88 x 52").build(engine.clone(), Some(node_12));
                            }
                            node_12;
                        }
                        node_9;
                        let node_13 = div()
                            .class("feature-note")
                            .build(engine.clone(), Some(node_7));
                        {
                            div () . text ("`width` `height` `flex-direction` `justify-content` `align-items`") . build (engine . clone () , Some (node_13)) ;
                        }
                        node_13;
                    }
                    node_7;
                    let node_14 = div()
                        .class("demo-card card-b")
                        .build(engine.clone(), Some(node_6));
                    {
                        let node_15 = div()
                            .class("card-title")
                            .build(engine.clone(), Some(node_14));
                        {
                            div()
                                .text("Spacing + Shadow")
                                .build(engine.clone(), Some(node_15));
                        }
                        node_15;
                        let node_16 = div()
                            .class("spacing-stage")
                            .build(engine.clone(), Some(node_14));
                        {
                            let node_17 = div()
                                .class("margin-shell")
                                .build(engine.clone(), Some(node_16));
                            {
                                let node_18 = div()
                                    .class("padding-shell")
                                    .build(engine.clone(), Some(node_17));
                                {
                                    let node_19 = div()
                                        .class("content-chip")
                                        .build(engine.clone(), Some(node_18));
                                    {
                                        div().text("content").build(engine.clone(), Some(node_19));
                                    }
                                    node_19;
                                }
                                node_18;
                            }
                            node_17;
                            let node_20 = div()
                                .class("shadow-chip")
                                .build(engine.clone(), Some(node_16));
                            {
                                div().text("shadow").build(engine.clone(), Some(node_20));
                            }
                            node_20;
                        }
                        node_16;
                        let node_21 = div()
                            .class("feature-note")
                            .build(engine.clone(), Some(node_14));
                        {
                            div()
                                .text("`margin` `padding` `box-shadow`")
                                .build(engine.clone(), Some(node_21));
                        }
                        node_21;
                    }
                    node_14;
                    let node_22 = div()
                        .class("demo-card card-c")
                        .build(engine.clone(), Some(node_6));
                    {
                        let node_23 = div()
                            .class("card-title")
                            .build(engine.clone(), Some(node_22));
                        {
                            div()
                                .text("Borders + Outline")
                                .build(engine.clone(), Some(node_23));
                        }
                        node_23;
                        let node_24 = div().class("chip-row").build(engine.clone(), Some(node_22));
                        {
                            let node_25 = div()
                                .class("chip chip-red")
                                .build(engine.clone(), Some(node_24));
                            {
                                div().text("top").build(engine.clone(), Some(node_25));
                            }
                            node_25;
                            let node_26 = div()
                                .class("chip chip-green")
                                .build(engine.clone(), Some(node_24));
                            {
                                div().text("right").build(engine.clone(), Some(node_26));
                            }
                            node_26;
                            let node_27 = div()
                                .class("chip chip-blue")
                                .build(engine.clone(), Some(node_24));
                            {
                                div().text("left").build(engine.clone(), Some(node_27));
                            }
                            node_27;
                        }
                        node_24;
                        let node_28 = div()
                            .class("feature-note")
                            .build(engine.clone(), Some(node_22));
                        {
                            div () . text ("`border` `border-width` `border-color` `outline` `outline-offset`") . build (engine . clone () , Some (node_28)) ;
                        }
                        node_28;
                    }
                    node_22;
                }
                node_6;
                let node_29 = div().class("demo-row").build(engine.clone(), Some(node_5));
                {
                    let node_30 = div()
                        .class("demo-card card-d")
                        .build(engine.clone(), Some(node_29));
                    {
                        let node_31 = div()
                            .class("card-title")
                            .build(engine.clone(), Some(node_30));
                        {
                            div()
                                .text("Position + Z Index")
                                .build(engine.clone(), Some(node_31));
                        }
                        node_31;
                        let node_32 = div()
                            .class("position-stage")
                            .build(engine.clone(), Some(node_30));
                        {
                            let node_33 = div()
                                .class("square back")
                                .build(engine.clone(), Some(node_32));
                            {
                                div().text("Back").build(engine.clone(), Some(node_33));
                            }
                            node_33;
                            let node_34 = div()
                                .class("square front")
                                .build(engine.clone(), Some(node_32));
                            {
                                div().text("Front").build(engine.clone(), Some(node_34));
                            }
                            node_34;
                        }
                        node_32;
                        let node_35 = div()
                            .class("feature-note")
                            .build(engine.clone(), Some(node_30));
                        {
                            div()
                                .text("`position` `top` `left` `z-index`")
                                .build(engine.clone(), Some(node_35));
                        }
                        node_35;
                    }
                    node_30;
                    let node_36 = div()
                        .class("demo-card card-e")
                        .build(engine.clone(), Some(node_29));
                    {
                        let node_37 = div()
                            .class("card-title")
                            .build(engine.clone(), Some(node_36));
                        {
                            div()
                                .text("Typography")
                                .build(engine.clone(), Some(node_37));
                        }
                        node_37;
                        let node_38 = div()
                            .class("type-large")
                            .build(engine.clone(), Some(node_36));
                        {
                            div()
                                .text("Typography Demo")
                                .build(engine.clone(), Some(node_38));
                        }
                        node_38;
                        let node_39 = div()
                            .class("type-row type-center")
                            .build(engine.clone(), Some(node_36));
                        {
                            div()
                                .text("center align + wider spacing")
                                .build(engine.clone(), Some(node_39));
                        }
                        node_39;
                        let node_40 = div()
                            .class("type-row type-right")
                            .build(engine.clone(), Some(node_36));
                        {
                            div()
                                .text("italic right aligned")
                                .build(engine.clone(), Some(node_40));
                        }
                        node_40;
                        let node_41 = div()
                            .class("type-transform")
                            .build(engine.clone(), Some(node_36));
                        {
                            div()
                                .text("mixed case becomes uppercase")
                                .build(engine.clone(), Some(node_41));
                        }
                        node_41;
                        let node_42 = div()
                            .class("feature-note")
                            .build(engine.clone(), Some(node_36));
                        {
                            div () . text ("`font-size` `font-weight` `font-style` `line-height` `letter-spacing` `word-spacing` `text-align` `text-transform`") . build (engine . clone () , Some (node_42)) ;
                        }
                        node_42;
                    }
                    node_36;
                    let node_43 = div()
                        .class("demo-card card-f")
                        .build(engine.clone(), Some(node_29));
                    {
                        let node_44 = div()
                            .class("card-title")
                            .build(engine.clone(), Some(node_43));
                        {
                            div()
                                .text("Color + Paint")
                                .build(engine.clone(), Some(node_44));
                        }
                        node_44;
                        let node_45 = div()
                            .class("paint-text")
                            .build(engine.clone(), Some(node_43));
                        {
                            div()
                                .text("Text color and background color")
                                .build(engine.clone(), Some(node_45));
                        }
                        node_45;
                        let node_46 = div()
                            .class("paint-vector")
                            .build(engine.clone(), Some(node_43));
                        {
                            div()
                                .text("Fill Stroke")
                                .build(engine.clone(), Some(node_46));
                        }
                        node_46;
                        let node_47 = div()
                            .class("feature-note")
                            .build(engine.clone(), Some(node_43));
                        {
                            div()
                                .text("`background-color` `color` `fill` `stroke` `stroke-width`")
                                .build(engine.clone(), Some(node_47));
                        }
                        node_47;
                    }
                    node_43;
                }
                node_29;
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
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.078431375f32, 0.078431375f32, 0.078431375f32, 1f32),
        );
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
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
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        e.add_style_rule(".demo-root".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        e.add_style_rule(".demo-header".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(30f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
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
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.61960787f32, 0.61960787f32, 0.61960787f32, 1f32),
        );
        e.add_style_rule(".demo-subtitle".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        e.add_style_rule(".demo-grid".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("space-between".to_string()),
        );
        e.add_style_rule(".demo-row".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".demo-row:last-child".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(32f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.35f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.1764706f32, 0.1764706f32, 1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(250f32));
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.1764706f32, 0.1764706f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.1764706f32, 0.1764706f32, 1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Percent(1.5f32),
        );
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.1764706f32, 0.1764706f32, 1f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.12156863f32, 0.12156863f32, 0.12156863f32, 1f32),
        );
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
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
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        e.add_style_rule(".card-title".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.25882354f32, 0.36078432f32, 1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.10980392f32, 0.13333334f32, 0.18039216f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.25882354f32, 0.36078432f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.25882354f32, 0.36078432f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.25882354f32, 0.36078432f32, 1f32),
        );
        e.add_style_rule(".card-a".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.06666667f32, 0.09803922f32, 0.13725491f32, 1f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("space-evenly".to_string()),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(140f32));
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        e.add_style_rule(".layout-stage".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8627451f32, 0.9137255f32, 1f32, 1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(11f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.47058824f32, 0.6431373f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.47058824f32, 0.6431373f32, 1f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.47058824f32, 0.6431373f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.47058824f32, 0.6431373f32, 1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.23921569f32, 0.33333334f32, 1f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        e.add_style_rule(".layout-item".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(44f32));
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(100f32));
        e.add_style_rule(".layout-item.one".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(130f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(36f32));
        e.add_style_rule(".layout-item.two".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(88f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(52f32));
        e.add_style_rule(".layout-item.three".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("space-evenly".to_string()),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        e.add_style_rule(".chip-row".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.16862746f32, 0.16862746f32, 1f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(3f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        e.add_style_rule(".chip".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "outline-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "outline-color-bottom".to_string(),
            renderer_core::StyleValue::Color(1f32, 0.6039216f32, 0.6039216f32, 1f32),
        );
        decls.insert(
            "outline-color-left".to_string(),
            renderer_core::StyleValue::Color(1f32, 0.6039216f32, 0.6039216f32, 1f32),
        );
        decls.insert(
            "outline-offset".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.78039217f32, 0.26666668f32, 0.20784314f32, 1f32),
        );
        decls.insert(
            "outline-color-right".to_string(),
            renderer_core::StyleValue::Color(1f32, 0.6039216f32, 0.6039216f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.90588236f32, 0.29803923f32, 0.23529412f32, 1f32),
        );
        decls.insert(
            "outline-color-top".to_string(),
            renderer_core::StyleValue::Color(1f32, 0.6039216f32, 0.6039216f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.7490196f32, 0.24705882f32, 0.19607843f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.6509804f32, 0.21568628f32, 0.16862746f32, 1f32),
        );
        e.add_style_rule(".chip-red".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "outline-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "outline-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.56078434f32, 1f32, 0.74509805f32, 1f32),
        );
        decls.insert(
            "outline-color-left".to_string(),
            renderer_core::StyleValue::Color(0.56078434f32, 1f32, 0.74509805f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.15686275f32, 0.7254902f32, 0.4f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.14117648f32, 0.62352943f32, 0.34901962f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.18039216f32, 0.8f32, 0.44313726f32, 1f32),
        );
        decls.insert(
            "outline-offset".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.16470589f32, 0.72156864f32, 0.4f32, 1f32),
        );
        decls.insert(
            "outline-color-top".to_string(),
            renderer_core::StyleValue::Color(0.56078434f32, 1f32, 0.74509805f32, 1f32),
        );
        decls.insert(
            "outline-color-right".to_string(),
            renderer_core::StyleValue::Color(0.56078434f32, 1f32, 0.74509805f32, 1f32),
        );
        e.add_style_rule(".chip-green".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "outline-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.5568628f32, 0.8117647f32, 1f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.18431373f32, 0.5372549f32, 0.76862746f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.18431373f32, 0.5372549f32, 0.76862746f32, 1f32),
        );
        decls.insert(
            "outline-color-left".to_string(),
            renderer_core::StyleValue::Color(0.5568628f32, 0.8117647f32, 1f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.16470589f32, 0.47058824f32, 0.6784314f32, 1f32),
        );
        decls.insert(
            "outline-offset".to_string(),
            renderer_core::StyleValue::Px(3f32),
        );
        decls.insert(
            "outline-color-right".to_string(),
            renderer_core::StyleValue::Color(0.5568628f32, 0.8117647f32, 1f32, 1f32),
        );
        decls.insert(
            "outline-color-top".to_string(),
            renderer_core::StyleValue::Color(0.5568628f32, 0.8117647f32, 1f32, 1f32),
        );
        decls.insert(
            "outline-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        e.add_style_rule(".chip-blue".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.3529412f32, 0.26666668f32, 0.12941177f32, 1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.14117648f32, 0.1254902f32, 0.101960786f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.3529412f32, 0.26666668f32, 0.12941177f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.3529412f32, 0.26666668f32, 0.12941177f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.3529412f32, 0.26666668f32, 0.12941177f32, 1f32),
        );
        e.add_style_rule(".card-b".to_string(), decls);
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
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.09803922f32, 0.078431375f32, 0.05490196f32, 1f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(140f32));
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        e.add_style_rule(".spacing-stage".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(18f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(18f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.41960785f32, 0.23137255f32, 0.07058824f32, 1f32),
        );
        e.add_style_rule(".margin-shell".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.70980394f32, 0.41568628f32, 0.15294118f32, 1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(22f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        e.add_style_rule(".padding-shell".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.96862745f32, 0.84705883f32, 0.69803923f32, 1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.16470589f32, 0.105882354f32, 0.050980393f32, 1f32),
        );
        e.add_style_rule(".content-chip".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.9490196f32, 0.9490196f32, 0.9490196f32, 1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.16862746f32, 0.16862746f32, 1f32),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(3f32),
        );
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.45f32),
        );
        e.add_style_rule(".shadow-chip".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "outline-color-right".to_string(),
            renderer_core::StyleValue::Color(0f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "outline-offset".to_string(),
            renderer_core::StyleValue::Px(3f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "outline-color-left".to_string(),
            renderer_core::StyleValue::Color(0f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.48235294f32, 1f32, 0.62352943f32, 1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(3f32),
        );
        decls.insert(
            "outline-color-top".to_string(),
            renderer_core::StyleValue::Color(0f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(1f32, 0.48235294f32, 0.48235294f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(1f32, 0.8862745f32, 0.48235294f32, 1f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.48235294f32, 0.7176471f32, 1f32, 1f32),
        );
        decls.insert(
            "outline-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "outline-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        e.add_style_rule(".card-c".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-right-width".to_string(),
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
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.18431373f32, 0.18431373f32, 0.18431373f32, 1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        e.add_style_rule(".card-d".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.16862746f32, 0.16862746f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.16862746f32, 0.16862746f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.16862746f32, 0.16862746f32, 1f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
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
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(160f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.09019608f32, 0.09019608f32, 0.09019608f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.16862746f32, 0.16862746f32, 0.16862746f32, 1f32),
        );
        e.add_style_rule(".position-stage".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(70f32));
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(100f32));
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(13f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        e.add_style_rule(".square".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(32f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16078432f32, 0.5019608f32, 0.7254902f32, 1f32),
        );
        decls.insert("z-index".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(44f32));
        e.add_style_rule(".back".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.90588236f32, 0.29803923f32, 0.23529412f32, 1f32),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(70f32));
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.4f32),
        );
        decls.insert("z-index".to_string(), renderer_core::StyleValue::Px(5f32));
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(70f32));
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        e.add_style_rule(".front".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.2901961f32, 0.25490198f32, 0.39607844f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.2901961f32, 0.25490198f32, 0.39607844f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.2901961f32, 0.25490198f32, 0.39607844f32, 1f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.95686275f32, 0.9490196f32, 1f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.2901961f32, 0.25490198f32, 0.39607844f32, 1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.12156863f32, 0.11372549f32, 0.15294118f32, 1f32),
        );
        e.add_style_rule(".card-e".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "line-height".to_string(),
            renderer_core::StyleValue::Px(1.1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(26f32),
        );
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(700f32),
        );
        e.add_style_rule(".type-large".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.18039216f32, 0.16078432f32, 0.2509804f32, 1f32),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        e.add_style_rule(".type-row".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "word-spacing".to_string(),
            renderer_core::StyleValue::Em(0.25f32),
        );
        decls.insert(
            "letter-spacing".to_string(),
            renderer_core::StyleValue::Em(0.1f32),
        );
        decls.insert(
            "text-align".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        e.add_style_rule(".type-center".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(300f32),
        );
        decls.insert(
            "text-align".to_string(),
            renderer_core::StyleValue::Ident("right".to_string()),
        );
        decls.insert(
            "font-style".to_string(),
            renderer_core::StyleValue::Ident("italic".to_string()),
        );
        e.add_style_rule(".type-right".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8117647f32, 0.78039217f32, 1f32, 1f32),
        );
        decls.insert(
            "text-transform".to_string(),
            renderer_core::StyleValue::Ident("uppercase".to_string()),
        );
        decls.insert(
            "letter-spacing".to_string(),
            renderer_core::StyleValue::Em(0.07f32),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        e.add_style_rule(".type-transform".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.13725491f32, 0.12941177f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.24313726f32, 0.36862746f32, 0.32156864f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.24313726f32, 0.36862746f32, 0.32156864f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.24313726f32, 0.36862746f32, 0.32156864f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.24313726f32, 0.36862746f32, 0.32156864f32, 1f32),
        );
        e.add_style_rule(".card-f".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.08627451f32, 0.2627451f32, 0.21176471f32, 1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.56078434f32, 1f32, 0.8156863f32, 1f32),
        );
        e.add_style_rule(".paint-text".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "fill".to_string(),
            renderer_core::StyleValue::Color(0.94509804f32, 0.76862746f32, 0.05882353f32, 1f32),
        );
        decls.insert(
            "stroke".to_string(),
            renderer_core::StyleValue::Color(0.06666667f32, 0.06666667f32, 0.06666667f32, 1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "stroke-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(700f32),
        );
        e.add_style_rule(".paint-vector".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.7019608f32, 0.7019608f32, 0.7019608f32, 1f32),
        );
        decls.insert(
            "line-height".to_string(),
            renderer_core::StyleValue::Px(1.3f32),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        e.add_style_rule(".feature-note".to_string(), decls);
    }
}
