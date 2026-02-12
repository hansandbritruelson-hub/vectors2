#![allow(unused_imports)]
#![allow(dead_code)]
use nom::{
    bytes::complete::{tag, take_until, is_not, take_while1},
    character::complete::{multispace0, multispace1, alphanumeric1, hex_digit1},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    combinator::{map, opt, recognize},
    branch::alt,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum StyleValue {
    Px(f32),
    Percent(f32),
    Em(f32),
    Vh(f32),
    Vw(f32),
    Color(f32, f32, f32, f32),
    Ident(String),
    String(String),
    Auto,
}

#[derive(Debug, Clone)]
pub struct StyleRule {
    pub selector: String,
    pub declarations: HashMap<String, StyleValue>,
}

fn parse_f32(input: &str) -> IResult<&str, f32> {
    let (input, s) = recognize(tuple((
        opt(tag("-")),
        take_while1(|c: char| c.is_digit(10) || c == '.'),
    )))(input)?;
    Ok((input, s.parse().unwrap_or(0.0)))
}

fn color_hex(input: &str) -> IResult<&str, StyleValue> {
    let (input, _) = tag("#")(input)?;
    let (input, hex) = hex_digit1(input)?;
    
    let (r, g, b, a) = match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).unwrap_or(0) as f32 / 15.0;
            let g = u8::from_str_radix(&hex[1..2], 16).unwrap_or(0) as f32 / 15.0;
            let b = u8::from_str_radix(&hex[2..3], 16).unwrap_or(0) as f32 / 15.0;
            (r, g, b, 1.0)
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
            (r, g, b, 1.0)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(0) as f32 / 255.0;
            (r, g, b, a)
        }
        _ => (0.0, 0.0, 0.0, 1.0),
    };
    
    Ok((input, StyleValue::Color(r, g, b, a)))
}

fn color_rgb(input: &str) -> IResult<&str, StyleValue> {
    let (input, _) = alt((tag("rgba("), tag("rgb(")))(input)?;
    let (input, vals) = separated_list0(tuple((multispace0, tag(","), multispace0)), parse_f32)(input)?;
    let (input, _) = tag(")")(input)?;
    
    let r = vals.get(0).cloned().unwrap_or(0.0) / 255.0;
    let g = vals.get(1).cloned().unwrap_or(0.0) / 255.0;
    let b = vals.get(2).cloned().unwrap_or(0.0) / 255.0;
    let a = vals.get(3).cloned().unwrap_or(1.0);
    
    Ok((input, StyleValue::Color(r, g, b, a)))
}

fn style_value(input: &str) -> IResult<&str, StyleValue> {
    alt((
        color_hex,
        color_rgb,
        map(pair(parse_f32, tag("px")), |(v, _)| StyleValue::Px(v)),
        map(pair(parse_f32, tag("%")), |(v, _)| StyleValue::Percent(v)),
        map(pair(parse_f32, tag("em")), |(v, _)| StyleValue::Em(v)),
        map(pair(parse_f32, tag("vh")), |(v, _)| StyleValue::Vh(v)),
        map(pair(parse_f32, tag("vw")), |(v, _)| StyleValue::Vw(v)),
        map(parse_f32, |v| StyleValue::Px(v)), // Default to px if no unit
        map(tag("auto"), |_| StyleValue::Auto),
        map(is_not(";{} "), |s: &str| StyleValue::Ident(s.to_string())),
    ))(input)
}

fn declaration(input: &str) -> IResult<&str, (String, StyleValue)> {
    let (input, _) = multispace0(input)?;
    let (input, property) = is_not(":}")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = style_value(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, (property.trim().to_string(), value)))
}

fn rule(input: &str) -> IResult<&str, StyleRule> {
    let (input, _) = multispace0(input)?;
    let (input, selector) = is_not("{")(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, decls) = many0(declaration)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = multispace0(input)?;

    let mut declarations = HashMap::new();
    for (p, v) in decls {
        // Expand shorthands
        match p.as_str() {
            "padding" => {
                if let StyleValue::Px(v) = v {
                    declarations.insert("padding-top".into(), StyleValue::Px(v));
                    declarations.insert("padding-right".into(), StyleValue::Px(v));
                    declarations.insert("padding-bottom".into(), StyleValue::Px(v));
                    declarations.insert("padding-left".into(), StyleValue::Px(v));
                }
            }
            "margin" => {
                if let StyleValue::Px(v) = v {
                    declarations.insert("margin-top".into(), StyleValue::Px(v));
                    declarations.insert("margin-right".into(), StyleValue::Px(v));
                    declarations.insert("margin-bottom".into(), StyleValue::Px(v));
                    declarations.insert("margin-left".into(), StyleValue::Px(v));
                }
            }
            "flex-flow" => {
                // simple split for flex-flow
                if let StyleValue::Ident(ref s) = v {
                    let parts: Vec<&str> = s.split_whitespace().collect();
                    if let Some(&p) = parts.get(0) { declarations.insert("flex-direction".into(), StyleValue::Ident(p.into())); }
                    if let Some(&p) = parts.get(1) { declarations.insert("flex-wrap".into(), StyleValue::Ident(p.into())); }
                }
            }
            _ => {
                declarations.insert(p, v);
            }
        }
    }

    Ok((input, StyleRule {
        selector: selector.trim().to_string(),
        declarations,
    }))
}

pub fn parse_css(input: &str) -> IResult<&str, Vec<StyleRule>> {
    many0(rule)(input)
}
