#[cfg(not(feature = "wasm"))]
use color_print::cprintln;
use anyhow::{Context, Result, anyhow};
use std::ops::{Deref, DerefMut};

use crate::{
    any_non_empty, 
    bibliograph_name, 
    ordered_map::OrderedMap, 
    paws_markdown::BlogBody, 
    to_citation, 
    Alternative, 
    BlogHeader, 
    FactBox, 
    PMDSerializer, 
    PawsMarkdown, 
    ReferenceDefinition, 
    Span
};

pub struct Reference<T> {
    pub def: T,
    pub times_used: usize
}

impl<T> Reference<T> {
    pub fn new(def: T) -> Self {
        Self { def, times_used: 0 }
    }
}

pub struct Weak<T> {
    data: *mut T,
}

impl<T> Weak<T> {
    pub fn new(value: &mut T) -> Self {
        Self { data: value as *mut T }
    }
    #[allow(dead_code)]
    pub fn from_ref(value: &T) -> Self {
        Self { data: (value as *const T) as *mut T}
    }
    pub fn uninit() -> Self {
        Self { data: std::ptr::null::<T>() as *mut T }
    }
    pub fn launder(ptr: &Self) -> Self {
        Self { data: ptr.data }
    }

    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.data == std::ptr::null_mut::<T>()
    }

    pub fn as_ref(&self) -> &T {
        unsafe { self.data.as_ref_unchecked() }
    }

    pub fn as_mut(&self) -> &mut T {
        unsafe { self.data.as_mut_unchecked() }
    } 
}

impl<T> Deref for Weak<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for Weak<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[derive(Clone)]
pub enum ObjectKind {
    Header(usize),
    Paragraph,
    CodeBlock,
    Quote,
    Image,
    List,
    FactBox,
}

pub trait PMDSharedHTMLSerializer: PMDSerializer {
    const LINK_ELEMENTS: bool = true;
    const POPUPS: bool = true;
    const SHOW_BACKREFS: bool = true;

    // rust is dumb as shit so we have to copy the header
    fn get_header(&self) -> &BlogHeader;
    fn get_description(&mut self, md: &PawsMarkdown) -> Result<String>;
    fn prepare_html_header(&mut self, description: &String) -> String;
    fn convert_body(&mut self, md: &PawsMarkdown) -> Result<String>;
    fn generate_link(&mut self, id: &String, kind: ObjectKind) -> String;

    fn notes_id(&mut self) -> String;
    fn bibliography_id(&mut self) -> String;

    fn references(&mut self) -> &OrderedMap<String, Reference<ReferenceDefinition>>;
    
    fn mut_references(&mut self) -> &mut OrderedMap<String, Reference<ReferenceDefinition>>;
    
    fn get_mut_reference<T: AsRef<str>>(&mut self, key: T) -> Option<&mut Reference<ReferenceDefinition>>;

    fn get_reference<T: AsRef<str>>(&mut self, key: T) -> Option<&Reference<ReferenceDefinition>>;
}

pub struct PMDHTML<T: PMDSharedHTMLSerializer> {
    pub filename: String,
    pub parent: Weak<T>,
    num_tabs:   usize,
    current_factbox: Option<(FactBox, Option<String>)>
}

impl<T: PMDSharedHTMLSerializer> PMDHTML<T> {
    pub fn new(filename: &str, parent: &mut T) -> Self {
        Self { 
            filename: filename.into(),
            parent: Weak::new(parent),
            num_tabs:   0,
            current_factbox: None,
        }
    }

    pub fn uninit() -> Self {
        Self { 
            filename: String::new(),
            parent: Weak::uninit(),
            num_tabs:   0,
            current_factbox: None,
        }
    }
    
    /*
    pub fn generate_id<F: FnMut()->String>(text: &String, kind: ObjectKind, mut default_generator: F) -> String {
        let result = if text.len() == 0 { default_generator() } else {
            let mut len = 0;
            for c in text.chars() {
                if c.is_ascii_punctuation() { 
                    break
                } else {
                    len += c.len_utf8();
                }
            }
            if len == 0 || text[0..len].trim_start().trim_end().len() == 0 { default_generator() } else {
                let mut result = String::new();
                for c in text[0..len].trim_start().trim_end().chars() {
                    result.push(if c.is_whitespace() { '-' } else { c })
                }
                result.remove_matches(|x: char| x != '-' && x.is_ascii_punctuation());
                sanitize_text(&result.to_lowercase())
            }
        };

        match kind {
            ObjectKind::Header(depth) => format!("{result}-h{depth}"),
            ObjectKind::Paragraph => format!("{result}-p"),
            ObjectKind::CodeBlock => format!("{result}-cb"),
            ObjectKind::Quote => format!("{result}-q"),
            ObjectKind::Image => format!("{result}-img"),
            ObjectKind::List => format!("{result}-list"),
            ObjectKind::FactBox => format!("{result}-factbox"),
        }
    }
    */
    
