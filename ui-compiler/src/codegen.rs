use crate::parser::{Template, Node, Element};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

pub fn generate_rust(template: &Template) -> String {
    let mut id_gen = 0;
    let mut root_nodes = Vec::new();
    for node in &template.root {
        root_nodes.push(generate_node(node, None, &mut id_gen));
    }

    let script = template.script.as_deref().unwrap_or("");
    let script_code: TokenStream = script.parse().unwrap_or_else(|_| quote! {});
    
    let expanded = quote! {
        use crate::FlexEngine;
        use crate::ui::{div, text, mount_list, Element};
        use crate::signals::{ReadSignal, create_effect, create_signal};
        use std::rc::Rc;
        use std::cell::RefCell;

        pub fn build_generated_ui(engine: Rc<RefCell<FlexEngine>>) {
            // Script Block
            #script_code

            // UI Construction
            {
                #(#root_nodes)*
            }
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
                div().child(text(&#expr)).build(engine.clone(), #parent_token);
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
            "width" => {
                let val: f32 = attr.value.parse().unwrap_or(0.0);
                builder = quote! { #builder.width(#val) };
            }
            "height" => {
                let val: f32 = attr.value.parse().unwrap_or(0.0);
                builder = quote! { #builder.height(#val) };
            }
            "color" => {
                let colors: Vec<f32> = attr.value.split(',').map(|s| s.trim().parse().unwrap_or(0.0)).collect();
                if colors.len() == 4 {
                    let (r, g, b, a) = (colors[0], colors[1], colors[2], colors[3]);
                    builder = quote! { #builder.color(#r, #g, #b, #a) };
                }
            }
            "image" => {
                let val = &attr.value;
                builder = quote! { #builder.image(#val) };
            }
            "row" => { builder = quote! { #builder.row() }; }
            "col" => { builder = quote! { #builder.col() }; }
            _ => {
                 if attr.is_dynamic {
                     let expr: syn::Expr = syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("\"error\"").unwrap());
                     if attr.name == "text" {
                         if let syn::Expr::Path(_) = expr {
                             builder = quote! { #builder.bind_text(#expr) };
                         } else {
                             builder = quote! { #builder.text(&#expr) };
                         }
                     } else if attr.name == "flags" {
                         builder = quote! { #builder.bind_flags(#expr) };
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
                builder = quote! { #builder.child(text(&#expr)) };
            }
        }
    }

    builder
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
                "width" => {
                    let val: f32 = attr.value.parse().unwrap_or(0.0);
                    builder = quote! { #builder.width(#val) };
                }
                "height" => {
                    let val: f32 = attr.value.parse().unwrap_or(0.0);
                    builder = quote! { #builder.height(#val) };
                }
                "color" => {
                    let colors: Vec<f32> = attr.value.split(',').map(|s| s.trim().parse().unwrap_or(0.0)).collect();
                    if colors.len() == 4 {
                        let (r, g, b, a) = (colors[0], colors[1], colors[2], colors[3]);
                        builder = quote! { #builder.color(#r, #g, #b, #a) };
                    }
                }
                "image" => {
                    let val = &attr.value;
                    builder = quote! { #builder.image(#val) };
                }
                "row" => { builder = quote! { #builder.row() }; }
                "col" => { builder = quote! { #builder.col() }; }
                _ => {
                     if attr.is_dynamic {
                         let expr: syn::Expr = syn::parse_str(&attr.value).unwrap_or_else(|_| syn::parse_str("\"error\"").unwrap());
                         if attr.name == "text" {
                             if let syn::Expr::Path(_) = expr {
                                 builder = quote! { #builder.bind_text(#expr) };
                             } else {
                                 builder = quote! { #builder.text(&#expr) };
                             }
                         } else if attr.name == "flags" {
                             builder = quote! { #builder.bind_flags(#expr) };
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
