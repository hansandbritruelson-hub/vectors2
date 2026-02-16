use crate::css::{self, StyleValue};
use crate::parser::{Attribute, Element, Node, Template};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

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

    // Process Imports
    let mut import_tokens = Vec::new();
    for import in &template.imports {
        let mod_name = format_ident!("{}", import.module_name);
        if import.is_component {
            import_tokens.push(quote! {
                #[allow(non_snake_case)]
                pub mod #mod_name;
            });
        }
    }

    // Check for Props using syn (robust)
    let script = template.script.as_deref().unwrap_or("");
    let wrapped_script = format!("{{ {} }}", script);
    let script_block = syn::parse_str::<syn::Block>(&wrapped_script)
        .unwrap_or_else(|_| syn::parse_str("{}").unwrap());

    let mut module_items = Vec::new();
    let mut function_stmts = Vec::new();

    for stmt in script_block.stmts {
        match stmt {
            syn::Stmt::Item(_) => module_items.push(stmt),
            _ => function_stmts.push(stmt),
        }
    }

    let mut has_props_struct = false;
    for stmt in &module_items {
        if let syn::Stmt::Item(syn::Item::Struct(s)) = stmt {
            if s.ident == "Props" {
                has_props_struct = true;
                break;
            }
        }
    }

    let props_def = if !has_props_struct {
        quote! {
            #[allow(dead_code)]
            #[derive(Clone)]
            pub struct Props { }
        }
    } else {
        quote! {}
    };

    // Build signature
    let build_args = if has_props_struct {
        quote! { props: Props }
    } else {
        quote! { _props: Props }
    };

    let mut root_nodes = Vec::new();
    if template.root.is_empty() {
        root_nodes.push(quote! { 0 });
    } else {
        for (i, node) in template.root.iter().enumerate() {
            let code = generate_node(node, None, &mut id_gen);
            if i == template.root.len() - 1 {
                root_nodes.push(code);
            } else {
                root_nodes.push(quote! { #code; });
            }
        }
    }

    let expanded = quote! {
        #![allow(unused_imports)]
        #![allow(non_snake_case)]
        #![allow(dead_code)]
        #![allow(unused_variables)]

        use renderer_core::FlexEngine;
        use renderer_core::ui::{div, text, input, img, mount_list, mount_if, Element};
        use renderer_core::signals::{ReadSignal, create_effect, create_signal, create_memo, ToReactiveString, ToBool};
        use std::rc::Rc;
        use std::cell::RefCell;

        // Generated Imports
        #(#import_tokens)*

        // Script Items (Structs, Use, Enums)
        #(#module_items)*

        // Default Props Definition (if none provided)
        #props_def

        #[allow(unused_variables)]
        pub fn build(engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, #build_args) -> u32 {
            // Register Styles
            register_styles(engine.clone());

            // Script Logic (let bindings, effects)
            #(#function_stmts)*

            // UI Construction
            let root_id = {
                #(#root_nodes)*
            };
            root_id
        }

        fn register_styles(engine: Rc<RefCell<FlexEngine>>) {
            #[allow(unused_mut)]
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
            let Some(text_value) = normalize_text_content(t) else {
                return quote! {};
            };
            let parent_token = if let Some(p) = parent_name {
                let p_ident = format_ident!("{}", p);
                quote! { Some(#p_ident) }
            } else {
                quote! { None }
            };
            quote! {
                div().text(#text_value).build(engine.clone(), #parent_token);
            }
        }
        Node::Binding(b) => {
            let expr: syn::Expr =
                syn::parse_str(b).unwrap_or_else(|_| syn::parse_str("\"error\"").unwrap());
            let parent_token = if let Some(p) = parent_name {
                let p_ident = format_ident!("{}", p);
                quote! { Some(#p_ident) }
            } else {
                quote! { None }
            };
            quote! {
                div().value(create_memo({
                    let val = #expr.clone();
                    move || val.to_reactive_string()
                })).build(engine.clone(), #parent_token);
            }
        }
    }
}

fn generate_element_builder(el: &Element, id_gen: &mut u32) -> TokenStream {
    *id_gen += 1;

    // Check for Component (PascalCase)
    let is_component = el
        .name
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false);

    if is_component {
        let component_name = format_ident!("{}", el.name);
        let mut field_assignments = Vec::new();
        for attr in &el.attributes {
            let field_name = format_ident!("{}", attr.name.trim_start_matches(':'));
            if attr.is_dynamic {
                let expr: syn::Expr =
                    syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("()").unwrap());
                field_assignments.push(quote! { #field_name: #expr });
            } else {
                let val = &attr.value;
                field_assignments.push(quote! { #field_name: #val.to_string() });
            }
        }

        return quote! {
            self::#component_name::build(
                engine.clone(),
                None,
                self::#component_name::Props {
                    #(#field_assignments),*
                }
            )
        };
    }

    let mut builder = match el.name.as_str() {
        "div" => quote! { div() },
        "text" => quote! { text("") },
        "input" => quote! { input() },
        "img" => quote! { img() },
        "bezier-curve" => quote! { div() },
        _ => quote! { div() },
    };

    builder = apply_attributes(builder, &el.attributes);

    for child in &el.children {
        match child {
            Node::Element(child_el) => {
                let child_builder = generate_element_builder(child_el, id_gen);
                builder = quote! { #builder.child(#child_builder) };
            }
            Node::Text(t) => {
                if let Some(text_value) = normalize_text_content(t) {
                    builder = quote! { #builder.child(div().text(#text_value)) };
                }
            }
            Node::Binding(b) => {
                let expr: syn::Expr =
                    syn::parse_str(b).unwrap_or_else(|_| syn::parse_str("\"error\"").unwrap());
                builder = quote! { #builder.child(div().value(create_memo({
                    let val = #expr.clone();
                    move || val.to_reactive_string()
                }))) };
            }
        }
    }

    builder
}

fn generate_element_code(el: &Element, parent_name: Option<&str>, id_gen: &mut u32) -> TokenStream {
    // Check for special directives first (v-if, v-for)
    let mut v_if = None;
    let mut v_for = None;
    for attr in &el.attributes {
        if attr.name == "v-if" {
            v_if = Some(attr.value.clone());
        } else if attr.name == "v-for" {
            v_for = Some(attr.value.clone());
        }
    }

    if let Some(condition) = v_if {
        let sanitized_condition = condition.replace('\'', "\"");
        let condition_expr: syn::Expr = syn::parse_str(&sanitized_condition)
            .unwrap_or_else(|_| syn::parse_str("true").unwrap());

        let mut inner_el = el.clone();
        inner_el.attributes.retain(|a| a.name != "v-if");

        let parent_token = if let Some(p) = parent_name {
            let p_ident = format_ident!("{}", p);
            quote! { #p_ident }
        } else {
            quote! { parent.unwrap_or(0) }
        };

        let inner_code = generate_element_code(&inner_el, None, id_gen);

        return quote! {
            {
                let engine_c = engine.clone();
                mount_if(engine.clone(), #parent_token, create_memo(
                    move || (#condition_expr).to_bool()
                ), move || {
                    let engine = engine_c.clone();
                    #inner_code
                });
                0
            }
        };
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
            quote! { 0 }
        };

        let builder = generate_element_builder(el, id_gen);

        return quote! {
            {
                mount_list(engine.clone(), #parent_token, #collection_ident, |item| item.id.clone(), move |#item_ident| {
                    #builder
                });
                0
            }
        };
    }

    // PascalCase Components
    let is_component = el
        .name
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false);

    if is_component {
        let component_name = format_ident!("{}", el.name);
        let parent_token = if let Some(p) = parent_name {
            let p_ident = format_ident!("{}", p);
            quote! { Some(#p_ident) }
        } else {
            quote! { None }
        };

        let mut field_assignments = Vec::new();
        for attr in &el.attributes {
            if attr.name == "v-if" || attr.name == "v-for" {
                continue;
            }
            let field_name = format_ident!("{}", attr.name.trim_start_matches(':'));
            if attr.is_dynamic {
                let expr: syn::Expr =
                    syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("()").unwrap());
                field_assignments.push(quote! { #field_name: #expr });
            } else {
                let val = &attr.value;
                field_assignments.push(quote! { #field_name: #val.to_string() });
            }
        }

        return quote! {
             self::#component_name::build(
                engine.clone(),
                #parent_token,
                self::#component_name::Props {
                    #(#field_assignments),*
                }
            )
        };
    }

    *id_gen += 1;
    let node_var = format_ident!("node_{}", id_gen);
    let parent_token = if let Some(p) = parent_name {
        let p_ident = format_ident!("{}", p);
        quote! { Some(#p_ident) }
    } else {
        quote! { parent }
    };

    let mut builder = match el.name.as_str() {
        "div" => quote! { div() },
        "text" => quote! { text("") },
        "input" => quote! { input() },
        "img" => quote! { img() },
        "bezier-curve" => quote! { div() },
        _ => quote! { div() },
    };

    builder = apply_attributes(builder, &el.attributes);

    let child_codes: Vec<TokenStream> = el
        .children
        .iter()
        .map(|c| {
            let code = generate_node(c, Some(&node_var.to_string()), id_gen);
            quote! { #code; }
        })
        .collect();

    quote! {
        let #node_var = #builder.build(engine.clone(), #parent_token);
        {
            #(#child_codes)*
        }
        #node_var
    }
}

fn apply_attributes(mut builder: TokenStream, attributes: &[Attribute]) -> TokenStream {
    for attr in attributes {
        if attr.name == "v-for" || attr.name == "v-if" {
            continue;
        }

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
            "image" | "src" => {
                let val = &attr.value;
                builder = quote! { #builder.image(#val) };
            }
            "data" | "d" => {
                let val = &attr.value;
                builder = quote! { #builder.path(#val) };
            }
            "type" => {
                let val = &attr.value;
                builder = quote! { #builder.input_type(#val) };
            }
            "@update:modelValue" => {
                let raw_val = attr.value.replace("=>", "|val|");
                let val = if (raw_val.contains('|') || raw_val.contains('{'))
                    && !raw_val.trim().starts_with("move")
                {
                    format!("move {}", raw_val)
                } else {
                    raw_val
                };
                let expr: syn::Expr = syn::parse_str(&val)
                    .unwrap_or_else(|_| syn::parse_str("move |val| {}").unwrap());
                if val.contains('|') {
                    builder = quote! { #builder.on_update_model_value(#expr) };
                } else {
                    builder = quote! { #builder.on_update_model_value(move |val| { #expr(val) }) };
                }
            }
            "@click" => {
                let handler = parse_event_handler(&attr.value);
                builder = quote! { #builder.on_click(#handler) };
            }
            "@mouseenter" => {
                let handler = parse_event_handler(&attr.value);
                builder = quote! { #builder.on_mouse_enter(#handler) };
            }
            "@mouseleave" => {
                let handler = parse_event_handler(&attr.value);
                builder = quote! { #builder.on_mouse_leave(#handler) };
            }
            "text" | ":text" | "value" | ":value" => {
                if attr.is_dynamic || attr.name.starts_with(':') {
                    let expr: syn::Expr = syn::parse_str(&attr.value)
                        .unwrap_or_else(|_| syn::parse_str("\"error\"").unwrap());
                    builder = quote! { #builder.value(create_memo({
                        let val = #expr.clone();
                        move || val.to_reactive_string()
                    })) };
                } else if let Some(text_value) = normalize_text_content(&attr.value) {
                    builder = quote! { #builder.text(#text_value) };
                }
            }
            _ => {}
        }
    }

    builder
}

fn parse_event_handler(raw: &str) -> TokenStream {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        panic!("Event handler cannot be empty");
    }

    if let Ok(closure) = syn::parse_str::<syn::ExprClosure>(trimmed) {
        return quote! { #closure };
    }

    if let Ok(block) = syn::parse_str::<syn::Block>(trimmed) {
        return quote! { move |event: renderer_core::UiEvent| #block };
    }

    if let Ok(expr) = syn::parse_str::<syn::Expr>(trimmed) {
        return match expr {
            syn::Expr::Path(_) => quote! {
                move |event: renderer_core::UiEvent| {
                    (#expr)(event);
                }
            },
            _ => quote! {
                move |event: renderer_core::UiEvent| {
                    #expr;
                }
            },
        };
    }

    panic!("Invalid event handler syntax: {}", raw)
}

fn normalize_text_content(raw: &str) -> Option<String> {
    let mut normalized = String::new();
    let mut in_whitespace = false;

    for ch in raw.chars() {
        if ch.is_whitespace() {
            if !in_whitespace {
                normalized.push(' ');
                in_whitespace = true;
            }
        } else {
            normalized.push(ch);
            in_whitespace = false;
        }
    }

    let trimmed = normalized.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn style_value_to_tokens(val: &StyleValue) -> TokenStream {
    match val {
        StyleValue::Px(v) => quote! { renderer_core::StyleValue::Px(#v) },
        StyleValue::Percent(v) => quote! { renderer_core::StyleValue::Percent(#v) },
        StyleValue::Em(v) => quote! { renderer_core::StyleValue::Em(#v) },
        StyleValue::Vh(v) => quote! { renderer_core::StyleValue::Vh(#v) },
        StyleValue::Vw(v) => quote! { renderer_core::StyleValue::Vw(#v) },
        StyleValue::Color(r, g, b, a) => {
            quote! { renderer_core::StyleValue::Color(#r, #g, #b, #a) }
        }
        StyleValue::Ident(s) => quote! { renderer_core::StyleValue::Ident(#s.to_string()) },
        StyleValue::String(s) => quote! { renderer_core::StyleValue::String(#s.to_string()) },
        StyleValue::Auto => quote! { renderer_core::StyleValue::Auto },
    }
}
