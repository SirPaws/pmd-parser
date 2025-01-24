use std::collections::HashSet;

#[cfg(not(feature = "wasm"))]
use anyhow::{Result, Context};
use color_print::{cprintln,cformat};
use super::config::*;
use super::factbox::parse_factbox_element;
use super::inline::parse_inline;
use super::parser_util::{check_frontmatter, generate_id, get_bibliography_title, get_blog_dir, get_data_dir, get_date, get_last_update, get_url, is_valid_id, ParserInfo};
use super::util::ordered_map::OrderedMap;
use super::toplevel::{toplevel_parse, PmdDate, ReferenceDefinition, TopLevelSyntax};
use super::frontmatter::*;
use super::structs::*;
use super::parser::*;
// this is so stupid, why can't I just have macros be scoped like ever other fucking thing?
use crate::{error, warning};

macro_rules! no_id {
    ($e: expr) => { ($e, &String::new()) }
}


pub fn parse_file(file_path: &String) -> Result<AssDownDocument> {
    parse(&std::fs::read_to_string(file_path)?, Some(file_path))
}

pub fn parse(file_content: &String, file_path: Option<&String>) -> Result<AssDownDocument> {
    let toplevel_syntax = toplevel_parse(file_content)?;
    println!("parsed toplevel!");

    let mut info = ParserInfo::new();

    for elem in &toplevel_syntax {
        let last_length = info.body.len();

        if let TopLevelSyntax::FrontMatter(frontmatter) = elem {
            info.metadata.frontmatter = Some(frontmatter.clone());
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
        
        if let MaybeElement::Yes(element) = parse_heading_element(&mut info, elem)? {
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

    if let Some(frontmatter) = &info.metadata.frontmatter {
        if let Some(title) = frontmatter["title"].as_string() {
            info.metadata.title = title;
        } else {
            if let Some(file_path) = file_path {
                #[cfg(not(feature = "wasm"))]
                cprintln!("<r>error:</> Document '{}' is missing a title, see 'pmd explain frontmatter'", file_path);
            }
        }
        
        if let Some(subtitle) = frontmatter["subtitle"].as_string() {
            info.metadata.subtitle = subtitle;
        }
        
        if let Some(banner) = frontmatter["banner"].as_string() {
            info.metadata.banner = banner;
        }
        
        if let Some(title) = frontmatter["notes-title"].as_string() {
            info.metadata.notes_title = title;
        }

        if let Some(title) = get_bibliography_title(frontmatter) {
            info.metadata.bibliography_title = title;
        }

        if let Some(date) = get_date(frontmatter) {
            info.metadata.date_written = PmdDate::String(date);
        } else {
            if let Some(file_path) = file_path {
                #[cfg(not(feature = "wasm"))]
                cprintln!("<r>error:</> Document '{}' is missing a date, see 'pmd explain frontmatter'", file_path);
            }
        }
        
        if let Some(update) = get_last_update(frontmatter) {
            info.metadata.last_update = PmdDate::String(update);
        }
        
        if let Some(url) = get_url(frontmatter) {
            info.metadata.url = url;
        }
        if let Some(data_dir) = get_data_dir(frontmatter) {
            info.metadata.data_dir = data_dir;
        }
        if let Some(blog_dir) = get_blog_dir(frontmatter) {
            info.metadata.blog_dir = blog_dir;
        }

        info.metadata.hide_notes      = check_frontmatter(frontmatter, &FRONTMATTER_HIDE_NOTES);
        info.metadata.hide_references = check_frontmatter(frontmatter, &FRONTMATTER_HIDE_REFERENCES);
        info.metadata.hide_contacts   = check_frontmatter(frontmatter, &FRONTMATTER_HIDE_CONTACTS);
    } else {
        //TODO(Paw): this should really be a warning
        if let Some(file_path) = file_path {
            #[cfg(not(feature = "wasm"))]
            cprintln!("<r>error:</> Document '{}' is missing frontmatter, see 'pmd explain frontmatter'", file_path);
        }
    }

    let notes_id     = if let Some(id) = generate_id(&info.metadata.notes_title) { id } else {
        let default_id = generate_id(&DEFAULT_NOTES_TITLE.to_string()).unwrap();
        default_id
    };
    let bibliography_id = if let Some(id) = generate_id(&info.metadata.bibliography_title) { id } else {
        let default_id = generate_id(&DEFAULT_BIBLIOGRAPHY_TITLE.to_string()).unwrap();
        default_id
    };

    if !info.notes.is_empty() {
        if info.ids.contains(&notes_id) {
            'outer: for (elem, id) in &mut info.body.iter_mut() {
                if let Element::FactBox(factbox) = elem {
                    for (_, factbox_id) in &mut factbox.body {
                        if factbox_id != &notes_id { continue }
        
                        while info.ids.contains(factbox_id) {
                            *factbox_id = format!("{factbox_id}-disass");
                        }
                        break 'outer;
                    }
                }
                if id != &notes_id { continue }
                while info.ids.contains(id) {
                    *id = format!("{id}-disass");
                }
        
                break;
            }
        }
    }
    
    if !info.references.is_empty() {
        if info.ids.contains(&bibliography_id) {
            'outer: for (elem, id) in &mut info.body.iter_mut() {
                if let Element::FactBox(factbox) = elem {
                    for (_, factbox_id) in &mut factbox.body {
                        if factbox_id != &bibliography_id { continue }
        
                        while info.ids.contains(factbox_id) {
                            *factbox_id = format!("{factbox_id}-disass");
                        }
                        break 'outer;
                    }
                }
                if id != &bibliography_id { continue }
                while info.ids.contains(id) {
                    *id = format!("{id}-disass");
                }
        
                break;
            }
        }
    }
    
    if let Some(toc) = info.metadata.toc.as_mut() {
        for (i, (item, id)) in info.body.iter().enumerate() {
            if i < toc.index { continue; }
            if let &Element::Header(text, depth) = &item {
                if depth > &toc.max_depth {
                    toc.max_depth = *depth;
                }
                toc.headers.push((text.clone(), depth.clone(), id.clone()));
            }
            else if let &Element::FactBox(_) = &item {
                if 2 > toc.max_depth {
                    toc.max_depth = 2;
                }
                toc.headers.push((Box::new(item.clone()), 2, id.clone()));
            }
        }

        if !info.notes.is_empty() {
            toc.headers.push((Box::new(Element::Text(info.metadata.notes_title.clone())), 1, notes_id.clone()))
        }

        if !info.references.is_empty() {
            toc.headers.push((Box::new(Element::Text(info.metadata.bibliography_title.clone())), 1, bibliography_id.clone()))
        }
    }

    Ok(info.build(notes_id, bibliography_id))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_remove_escaped() {
        let text: String = "\\[]".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Text("[]".into()));
    }
    
    #[test]
    fn test_parse_remove_escaped_embedding() {
        let text: String = "\\%[]".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Text("%[]".into()));
    }
    
    #[test]
    fn test_parse_hover() {
        let text: String = "%[abc](def)".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Hoverable(Alternative{ base: Box::new(Element::Text("abc".into())), alt: Box::new(Element::Text("def".into()))}));
    }
    
    #[test]
    fn test_parse_styling() {
        let text: String = "£{abc}(def)".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Styled(Alternative{ base: Box::new(Element::Text("abc".into())), alt: Box::new(Element::Text("def".into()))}));
    }
    
    #[test]
    fn test_parse_hover_with_inner_styling_left() {
        let text: String = "%[£{style}(text)](alternative)".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Hoverable(Alternative{ 
            base: Box::new(Element::Styled(
                Alternative {
                    base: Box::new(Element::Text("style".into())),
                    alt: Box::new(Element::Text("text".into()))
                }
            )), 
            alt: Box::new(Element::Text("alternative".into()))
        }));
    }
    
    #[test]
    fn test_parse_hover_with_inner_styling_right() {
        let text: String = "%[base](£{style}(text))".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Hoverable(Alternative{ 
            base: Box::new(Element::Text("base".into())),
            alt: Box::new(Element::Styled(
                Alternative {
                    base: Box::new(Element::Text("style".into())),
                    alt: Box::new(Element::Text("text".into()))
                }
            )), 
        }));
    }

    #[test]
    fn test_parse_link() {
        let text: String = "[abc](def)".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Link(Alternative{ base: Box::new(Element::Text("abc".into())), alt: Box::new(Element::Text("def".into()))}));
    }
    
    #[test]
    fn test_parse_link_with_styling() {
        let text: String = "[link](£{style}(text))".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Link(Alternative{ 
            base: Box::new(Element::Text("link".into())),
            alt: Box::new(Element::Styled(
                Alternative {
                    base: Box::new(Element::Text("style".into())),
                    alt: Box::new(Element::Text("text".into()))
                }
            )), 
        }));
    }

    #[test]
    fn test_parse_inline_code() {
        let text: String = "`here's some code`".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::InlineCode("here's some code".into()))
    }

    #[test]
    fn test_parse_italics() {
        let text: String = "*italics*".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Italics(Box::new(Element::Text("italics".into()))))
    }
    
    #[test]
    fn test_parse_bold() {
        let text: String = "**bold**".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Bold(Box::new(Element::Text("bold".into()))))
    }
    
    #[test]
    fn test_parse_bold_and_italics() {
        let text: String = "***italics and bold***".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Bold(
            Box::new(Element::Italics(
                Box::new(Element::Text("italics and bold".into()))
            ))
        ))
    }
    
    #[test]
    fn test_parse_bold_with_inner_italics() {
        let text: String = "**bold*italics*bold**".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Bold(
            Box::new(Element::Span(Span{ elements:
                vec![ 
                    Element::Text("bold".into()),
                    Element::Italics(
                        Box::new(Element::Text("italics".into()))
                    ),
                    Element::Text("bold".into()),
                ]
            }))
        ))
    }

    #[test]
    fn test_parse_inline_note() {
        let text: String = "[^0]".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Note("0".into()))
    }

    #[test]
    fn test_parse_citation_alphabetic() {
        let text: String = "[£example]".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Citation("example".into()))
    }
    
    #[test]
    fn test_parse_citation_dash() {
        let text: String = "[£-other-example]".into();
        let result = parse_inline(&text);
        assert!(result.is_ok());
        let inner = Box::into_inner(result.unwrap().0);
        assert!(inner == Element::Citation("-other-example".into()))
    }
}


