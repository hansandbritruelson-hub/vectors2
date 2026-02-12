use crate::parser::{Template, Node, Element};
use crate::css::{self, StyleValue};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

pub fn generate_rust(template: &Template) -> String {
    let mut id_gen = 0;
    
    // Parse CSS
    let mut all_rules = Vec::new();
    for style_content in &template.styles {
        if let Ok((_, rules)) = css::parse_css(style_content) {
            all_rules.extend(rules);
        }
    }

    let mut style_registrations = Vec::new();
    for rule in all_rules {
        let selector = rule.selector;
        let mut decl_tokens = Vec::new();
        for (prop, val) in rule.declarations {
            let val_tokens = style_value_to_tokens(&val);
            decl_tokens.push(quote! { decls.insert(#prop.to_string(), #val_tokens); });
        }
        style_registrations.push(quote! {
            {
                let mut decls = std::collections::HashMap::new();
                #(#decl_tokens)*
                e.add_style_rule(#selector.to_string(), decls);
            }
        });
    }

    let mut root_nodes = Vec::new();
    for node in &template.root {
        root_nodes.push(generate_node(node, None, &mut id_gen));
    }

    let script = template.script.as_deref().unwrap_or("");
    let script_code: TokenStream = script.parse().unwrap_or_else(|_| quote! {});
    
    let expanded = quote! {
        use renderer_core::FlexEngine;
        use renderer_core::ui::{div, text, mount_list, Element};
        use renderer_core::signals::{ReadSignal, create_effect, create_signal, create_memo, ToReactiveString};
        use std::rc::Rc;
        use std::cell::RefCell;

        pub fn build_generated_ui(engine: Rc<RefCell<FlexEngine>>) {
            // Register Styles
            register_styles(engine.clone());

            // Script Block
            #script_code

            // UI Construction
            {
                #(#root_nodes)*
            }
        }

        fn register_styles(engine: Rc<RefCell<FlexEngine>>) {
            let mut e = engine.borrow_mut();
            #(#style_registrations)*
        }
    };

    expanded.to_string()
}

fn generate_node(node: &Node, parent_name: Option<&str>, id_gen: &mut u32) -> TokenStream {
    match node {
        Node::Element(el) => generate_element_code(el, parent_name, id_gen),
        Node::Text(t) => {
            let parent_token = if let Some(p) = parent_name {
                let p_ident = format_ident!("{}", p);
                quote! { Some(#p_ident) }
            } else {
                quote! { None }
            };
            quote! {
                text(#t).build(engine.clone(), #parent_token);
            }
        },
        Node::Binding(b) => {
            let expr: syn::Expr = syn::parse_str(b).unwrap_or_else(|_| syn::parse_str("\"error\"").unwrap());
            let parent_token = if let Some(p) = parent_name {
                let p_ident = format_ident!("{}", p);
                quote! { Some(#p_ident) }
            } else {
                quote! { None }
            };
            quote! {
                div().child(text("").bind_text(create_memo({
                    let val = #expr.clone();
                    move || val.to_reactive_string()
                }))).build(engine.clone(), #parent_token);
            }
        }
    }
}

fn generate_element_builder(el: &Element, id_gen: &mut u32) -> TokenStream {
    *id_gen += 1;
    
    let mut builder = if el.name == "div" {
        quote! { div() }
    } else if el.name == "text" {
        quote! { text("") }
    } else {
        quote! { div() }
    };

    for attr in &el.attributes {
        if attr.name == "v-for" { continue; }
        
        match attr.name.as_str() {
            "class" => {
                let val = &attr.value;
                builder = quote! { #builder.class(#val) };
            }
            "style" => {
                // In a perfect world, we'd parse the inline style too. 
                // For now, let's just emit it if it's literally key:value;
                // Actually, let's try to parse it.
                let inline_style = format!("temp {{ {} }}", attr.value);
                if let Ok((_, rules)) = css::parse_css(&inline_style) {
                    if let Some(rule) = rules.first() {
                        for (prop, val) in &rule.declarations {
                            let val_tokens = style_value_to_tokens(val);
                            builder = quote! { #builder.style(#prop, #val_tokens) };
                        }
                    }
                }
            }
            "image" => {
                let val = &attr.value;
                builder = quote! { #builder.image(#val) };
            }
            _ => {
                 if attr.name == "@click" {
                     let expr: syn::Expr = syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("{}").unwrap());
                     builder = quote! { #builder.on_click(move || { #expr }) };
                 } else if attr.name == "text" || attr.name == ":text" {
                     if attr.is_dynamic || attr.name == ":text" {
                         let expr: syn::Expr = syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("\"error\"").unwrap());
                         builder = quote! { #builder.bind_text(create_memo({
                             let val = #expr.clone();
                             move || val.to_reactive_string()
                         })) };
                     } else {
                         let val = &attr.value;
                         builder = quote! { #builder.text(#val) };
                     }
                 }
            }
        }
    }

    for child in &el.children {
        match child {
            Node::Element(child_el) => {
                let child_builder = generate_element_builder(child_el, id_gen);
                builder = quote! { #builder.child(#child_builder) };
            }
            Node::Text(t) => {
                builder = quote! { #builder.child(text(#t)) };
            }
            Node::Binding(b) => {
                let expr: syn::Expr = syn::parse_str(b).unwrap_or_else(|_| syn::parse_str("\"error\"").unwrap());
                builder = quote! { #builder.child(text("").bind_text(create_memo({
                    let val = #expr.clone();
                    move || val.to_reactive_string()
                }))) };
            }
        }
    }

    builder
}

fn style_value_to_tokens(val: &StyleValue) -> TokenStream {
    match val {
        StyleValue::Px(v) => quote! { renderer_core::StyleValue::Px(#v) },
        StyleValue::Percent(v) => quote! { renderer_core::StyleValue::Percent(#v) },
        StyleValue::Em(v) => quote! { renderer_core::StyleValue::Em(#v) },
        StyleValue::Vh(v) => quote! { renderer_core::StyleValue::Vh(#v) },
        StyleValue::Vw(v) => quote! { renderer_core::StyleValue::Vw(#v) },
        StyleValue::Color(r, g, b, a) => quote! { renderer_core::StyleValue::Color(#r, #g, #b, #a) },
        StyleValue::Ident(s) => quote! { renderer_core::StyleValue::Ident(#s.to_string()) },
        StyleValue::String(s) => quote! { renderer_core::StyleValue::String(#s.to_string()) },
        StyleValue::Auto => quote! { renderer_core::StyleValue::Auto },
    }
}

fn generate_element_code(el: &Element, parent_name: Option<&str>, id_gen: &mut u32) -> TokenStream {
    let mut v_for = None;
    for attr in &el.attributes {
        if attr.name == "v-for" {
            v_for = Some(attr.value.clone());
        }
    }

    if let Some(v_for_expr) = v_for {
        let mut parts = v_for_expr.splitn(2, " in ");
        let item = parts.next().unwrap().trim();
        let collection = parts.next().unwrap().trim();
        let item_ident = format_ident!("{}", item);
        let collection_ident = format_ident!("{}", collection);

        let parent_token = if let Some(p) = parent_name {
            let p_ident = format_ident!("{}", p);
            quote! { #p_ident }
        } else {
            // v-for must have a parent in mount_list
            quote! { 0 } 
        };

        // We need to generate the builder for the element *without* the v-for attribute
        let builder = generate_element_builder(el, id_gen);

        quote! {
            mount_list(engine.clone(), #parent_token, #collection_ident, |item| item.id.clone(), move |#item_ident| {
                #builder
            });
        }
    } else {
        *id_gen += 1;
        let node_var = format_ident!("node_{}", id_gen);
        let parent_token = if let Some(p) = parent_name {
            let p_ident = format_ident!("{}", p);
            quote! { Some(#p_ident) }
        } else {
            quote! { None }
        };

        let mut builder = if el.name == "div" {
            quote! { div() }
        } else if el.name == "text" {
            quote! { text("") }
        } else {
            quote! { div() }
        };

        for attr in &el.attributes {
            match attr.name.as_str() {
                "class" => {
                    let val = &attr.value;
                    builder = quote! { #builder.class(#val) };
                }
                "style" => {
                    let inline_style = format!("temp {{ {} }}", attr.value);
                    if let Ok((_, rules)) = css::parse_css(&inline_style) {
                        if let Some(rule) = rules.first() {
                            for (prop, val) in &rule.declarations {
                                let val_tokens = style_value_to_tokens(val);
                                builder = quote! { #builder.style(#prop, #val_tokens) };
                            }
                        }
                    }
                }
                "image" => {
                    let val = &attr.value;
                    builder = quote! { #builder.image(#val) };
                }
                _ => {
                     if attr.name == "@click" {
                         let expr: syn::Expr = syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("{}").unwrap());
                         builder = quote! { #builder.on_click(move || { #expr }) };
                     } else if attr.name == "text" || attr.name == ":text" {
                         if attr.is_dynamic || attr.name == ":text" {
                             let expr: syn::Expr = syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("\"error\"").unwrap());
                             builder = quote! { #builder.bind_text(create_memo({
                                 let val = #expr.clone();
                                 move || val.to_reactive_string()
                             })) };
                         } else {
                             let val = &attr.value;
                             builder = quote! { #builder.text(#val) };
                         }
                     }
                }
            }
        }

        let child_codes: Vec<TokenStream> = el.children.iter().map(|c| generate_node(c, Some(&node_var.to_string()), id_gen)).collect();

        quote! {
            let #node_var = #builder.build(engine.clone(), #parent_token);
            {
                #(#child_codes)*
            }
        }
    }
}
