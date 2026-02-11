use nom::{
    bytes::complete::{tag, take_until, is_not},
    character::complete::{multispace0, multispace1, alphanumeric1},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StyleRule {
    pub selector: String,
    pub declarations: HashMap<String, String>,
}

fn declaration(input: &str) -> IResult<&str, (String, String)> {
    let (input, _) = multispace0(input)?;
    let (input, property) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, value) = take_until(";")(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, (property.trim().to_string(), value.trim().to_string())))
}

fn rule(input: &str) -> IResult<&str, StyleRule> {
    let (input, _) = multispace0(input)?;
    let (input, selector) = take_until("{")(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, decls) = many0(declaration)(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = multispace0(input)?;

    let mut declarations = HashMap::new();
    for (p, v) in decls {
        declarations.insert(p, v);
    }

    Ok((input, StyleRule {
        selector: selector.trim().to_string(),
        declarations,
    }))
}

pub fn parse_css(input: &str) -> IResult<&str, Vec<StyleRule>> {
    many0(rule)(input)
}
