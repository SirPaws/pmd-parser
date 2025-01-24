use crate::parser::inline::parse_inline;
use crate::parser::parser::check_id;

use super::{parser_util::generate_id, toplevel::TopLevelSyntax};
use super::structs::*;
use super::util::ordered_map::OrderedMap;
use std::collections::HashSet;
use super::parser_util::{is_valid_id, ParserInfo};
use anyhow::{Result, Context};
use color_print::cformat;
use super::parser::*;


pub fn parse_factbox_element(info: &mut ParserInfo, elem: &TopLevelSyntax)  -> Result<MaybeElement> {

    let TopLevelSyntax::FactBox{ title, body} = elem else {
        return Ok(MaybeElement::No)
    };
    let factbox_parsed = parse_factbox_body(body)?;
    let mut factbox = FactBox {
        title: title.clone(),
        notes: OrderedMap::new(),
        body: factbox_parsed.body,
    };
    
    if !factbox_parsed.references.is_empty() {
        for (key, def) in &factbox_parsed.references {
            info.references.insert(key.clone(), def.clone());
        }
    }
    let id = if let Some(id) = generate_id(title) { id } else { format!("factbox-{}", info.num_factboxes) };
    for (_, object_id) in &mut factbox.body {
        if is_valid_id(object_id) {
            let last_length = info.body.len();
            *object_id = format!("{id}-{object_id}");
            while info.ids.contains(object_id) {
                *object_id += format!("-{last_length}").as_str();
            }
            info.ids.insert(id.clone());
        }
    }
    
    for (key, elem) in &factbox_parsed.notes {
        factbox.notes.insert(key.clone(), (elem.clone(), format!("{id}-{key}")))
    }
    
    info.num_factboxes += 1;
    Ok(MaybeElement::Yes((
            Element::FactBox(factbox), 
            id
    )))
}

fn parse_factbox_body(toplevel_syntax: &Vec<TopLevelSyntax>) -> Result<AssDownDocument> {
    let mut info = ParserInfo::new();

    for elem in toplevel_syntax {
        let last_length = info.body.len();

        if let TopLevelSyntax::FactBox{title: _, body: _} = elem {
            error!("fact boxes inside of fact boxes is not allowed");
            continue;
        }
        if let TopLevelSyntax::FrontMatter(frontmatter) = elem {
            error!("fact boxes frontmatter of fact boxes is not allowed");
            continue;
        }
        if let TopLevelSyntax::ReferenceDefinition(reference) = elem {
            info.references.insert(reference.id.clone(), reference.clone());
            continue;
        }
        if let TopLevelSyntax::NoteDefinition { id, text } = elem {
            let (object, _) = parse_inline(&text)?;
            info.notes.insert(id.clone(), Box::into_inner(object));
            continue;
        }
        if let TopLevelSyntax::TOC(title) = elem {
            if info.metadata.toc.is_none() {
                info.metadata.toc = Some(TableOfContent{ title: title.clone(), index: info.body.len(), headers: vec![], max_depth: 1,});
                info.body.push((Element::TOCLocationMarker, String::new()));
            }
            continue;
        }

        if let MaybeElement::Yes(element) = parse_factbox_element(&mut info, elem)? {
            info.body.push(element);
            check_id(&mut info, last_length);
            continue;
        }

        if let MaybeElement::Yes(element) = parse_pagebreak_element(&mut info, elem)? {
            info.body.push(element);
            check_id(&mut info, last_length);
            continue;
        }

        if let MaybeElement::Yes(element) = parse_codeblock_element(&mut info, elem)? {
            info.body.push(element);
            check_id(&mut info, last_length);
            continue;
        }
        
        if let MaybeElement::Yes(element) = parse_image_element(&mut info, elem)? {
            info.body.push(element);
            check_id(&mut info, last_length);
            continue;
        }
        
        if let MaybeElement::Yes(element) = parse_list_element(&mut info, elem)? {
            info.body.push(element);
            check_id(&mut info, last_length);
            continue;
        }

        if let MaybeElement::Yes(element) = parse_paragraph_element(&mut info, elem)? {
            info.body.push(element);
            check_id(&mut info, last_length);
            continue;
        }

        if let MaybeElement::Yes(element) = parse_quote_element(&mut info, elem)? {
            info.body.push(element);
            check_id(&mut info, last_length);
            continue;
        }
        
        panic!("element was not parsed {:?}", elem);
    }

        Ok(info.build(String::new(), String::new()))
}
