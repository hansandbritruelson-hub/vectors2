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
    let (shape_menu_open, set_shape_menu_open) = create_signal(false);
    let root_id = {
        let node_1 = div()
            .class("smart-shape-menu")
            .build(engine.clone(), parent);
        {
            let node_2 = div()
                .class("smart-shape-trigger-hitbox")
                .on_mouse_enter(move |event: renderer_core::UiEvent| {
                    set_shape_menu_open.set(true);
                })
                .on_mouse_leave(move |event: renderer_core::UiEvent| {
                    set_shape_menu_open.set(false);
                })
                .build(engine.clone(), Some(node_1));
            {
                {
                    let __slot_name = "default".to_string();
                    let mut __slot_values = std::collections::HashMap::new();
                    let __slot_scope = SlotScope::new(__slot_values);
                    let __slot_renderer = if __slot_name == "default" {
                        slots.default.clone()
                    } else {
                        slots.get(&__slot_name)
                    };
                    if let Some(__render_slot) = __slot_renderer {
                        __render_slot(engine.clone(), node_2, __slot_scope);
                    } else {
                        let node_3 = div()
                            .class("smart-shape-trigger-icon")
                            .build(engine.clone(), Some(node_2));
                        {
                            let node_4 = img()
                                .image("asset://phosphor/square.svg")
                                .class("smart-shape-icon-img")
                                .build(engine.clone(), Some(node_3));
                            {}
                            node_4;
                        }
                        node_3;
                    }
                    0
                };
            }
            node_2;
            let node_5 = div()
                .class("smart-shape-flyout")
                .on_mouse_enter(move |event: renderer_core::UiEvent| {
                    set_shape_menu_open.set(true);
                })
                .on_mouse_leave(move |event: renderer_core::UiEvent| {
                    set_shape_menu_open.set(false);
                })
                .on_click(move |event: renderer_core::UiEvent| {
                    set_shape_menu_open.set(false);
                })
                .build(engine.clone(), Some(node_1));
            {
                {
                    let __slot_name = "flyout".to_string();
                    let mut __slot_values = std::collections::HashMap::new();
                    let __slot_scope = SlotScope::new(__slot_values);
                    let __slot_renderer = if __slot_name == "default" {
                        slots.default.clone()
                    } else {
                        slots.get(&__slot_name)
                    };
                    if let Some(__render_slot) = __slot_renderer {
                        __render_slot(engine.clone(), node_5, __slot_scope);
                    } else {
                        let node_6 = div()
                            .class("smart-shape-flyout-tool-icon")
                            .build(engine.clone(), Some(node_5));
                        {
                            let node_7 = img()
                                .image("asset://phosphor/circle.svg")
                                .class("smart-shape-icon-img")
                                .build(engine.clone(), Some(node_6));
                            {}
                            node_7;
                        }
                        node_6;
                        let node_8 = div()
                            .class("smart-shape-flyout-tool-icon")
                            .build(engine.clone(), Some(node_5));
                        {
                            let node_9 = img()
                                .image("asset://phosphor/line.svg")
                                .class("smart-shape-icon-img")
                                .build(engine.clone(), Some(node_8));
                            {}
                            node_9;
                        }
                        node_8;
                    }
                    0
                };
            }
            create_effect({
                let engine = engine.clone();
                move || {
                    let visible = (shape_menu_open == true).to_bool();
                    engine.borrow_mut().set_node_visible(node_5, visible);
                }
            });
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
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(44f32));
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(44f32));
        e.add_style_rule(".smart-shape-menu".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        e.add_style_rule(".smart-shape-trigger-hitbox".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        e.add_style_rule(".smart-shape-trigger-icon".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        e.add_style_rule(".smart-shape-trigger-icon:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        e.add_style_rule(".smart-shape-trigger-icon.active".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "left".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(24f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.23921569f32, 0.23921569f32, 0.23921569f32, 1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.23921569f32, 0.23921569f32, 0.23921569f32, 1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.23921569f32, 0.23921569f32, 0.23921569f32, 1f32),
        );
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "z-index".to_string(),
            renderer_core::StyleValue::Px(9500f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.23921569f32, 0.23921569f32, 0.23921569f32, 1f32),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.45f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.12156863f32, 0.12156863f32, 0.12156863f32, 1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        e.add_style_rule(".smart-shape-flyout".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(4f32),
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
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        e.add_style_rule(".smart-shape-flyout-tool-icon".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        e.add_style_rule(".smart-shape-flyout-tool-icon:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
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
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        e.add_style_rule(".smart-shape-flyout-tool-icon.active".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(26f32));
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(26f32));
        e.add_style_rule(".smart-shape-icon-img".to_string(), decls);
    }
}
