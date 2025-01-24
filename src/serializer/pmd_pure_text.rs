use crate::*;
use anyhow::*;
use ordered_map::OrderedMap;

pub struct PMDPureTextSerializer {
    pub notes_title: String,
    pub hide_references: bool,
    pub hide_notes: bool,
    pub hide_contacts: bool,
    pub toc: Option<TableOfContent>,
    pub references: OrderedMap<String, ReferenceDefinition>,
}

impl PMDPureTextSerializer {
    pub fn new() -> Self { 
        Self {
            notes_title: String::new(),
            hide_references: false, 
            hide_notes: false, 
            hide_contacts: false, 
            toc: None, 
            references: OrderedMap::new()
        } 
    }
}

impl PMDSerializer for PMDPureTextSerializer {
    fn current_factbox(&mut self) -> Option<(FactBox, Option<String>)> {
        None
    }

    fn convert_factbox(&mut self, factbox: &FactBox, _: &String) -> Result<String> {
        let title = &factbox.title;
        let mut result = String::new();
        result += "--------------------------------------------------------------------------------\n";
        result += format!("| {title}   \n").as_str();
        result += "--------------------------------------------------------------------------------\n";
        
        let elements = self.convert_factbox_elements(factbox, None)?;
        for element in elements {
            for line in element.lines() {
                result += format!("| {line}\n").as_str();
            }
            result += "| \n";
        }
        result += "--------------------------------------------------------------------------------\n";

        if !factbox.notes.is_empty() {
            result += format!("| {}: \n", self.notes_title).as_str();
            for (key, (val, _)) in &factbox.notes {
                let element = self.convert_element(no_id!(val))?;
                result += format!("|     ^{key}: {element}\n").as_str();
            }
            result += "| \n";
            result += "--------------------------------------------------------------------------------\n";
        }
        result.push('\n');
        Ok(result)
    }

    fn convert_hoverable(&mut self, hoverable: &Alternative) -> Result<String> {
        let alt    = self.convert_element(no_id!(&hoverable.alt))?;
        let actual = self.convert_element(no_id!(&hoverable.base))?;
        Ok(format!("{alt}({actual})"))
    }
    

    fn convert_styled(&mut self, styled: &Alternative) -> Result<String> {
        self.convert_element((&styled.alt, &String::new()))
    }

    // fn convert_embedded_link(&mut self, src: &String, alt: &String) -> Result<String> {
    //     self.convert_link(&Alternative{base: Box::new(BlogBody::Text(src.to_string())), alt: Box::new(BlogBody::Text(alt.to_string())) })   
    // }

    fn convert_link(&mut self, link: &Alternative) -> Result<String> {
        let text = self.convert_element(no_id!(&link.alt))?;
        match &link.base {
            box BlogBody::Citation(_) | box BlogBody::Note(_) => {
                Ok(format!("{text}"))
            },
            _ => {
                let link = self.convert_element(no_id!(&link.base))?;
                Ok(format!("{link}({text})"))
            }
        }
    }

    fn convert_header(&mut self, text: &Box<BlogBody>, _: usize, _: &String) -> Result<String> {
        self.convert_element(no_id!(text))
    }

    fn convert_italics(&mut self, text: &Box<BlogBody>) -> Result<String> {
        self.convert_element(no_id!(text))
    }

    fn convert_bold(&mut self, text: &Box<BlogBody>) -> Result<String> {
        self.convert_element(no_id!(text))
    }

    fn convert_inlinecode(&mut self, text: &String) -> Result<String> {
        Ok(text.clone())
    }

    fn convert_codeblock(&mut self, text: &String, _: &String) -> Result<String> {
        let mut result :String = "-----\n".to_string();
        result += text.as_str();
        result += "-----\n";
        Ok(result)
    }

    fn convert_image(&mut self, src: &String, alt: &String, _: &String) -> Result<String> {
        Ok(format!("{alt}({src})"))
    }