    pub fn push_tab(&mut self) { self.num_tabs += 1; }
    pub fn pop_tab(&mut self) { if self.num_tabs > 0 { self.num_tabs -= 1; } }
    pub fn tab(&mut self) -> String {
        if self.num_tabs == 0 { "".to_string() }
        else {
            let mut result = String::new();
            for _ in 0..(self.num_tabs) {
                result += "    ";
            }
            result
        }
    }

    pub fn current_factbox(&mut self) -> Option<(FactBox, Option<String>)> {
        self.current_factbox.clone()
    }

    pub fn html(
        &mut self, md: &PawsMarkdown, navbar: Option<&String>, references: &OrderedMap<String, ReferenceDefinition>, notes: &OrderedMap<String, BlogBody>) 
        -> Result<String> 
    {
        let mut output = String::new();
        let laundered_parent = Weak::launder(&self.parent);
        let description = self.parent.get_description(md)?;
        let header = self.parent.prepare_html_header(&description);
        let blog_header = laundered_parent.get_header();
        output +=   "<!doctype html>\n";
        output +=   "<html>\n";
        self.push_tab();

        output += self.tab().as_str();
        output +=   "<head>\n";
        self.push_tab();
        for line in header.lines() {
            output += self.tab().as_str();
            output += line;
            output += "\n";
        }
        self.pop_tab();
        output += self.tab().as_str();
        output += "</head>\n";

        output += self.tab().as_str();
        output += "<body>\n";
        self.push_tab();

        if let Some(navbar) = navbar {
            for line in navbar.lines() {
                output += self.tab().as_str();
                output += line;
                output += "\n";
            }
        }

        output += self.tab().as_str();
        output += "<main>\n";
        self.push_tab();

        {
            let title    = &blog_header.title;
            let subtitle = &blog_header.subtitle;
            output += self.tab().as_str();
            output += "<section class='title'>\n";
            self.push_tab();
                output += self.tab().as_str();
                output += format!("<h1>{title}</h1>\n").as_str();
                
                output += self.tab().as_str();
                output += format!("<p class='subtitle'>{subtitle}</p>\n").as_str();
            self.pop_tab();

            output += self.tab().as_str();
            output += format!("</section>\n").as_str();
        }

        output += self.parent.convert_body(md)?.as_str();

        if !(notes.is_empty() || blog_header.hide_notes) {
            let id = self.parent.notes_id();
            let title = &blog_header.notes_title;
            let link = if T::LINK_ELEMENTS {
                self.parent.generate_link(&id, ObjectKind::Header(1))
            } else {
                "".into()
            };

            
            output += self.tab().as_str();
            output += format!("<section class='page-break'>\n").as_str();
            self.push_tab();
                output += self.tab().as_str();
                output += "<hr>\n";
            self.pop_tab();
            output += self.tab().as_str();
            output += format!("</section>\n").as_str();
                
            output += self.tab().as_str();
            output += format!("<section class='notes' id='{id}'>\n").as_str();
            self.push_tab();
            if T::LINK_ELEMENTS {
                output += self.tab().as_str();
                output += link.as_str();
                output.push('\n');
            }
                
                output += self.tab().as_str();
                output += format!("<h1>{title}</h1>\n").as_str();
            self.pop_tab();
            output += self.tab().as_str();
            output += format!("</section>\n").as_str();
                
            for (key, val) in notes {
                let result = self.parent.convert_element((val, &String::new()))?;
                let link = format!("<a href='#^{key}'>^{key}:</a>");
            
                output += self.tab().as_str();
                output += format!("<section class='note' id=\"^{key}\">").as_str();
                output.push('\n');
                self.push_tab();
            
                    output += self.tab().as_str();
                    output += "<sup>";
                    self.push_tab();
                        output += link.as_str();
                        output.push('\n');
                        
                        output += self.tab().as_str();
                        output += result.as_str();
                        output.push('\n');
                        
                    if T::SHOW_BACKREFS {
                        output += self.tab().as_str();
                        output += format!("<a href=\"#{key}-backref\">").as_str();
                        output += "↩";
                        output += "</a>";
                        output.push('\n');
                    }
                    self.pop_tab();
                    output += self.tab().as_str();
                    output += "</sup>";
                    output.push('\n');
            
                self.pop_tab();
                output += self.tab().as_str();
                output += "</section>\n";
            }
        }

        if !(references.is_empty() || blog_header.hide_references) {
            let id = self.parent.bibliography_id();
            let title = &blog_header.bibliography_title;
            let link = if T::LINK_ELEMENTS {
                self.parent.generate_link(&id, ObjectKind::Header(1))
            } else {
                "".into()
            };

            output += self.tab().as_str();
            output += format!("<section class='page-break'>\n").as_str();
            self.push_tab();
                output += self.tab().as_str();
                output += "<hr>\n";
            self.pop_tab();
            output += self.tab().as_str();
            output += format!("</section>\n").as_str();

            output += self.tab().as_str();
            output += format!("<section id='{id}'>\n").as_str();
            self.push_tab();
            if T::LINK_ELEMENTS {
                output += self.tab().as_str();
                output += link.as_str();
                output.push('\n');
            }
                
                output += self.tab().as_str();
                output += format!("<h1>{title}</h1>\n").as_str();
            self.pop_tab();
            output += self.tab().as_str();
            output += format!("</section>\n").as_str();

            for (key, val) in self.parent.references() {
                if val.times_used == 0 {
                    #[cfg(not(feature = "wasm"))]
                    cprintln!("<y>warning:</> reference '{}' is not used and will not be included", key);
                }
            }
            
            for (key, val) in references {
                let reference = self.parent.get_reference(key).unwrap();
                if reference.times_used == 0 { continue; }

                output += self.tab().as_str();
                output += format!("<section class='citation' id='{key}'>\n").as_str();
                self.push_tab();
                    output += self.tab().as_str();
                    output += format!("<p>\n").as_str();
                    self.push_tab();
                    
                        output += to_html_bibliography(val).as_str();
                        output.push('\n');
                    
                    if T::SHOW_BACKREFS {
                        output += self.tab().as_str();
                        output += format!("<a href=''>").as_str();
                        output += "↩";
                        output += "</a>";
                        output.push('\n');
                    }
                    self.pop_tab();
                    output += self.tab().as_str();
                    output += format!("</p>\n").as_str();

                self.pop_tab();
                output += self.tab().as_str();
                output += "</section>\n";
            }
        }        

        self.pop_tab();
        output += self.tab().as_str();
        output += "</main>\n";

        if T::POPUPS {
            output += self.tab().as_str();
            output +="<div id='popup' class='popup-hidden' aria-hidden='true'>\n";
            self.push_tab();
            
                output += self.tab().as_str();
                output +="<div class='popup-clickable-region' onclick='close_popup(this)' ></div>\n";
                output += self.tab().as_str();
                output +="<div class='popup-container'>\n";
                self.push_tab();
            
                    output += self.tab().as_str();
                    output +="<img id='popup-image'>\n";
                    output += self.tab().as_str();
                    output +="<p id='popup-caption'></p>\n";
            
                self.pop_tab();
                output += self.tab().as_str();
                output +="</div>\n";
                self.pop_tab();

            output += self.tab().as_str();
            output +="</div>\n";
        }

        self.pop_tab();
        output += self.tab().as_str();
        output +=   "</body>\n";

        self.pop_tab();
        output += self.tab().as_str();
        output +=   "</html>\n";

        Ok(output)
    }
    
