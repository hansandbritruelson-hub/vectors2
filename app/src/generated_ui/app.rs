#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
use renderer_core::signals::{
    create_effect, create_memo, create_signal, ReadSignal, ToReactiveString,
};
use renderer_core::ui::{div, mount_list, text, Element};
use renderer_core::FlexEngine;
use std::cell::RefCell;
use std::rc::Rc;
#[allow(non_snake_case)]
pub mod TestComponent;
#[derive(Clone)]
struct User {
    id: String,
    name: String,
}
#[allow(dead_code)]
#[derive(Clone)]
pub struct Props {}
#[allow(unused_variables)]
pub fn build(engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, _props: Props) {
    register_styles(engine.clone());
    let (sidebar_content, set_sidebar_content) =
        crate::signals::create_signal("SIDEBAR\n(Reactive)".to_string());
    let (count, set_count) = crate::signals::create_signal(0);
    let (users, set_users) = crate::signals::create_signal(vec![
        User {
            id: "1".into(),
            name: "Alice".into(),
        },
        User {
            id: "2".into(),
            name: "Bob".into(),
        },
    ]);
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        let mut count = 0;
        let closure = Closure::wrap(Box::new(move || {
            count += 1;
            set_sidebar_content.set(format!("SIDEBAR\nTick: {}", count));
            if count % 2 == 0 {
                set_users.set(vec![
                    User {
                        id: "1".into(),
                        name: "Alice".into(),
                    },
                    User {
                        id: "3".into(),
                        name: format!("New User {}", count),
                    },
                    User {
                        id: "2".into(),
                        name: "Bob (Moved)".into(),
                    },
                ]);
            } else {
                set_users.set(vec![
                    User {
                        id: "2".into(),
                        name: "Bob".into(),
                    },
                    User {
                        id: "1".into(),
                        name: "Alice".into(),
                    },
                ]);
            }
        }) as Box<dyn FnMut()>);
        if let Some(window) = crate::web_bindings::get_window() {
            window.set_interval(closure.as_ref().unchecked_ref(), 2000);
        }
        closure.forget();
    }
    {
        let node_1 = div().class("main").build(engine.clone(), parent);
        {
            let node_2 = div()
                .class("sidebar")
                .on_click(move || set_count.set(count.get() + 1))
                .build(engine.clone(), Some(node_1));
            {
                let node_3 = div()
                    .class("count-text")
                    .build(engine.clone(), Some(node_2));
                {
                    text("Count: ").build(engine.clone(), Some(node_3));
                    div()
                        .child(text("").bind_text(create_memo({
                            let val = count.clone();
                            move || val.to_reactive_string()
                        })))
                        .build(engine.clone(), Some(node_3));
                }
                let node_4 = div()
                    .bind_text(create_memo({
                        let val = sidebar_content.clone();
                        move || val.to_reactive_string()
                    }))
                    .build(engine.clone(), Some(node_2));
                {}
                let node_5 = div()
                    .class("icon")
                    .image("paintbrush.svg")
                    .build(engine.clone(), Some(node_2));
                {}
                let node_6 = div()
                    .class("icon")
                    .image("paintbrush.svg")
                    .build(engine.clone(), Some(node_2));
                {}
            }
            let node_7 = div()
                .class("right-pane")
                .build(engine.clone(), Some(node_1));
            {
                let node_8 = div().class("row1").build(engine.clone(), Some(node_7));
                {
                    let node_9 = div().class("r1-left").build(engine.clone(), Some(node_8));
                    {
                        text("Row 1aa - Left Div").build(engine.clone(), Some(node_9));
                    }
                    let node_10 = div().class("r1-right").build(engine.clone(), Some(node_8));
                    {
                        text("Row 1 - Right Div").build(engine.clone(), Some(node_10));
                    }
                }
                let node_11 = div()
                    .class("curve-test")
                    .style(
                        "background-color",
                        renderer_core::StyleValue::Color(
                            0.13333334f32,
                            0.13333334f32,
                            0.13333334f32,
                            1f32,
                        ),
                    )
                    .style("height", renderer_core::StyleValue::Px(150f32))
                    .build(engine.clone(), Some(node_7));
                {
                    let node_12 = div()
                        .path("M 10 10 L 90 10 L 90 90 Z")
                        .style("width", renderer_core::StyleValue::Px(100f32))
                        .style("height", renderer_core::StyleValue::Px(100f32))
                        .style(
                            "color",
                            renderer_core::StyleValue::Color(1f32, 0f32, 0f32, 1f32),
                        )
                        .build(engine.clone(), Some(node_11));
                    {}
                    let node_13 = div()
                        .path("M 10 10 C 10 10, 50 10, 50 50 C 50 90, 90 90, 90 90")
                        .style(
                            "color",
                            renderer_core::StyleValue::Color(0f32, 0f32, 1f32, 1f32),
                        )
                        .style("width", renderer_core::StyleValue::Px(100f32))
                        .style("height", renderer_core::StyleValue::Px(100f32))
                        .build(engine.clone(), Some(node_11));
                    {}
                }
                self::TestComponent::build(
                    engine.clone(),
                    Some(node_7),
                    self::TestComponent::Props {
                        text: "Hello from Prop!".to_string(),
                    },
                );
                let node_14 = div().class("row2").build(engine.clone(), Some(node_7));
                {
                    let node_15 = div().class("r2-text1").build(engine.clone(), Some(node_14));
                    {
                        text ("This is a reasonably long piece of text that is intended to test the wrapping capabilities of our flex engine. It should flow nicely within its container.") . build (engine . clone () , Some (node_15)) ;
                    }
                    let node_16 = div().class("r2-text2").build(engine.clone(), Some(node_14));
                    {
                        text ("Another long block of text here, serving as the second part of Row 2. We want to ensure that multiple wrapping blocks can coexist side-by-side in a row.") . build (engine . clone () , Some (node_16)) ;
                    }
                }
                let node_17 = div().class("row3").build(engine.clone(), Some(node_7));
                {
                    let node_18 = div().build(engine.clone(), Some(node_17));
                    {
                        text("Row 3: Keyed Reusable List (v4 Sample):")
                            .build(engine.clone(), Some(node_18));
                    }
                    mount_list(
                        engine.clone(),
                        node_17,
                        users,
                        |item| item.id.clone(),
                        move |user| {
                            div()
                                .class("user-item")
                                .child(text("").bind_text(create_memo({
                                    let val = user.name.clone();
                                    move || val.to_reactive_string()
                                })))
                        },
                    );
                }
            }
        }
    }
}
fn register_styles(engine: Rc<RefCell<FlexEngine>>) {
    #[allow(unused_mut)]
    let mut e = engine.borrow_mut();
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.101960786f32, 0.101960786f32, 0.101960786f32, 1f32),
        );
        e.add_style_rule(".main".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2509804f32, 1f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(75f32));
        e.add_style_rule(".sidebar".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 0f32, 1f32, 1f32),
        );
        e.add_style_rule(".count-text".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(64f32));
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(64f32));
        e.add_style_rule(".icon".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.14901961f32, 0.14901961f32, 0.14901961f32, 1f32),
        );
        e.add_style_rule(".right-pane".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        e.add_style_rule(".row1".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.3019608f32, 0.3019608f32, 0.34901962f32, 1f32),
        );
        e.add_style_rule(".r1-left".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.34901962f32, 0.3019608f32, 0.3019608f32, 1f32),
        );
        e.add_style_rule(".r1-right".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2509804f32, 0.2f32, 1f32),
        );
        e.add_style_rule(".row2".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2509804f32, 0.34901962f32, 0.2509804f32, 1f32),
        );
        e.add_style_rule(".r2-text1".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2509804f32, 0.2509804f32, 0.34901962f32, 1f32),
        );
        e.add_style_rule(".r2-text2".to_string(), decls);
    }
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
        e.add_style_rule(".row3".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.3019608f32, 0.7019608f32, 0.3019608f32, 1f32),
        );
        e.add_style_rule(".user-item".to_string(), decls);
    }
}
