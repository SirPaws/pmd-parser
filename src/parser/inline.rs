
use anyhow::{Result, Context};
use super::structs::*;
use super::parser_util::{gather_link, get_citation, generate_id};


pub fn parse_inline(text: &String) -> Result<(Box<Element>, String)> {
    let mut body = Vec::<Element>::new();
    let mut buffer = String::new();
    let mut peekable = text.chars().peekable();
    let mut tmp_id = String::new();
    while let Some(character) = peekable.peek() {
        match character {
            '\\' => {
                peekable.next(); 
                let mut copy = peekable.clone();
                if let Some(&escaped_character) = copy.peek() {
                    copy.next();
                    match escaped_character {
                        '%'|'£' => { 
                            buffer.push(escaped_character);
                            tmp_id.push(escaped_character);
                            if copy.peek().is_some_and(|&possible_brace| possible_brace == '[') {
                                peekable.next();
                                buffer.push('[');
                                tmp_id.push(escaped_character);
                            }
                        },
                        _ => {
                            buffer.push(escaped_character);
                            tmp_id.push(escaped_character);
                        }
                    }
                } else {
                    buffer.push('\\');
                    tmp_id.push('\\');
                }
            },
            '£'|'%' => {
                if buffer.len() != 0 {
                    body.push(Element::Text(buffer));
                    buffer = String::new();
                }

                let start_char = *character;
                let mut make_object = |base: &String, alt: &String|
                    anyhow::Ok(if start_char == '%' { 
                        let (base, id) = parse_inline(base)?;
                        let (alt, _) = parse_inline(alt)?;
                        tmp_id += id.as_str();
                        Element::Hoverable(Alternative{base, alt})
                    } else {
                        let (base, _) = parse_inline(base)?;
                        let (alt,  id) = parse_inline(alt)?;
                        tmp_id.push(' ');
                        tmp_id += id.as_str();
                        tmp_id.push(' ');
                        Element::Styled(Alternative{base, alt})
                    });
                
                let search_begin_char = if start_char == '%' { '[' } else { '{' };
                let search_end_char   = if start_char == '%' { ']' } else { '}' };

                peekable.next();
                if peekable.peek().is_some_and(|&brace| brace == search_begin_char) {
                    let mut base = String::new(); 
                    peekable.next();
                    let mut depth = 0;
                    let mut end  = peekable.clone();
                    while !(end.peek() == Some(&search_end_char) && depth == 0) {
                        let character = end.next().context("expected ']'")?;
                        if      character == search_begin_char { depth += 1 }
                        else if character == search_end_char { depth -= 1 }
                        base.push(character);
                    }
                    end.next();
                    let mut alt = String::new();
                    // this is unreadable
                    body.push(make_object(&base, 
                        if end.peek() == Some(&'(') {
                            end.next();
                            while !(end.peek() == Some(&')') && depth == 0) {
                                let character = end.next().context("expected ')'")?;
                                match character {
                                    '(' => depth += 1,
                                    ')' => if depth != 0 { depth -= 1 } else {},
                                    _   => {}
                                }
                                alt.push(character);
                            }
                            end.next();
                            peekable = end.clone();
                            &alt
                        }
                        else {
                            peekable = end.clone();
                            &alt
                        })?);
                } else {
                    buffer.push(start_char);
                    tmp_id.push(start_char);
                }
                continue;
            },
            '[' => {
                if buffer.len() != 0 {
                    body.push(Element::Text(buffer));
                    buffer = String::new();
                }

                let mut base = String::new(); 
                peekable.next();

                let mut depth = 0;
                let mut end  = peekable.clone();
                while !(end.peek() == Some(&']') && depth == 0) {
                    let character = end.next().context("expected ']'")?;
                    match character {
                        '[' => depth += 1,
                        ']' => if depth != 0 { depth -= 1 } else {},
                        _   => {}
                    }
                    base.push(character);
                }

                if base.starts_with('£') && base.trim_start().chars().nth(1).is_some_and(|x| x.is_alphabetic() || x == '-') {
                    // this is a citation
                    let citation : String = base.chars().skip(1).collect();
                    body.push(Element::Citation(citation));
                    end.next();
                    peekable = end.clone();
                    continue;
                }
                
                
                if base.starts_with('^') && base.len() > 1 {
                    // this is a citation
                    let citation : String = base.chars().skip(1).collect();
                    body.push(Element::Note(citation));
                    end.next();
                    peekable = end.clone();
                    continue;
                }

                end.next();
                let alt;
                (alt, peekable) = gather_link(end, &mut depth)?;
                if let Some(element) = get_citation(&alt) {
                    let (base, _) = parse_inline(&base)?;
                    body.push(Element::Link(
                        Alternative { base, alt: Box::new(element) }
                    ))
                } else {
                    let (base, id) = parse_inline(&base)?;
                    let (alt, _) = parse_inline(&alt)?;
                    tmp_id.push(' ');
                    tmp_id += id.as_str();
                    tmp_id.push(' ');

                    body.push(Element::Link(Alternative{ base, alt}))
                }
                continue;
            },
            '`' => {
                if buffer.len() != 0 {
                    body.push(Element::Text(buffer));
                    buffer = String::new();
                }

                let mut base = String::new(); 
                peekable.next();
                let mut end  = peekable.clone();
                while end.peek() != Some(&'`') {
                    let character = end.next().context("expected ']'")?;
                    base.push(character);
                }
                end.next();
                peekable = end.clone();

                tmp_id.push(' ');
                tmp_id += base.as_str();
                tmp_id.push(' ');
                body.push(Element::InlineCode(base));
                continue;
            },
            '*' => {
                if buffer.len() != 0 {
                    body.push(Element::Text(buffer));
                    buffer = String::new();
                }

                peekable.next();
                if peekable.peek() == Some(&'*') {
                    peekable.next();
                    let mut depth = 0;
                    let mut result = String::new();
                    while peekable.peek().is_some() {
                        if peekable.peek() == Some(&'*') {
                            peekable.next();
                            if peekable.peek() == Some(&'*') && depth == 0 {
                                break;
                            } else {
                                if depth == 0 { depth += 1; } else { depth -= 1; }
                            }
                            result.push('*');
                            continue;
                        }
                        result.push(peekable.next().unwrap());
                    }
                    if peekable.peek() == Some(&'*') {
                        peekable.next();
                    }
                    let (text, id) = parse_inline(&result)?;
                    tmp_id += id.as_str();
                    body.push(Element::Bold(text))
                } else {
                    let mut result = String::new();
                    while peekable.peek() != Some(&'*') {
                        if peekable.peek().is_none() { break }
                        result.push(peekable.next().unwrap());
                    }
                    if peekable.peek() == Some(&'*') {
                        peekable.next();
                    }
                    let (text, id) = parse_inline(&result)?;
                    tmp_id += id.as_str();
                    body.push(Element::Italics(text))
                }
                continue;
                
            },
            _   => {
                buffer.push(*character); 
                tmp_id.push(*character); 
            },
        }

        peekable.next();
    }

    if buffer.len() != 0 {
        body.push(Element::Text(buffer));
    }

    let id = generate_id(&tmp_id);

    match body.len() {
        0 => Ok((Box::new(Element::Span(Span{elements: vec![]})), String::new())),
        1 => Ok((Box::new(body[0].clone()), id.unwrap_or(String::new()))),
        _ => Ok((Box::new(Element::Span(Span{elements: body})), id.unwrap_or(String::new()))),
    }
}