    pub fn convert_factbox(&mut self, factbox: &FactBox, id: &String) -> Result<String> {
        let id = sanitize_id(id);
        let link = if T::LINK_ELEMENTS { self.parent.generate_link(&id, ObjectKind::FactBox) } else { String::new() };
        let title = &factbox.title;
        let laundered_parent = Weak::launder(&self.parent);
        let header = laundered_parent.get_header();

        let mut output = self.tab();
        output += format!("<section id='{id}'>\n").as_str();
        self.push_tab();
        if T::LINK_ELEMENTS {
            output += self.tab().as_str();
            output += format!("{link}\n").as_str();
        }
            
            output += self.tab().as_str();
            output += "<article class='factbox'>\n";
            self.push_tab();

                output += self.tab().as_str();
                output += "<header class='factbox-header'>\n";
                self.push_tab();

                    output += self.tab().as_str();
                    output += format!("<h2>{title}</h2>\n").as_str();

                self.pop_tab();
                output += self.tab().as_str();
                output += "</header>\n";
                
                output += self.tab().as_str();
                output += "<section class='factbox-content'>\n";
                self.push_tab();

                self.current_factbox = Some((factbox.clone(), Some(id.clone())));

                for element in self.parent.convert_factbox_elements(factbox, Some(&id))? {
                    output += element.as_str();
                }
                
                self.current_factbox = None;


                if !(factbox.notes.is_empty() || header.hide_notes) {
                    let notes_id = self.parent.notes_id();
                    let id = format!("{id}-{notes_id}");
                    let title = header.notes_title.clone();
                    let link = if T::LINK_ELEMENTS { self.parent.generate_link(&id, ObjectKind::Header(1)) } else { String::new() };
                
                    
                    output += self.tab().as_str();
                    output += format!("<section class='page-break'>\n").as_str();
                    self.push_tab();
                        output += self.tab().as_str();
                        output += "<hr>\n";
                    self.pop_tab();
                    output += self.tab().as_str();
                    output += format!("</section>\n").as_str();
                        
                    output += self.tab().as_str();
                    output += format!("<section class='notes' id='{id}'>\n").as_str();
                    self.push_tab();
                    if T::LINK_ELEMENTS {
                        output += self.tab().as_str();
                        output += link.as_str();
                        output.push('\n');
                    }
                        
                        output += self.tab().as_str();
                        output += format!("<h1>{title}</h1>\n").as_str();
                    self.pop_tab();
                    output += self.tab().as_str();
                    output += format!("</section>\n").as_str();
                        
                    for (key, (val, note_id)) in &factbox.notes {
                        let result = self.parent.convert_element(no_id!(val))?;
                        let link = format!("<a href='#^{note_id}'>^{key}:</a>");
                    
                        output += self.tab().as_str();
                        output += format!("<section class='note' id=\"^{note_id}\">").as_str();
                        output.push('\n');
                        self.push_tab();
                    
                            output += self.tab().as_str();
                            output += "<sup>";
                            self.push_tab();
                                output += link.as_str();
                                output.push('\n');
                                
                                output += self.tab().as_str();
                                output += result.as_str();
                                output.push('\n');
                                
                            if T::SHOW_BACKREFS {
                                output += self.tab().as_str();
                                output += format!("<a href=\"#{note_id}-backref\">").as_str();
                                output += "↩";
                                output += "</a>";
                                output.push('\n');
                            }
                            self.pop_tab();
                            output += self.tab().as_str();
                            output += "</sup>";
                            output.push('\n');
                    
                        self.pop_tab();
                        output += self.tab().as_str();
                        output += "</section>\n";
                    }
                }

                self.pop_tab();
                output += self.tab().as_str();
                output += "</section>\n";
            
            self.pop_tab();
            output += self.tab().as_str();
            output += "</article>\n";
            
        self.pop_tab();
        output += self.tab().as_str();
        output += "</section>\n";

        Ok(output)
    }
    
