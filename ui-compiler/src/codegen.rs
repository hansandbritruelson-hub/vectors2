use crate::css::{self, StyleValue};
use crate::parser::{Attribute, Element, Node, Template};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;

#[derive(Clone)]
struct SlotContent {
    scope_binding: Option<String>,
    nodes: Vec<Node>,
}

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
    let script_block =
        syn::parse_str::<syn::Block>(&wrapped_script).unwrap_or_else(|_| syn::parse_str("{}").unwrap());

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

    let build_args = if has_props_struct {
        quote! { props: Props }
    } else {
        quote! { _props: Props }
    };

    let build_with_slots_args = if has_props_struct {
        quote! { props: Props, slots: Slots }
    } else {
        quote! { _props: Props, slots: Slots }
    };

    let props_forward = if has_props_struct {
        quote! { props }
    } else {
        quote! { _props }
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
        use renderer_core::signals::{ReadSignal, ToBool, ToReactiveString, create_effect, create_memo, create_signal};
        use renderer_core::ui::{Element, div, img, input, mount_if, mount_list, text};
        use std::cell::RefCell;
        use std::rc::Rc;

        // Generated Imports
        #(#import_tokens)*

        // Script Items (Structs, Use, Enums)
        #(#module_items)*

        // Default Props Definition (if none provided)
        #props_def

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
                self.values.get(key).map(|value| value.as_str()).unwrap_or("")
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

        pub fn build(engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, #build_args) -> u32 {
            build_with_slots(engine, parent, #props_forward, Slots::default())
        }

        #[allow(unused_variables)]
        pub fn build_with_slots(
            engine: Rc<RefCell<FlexEngine>>,
            parent: Option<u32>,
            #build_with_slots_args
        ) -> u32 {
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

fn is_component_name(name: &str) -> bool {
    name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

fn is_slot_syntax_attr_name(name: &str) -> bool {
    name.starts_with('#') || name == "v-slot" || name.starts_with("v-slot:")
}

fn parse_scope_binding(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "true" {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn normalize_slot_name(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        "default".to_string()
    } else {
        trimmed.to_string()
    }
}

fn slot_name_from_slot_syntax_attr(attr: &Attribute) -> Option<String> {
    if attr.name.starts_with('#') {
        let name = attr.name.trim_start_matches('#');
        return Some(normalize_slot_name(name));
    }

    if attr.name == "v-slot" {
        return Some("default".to_string());
    }

    if attr.name.starts_with("v-slot:") {
        let name = attr.name.trim_start_matches("v-slot:");
        return Some(normalize_slot_name(name));
    }

    None
}

fn slot_binding_from_template_attrs(attrs: &[Attribute]) -> Option<(String, Option<String>)> {
    let mut slot_name = None;
    let mut scope_binding = None;

    for attr in attrs {
        if is_slot_syntax_attr_name(&attr.name) {
            if slot_name.is_none() {
                slot_name = slot_name_from_slot_syntax_attr(attr);
            }
            if scope_binding.is_none() {
                scope_binding = parse_scope_binding(&attr.value);
            }
        } else if attr.name == "slot" {
            if slot_name.is_none() {
                slot_name = Some(normalize_slot_name(&attr.value));
            }
        } else if attr.name == "slot-scope" {
            if scope_binding.is_none() {
                scope_binding = parse_scope_binding(&attr.value);
            }
        }
    }

    slot_name.map(|name| (name, scope_binding))
}

fn component_default_scope_binding(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if is_slot_syntax_attr_name(&attr.name) {
            if let Some(name) = slot_name_from_slot_syntax_attr(attr) {
                if name == "default" {
                    return parse_scope_binding(&attr.value);
                }
            }
        } else if attr.name == "slot-scope" {
            return parse_scope_binding(&attr.value);
        }
    }
    None
}

fn push_slot_content(
    slots: &mut BTreeMap<String, SlotContent>,
    slot_name: String,
    scope_binding: Option<String>,
    nodes: Vec<Node>,
) {
    if nodes.is_empty() {
        return;
    }

    let entry = slots.entry(slot_name).or_insert_with(|| SlotContent {
        scope_binding: scope_binding.clone(),
        nodes: vec![],
    });

    if entry.scope_binding.is_none() {
        entry.scope_binding = scope_binding;
    }

    entry.nodes.extend(nodes);
}

fn split_component_slots(el: &Element) -> BTreeMap<String, SlotContent> {
    let mut slots: BTreeMap<String, SlotContent> = BTreeMap::new();
    let component_default_scope = component_default_scope_binding(&el.attributes);

    for child in &el.children {
        match child {
            Node::Element(child_el) => {
                if child_el.name == "template" {
                    if let Some((slot_name, scope_binding)) =
                        slot_binding_from_template_attrs(&child_el.attributes)
                    {
                        push_slot_content(
                            &mut slots,
                            slot_name,
                            scope_binding,
                            child_el.children.clone(),
                        );
                        continue;
                    }
                }

                if let Some(slot_attr) = child_el.attributes.iter().find(|attr| attr.name == "slot") {
                    let slot_name = normalize_slot_name(&slot_attr.value);
                    let mut moved_node = child_el.clone();
                    moved_node
                        .attributes
                        .retain(|attr| attr.name != "slot" && attr.name != "slot-scope");
                    push_slot_content(
                        &mut slots,
                        slot_name,
                        None,
                        vec![Node::Element(moved_node)],
                    );
                    continue;
                }

                push_slot_content(
                    &mut slots,
                    "default".to_string(),
                    component_default_scope.clone(),
                    vec![child.clone()],
                );
            }
            _ => {
                push_slot_content(
                    &mut slots,
                    "default".to_string(),
                    component_default_scope.clone(),
                    vec![child.clone()],
                );
            }
        }
    }

    slots
}

fn is_component_prop_attr(attr: &Attribute) -> bool {
    !(attr.name == "v-for"
        || attr.name == "v-if"
        || attr.name == "v-show"
        || is_slot_syntax_attr_name(&attr.name)
        || attr.name == "slot-scope")
}

fn generate_slot_renderer(
    nodes: &[Node],
    scope_binding: Option<&str>,
    id_gen: &mut u32,
    component_name: &syn::Ident,
) -> TokenStream {
    let scope_binding_tokens = if let Some(binding_raw) = scope_binding {
        let trimmed = binding_raw.trim();
        if trimmed.is_empty() || trimmed == "true" {
            quote! {}
        } else if let Ok(ident) = syn::parse_str::<syn::Ident>(trimmed) {
            quote! { let #ident = slot_scope.clone(); }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    };

    let mut slot_node_tokens = Vec::new();
    for node in nodes {
        let code = generate_node(node, Some("__slot_parent"), id_gen);
        slot_node_tokens.push(quote! { #code; });
    }

    quote! {
        Rc::new(move |engine: Rc<RefCell<FlexEngine>>, slot_parent: u32, slot_scope: self::#component_name::SlotScope| {
            let __slot_parent = slot_parent;
            #scope_binding_tokens
            #(#slot_node_tokens)*
        }) as self::#component_name::SlotRenderFn
    }
}

fn generate_component_slots(el: &Element, id_gen: &mut u32, component_name: &syn::Ident) -> TokenStream {
    let slot_map = split_component_slots(el);

    if slot_map.is_empty() {
        return quote! { self::#component_name::Slots::default() };
    }

    let default_expr = if let Some(default_slot) = slot_map.get("default") {
        let renderer = generate_slot_renderer(
            &default_slot.nodes,
            default_slot.scope_binding.as_deref(),
            id_gen,
            component_name,
        );
        quote! { Some(#renderer) }
    } else {
        quote! { None }
    };

    let mut named_insertions = Vec::new();
    for (slot_name, slot_content) in &slot_map {
        if slot_name == "default" {
            continue;
        }
        let renderer = generate_slot_renderer(
            &slot_content.nodes,
            slot_content.scope_binding.as_deref(),
            id_gen,
            component_name,
        );
        named_insertions.push(quote! {
            __named_slots.insert(#slot_name.to_string(), #renderer);
        });
    }

    quote! {
        {
            let mut __named_slots: std::collections::HashMap<String, self::#component_name::SlotRenderFn> = std::collections::HashMap::new();
            #(#named_insertions)*
            self::#component_name::Slots {
                default: #default_expr,
                named: __named_slots,
            }
        }
    }
}

fn generate_slot_outlet_code(el: &Element, parent_name: Option<&str>, id_gen: &mut u32) -> TokenStream {
    let parent_token = if let Some(p) = parent_name {
        let p_ident = format_ident!("{}", p);
        quote! { #p_ident }
    } else {
        quote! { parent.unwrap_or(0) }
    };

    let mut slot_name_dynamic_expr: Option<syn::Expr> = None;
    let mut slot_name_static = "default".to_string();
    let mut scope_entries = Vec::new();

    for attr in &el.attributes {
        if attr.name == "name" {
            slot_name_static = normalize_slot_name(&attr.value);
            continue;
        }

        if attr.name == ":name" {
            let expr: syn::Expr =
                syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("\"default\"").unwrap());
            slot_name_dynamic_expr = Some(expr);
            continue;
        }

        if attr.name == "v-if" || attr.name == "v-for" || attr.name == "v-show" {
            continue;
        }

        if attr.name.starts_with('@') || is_slot_syntax_attr_name(&attr.name) {
            continue;
        }

        if attr.name.starts_with(':') {
            let key = attr.name.trim_start_matches(':').to_string();
            let expr: syn::Expr =
                syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("\"\"").unwrap());
            scope_entries.push(quote! {
                __slot_values.insert(#key.to_string(), (#expr).to_reactive_string());
            });
            continue;
        }

        if attr.name != "slot" && attr.name != "slot-scope" {
            let key = attr.name.clone();
            let val = attr.value.clone();
            scope_entries.push(quote! {
                __slot_values.insert(#key.to_string(), #val.to_string());
            });
        }
    }

    let slot_name_code = if let Some(dynamic_expr) = slot_name_dynamic_expr {
        quote! {
            let __slot_name = (#dynamic_expr).to_reactive_string();
        }
    } else {
        quote! {
            let __slot_name = #slot_name_static.to_string();
        }
    };

    let mut fallback_tokens = Vec::new();
    for child in &el.children {
        let code = generate_node(child, parent_name, id_gen);
        fallback_tokens.push(quote! { #code; });
    }

    quote! {
        {
            #slot_name_code
            let mut __slot_values = std::collections::HashMap::new();
            #(#scope_entries)*
            let __slot_scope = SlotScope::new(__slot_values);

            let __slot_renderer = if __slot_name == "default" {
                slots.default.clone()
            } else {
                slots.get(&__slot_name)
            };

            if let Some(__render_slot) = __slot_renderer {
                __render_slot(engine.clone(), #parent_token, __slot_scope);
            } else {
                #(#fallback_tokens)*
            }

            0
        }
    }
}

fn generate_element_builder(el: &Element, id_gen: &mut u32) -> TokenStream {
    *id_gen += 1;

    // Check for Component (PascalCase)
    if is_component_name(&el.name) {
        let component_name = format_ident!("{}", el.name);
        let mut field_assignments = Vec::new();
        for attr in &el.attributes {
            if !is_component_prop_attr(attr) {
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
    let mut v_show = None;
    for attr in &el.attributes {
        if attr.name == "v-if" {
            v_if = Some(attr.value.clone());
        } else if attr.name == "v-for" {
            v_for = Some(attr.value.clone());
        } else if attr.name == "v-show" {
            v_show = Some(attr.value.clone());
        }
    }

    if let Some(condition) = v_if {
        let sanitized_condition = condition.replace('\'', "\"");
        let condition_expr: syn::Expr =
            syn::parse_str(&sanitized_condition).unwrap_or_else(|_| syn::parse_str("true").unwrap());

        let mut inner_el = el.clone();
        inner_el.attributes.retain(|a| a.name != "v-if");

        let parent_token = if let Some(p) = parent_name {
            let p_ident = format_ident!("{}", p);
            quote! { #p_ident }
        } else {
            quote! { parent.unwrap_or(0) }
        };

        let inner_parent_ident = format_ident!("__mount_if_parent");
        let inner_parent_name = inner_parent_ident.to_string();
        let inner_code = generate_element_code(&inner_el, Some(&inner_parent_name), id_gen);

        return quote! {
            {
                let engine_c = engine.clone();
                mount_if(engine.clone(), #parent_token, create_memo(
                    move || (#condition_expr).to_bool()
                ), move || {
                    let engine = engine_c.clone();
                    let #inner_parent_ident = #parent_token;
                    #inner_code
                });
                0
            }
        };
    }

    if let Some(v_for_expr) = v_for {
        let mut parts = v_for_expr.splitn(2, " in ");
        let item = parts.next().unwrap_or("item").trim();
        let collection = parts.next().unwrap_or("").trim();
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

    // <template> is a transparent fragment container.
    if el.name == "template" {
        if el.children.is_empty() {
            return quote! { 0 };
        }

        let mut fragment_tokens = Vec::new();
        for (i, child) in el.children.iter().enumerate() {
            let code = generate_node(child, parent_name, id_gen);
            if i == el.children.len() - 1 {
                fragment_tokens.push(code);
            } else {
                fragment_tokens.push(quote! { #code; });
            }
        }

        return quote! {
            {
                #(#fragment_tokens)*
            }
        };
    }

    // <slot> outlet rendering.
    if el.name == "slot" {
        return generate_slot_outlet_code(el, parent_name, id_gen);
    }

    // PascalCase Components
    if is_component_name(&el.name) {
        let component_name = format_ident!("{}", el.name);
        let parent_token = if let Some(p) = parent_name {
            let p_ident = format_ident!("{}", p);
            quote! { Some(#p_ident) }
        } else {
            quote! { None }
        };

        let mut field_assignments = Vec::new();
        for attr in &el.attributes {
            if !is_component_prop_attr(attr) {
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

        let slots_value = generate_component_slots(el, id_gen, &component_name);

        return quote! {
             self::#component_name::build_with_slots(
                engine.clone(),
                #parent_token,
                self::#component_name::Props {
                    #(#field_assignments),*
                },
                #slots_value
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

    let v_show_code = if let Some(show_condition) = v_show {
        let sanitized_show = show_condition.replace('\'', "\"");
        let show_expr: syn::Expr =
            syn::parse_str(&sanitized_show).unwrap_or_else(|_| syn::parse_str("true").unwrap());
        quote! {
            create_effect({
                let engine = engine.clone();
                move || {
                    let visible = (#show_expr).to_bool();
                    engine.borrow_mut().set_node_visible(#node_var, visible);
                }
            });
        }
    } else {
        quote! {}
    };

    quote! {
        let #node_var = #builder.build(engine.clone(), #parent_token);
        {
            #(#child_codes)*
        }
        #v_show_code
        #node_var
    }
}

fn apply_attributes(mut builder: TokenStream, attributes: &[Attribute]) -> TokenStream {
    for attr in attributes {
        if attr.name == "v-for"
            || attr.name == "v-if"
            || attr.name == "v-show"
            || is_slot_syntax_attr_name(&attr.name)
            || attr.name == "slot"
            || attr.name == "slot-scope"
        {
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
            "data" | "d" | ":data" | ":d" => {
                if attr.is_dynamic || attr.name.starts_with(':') {
                    let expr: syn::Expr = syn::parse_str(&attr.value)
                        .unwrap_or_else(|_| syn::parse_str("\"\".to_string()").unwrap());
                    builder = quote! { #builder.bind_path(create_memo({
                        let val = #expr.clone();
                        move || val.to_reactive_string()
                    })) };
                } else {
                    let val = &attr.value;
                    builder = quote! { #builder.path(#val) };
                }
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
