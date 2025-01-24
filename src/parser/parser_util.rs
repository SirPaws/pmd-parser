#[cfg(not(feature = "wasm"))]
use anyhow::{Result, Context};
use super::structs::*;
use super::frontmatter::*;
use super::config::*;
use super::util::ordered_map::OrderedMap;
use std::collections::HashSet;

pub(super) struct ParserInfo {
    pub(super) notes: OrderedMap<String, Element>,
    pub(super) references: OrderedMap<String, ReferenceDefinition>,
    pub(super) metadata: MetaData,
    pub(super) body: Vec::<(Element, String)>,

    pub(super) ids: HashSet<String>,
    pub(super) num_codeblocks: usize,
    pub(super) num_image: usize,
    pub(super) num_lists: usize,
    pub(super) num_quotes: usize,
    pub(super) num_factboxes: usize,
}

impl ParserInfo {
    pub(super) fn new() -> Self {
        let mut notes      = OrderedMap::<String, Element>::new();
        let mut references = OrderedMap::<String, ReferenceDefinition>::new();
        let mut metadata = MetaData::default();
        let mut body = Vec::<(Element, String)>::new();
        
        let mut ids = HashSet::<String>::new();
        let mut num_codeblocks = 0usize;
        let mut num_image = 0usize;
        let mut num_lists = 0usize;
        let mut num_quotes = 0usize;
        let mut num_factboxes = 0usize;
        Self {
            notes,
            references,
            metadata,
            body,
            
            ids,
            num_codeblocks,
            num_image,
            num_lists,
            num_quotes,
            num_factboxes,
        }
    } 

    pub(super) fn build(self, notes_id: String, bibliography_id: String) -> AssDownDocument {
        AssDownDocument { 
            meta: self.metadata, 
            bibliography_id,
            notes_id,
            references: self.references, 
            notes: self.notes,
            body: self.body
        }
    }
}


type PeekableChars<'l> = std::iter::Peekable<std::str::Chars<'l>>;
pub(super) fn gather_link<'l>(mut end: PeekableChars<'l>, depth: &mut i32) -> Result<(String, PeekableChars<'l>)> {
    let mut alt = String::new();
    if end.peek() == Some(&'(') {
        end.next();
        while !(end.peek() == Some(&')') && depth == &0) {
            let character = end.next().context("expected ')'")?;
            match character {
                '(' => *depth += 1,
                ')' => if depth != &0 { *depth -= 1 } else {},
                _   => {}
            }
            alt.push(character);
        }
        end.next();
        Ok((alt, end.clone()))
    }
    else {
        Ok((alt, end.clone()))
    }
}

pub(super) fn get_citation(text: &String) -> Option<Element> {
    if text.starts_with('£') && text.trim_start().chars().nth(1).is_some_and(|x| x.is_alphabetic() || x == '-') {
        // this is a citation
        let citation : String = text.chars().skip(1).collect();
        Some(Element::Citation(citation))
    } else if text.starts_with('^') && text.len() > 1 {
        // this is a citation
        let citation : String = text.chars().skip(1).collect();
        Some(Element::Note(citation))
    } else {
        None
    }
}

pub(super) fn generate_id(text: &String) -> Option<String> {
    if text.split_whitespace().collect::<String>().is_empty() {
        None
    } else {
        let words = text.split_whitespace().collect::<Vec<_>>();
        let mut id = String::new();
        for word in words {
            if id.len() + word.len() > MAX_ID_LENGTH {
                id = word.to_string();
                break;
            }
            id += word;
            id.push('-');
        }
        if id.len() > MAX_ID_LENGTH {
            Some(id[..MAX_ID_LENGTH].to_string())
        } else {
            id.remove(id.len() - 1);
            Some(id)
        }
    }
}

pub(super) fn is_valid_id(text: &String) -> bool {
    !text.split_whitespace().collect::<String>().is_empty()
}

pub(super) fn get_url(data: &Frontmatter) -> Option<String> {
    if let Some(url) = data["url"].as_string() {
        Some(url)
    } else if let Some(url) = data["base_url"].as_string() {
        Some(url)
    } else if let Some(url) = data["base url"].as_string() {
        Some(url)
    } else if let Some(url) = data["base-url"].as_string() {
        Some(url)
    } else {
        None
    }
}

pub(super) fn get_data_dir(data: &Frontmatter) -> Option<String> {
    if let Some(url) = data["data"].as_string() {
        Some(url)
    } else if let Some(url) = data["data_dir"].as_string() {
        Some(url)
    } else if let Some(url) = data["data-dir"].as_string() {
        Some(url)
    } else if let Some(url) = data["data dir"].as_string() {
        Some(url)
    } else {
        None
    }
}

pub(super) fn get_blog_dir(data: &Frontmatter) -> Option<String> {
    if let Some(url) = data["blog"].as_string() {
        Some(url)
    } else if let Some(url) = data["blog_dir"].as_string() {
        Some(url)
    } else if let Some(url) = data["blog-dir"].as_string() {
        Some(url)
    } else if let Some(url) = data["blog dir"].as_string() {
        Some(url)
    } else {
        None
    }
}

pub(super) fn check_frontmatter(fm: &Frontmatter, keys: &[&str]) -> bool {
    for checked_key in keys {
        for key in fm.keys() {
            let key = key.trim().replace('_', "-").replace(' ', "-");
            if &key == checked_key {
                return true;
            }
        }
    }
    false
}

pub(super) fn get_frontmatter_text(fm: &Frontmatter, keys: &[&str]) -> String {
    for checked_key in keys {
        for key in fm.keys() {
            let key = key.trim().replace('_', "-").replace(' ', "-");
            if &key == checked_key {
                return fm[key].as_string().unwrap_or(String::new());
            }
        }
    }
    String::new()
}

pub(super) fn get_date(data: &Frontmatter) -> Option<String> {
    if let Some(date) = data["date"].as_string() {
        Some(date)
    } else if let Some(date) = data.get_with_joined_key("date-written").as_string() {
        Some(date)
    } else {
        None
    }
}

pub(super) fn get_last_update(data: &Frontmatter) -> Option<String> {
    if let Some(date) = data.get_with_joined_key("last-update").as_string() {
        Some(date)
    } else if let Some(date) = data.get_with_joined_key("last-updated").as_string() {
        Some(date)
    } else {
        None
    }
}

pub(super) fn get_bibliography_title(data: &Frontmatter) -> Option<String> {
    if let Some(title) = data.get_with_joined_key("bibliography-title").as_string() {
        Some(title)
    } else if let Some(title) = data.get_with_joined_key("references-title").as_string() {
        Some(title)
    } else if let Some(title) = data.get_with_joined_key("sources-title").as_string() {
        Some(title)
    } else {
        None
    }
}