    pub fn convert_hoverable(&mut self, hoverable: &Alternative) -> Result<String> {
        let base = self.parent.convert_element(no_id!(&hoverable.base))?;
        let alt  = self.parent.convert_element(no_id!(&hoverable.alt))?;
        Ok(format!("<span class='hoverable'><span class='hover-base'>{base}</span><span class='hover-alt'>{alt}</span></span>"))
    }

    pub fn convert_styled(&mut self, styled: &Alternative) -> Result<String> {
        let text  = self.parent.convert_element(no_id!(&styled.alt))?;
        let style = self.parent.convert_element(no_id!(&styled.base))?;
        Ok(format!("<span class='embedded-style' style='{style}'>{text}</span>"))
    }
    
    // fn convert_embedded_link<T: PMDSharedHTMLSerializer>(&mut self, src: &String, alt: &String) -> Result<String> {
    // }

    pub fn convert_link(&mut self, link: &Alternative) -> Result<String> {
        let laundered_parent = Weak::launder(&self.parent);
        let href = self.parent.convert_element(no_id!(&link.alt))?;
        let text = self.parent.convert_element(no_id!(&link.base))?;
        let header = laundered_parent.get_header();
        match &link.alt {
            box BlogBody::Citation(citation) => {
                if let Some(reference) = convert_custom_citation(
                    self.parent.mut_references().get_mut(citation.as_str()) , 
                    &citation, &text, header.hide_references, T::SHOW_BACKREFS)
                {
                    Ok(reference)
                } else {
                    #[cfg(not(feature = "wasm"))]
                    cprintln!("<y>warning:</> {} has no source", citation);
                    Ok(format!("<cite style='color=red; background-color: yellow'>{text}</cite>"))
                }
            }, 
            box BlogBody::Note(note) => {
                if header.hide_notes {
                    Ok("".into())
                } else {
                    Ok(format!("<a class='inline-link' href='#^{note}'>{text}</a>"))
                }
            },
            _ => {
                Ok(format!("<a class='inline-link' href='{href}'>{text}</a>"))
            }
        }
    }