    fn convert_quote(&mut self, lines: &Vec<BlogBody>, _: &String) -> Result<String> {
        let mut quote_elements : Vec<String> = vec![];
        for elem in lines {
            let text = self.convert_element(no_id!(elem))?;
            quote_elements.push(format!("{text}\n"));
        }
        let text = quote_elements.join("\n");
        Ok(format!("\"{text}\""))
    }
    
    fn convert_citation(&mut self, citation: &String) -> Result<String> {
        let maybe_source = self.references.get(citation);
        if let Some(source) = maybe_source {
            if self.hide_references {
                Ok("".into())
            } else {
                Ok(to_citation(source))
            }
        } else {
            Ok("(Missing Source)".to_string())
        }
    }
    
    fn convert_note(&mut self, id: &String) -> Result<String> {
        if !self.hide_notes {
            Ok(format!("^{id}").to_string())
        } else {
            Ok("".into())
        }
    }
    
    fn convert_factbox_note(&mut self, _: &FactBox, _: Option<&String>, id: &String) -> Result<String> {
        Ok(format!("^{id}").to_string())
    }

    fn convert_list(&mut self, list: &Vec<BlogBody>, _: &String) -> Result<String> {
        let mut list_elements: Vec<String> = vec![];
        for elem in list {
            let text = self.convert_element(no_id!(elem))?;
            list_elements.push(format!("- {text}"));
        }
        let text = list_elements.join("\n");
        Ok(text)
    }

    fn convert_paragraph(&mut self, text: &Box<BlogBody>, _: &String) -> Result<String> {
        self.convert_element(no_id!(text))
    }

    fn convert_text(&mut self, text: &String) -> Result<String> {
        Ok(text.clone())
    }

    fn convert_span(&mut self, span: &Span) -> Result<String> {
        let mut result = String::new();
        for elem in &span.elements {
            result += if let BlogBody::Text(text) = elem {
                text.to_string()
            } else {
                let mut inner = String::new();
                inner += self.convert_element(no_id!(elem))?.as_str();
                inner
            }.as_str()
        }
        Ok(result)
    }
    
    fn convert_toc(&mut self) -> Result<String> {
        if self.toc.is_none() { return Err(anyhow!("")); }
        let mut result = String::new();
        let toc = self.toc.clone().unwrap();
        let title = &toc.title;

        result += format!("{title}:\n").as_str();
        for (text, depth, _) in &toc.headers {
            for _ in 0..depth.clone() {
                result += "    ";
            }
            result += self.convert_element(no_id!(&text))?.as_str();
            result += "\n"
        }

        Ok(result)
    }


    fn convert_page_break(&mut self) -> Result<String> {
        Ok("---\n".into())
    }

    fn convert(&mut self, md: &PawsMarkdown) -> Result<String> {
        let mut output = String::new();
        self.references = md.references.clone();
        self.toc = md.header.toc.clone();
        self.hide_references = md.header.hide_references;
        self.hide_notes      = md.header.hide_notes;
        self.hide_contacts   = md.header.hide_contacts;
        self.notes_title     = md.header.notes_title.clone();

        for (element, _) in &md.body {
            let result = self.convert_element(no_id!(element))?;
            output += result.as_str();
            match element {
                BlogBody::Header(_, _) => {
                    output += ":\n";
                },
                _ => {
                    output += "\n";
                    output += "\n";
                }
            }
        }

        if !(md.notes.is_empty() || self.hide_notes) {
            output += "--------------------------------------------------------------------------------\n";
            output += format!("{}: \n", md.header.notes_title).as_str();
            for (key, val) in &md.notes {
                let result = self.convert_element(no_id!(val))?;
                output += format!("    ^{key}: {result}\n").as_str();
            }
            output.push('\n');
        }

        if !(self.references.is_empty() || self.hide_references) {
            output += "--------------------------------------------------------------------------------\n";
            output += format!("{}: \n", md.header.bibliography_title).as_str();
            for (_, val) in &self.references {
                output += to_bibliography(val).as_str();
                output += "\n";
            }
        }

        Ok(output)
    }
}
