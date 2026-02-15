#![allow(unused_imports)]
#![allow(dead_code)]
use nom::{
    bytes::complete::{tag, take_until, is_not, take_while1},
    character::complete::{multispace1, alphanumeric1, hex_digit1},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    combinator::{map, opt, recognize, value},
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
    let (input, vals) = separated_list0(tuple((ws0, tag(","), ws0)), parse_f32)(input)?;
    let (input, _) = tag(")")(input)?;
    
    let r = vals.get(0).cloned().unwrap_or(0.0) / 255.0;
    let g = vals.get(1).cloned().unwrap_or(0.0) / 255.0;
    let b = vals.get(2).cloned().unwrap_or(0.0) / 255.0;
    let a = vals.get(3).cloned().unwrap_or(1.0);
    
    Ok((input, StyleValue::Color(r, g, b, a)))
}

fn single_line_comment(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("//")(input)?;
    if let Some(idx) = input.find('\n') {
        Ok((&input[idx..], ()))
    } else {
        Ok(("", ()))
    }
}

fn multi_line_comment(input: &str) -> IResult<&str, ()> {
    value((), tuple((tag("/*"), take_until("*/"), tag("*/"))))(input)
}

fn ws0(input: &str) -> IResult<&str, ()> {
    value((), many0(alt((value((), multispace1), single_line_comment, multi_line_comment))))(input)
}