    pub fn convert_header(&mut self, text: &Box<BlogBody>, depth: usize, id: &String) -> Result<String> {
        let id = sanitize_id(id);
        let text  = self.parent.convert_element(no_id!(text))?;
        let link = if T::LINK_ELEMENTS { self.parent.generate_link(&id, ObjectKind::Header(depth)) } else { "".into() };
        
        let mut result = self.tab();
        result += format!("<section id='{id}'>\n").as_str();
        self.push_tab();
        if T::LINK_ELEMENTS {
            result += self.tab().as_str();
            result += link.as_str();
            result.push('\n');
        }
            
            result += self.tab().as_str();
            result += format!("<h{depth}>{text}</h{depth}>\n").as_str();
        self.pop_tab();
        result += self.tab().as_str();
        result += "</section>\n";

        Ok(result)
    }

    pub fn convert_italics(&mut self, text: &Box<BlogBody>) -> Result<String> {
        let inner_text = self.parent.convert_element(no_id!(&text))?;
        Ok(format!("<i>{inner_text}</i>"))
    }

    pub fn convert_bold(&mut self, text: &Box<BlogBody>) -> Result<String> {
        let inner_text = self.parent.convert_element(no_id!(&text))?;
        Ok(format!("<b>{inner_text}</b>"))
    }

    pub fn convert_inlinecode(&mut self, text: &String) -> Result<String> {
        let text = sanitize_text(text);
        Ok(format!("<code>{text}</code>"))
    }

    pub fn convert_codeblock(&mut self, text: &String, id: &String) -> Result<String> {
        let id = sanitize_id(id);
        let first_line = text.lines().nth(0).context("expected at least one line in codeblock")?;
        let mut words = first_line.split(|x: char| x.is_whitespace());
        let lang = words.nth(0).unwrap_or("plaintext");
        let mut body = text[text.find('\n').context("expected at least one line in codeblock")? + 1..].to_string();
        let link = if T::LINK_ELEMENTS { self.parent.generate_link(&id, ObjectKind::CodeBlock) } else { String::new() };
        

        body = sanitize_text(&body);
        body = body.trim_end().replace("\r\n", "\n");
        
        let mut result = self.tab();
        result += format!("<section class='code-block' id='{id}'>\n").as_str();
        self.push_tab();
        if T::LINK_ELEMENTS {
            result += self.tab().as_str();
            result += format!("{link}\n").as_str();
        }
            
            result += self.tab().as_str();
            result += format!("<pre><code class='language-{lang}'>{body}</code></pre>\n").as_str();
        self.pop_tab();
        result += self.tab().as_str();
        result += "</section>\n";

        Ok(result)
    }

    pub fn convert_image(&mut self, src: &String, alt: &String, id: &String) -> Result<String> {
        let id = sanitize_id(id);
        let link = if T::LINK_ELEMENTS { self.parent.generate_link(&id, ObjectKind::Image) } else { String::new() };

        let mut result = self.tab();
        result += format!("<section class='image' id='{id}'>\n").as_str();
        self.push_tab();
        if T::LINK_ELEMENTS {
            result += self.tab().as_str();
            result += format!("{link}\n").as_str();
        }
            
            result += self.tab().as_str();
            result += format!("<img onclick='makePopup(this)' src='{src}' alt='{alt}'></img>\n").as_str();
        self.pop_tab();
        result += self.tab().as_str();
        result += "</section>\n";

        Ok(result)
    }

