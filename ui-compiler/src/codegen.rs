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
    let script_block = syn::parse_str::<syn::Block>(&wrapped_script).unwrap_or_else(|_| syn::parse_str("{}").unwrap());
    
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
        quote! { }
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
        use renderer_core::ui::{div, text, mount_list, mount_if, Element};
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
    
    // Check for Component (PascalCase)
    let is_component = el.name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false);

    if is_component {
        let component_name = format_ident!("{}", el.name);
        
        let mut field_assignments = Vec::new();
        
        for attr in &el.attributes {
            let field_name = format_ident!("{}", attr.name.trim_start_matches(':'));
             if attr.is_dynamic {
                 let expr: syn::Expr = syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("()").unwrap());
                 field_assignments.push(quote! { #field_name: #expr });
             } else {
                 let val = &attr.value;
                 field_assignments.push(quote! { #field_name: #val.to_string() });
             }
        }

        // For components, we return a block that calls build()
        return quote! {
            self::#component_name::build(
                engine.clone(), 
                // Parent handling for components is tricky in this builder context
                // We'll need to adapt the architecture later for proper component composition inside children
                // For now, let's assume components are built imperatively
                None, 
                self::#component_name::Props {
                    #(#field_assignments),*
                }
            )
        };
    }

    let mut builder = if el.name == "div" {
        quote! { div() }
    } else if el.name == "text" {
        quote! { text("") }
    } else if el.name == "bezier-curve" {
        // bezier-curve support
        quote! { div() } 
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
            "data" | "d" => {
                let val = &attr.value;
                builder = quote! { #builder.path(#val) };
            }
            _ => {
                 if attr.name == "@click" {
                     let sanitized_value = attr.value.replace('\'', "\"");
                     let expr: syn::Expr = syn::parse_str(&sanitized_value).unwrap_or_else(|_| syn::parse_str("{}").unwrap());
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
                
                // If child is a component, it returns () from build().
                // We can't child() it. 
                // Components must be top-level or handled differently.
                // Or we make components return their root ID?
                // The current build() signature returns ().
                // If we change build() to return u32, we can compose.
                
                // Fallback: If "child_builder" starts with "self::", it's a component call.
                // We should probably just emit it separately? 
                // But 'child()' expects an Element.
                
                // Fix: Components should inject themselves via side-effect (build calls).
                // They attach to 'parent'.
                
                // But here we are building the *Builder* for the *Parent*.
                // The parent hasn't been built yet!
                // We cannot pass the parent ID to the child yet.
                
                // This implies a fundamental change in how children are handled.
                // Existing: builder.child(child_builder) -> adds to 'children' vec.
                // New: 
                // If child is element: add to children vec.
                // If child is component: ???
                
                // Temporary solution implemented below in generate_element_code to handle this mix.
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
        // Sanitize condition for Rust: replace single quotes with double quotes
        let sanitized_condition = condition.replace('\'', "\"");
        let condition_expr: syn::Expr = syn::parse_str(&sanitized_condition).unwrap_or_else(|_| syn::parse_str("true").unwrap());
        
        let mut inner_el = el.clone();
        inner_el.attributes.retain(|a| a.name != "v-if");
        
        let parent_token = if let Some(p) = parent_name {
            let p_ident = format_ident!("{}", p);
            quote! { #p_ident }
        } else {
            quote! { parent.unwrap_or(0) }
        };

        // We build the inner element code BUT we need to make sure the closure captures the correct logic.
        // The previous implementation was parsing condition_expr outside the memoized closure,
        // which is fine, but we need to ensure the expression is evaluated INSIDE the closure for reactivity.
        
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

    // Now check for Component (PascalCase)
    let is_component = el.name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false);

    if is_component {
        let component_name = format_ident!("{}", el.name);
        
        // Parent Token
         let parent_token = if let Some(p) = parent_name {
            let p_ident = format_ident!("{}", p);
            quote! { Some(#p_ident) }
        } else {
            quote! { None } 
        };
        
        // Props Construction
        let mut field_assignments = Vec::new();
        for attr in &el.attributes {
            if attr.name == "v-if" || attr.name == "v-for" { continue; }
            let field_name = format_ident!("{}", attr.name.trim_start_matches(':'));
             if attr.is_dynamic {
                 let expr: syn::Expr = syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("()").unwrap());
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
            quote! { None } // This should probably be 'parent' argument if at root?
        };
        
        // Correct parent token logic for root nodes
        let actual_parent_token = if parent_name.is_none() {
            quote! { parent } // Use the function argument 'parent'
        } else {
            parent_token
        };

        let mut builder = if el.name == "div" {
            quote! { div() }
        } else if el.name == "text" {
            quote! { text("") }
        } else if el.name == "bezier-curve" {
            quote! { div() }
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
                "data" | "d" => {
                    let val = &attr.value;
                    builder = quote! { #builder.path(#val) };
                }
                _ => {
                     if attr.name == "@click" {
                         let sanitized_value = attr.value.replace('\'', "\"");
                         let expr: syn::Expr = syn::parse_str(&sanitized_value).unwrap_or_else(|_| syn::parse_str("{}").unwrap());
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

        // Child logic improvement
        // We separate children into Elements (built via builder) and Components (built via side-effect)
        // If it's a Component, we can't use .child().
        // We MUST build the parent node first, then pass its ID to the children.
        // The current builder pattern assumes .child( Element ) returns Self.
        // But Components return ().
        
        // Solution:
        // 1. Build 'builder' (the current node).
        // 2. Iterate children.
        //    If Element -> recursively call.
        //    If Component -> call Component::build(engine, Some(node_var), props).
        
        // We need to NOT adds children to the builder if they are actually components or if we want to build them imperatively.
        // Actually, 'generate_element_builder' handles the recursive .child() calls.
        // We should STOP doing that in 'generate_element_builder' if we are moving to imperative style here!
        
        // BUT 'generate_element_builder' is used by v-for, which needs an Element return.
        // v-for lambda must return Element.
        // So v-for content CANNOT contain top-level Components unless we wrap them in a div?
        // Or unless 'Component::build' returns the root ID?
        // Returning ID is safer. 
        // Let's change Component::build to return u32 (Root ID) or Option<u32>.
        // But for now, let's stick to imperative construction in 'generate_element_code'.
        
        // We need to Filter children in 'generate_element_builder' to ONLY include Elements that can be built inline?
        // Or we just don't add children in 'generate_element_builder' at all, and do it all here?
        // 'builder' pattern supports adding children.
        // If we do it here, we use 'engine.set_parent(child_id, node_var)'.
        
        // Let's modify 'generate_element_builder' to NOT recurse children.
        // We will loop children HERE in 'generate_element_code'.
        
        let child_codes: Vec<TokenStream> = el.children.iter().map(|c| {
            let code = generate_node(c, Some(&node_var.to_string()), id_gen);
            quote! { #code; }
        }).collect();

        quote! {
            let #node_var = #builder.build(engine.clone(), #actual_parent_token);
            {
                #(#child_codes)*
            }
            #node_var
        }
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
