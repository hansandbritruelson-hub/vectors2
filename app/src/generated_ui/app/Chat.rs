#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
use renderer_core::println;
use renderer_core::signals::{
    create_effect, create_memo, create_signal, ReadSignal, ToBool, ToReactiveString,
};
use renderer_core::ui::{div, img, input, mount_if, mount_list, text, Element};
use renderer_core::FlexEngine;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
pub struct Message {
    pub id: String,
    pub text: String,
    pub sender: String,
}
pub struct Props {}
#[allow(unused_variables)]
pub fn build(engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, props: Props) -> u32 {
    register_styles(engine.clone());
    let (open, set_open) = create_signal(true);
    let (input_text, set_input_text) = create_signal("".to_string());
    let (messages, set_messages) = create_signal::<Vec<Message>>(vec![]);
    let toggle_open = Rc::new(move |_| {
        set_open.set(!open.get());
    });
    let send_message_logic = Rc::new(move |_| {
        let text = input_text.get();
        if text.len() > 0 {
            let mut current = messages.get();
            current.push(Message {
                id: format!("{}", current.len()),
                text: text,
                sender: "user".to_string(),
            });
            set_messages.set(current);
            set_input_text.set("".to_string());
        }
    });
    let (send_message, _) = create_signal(send_message_logic);
    let root_id = {
        let node_1 = div().class("chat-container").build(engine.clone(), parent);
        {
            let node_2 = div()
                .class("chat-header")
                .on_click(move |event| (toggle_open)(()))
                .build(engine.clone(), Some(node_1));
            {
                let node_3 = div()
                    .class("chat-title")
                    .text("Assistant")
                    .build(engine.clone(), Some(node_2));
                {}
                node_3;
                let node_4 = div()
                    .class("chat-toggle")
                    .value(create_memo({
                        let val = "error".clone();
                        move || val.to_reactive_string()
                    }))
                    .build(engine.clone(), Some(node_2));
                {}
                node_4;
            }
            node_2;
            {
                let engine_c = engine.clone();
                mount_if(
                    engine.clone(),
                    node_1,
                    create_memo(move || (open).to_bool()),
                    move || {
                        let engine = engine_c.clone();
                        let node_5 = div().class("chat-body").build(engine.clone(), parent);
                        {
                            let node_6 = div()
                                .class("messages-list")
                                .build(engine.clone(), Some(node_5));
                            {
                                let node_7 = div () . class ("message system") . text ("Hello! How can I help you with your design today?\n                ") . build (engine . clone () , Some (node_6)) ;
                                {}
                                node_7;
                                {
                                    mount_list(
                                        engine.clone(),
                                        node_6,
                                        messages,
                                        |item| item.id.clone(),
                                        move |msg| {
                                            div().class("message user").child(text("").value(
                                                create_memo({
                                                    let val = msg.text.clone();
                                                    move || val.to_reactive_string()
                                                }),
                                            ))
                                        },
                                    );
                                    0
                                };
                            }
                            node_6;
                            let node_9 = div()
                                .class("chat-input-area")
                                .build(engine.clone(), Some(node_5));
                            {
                                let node_10 = input()
                                    .class("chat-input")
                                    .input_type("text")
                                    .value(create_memo({
                                        let val = input_text.clone();
                                        move || val.to_reactive_string()
                                    }))
                                    .on_update_model_value(move |val| set_input_text.set(val))
                                    .build(engine.clone(), Some(node_9));
                                {}
                                node_10;
                                let node_11 = div()
                                    .class("send-button")
                                    .on_click(move |event| (send_message.get())(()))
                                    .text("Send")
                                    .build(engine.clone(), Some(node_9));
                                {}
                                node_11;
                            }
                            node_9;
                        }
                        node_5
                    },
                );
                0
            };
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
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert("bottom".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert("right".to_string(), renderer_core::StyleValue::Px(20f32));
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(320f32));
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        decls.insert(
            "border-top-left-radius".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.5f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "z-index".to_string(),
            renderer_core::StyleValue::Px(1000f32),
        );
        decls.insert(
            "border-top-right-radius".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        e.add_style_rule(".chat-container".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "border-top-right-radius".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 1f32),
        );
        decls.insert(
            "border-top-left-radius".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(48f32));
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "cursor".to_string(),
            renderer_core::StyleValue::Ident("pointer".to_string()),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("space-between".to_string()),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".chat-header".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(700f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        e.add_style_rule(".chat-title".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(700f32),
        );
        e.add_style_rule(".chat-toggle".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(400f32));
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.14509805f32, 0.14509805f32, 0.14901961f32, 1f32),
        );
        e.add_style_rule(".chat-body".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert("overflow-y".to_string(), renderer_core::StyleValue::Auto);
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert("flex".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        e.add_style_rule(".messages-list".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8784314f32, 0.8784314f32, 0.8784314f32, 1f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        decls.insert(
            "line-height".to_string(),
            renderer_core::StyleValue::Px(1.4f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        e.add_style_rule(".message".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.24313726f32, 0.24313726f32, 0.25882354f32, 1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 1f32),
        );
        e.add_style_rule(".message.system".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "align-self".to_string(),
            renderer_core::StyleValue::Ident("flex-end".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 1f32),
        );
        e.add_style_rule(".message.user".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(60f32));
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        e.add_style_rule(".chat-input-area".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        decls.insert("flex".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.1764706f32, 0.1764706f32, 0.1764706f32, 1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(16f32),
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
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        e.add_style_rule(".chat-input".to_string(), decls);
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
            "font-size".to_string(),
            renderer_core::StyleValue::Px(14f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.20392157f32, 0.59607846f32, 0.85882354f32, 1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "border-radius".to_string(),
            renderer_core::StyleValue::Px(20f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "cursor".to_string(),
            renderer_core::StyleValue::Ident("pointer".to_string()),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(700f32),
        );
        e.add_style_rule(".send-button".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16078432f32, 0.5019608f32, 0.7254902f32, 1f32),
        );
        e.add_style_rule(".send-button:hover".to_string(), decls);
    }
}