    pub fn convert_quote(&mut self, lines: &Vec<BlogBody>, id: &String) -> Result<String> {
        let id = sanitize_id(id);
        let link = if T::LINK_ELEMENTS { self.parent.generate_link(&id, ObjectKind::Quote) } else { String::new() };
        
        let mut quote_elements : Vec<String> = vec![];
        for elem in lines {
            let text = self.parent.convert_element(no_id!(elem))?;
            quote_elements.push(format!("{text}<br/>"));
        }
        let text = quote_elements.join("\n");

        let mut result = self.tab();
        result += format!("<section class='quote' id='{id}'>\n").as_str();
        self.push_tab();
        if T::LINK_ELEMENTS {
            result += self.tab().as_str();
            result += format!("{link}\n").as_str();
        }
            
            result += self.tab().as_str();
            result += format!("<div class='quote-line'></div><blockquote class='quote-text'>{text}</blockquote>\n").as_str();
        self.pop_tab();
        result += self.tab().as_str();
        result += "</section>\n";

        Ok(result)
    }

    pub fn convert_list(&mut self, list: &Vec<BlogBody>, id: &String) -> Result<String> {
        let id = sanitize_id(id);
        let link = if T::LINK_ELEMENTS { self.parent.generate_link(&id, ObjectKind::List) } else { String::new() };

        let mut result = self.tab();
        result += format!("<section class='list' id='{id}'>\n").as_str();
        self.push_tab();
        if T::LINK_ELEMENTS {
            result += self.tab().as_str();
            result += format!("{link}\n").as_str();
        }
            
            result += self.tab().as_str();
            result += "<ul>";
            self.push_tab();
            for elem in list {
                let text = self.parent.convert_element(no_id!(elem))?;
                result += self.tab().as_str();
                result += format!("<li>{text}</li>\n").as_str();
            }
            self.pop_tab();
            result += self.tab().as_str();
            result += "</ul>";
        self.pop_tab();
        result += self.tab().as_str();
        result += "</section>\n";

        Ok(result)
    }

    pub fn convert_paragraph(&mut self, text: &Box<BlogBody>, id: &String) -> Result<String> {
        let id = sanitize_id(id);
        let paragraph = self.parent.convert_element(no_id!(text))?;
        let link = if T::LINK_ELEMENTS { self.parent.generate_link(&id, ObjectKind::Paragraph) } else { String::new() };

        let mut result = self.tab();
        result += format!("<section id='{id}'>\n").as_str();
        self.push_tab();
        if T::LINK_ELEMENTS {
            result += self.tab().as_str();
            result += format!("{link}\n").as_str();
        }
            
            result += self.tab().as_str();
            result += "<p>\n";
            self.push_tab();
            
            for line in paragraph.trim_end().lines() {
                result += self.tab().as_str();
                result += line;
                result.push('\n');
            }

            self.pop_tab();
            result += self.tab().as_str();
            result += "</p>\n";
        self.pop_tab();
        result += self.tab().as_str();
        result += "</section>\n";

        Ok(result)
    }

    pub fn convert_text(&mut self, text: &String) -> Result<String> {
        Ok(sanitize_text(text))
    }

    pub fn convert_span(&mut self, span: &Span) -> Result<String> {
        let mut result = String::new();
        for elem in &span.elements {
            result += if let BlogBody::Text(text) = elem {
                text.to_string()
            } else {
                self.parent.as_mut().convert_element(no_id!(elem))?
            }.as_str()
        }
        Ok(result)
    }
    
    pub fn convert_citation(&mut self, id: &String) -> Result<String> {
        let laundered_parent = Weak::launder(&self.parent);
        if let Some(reference) = self.parent.get_mut_reference(id) {
            let num = reference.times_used;
            reference.times_used += 1;
            if laundered_parent.get_header().hide_references {
                Ok("".into())
            } else {
                let text = to_citation(&reference.def);
                let mut result = String::new();
                result += "<cite>";
                if T::SHOW_BACKREFS {
                    result += format!("<a id='{id}-{num}' href='#{id}' onclick='backref(\"{id}\", \"{id}-{num}\")'>").as_str();
                } else {
                    result += format!("<a id='{id}-{num}' href='#{id}'>").as_str();
                }
                result += text.as_str();
                result += "</a>";
                result += "</cite>";
                Ok(result)
            }
        } else {
            #[cfg(not(feature = "wasm"))]
            cprintln!("<y>warning:</> {} has no source", id);
            Ok(format!("(MISSING CITATION)").to_string())
        }
    }
    
