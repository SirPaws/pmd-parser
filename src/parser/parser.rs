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

// this is so stupid, why can't I just have macros be scoped like ever other fucking thing?
use crate::{error, warning};

pub fn parse_pagebreak_element(_: &mut ParserInfo, elem: &TopLevelSyntax) -> Result<MaybeElement> {
    let TopLevelSyntax::PageBreak = elem else { return Ok(MaybeElement::No) };
    Ok(MaybeElement::Yes((Element::PageBreak, String::new())))
}

pub fn parse_frontmatter_element(_: &mut ParserInfo, elem: &TopLevelSyntax) -> Result<MaybeElement> {
    let TopLevelSyntax::FrontMatter(frontmatter) = elem else { return Ok(MaybeElement::No) };
    Ok(MaybeElement::Yes((Element::PageBreak, String::new())))
}

pub fn parse_codeblock_element(info: &mut ParserInfo, elem: &TopLevelSyntax) -> Result<MaybeElement> {
    let TopLevelSyntax::CodeBlock(block) = elem else { return Ok(MaybeElement::No) };
    let id = info.num_codeblocks;
    info.num_codeblocks +=1;
    Ok(MaybeElement::Yes((Element::CodeBlock(block.to_string()),
        if id == 0 {
            format!("codeblock")
        } else {
            format!("codeblock-{id}")
        }
    )))
}

use super::toplevel as toplevel;

pub fn parse_image_element(info: &mut ParserInfo, elem: &TopLevelSyntax) -> Result<MaybeElement> {
    let TopLevelSyntax::Image(img) = elem else { return Ok(MaybeElement::No) };
    let size = match &img.size {
        toplevel::ImageSize::None => ImageSize::None,
        toplevel::ImageSize::Single(x) => ImageSize::Single(x.parse()?),
        toplevel::ImageSize::Double(x, y) => ImageSize::Double(x.parse()?, y.parse()?),
    };
    if let toplevel::ImageSize::None = img.size {} else { warning!("image sizing is currently unimplemented") };

    let num_image = info.num_image;
    info.num_image += 1;

    let (alt, mut id) = parse_inline(&img.alt)?;
    if id.is_empty() {
        id = format!("image-{num_image}");
    }

    Ok(MaybeElement::Yes((
            Element::Image(Image{src: img.image.to_string(), alt, size}),
            id
    )))
}

pub fn parse_heading_element(info: &mut ParserInfo, elem: &TopLevelSyntax) -> Result<MaybeElement> {
    let TopLevelSyntax::Heading(text, level) = elem else { return Ok(MaybeElement::No) };
    let (object, id) = parse_inline(&text)?;
    Ok(MaybeElement::Yes((Element::Header(object, *level), id)))
}

fn parse_list_element_inner(vec: &Vec<toplevel::ListItem>) -> Result<ListItem> {
    let mut result = Vec::new();
    for elem in vec {
        let object = match elem {
            toplevel::ListItem::Unordered(text)                        => ListItem::Unordered(parse_inline(&text)?.0),
            toplevel::ListItem::Numbered(number, text)         => ListItem::Numbered(*number, parse_inline(&text)?.0),
            toplevel::ListItem::Alphabetical(id, text)        => ListItem::Alphabetical(id.clone(), parse_inline(&text)?.0),
            toplevel::ListItem::NumberedRounded(number, text)  => ListItem::NumberedRounded(*number, parse_inline(&text)?.0),
            toplevel::ListItem::AlphabeticalRounded(id, text) => ListItem::AlphabeticalRounded(id.clone(), parse_inline(&text)?.0),
            toplevel::ListItem::InnerList(vec) => parse_list_element_inner(vec)?,
        };
        // let (object, _) = parse_inline(&elem)?;
        result.push(object)
    }

    Ok(ListItem::List(result))
}

pub fn parse_list_element(info: &mut ParserInfo, elem: &TopLevelSyntax) -> Result<MaybeElement> {
    let TopLevelSyntax::List(list) = elem else { return Ok(MaybeElement::No) };

    let mut result = Vec::new();
    for elem in list {
        let object = match elem {
            toplevel::ListItem::Unordered(text)                        => ListItem::Unordered(parse_inline(&text)?.0),
            toplevel::ListItem::Numbered(number, text)         => ListItem::Numbered(*number, parse_inline(&text)?.0),
            toplevel::ListItem::Alphabetical(id, text)        => ListItem::Alphabetical(id.clone(), parse_inline(&text)?.0),
            toplevel::ListItem::NumberedRounded(number, text)  => ListItem::NumberedRounded(*number, parse_inline(&text)?.0),
            toplevel::ListItem::AlphabeticalRounded(id, text) => ListItem::AlphabeticalRounded(id.clone(), parse_inline(&text)?.0),
            toplevel::ListItem::InnerList(vec) => parse_list_element_inner(vec)?,
        };
        result.push(object)
    }
    let id = info.num_lists;
    info.num_lists += 1;
    let result = Element::List(result);
    Ok(MaybeElement::Yes((result, format!("list-{id}"))))
}

pub fn parse_paragraph_element(info: &mut ParserInfo, elem: &TopLevelSyntax) -> Result<MaybeElement> {
    let TopLevelSyntax::Paragraph(text) = elem else { return Ok(MaybeElement::No) };

    let (object, id) = parse_inline(&text)?;
    Ok(MaybeElement::Yes((Element::Paragraph(object), id)))
}

pub fn parse_quote_element(info: &mut ParserInfo, elem: &TopLevelSyntax) -> Result<MaybeElement> {
    let TopLevelSyntax::Quote(list) = elem else { return Ok(MaybeElement::No) };
    let mut result = Vec::<Element>::new();
    for elem in list {
        let (object, _) = parse_inline(&elem)?;
        result.push(Box::into_inner(object));
    }
    let id = info.num_quotes;
    info.num_quotes += 1;
    Ok(MaybeElement::Yes((Element::Quote(result), format!("quote-{id}"))))
}

pub fn check_id(info: &mut ParserInfo, last_length: usize) {
    if info.body.len() != last_length {
        if let Some((_, id)) = info.body.last_mut() {
            if is_valid_id(id) {
                while info.ids.contains(id) {
                    *id += format!("-{last_length}").as_str();
                }
                info.ids.insert(id.clone());
            }
        }
    }
}