fn ws1(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((value((), multispace1), single_line_comment, multi_line_comment))(input)?;
    value((), many0(alt((value((), multispace1), single_line_comment, multi_line_comment))))(input)
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

fn declaration(input: &str) -> IResult<&str, (String, Vec<StyleValue>)> {
    let (input, _) = ws0(input)?;
    let (input, property) = is_not(":}")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = ws0(input)?;
    let (input, values) = separated_list0(ws1, style_value)(input)?;
    let (input, _) = ws0(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, _) = ws0(input)?;
    Ok((input, (property.trim().to_string(), values)))
}

fn rule(input: &str) -> IResult<&str, StyleRule> {
    let (input, _) = ws0(input)?;
    let (input, selector) = is_not("{")(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, decls) = many0(declaration)(input)?;
    let (input, _) = ws0(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = ws0(input)?;

    let mut declarations = HashMap::new();
    for (p, vals) in decls {
        match p.as_str() {
            "padding" => {
                match vals.len() {
                    1 => {
                        let v = &vals[0];
                        declarations.insert("padding-top".into(), v.clone());
                        declarations.insert("padding-right".into(), v.clone());
                        declarations.insert("padding-bottom".into(), v.clone());
                        declarations.insert("padding-left".into(), v.clone());
                    }
                    2 => {
                        let v = &vals[0];
                        let h = &vals[1];
                        declarations.insert("padding-top".into(), v.clone());
                        declarations.insert("padding-right".into(), h.clone());
                        declarations.insert("padding-bottom".into(), v.clone());
                        declarations.insert("padding-left".into(), h.clone());
                    }
                    3 => {
                        let t = &vals[0];
                        let h = &vals[1];
                        let b = &vals[2];
                        declarations.insert("padding-top".into(), t.clone());
                        declarations.insert("padding-right".into(), h.clone());
                        declarations.insert("padding-bottom".into(), b.clone());
                        declarations.insert("padding-left".into(), h.clone());
                    }
                    4 => {
                        declarations.insert("padding-top".into(), vals[0].clone());
                        declarations.insert("padding-right".into(), vals[1].clone());
                        declarations.insert("padding-bottom".into(), vals[2].clone());
                        declarations.insert("padding-left".into(), vals[3].clone());
                    }
                    _ => {}
                }
            }
            "margin" => {
                match vals.len() {
                    1 => {
                        let v = &vals[0];
                        declarations.insert("margin-top".into(), v.clone());
                        declarations.insert("margin-right".into(), v.clone());
                        declarations.insert("margin-bottom".into(), v.clone());
                        declarations.insert("margin-left".into(), v.clone());
                    }
                    2 => {
                        let v = &vals[0];
                        let h = &vals[1];
                        declarations.insert("margin-top".into(), v.clone());
                        declarations.insert("margin-right".into(), h.clone());
                        declarations.insert("margin-bottom".into(), v.clone());
                        declarations.insert("margin-left".into(), h.clone());
                    }
                    3 => {
                        let t = &vals[0];
                        let h = &vals[1];
                        let b = &vals[2];
                        declarations.insert("margin-top".into(), t.clone());
                        declarations.insert("margin-right".into(), h.clone());
                        declarations.insert("margin-bottom".into(), b.clone());
                        declarations.insert("margin-left".into(), h.clone());
                    }
                    4 => {
                        declarations.insert("margin-top".into(), vals[0].clone());
                        declarations.insert("margin-right".into(), vals[1].clone());
                        declarations.insert("margin-bottom".into(), vals[2].clone());
                        declarations.insert("margin-left".into(), vals[3].clone());
                    }
                    _ => {}
                }
            }
            "border" => {
                let mut width = StyleValue::Px(0.0);
                let mut color = StyleValue::Color(0.0, 0.0, 0.0, 1.0);
                for v in vals {
                    match v {
                        StyleValue::Px(_) => width = v,
                        StyleValue::Color(..) => color = v,
                        _ => {} // Skip "solid" etc.
                    }
                }
                declarations.insert("border-top-width".into(), width.clone());
                declarations.insert("border-right-width".into(), width.clone());
                declarations.insert("border-bottom-width".into(), width.clone());
                declarations.insert("border-left-width".into(), width);

                declarations.insert("border-color-top".into(), color.clone());
                declarations.insert("border-color-right".into(), color.clone());
                declarations.insert("border-color-bottom".into(), color.clone());
                declarations.insert("border-color-left".into(), color);
            }
            "outline" => {
                let mut width = StyleValue::Px(0.0);
                let mut color = StyleValue::Color(0.0, 0.0, 0.0, 1.0);
                for v in vals {
                    match v {
                        StyleValue::Px(_) => width = v,
                        StyleValue::Color(..) => color = v,
                        _ => {}
                    }
                }
                declarations.insert("outline-width".into(), width);
                declarations.insert("outline-color-top".into(), color.clone());
                declarations.insert("outline-color-right".into(), color.clone());
                declarations.insert("outline-color-bottom".into(), color.clone());
                declarations.insert("outline-color-left".into(), color);
            }
            "box-shadow" => {
                let mut h_offset = StyleValue::Px(0.0);
                let mut v_offset = StyleValue::Px(0.0);
                let mut blur = StyleValue::Px(0.0);
                let mut spread = StyleValue::Px(0.0);
                let mut color = StyleValue::Color(0.0, 0.0, 0.0, 1.0);
                
                let mut px_idx = 0;
                for v in vals {
                    match v {
                        StyleValue::Px(_) => {
                            match px_idx {
                                0 => h_offset = v,
                                1 => v_offset = v,
                                2 => blur = v,
                                3 => spread = v,
                                _ => {}
                            }
                            px_idx += 1;
                        }
                        StyleValue::Color(..) => color = v,
                        _ => {}
                    }
                }
                declarations.insert("box-shadow-h-offset".into(), h_offset);
                declarations.insert("box-shadow-v-offset".into(), v_offset);
                declarations.insert("box-shadow-blur".into(), blur);
                declarations.insert("box-shadow-spread".into(), spread);
                declarations.insert("box-shadow-color".into(), color);
            }
            "flex-flow" => {
                if let Some(v) = vals.get(0) { declarations.insert("flex-direction".into(), v.clone()); }
                if let Some(v) = vals.get(1) { declarations.insert("flex-wrap".into(), v.clone()); }
            }
            "background" => {
                if let Some(v) = vals.get(0) {
                    declarations.insert("background-color".into(), v.clone());
                }
            }
            _ => {
                if let Some(v) = vals.get(0) {
                    declarations.insert(p, v.clone());
                }
            }
        }
    }

    Ok((input, StyleRule {
        selector: selector.trim().to_string(),
        declarations,
    }))
}

pub fn parse_css(input: &str) -> IResult<&str, Vec<StyleRule>> {
    let (input, _) = ws0(input)?;
    let (input, rules) = many0(rule)(input)?;
    let (input, _) = ws0(input)?;
    Ok((input, rules))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_line_comments() {
        let css = r#"
        // before
        .a {
            color: #ff0000; // trailing
            // between declarations
            width: 10px;
        }
        "#;

        let (rest, rules) = parse_css(css).expect("css with // comments should parse");
        assert!(rest.trim().is_empty());
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].selector, ".a");
        assert!(rules[0].declarations.contains_key("color"));
        assert!(rules[0].declarations.contains_key("width"));
    }

    #[test]
    fn parses_multi_line_comments() {
        let css = r#"
        /* before */
        .b {
            /* comment in block */
            height: 20px;
            /* multi-line
               comment */
            margin: 4px 8px;
        }
        /* after */
        "#;

        let (rest, rules) = parse_css(css).expect("css with /* */ comments should parse");
        assert!(rest.trim().is_empty());
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].selector, ".b");
        assert!(rules[0].declarations.contains_key("height"));
        assert!(rules[0].declarations.contains_key("margin-top"));
        assert!(rules[0].declarations.contains_key("margin-right"));
    }
}