    pub fn convert_factbox_note(&mut self, factbox: &FactBox, _: Option<&String>, id: &String) -> Result<String> {
        // let factbox_id = factbox_id.unwrap();
        if self.parent.get_header().hide_notes {
            Ok("".into())
        } else {
            if let Some((_, actual_id)) = factbox.notes.get(id) {
                Ok(format!("<sup><a id='{actual_id}-backref' href='#^{actual_id}'>{id}</a></sup>"))
            } else {
                Ok(format!("<sup><a id='{id}-backref' href='#^{id}'>{id}</a></sup>"))
            }
        }
    }

    pub fn convert_note(&mut self, id: &String) -> Result<String> {
        if self.parent.as_mut().get_header().hide_notes {
            Ok("".into())
        } else {
            Ok(format!("<sup><a id='{id}-backref' href='#^{id}'>{id}</a></sup>"))
        }
    }

    pub fn convert_toc(&mut self) -> Result<String> {
        if self.parent.get_header().toc.is_none() { 
            return Err(anyhow!("expected a table of content but none was found")); 
        }
        let link = if T::LINK_ELEMENTS {
            self.parent.generate_link(&String::from("table-of-contents"), ObjectKind::Header(1))
        } else { String::new() };
        let toc = self.parent.get_header().toc.clone().unwrap();
        let title = &toc.title;

        let mut result = self.tab();
        
        result += "<section id='table-of-contents'>\n";
        self.push_tab();
        if T::LINK_ELEMENTS {
            result += self.tab().as_str();
            result += link.as_str();
            result.push('\n');
        }
            
            result += self.tab().as_str();
            result += format!("<h1>{title}</h1>\n").as_str();
        self.pop_tab();
        result += self.tab().as_str();
        result += "</section>\n";

        result += self.tab().as_str();
        result += "<section>\n";
        self.push_tab();
            result += self.tab().as_str();
            result += "<ul>\n";
            self.push_tab();

                for (elem, depth, id) in &toc.headers {
                    let text = if let &box BlogBody::FactBox(fbox) = &elem {
                        fbox.title.clone()
                    } else {
                        self.parent.convert_element(no_id!(elem))?
                    };

                    result += self.tab().as_str();
                    result += format!("<li class='toci-{depth}'><a href='#{id}'>{text}</a></li>\n").as_str();
                }
            self.pop_tab();
            result += self.tab().as_str();
            result += "</ul>\n";

        self.pop_tab();
        result += self.tab().as_str();
        result += "</section>\n";

        Ok(result)
    }

    pub fn convert_page_break(&mut self) -> Result<String> {
        let mut result = self.tab();

        result += format!("<section class='page-break'>\n").as_str();
        self.push_tab();
            result += self.tab().as_str();
            result += "<hr>\n";
        self.pop_tab();
        result += self.tab().as_str();
        result += format!("</section>\n").as_str();

        Ok(result)
    }
}

