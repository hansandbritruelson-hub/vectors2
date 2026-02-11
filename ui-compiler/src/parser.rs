use nom::{
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{multispace0, multispace1},
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    Binding(String),
}

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
    pub is_dynamic: bool,
}

#[derive(Debug, Clone)]
pub struct Template {
    pub root: Vec<Node>,
    pub styles: Vec<String>,
    pub script: Option<String>,
}

fn identifier(input: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            // First character can be alphanumeric, or one of ':', '@', '#', '_'
            take_while1(|c: char| c.is_alphanumeric() || c == ':' || c == '@' || c == '#'),
            // Subsequent characters can be alphanumeric, or one of '-', '_', ':', '.', '@', '#'
            take_while(|c: char| c.is_alphanumeric() || c == '-' || c == '_' || c == ':' || c == '.' || c == '@' || c == '#'),
        )),
        |s: &str| s.to_string(),
    )(input)
}

fn attribute(input: &str) -> IResult<&str, Attribute> {
    let (input, name) = identifier(input)?;
    let (input, value_opt) = opt(tuple((
        tag("="),
        delimited(tag("\""), take_until("\""), tag("\"")),
    )))(input)?;

    let is_dynamic = name.starts_with(':') || name.starts_with('@');
    let clean_name = &name;

    let value = value_opt.map(|(_, v)| v.to_string()).unwrap_or_else(|| "true".to_string());

    Ok((input, Attribute {
        name: clean_name.to_string(),
        value,
        is_dynamic,
    }))
}

fn open_tag(input: &str) -> IResult<&str, (String, Vec<Attribute>, bool)> {
    let (input, _) = tag("<")(input)?;
    let (input, name) = identifier(input)?;
    let (input, attributes) = many0(preceded(multispace1, attribute))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, self_closing) = map(opt(tag("/")), |o| o.is_some())(input)?;
    let (input, _) = tag(">")(input)?;

    Ok((input, (name.to_string(), attributes, self_closing)))
}

fn close_tag(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("</")(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(">")(input)?;
    Ok((input, name.to_string()))
}

fn text_node(input: &str) -> IResult<&str, Node> {
    let (input, content) = take_while1(|c| c != '<' && c != '{')(input)?;
    Ok((input, Node::Text(content.to_string())))
}

fn binding_node(input: &str) -> IResult<&str, Node> {
    let (input, content) = delimited(tag("{{"), take_until("}}"), tag("}}"))(input)?;
    Ok((input, Node::Binding(content.trim().to_string())))
}

fn element_node(input: &str) -> IResult<&str, Node> {
    let (input, (name, attributes, self_closing)) = open_tag(input)?;
    
    if self_closing {
        return Ok((input, Node::Element(Element {
            name,
            attributes,
            children: vec![],
        })));
    }

    let (input, children) = many0(node)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _close_name) = close_tag(input)?;
    
    Ok((input, Node::Element(Element {
        name,
        attributes,
        children,
    })))
}

fn node(input: &str) -> IResult<&str, Node> {
    let (next_input, _) = multispace0(input)?;
    if next_input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(next_input, nom::error::ErrorKind::Eof)));
    }
    
    if next_input.starts_with("{{") {
        binding_node(next_input)
    } else if next_input.starts_with('<') {
        if next_input.starts_with("</") {
            return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
        }
        element_node(next_input)
    } else {
        text_node(next_input)
    }
}

pub fn parse_template(input: &str) -> IResult<&str, Template> {
    let mut template = Template {
        root: vec![],
        styles: vec![],
        script: None,
    };

    let mut remaining = input;
    while !remaining.is_empty() {
        let (next_raw, _) = multispace0(remaining)?;
        if next_raw.is_empty() { break; }

        if next_raw.starts_with("<script") {
            let (next, _) = tag("<script")(next_raw)?;
            let (next, _) = many0(preceded(multispace1, attribute))(next)?;
            let (next, _) = multispace0(next)?;
            let (next, _) = tag(">")(next)?;
            let (next, content) = take_until("</script>")(next)?;
            let (next, _) = tag("</script>")(next)?;
            template.script = Some(content.to_string());
            remaining = next;
        } else if next_raw.starts_with("<style") {
            let (next, _) = tag("<style")(next_raw)?;
            let (next, _) = many0(preceded(multispace1, attribute))(next)?;
            let (next, _) = multispace0(next)?;
            let (next, _) = tag(">")(next)?;
            let (next, content) = take_until("</style>")(next)?;
            let (next, _) = tag("</style>")(next)?;
            template.styles.push(content.to_string());
            remaining = next;
        } else if next_raw.starts_with("<template") {
            let (next, _) = tag("<template")(next_raw)?;
            let (next, _) = many0(preceded(multispace1, attribute))(next)?;
            let (next, _) = multispace0(next)?;
            let (next, _) = tag(">")(next)?;
            let (next, children) = many0(node)(next)?;
            let (next, _) = multispace0(next)?;
            let (next, _) = close_tag(next)?;
            template.root.extend(children);
            remaining = next;
        } else {
            let (next, n) = node(next_raw)?;
            template.root.push(n);
            remaining = next;
        }
    }
    
    Ok((remaining, template))
}
