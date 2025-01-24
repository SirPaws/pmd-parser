use anyhow::Result;
use super::list_parser::*;
use super::parser_object::*;
use super::structs::*;
use super::reference::parse_reference;
use std::collections::HashMap;

use super::super::{config::DEFAULT_FACTBOX_TITLE, frontmatter::parse_frontmatter};

fn skip_comment(object: &mut ParseObject) -> bool {
    if object.current().trimmed_starts_with("%%") {
        if let Some(index) = object.trimmed_line_find("%%", 2) {
            object.skip(index + 2);
            return true;
        }
    
        if let Some(index) = object.trimmed_find("%%", 2) {
            object.skip(index + 2);
            return true;
        }
    }
    return false;
}

pub fn parse_quote(object: &mut ParseObject) -> Option<TopLevelSyntax> {
    if object.starts_with('>') {
        let mut list = Vec::<String>::new();
    
        let mut line = object.current();
        while line.starts_with('>') {
            let mut string = &line[1..];
            string = string.trim();
            list.push(string.into());
    
            object.next();
            if object.current().is_empty() { break }
            line = object.current();
        }
    
        Some(TopLevelSyntax::Quote(list))
    } else {
        None
    }
}

pub fn parse_pagebreak(object: &mut ParseObject) -> Option<TopLevelSyntax> {
    let current = object.current();
    if current.starts_with("---") && current.ends_with("---") {
        let mut is_line_break = true;
        for character in current.chars() {
            if character.is_whitespace() { continue; }
            if character != '-' {
                is_line_break = false;
                break;
            }
        }
        if is_line_break {
            Some(TopLevelSyntax::PageBreak)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn parse_codeblock(object: &mut ParseObject) -> Option<TopLevelSyntax> {
    if object.current().starts_with("```") {
        let Some(last) = object.find("```", 3) else { return None };
        let block = object.yoink(0, last + 3);
        let text = block[..block.len()][3..].to_string();

        Some(TopLevelSyntax::CodeBlock(text))
    } else {
        None
    }
}


fn string_has_delimeter(text: &str) -> Option<char> {
    for c in text.chars() {
        match c {
            '-'|'.' =>  { return Some(c) }
            _ if c.is_whitespace() => { return Some(c) },
            _ => { continue }
        }
    }
    return None;
}


fn trim_delimeter_start(text: &str) -> &str {
    if text.chars().nth(0).is_some_and(|c| !(c == '.' || c == '-' || c.is_whitespace())) {
        return text;
    }

    let mut num = 0;
    for c in text.chars() {
        match c {
            '-'|'.' => {
                num = num + 1;
            },
            _ if c.is_whitespace() => {
                num = num + 1;
            }
            _ => break,
        }
    }

    &text[num..]
}

// takes the inner string for a meta token
// an example would be #[title], here check would be equal to "title"
// this will also automatically remove whitespace so
// #[      title ] would still be valid.
// if the check string contains whitespace, '.', or '-' it'll be treated as a delimiter marker
// meaning that "last-update" would parse strings like #[last update], #[last.update], 
// #[last - update].
// note: it should only have a single character between meaning "last---update" would be erroneous
// note: all delimiters should be the same, meaning "is-this updated" would be erroneous
fn is_meta(text: &str, check: &str) -> Option<usize> {
    let initial_length = text.len();
    if !text.starts_with("#[") {
        return None;
    }

    let mut text = text[2..].trim_start();

    if let Some(c) = string_has_delimeter(check) {
        let strings : Vec<&str> = check.split(c).collect();

        if !text.starts_with(strings[0]) { return None; }
        
        let mut prev = strings[0];
        for check in strings.iter().skip(1) {
            text = trim_delimeter_start(&text[(prev.len())..]);
            if !text.starts_with(check) { return None; }
            prev = check;
        }

        text = &text[(prev.len())..].trim_start();
    } else {
        if !text.starts_with(check) { return None; }
        text = &text[(check.len())..].trim_start();
    }

    if text.starts_with("]") {
        Some((initial_length - text.len()) + 1)
    } else {
        None
    }
}

pub fn parse_table_of_contents(object: &mut ParseObject) -> Option<TopLevelSyntax> {
    let current = object.current();
    if let Some(n) = is_meta(current, "toc") {
        let text = current[n..].trim_start().to_string();
        return Some(TopLevelSyntax::TOC(text));
    }
    
    if let Some(n) = is_meta(current, "table-of-content") {
        let text = current[n..].trim_start().to_string();
        return Some(TopLevelSyntax::TOC(text));
    }
    
    if let Some(n) = is_meta(current, "table-of-contents") {
        let text = current[n..].trim_start().to_string();
        return Some(TopLevelSyntax::TOC(text));
    }

    None 
}

pub fn parse_heading(object: &mut ParseObject) -> Option<TopLevelSyntax> {
    if object.starts_with('#') {
        let current = object.current();
        let mut counter = 0;
        while current[counter..].starts_with('#') { 
            counter += 1; 
            if counter >= current.len() { break }
        }
        
        Some(TopLevelSyntax::Heading( current[counter..].trim_start().into(), counter))
    } else {
        None
    }
}

pub fn parse_note(object: &mut ParseObject) -> Option<TopLevelSyntax> {
    let current = object.current();
    if !current.starts_with("[") { return None }
    if !current[1..].trim_start().starts_with("^") { return None }

    // this is most likely a note definition. as in [^n]: ...
    let remaining = &current[1..].trim_start()[1..];
    let mut note_id = String::new();
    let mut peekable = remaining.chars().peekable();
    while let Some(character) = peekable.peek() && character != &']' {
        note_id.push(*character);
        peekable.next();
    }

    let Some(']') = peekable.next() else { return None };
    let Some(':') = peekable.next() else { return None };

    note_id = note_id.trim().to_string();
    if note_id.len() == 0 { return None }
    
    let text : String = peekable.collect();

    Some(TopLevelSyntax::NoteDefinition{id: note_id, text: text.trim().to_string()})
}

fn find_end_balanced(text: &str, delimiters: (char, char)) -> Option<usize> {
    let mut found_one = false;
    let mut character_count = 0;
    let mut delim_count: usize = 0;
    for character in text.chars() {
        if character == delimiters.0 {
            delim_count = delim_count + 1;
        }
        else if character == delimiters.1 {
            if delim_count == 0 {
                found_one = true;
                break; 
            }
            delim_count = delim_count - 1;
        }
        character_count = character_count + 1;
    }

    if found_one {
        Some(character_count)
    } else {
        None
    }
}

pub fn parse_factbox(object: &mut ParseObject) -> Option<TopLevelSyntax> {
    if !(object.current().starts_with("[[fact]") || object.current().starts_with("[[factbox]")) {
        return None;
    }

    let len = if object.current().starts_with("[[fact]") { "[[fact]".len() } else { "[[factbox]".len() };
    let last = find_end_balanced(&object.text()[len..], ('[', ']'))?;
    
    let mut text_to_parse = String::new();
    let text = &&object.text()[len..last + len];
    let title = text.lines().take(1).nth(0);
    for line in text.lines().skip(1) {
        let trimmed = line.trim_start();
        text_to_parse += trimmed;
        text_to_parse.push('\n');
    }
    
    let Ok(body) = toplevel_parse(&text_to_parse) else { return None };
    
    let title = if let Some(text) = title && !text.trim().is_empty() {
        text
    } else {  
        DEFAULT_FACTBOX_TITLE
    }.to_string();

    object.skip(last + len);
    Some(TopLevelSyntax::FactBox{title, body})
}

pub fn parse_embedding(object: &mut ParseObject) -> Option<TopLevelSyntax> {
    let current = object.current();
    if !(current.starts_with("[[") || current.starts_with("![[")) {
        return None;
    }

    let Some(first_character) = current.chars().nth(0) else { return None };
    let count = if first_character == '!' { 3 } else { 2 };
    let text :&str = &current[count..];
    let Some(img_end) = text.find(']') else { return None };
    let img = &text[0..img_end];
    let mut remaining_on_line = &text[img_end + 1..];
    
    let mut alt_text = String::new();
    if remaining_on_line.len() != 0 {
        remaining_on_line = remaining_on_line.trim_start();
        if remaining_on_line.starts_with(']') {
            return Some(TopLevelSyntax::Image(Image::new(img.into())));
        }
    
        if let Some(index) = remaining_on_line.find(']') {
            alt_text = remaining_on_line[0..index].into();
            return Some(TopLevelSyntax::Image(Image::new_with_alt(img.into(), alt_text)));
        }
    
        if remaining_on_line.len() > 0 {
            alt_text += remaining_on_line.into();
            alt_text += "\n";
        }
    }
    
    let image_text = String::from(img);
    let Some(end_index) = object.find(']', 0) else { return None };
    
    
    object.next();
    alt_text += &object.text()[..end_index];
    
    if !image_text.contains('|') {
        return Some(TopLevelSyntax::Image(Image::new_with_alt(image_text, alt_text)))
    }

    let elems = image_text.split('|').collect::<Vec<_>>();
    if elems.len() < 2 {
        return Some(TopLevelSyntax::Image(Image::new_with_alt(image_text, alt_text)))
    }

    let img = elems[0];
    let size = elems.last().expect("should work");
    let width  = if size.contains('x') {
        size.split('x').collect::<Vec<_>>().iter().nth(0).map_or_else(|| None, |text| Some(text.to_string()))
    } else { if size.parse::<usize>().is_ok() { Some(size.to_string()) } else { None } };
    
    let height = if size.contains('x') {
        size.split('x').collect::<Vec<_>>().iter().nth(1).map_or_else(|| None, |text| Some(text.to_string()))
    } else { None };

    let size = ImageSize::from_width_and_height(width, height);

    Some(TopLevelSyntax::Image(Image::new_with_alt_and_size(img.to_string(), alt_text, size)))
}

pub fn parse_citation(object: &mut ParseObject) -> Option<TopLevelSyntax> {
    let current = object.current();
    let Some('£') = current.chars().nth(0) else { return None };
    if !current.chars().nth(1).is_some_and(|c| c.is_alphabetic() || c == '-') { return None }
    
    

    let Some(end) = object.find('}', 0) else { return None };
    let Ok(citation) = parse_reference(object.text()[..end + 1].to_string()) else {
        return None;
    };
    Some(TopLevelSyntax::ReferenceDefinition(citation))
}

pub fn toplevel_parse(file_content: &String) -> Result<Vec<TopLevelSyntax>> {
    let (frontmatter, content) = parse_frontmatter(&file_content);
    let mut object = ParseObject::new(content);
    
    if let Some(frontmatter) = frontmatter {
        object.push(TopLevelSyntax::FrontMatter(frontmatter));
    }

    while object.has_text() {
        if object.current().is_empty() {
            object.next();
            object.consume();
            continue;
        }
        
        if skip_comment(&mut object) {
            continue;
        }
        
        if let Some(elem) = parse_quote(&mut object) {
            object.next();
            object.push(elem);
            continue;
        }
        
        if let Some(elem) = parse_pagebreak(&mut object) {
            object.next();
            object.push(elem);
            continue;
        }

        if let Some(elem) = parse_list(&mut object) {
            object.next();
            object.push(elem);
            continue;
        }

        if let Some(elem) = parse_codeblock(&mut object) {
            object.next();
            object.push(elem);
            continue;
        }

        if let Some(elem) = parse_table_of_contents(&mut object) {
            object.next();
            object.push(elem);
            continue;
        }
        
        if let Some(elem) = parse_heading(&mut object) {
            object.next();
            object.push(elem);
            continue;
        }
        
        if let Some(elem) = parse_note(&mut object) {
            object.next();
            object.push(elem);
            continue;
        }

        if let Some(elem) = parse_factbox(&mut object) {
            object.next();
            object.push(elem);
            continue;
        }

        if let Some(elem) = parse_embedding(&mut object) {
            object.next();
            object.push(elem);
            continue;
        }

        if let Some(elem) = parse_citation(&mut object) {
            object.next();
            object.push(elem);
            continue;
        }

        object.eat_line();
    }

    Ok(object.syntax)
}


mod tests {
    use super::*;
    
    mod util {
        use super::*;

        #[test]
        fn skipping_comment() {
            let mut obj = ParseObject::new("%% a single line comment%%");
            let result = skip_comment(&mut obj);
            assert_eq!(result, true);
            assert_eq!(obj.has_text(), false);
        }
        
        #[test]
        fn skipping_comment_with_starting_whitespace() {
            let mut obj = ParseObject::new("         %% a single line comment%%");
            let result = skip_comment(&mut obj);
            assert_eq!(result, true);
            assert_eq!(obj.has_text(), false);
        }
        #[test]
        fn skipping_comment_spanning_multiple_lines() {
            let mut obj = ParseObject::new("%% a comment\nover mulitple\nlines%%");
            let result = skip_comment(&mut obj);
            assert_eq!(result, true);
            assert_eq!(obj.has_text(), false);
        }
        
        #[test]
        fn skipping_comment_with_text_after() {
            let mut obj = ParseObject::new("%% a comment %%this should not be deleted");
            let result = skip_comment(&mut obj);
            assert_eq!(result, true);
            assert_eq!(obj.has_text(), true);
            assert_eq!(obj.text(), "this should not be deleted");
        }
        
        #[test]
        fn test_string_has_delimeter() {
            assert!(false);
        }
        
        #[test]
        fn test_trim_delimeter_start() {
            assert!(false);
        }

        #[test]
        fn test_is_meta() {
            assert!(false);
        }

        #[test]
        fn test_find_end_balanced() {
            assert!(false);
        }

    }

    mod element_parsers {
        use super::*;

    }

    mod full_parser {
        use super::*;
        
    }
}