pub fn to_html_bibliography(value: &ReferenceDefinition) -> String {
    //TODO(Paw): sanitize this properly 
    let mut result = "".to_string();

    for (n, author) in value.authors.iter().enumerate() {
        let name = &bibliograph_name(&author);
        result += name.trim();
        if n != value.authors.len() - 1 { result += ", " }
    }


    result += " (";
    let date = &value.date;
    if let Some(year) = date.get_year() {
        result += year.to_string().as_str();
    }
    
    if let Some(month) = date.get_month() {
        result.push(',');
        result.push(' ');
        result += month.to_string();
        result.push(' ');
    }

    if let Some(day) = date.get_day() {
        result += day.to_string().trim();
    }
    result.push(')');
    result.push('.');
    result.push(' ');

    result.push('"');
    result += value.title.trim();
    result.push('.');
    result.push('"');
    if !value.description.is_empty() {
        result.push(' ');
        result.push('[');
        result += value.description.trim();
        result.push(']');
    }

    if value.editors.len() != 0 {
        result += " edited by ";
        if value.editors.len() == 1 {
            let name = &value.editors[0];
            result += name.trim();
            result.push('.');
        } else {
            for (n, author) in value.editors.iter().enumerate() {
                let name = bibliograph_name(&author);
                result += name.trim();
                if n != value.editors.len() - 1 { result += ", " }
            }
        }
    }
    
    if value.translators.len() != 0 {
        result += ", translated by ";
        if value.translators.len() == 1 {
            let name = &value.translators[0];
            result += name.trim();
            result.push('.');
        } else {
            for (n, author) in value.translators.iter().enumerate() {
                let name = bibliograph_name(&author);
                result += name.trim();
                if n != value.translators.len() - 1 { result += ", " }
            }
        }
    }

    if !value.container_title.is_empty() {
        result += " in ";
        result += value.container_title.trim();
    }

    if any_non_empty(&[&value.volume, &value.issue, &value.pages, &value.edition, &value.version]) {
        result.push(' ');
        result.push('(');

        let mut found_one = false;
        if !value.version.is_empty() {
            result += value.version.trim();
            result += " vers.";
            found_one = true;
        }
        if !value.edition.is_empty() {
            if found_one {
                result += ", "
            }
            result += value.edition.trim();
            result += " ed.";
            found_one = true;
        }
        if !value.volume.is_empty() {
            if found_one {
                result += ", "
            }
            result += value.volume.trim();
            result += " vol.";
            found_one = true;
        }
        if !value.issue.is_empty() {
            if found_one {
                result += ", "
            }
            result += "issue ";
            result += value.issue.trim();
            found_one = true;
        }
        if !value.pages.is_empty() {
            if found_one {
                result += ", "
            }
            result += "pp. ";
            result += value.pages.trim();
        }
    
        result.push(')');
    }
    result.push('.');

    if !value.publisher.is_empty() {
        result.push(' ');
        result += value.publisher.trim();
        result.push('.');
    }

    let mut has_link = false;
    if value.link == value.doi && !value.link.is_empty() {
        let link = value.link.trim();
        result += ", ";
        result += format!("<a href={link}>{link}</a>").as_str();
        has_link = true;
    } else {
        if !value.doi.is_empty() {
            let doi = value.doi.trim();
            result.push(' ');
            if !value.link.is_empty() {
                result += "doi: ";
            }
            result += format!("<a href={doi}>{doi}</a>").as_str();
            has_link = true;
        }

        if !value.link.is_empty() {
            let link = value.link.trim();
            result += ", ";
            result += format!("<a href={link}>{link}</a>").as_str();
            has_link = true;
        }
    }

    if !value.esbn.is_empty() {
        if has_link {
            result += ", ";
        } else {
            result.push(' ');
        }

        result += "esbn: ";
        result += value.esbn.trim();
    }

    if value.date_retrieved.is_not_none() {
        result += " accessed ";
        if let Some(year) = date.get_year() {
            result += year.to_string().as_str();
        }
        
        if let Some(month) = date.get_month() {
            result.push(',');
            result.push(' ');
            result += month.to_string();
            result.push(' ');
            result += " ";
        }
        
        if let Some(day) = date.get_day() {
            result += day.to_string().as_str();
        }
    }

    result
}

pub fn sanitize_text(text: &String) -> String {
    let mut output = text.clone();
    output = if output.contains('&') { output.replace("&", "&amp;") } else { output.clone() };
    output = if output.contains('<') { output.replace("<", "&lt;")  } else { output.clone() };
    output = if output.contains('>') { output.replace(">", "&gt;")  } else { output.clone() };
    output
}

pub fn sanitize_id(text: &String) -> String {
    let mut output = text.clone();
    output = if output.contains('&')  { output.replace("&", "&amp;")   } else { output };
    output = if output.contains('<')  { output.replace("<", "&lt;")    } else { output };
    output = if output.contains('>')  { output.replace(">", "&gt;")    } else { output };
    output = if output.contains('\'') { output.replace("'", "&#39;")   } else { output };
    output = if output.contains('"')  { output.replace("\"", "&quot;") } else { output };
    output
}

pub fn convert_custom_citation<T>(citation: Option<&mut Reference<T>>, id: &String, text: &String, hide: bool, show_backrefs: bool) -> Option<String> {
    if let Some(reference) = citation {
        let num = reference.times_used;
        reference.times_used += 1;
        if hide {
            Some("".into())
        } else {
            let mut result = String::new();
            result += "<cite>";
        if show_backrefs {
            result += format!("<a id='{id}-{num}' href='#{id}' onclick='backref(\"{id}\", \"{id}-{num}\")'>").as_str();
        } else {
            result += format!("<a id='{id}-{num}' href='#{id}'>").as_str();
        }
            result += text.as_str();
            result += "</a>";
            result += "</cite>";
            Some(result)
        }
    } else {
        None
    }
}

