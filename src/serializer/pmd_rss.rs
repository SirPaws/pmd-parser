use color_print::cprintln;
use pmd_html_shared::sanitize_text;
use crate::*;

#[derive(Clone)]
struct Reference {
    def: ReferenceDefinition,
    times_used: usize,
}

impl Reference {
    fn new(def: ReferenceDefinition) -> Self {
        Self { def, times_used: 0 }
    }
} 

pub struct PMDRSSSerializer {
    filename: String,
    num_tabs:   usize,
    references: HashMap<String, Reference>,
    output: String,
}

impl PMDRSSSerializer {
    pub fn new(filename: &str) -> Self {
        Self { 
            filename: filename.into(),
            num_tabs: 0,
            references: HashMap::new(),
            output: String::new()
        }
    }

    fn push_line<S: AsRef<str>>(&mut self, text: S) {
        let result = self.tab();
        self.output += result.clone().as_str();
        self.output += text.as_ref();
        self.output.push('\n');
    }
    
    fn push_tab(&mut self) { self.num_tabs += 1; }
    fn pop_tab(&mut self) { if self.num_tabs > 0 { self.num_tabs -= 1; } }
    fn tab(&mut self) -> String {
        if self.num_tabs == 0 { "".to_string() }
        else {
            let mut result = String::new();
            for _ in 0..(self.num_tabs) {
                result += "    ";
            }
            result
        }
    } 
}

impl PMDSerializer for PMDRSSSerializer {
    fn current_factbox(&mut self) -> Option<(FactBox, Option<String>)> {
        None
    }

    fn convert_factbox(&mut self, _: &FactBox, _: &String) -> Result<String> {
        cprintln!("<r>error:</> fact boxes aren't implemented for html output");
        Ok("".into())
    }

    fn convert_hoverable(&mut self, hoverable: &Alternative) -> Result<String> {
        let base = self.convert_element(no_id!(&hoverable.base))?;
        let alt  = self.convert_element(no_id!(&hoverable.alt))?;
        Ok(format!("<span><span>{base}</span><span>({alt})</span></span>"))
    }

    fn convert_styled(&mut self, styled: &Alternative) -> Result<String> {
        let text  = self.convert_element(no_id!(&styled.alt))?;
        // let style = self.convert_element(no_id!(&styled.base))?;
        Ok(format!("<span>{text}</span>"))
    }

    // fn convert_embedded_link(&mut self, src: &String, alt: &String) -> Result<String> {
    //     self.convert_link(&Alternative{base: Box::new(BlogBody::Text(src.to_string())), alt: Box::new(BlogBody::Text(alt.to_string())) })
    // }

    fn convert_link(&mut self, link: &Alternative) -> Result<String> {
        let href = self.convert_element(no_id!(&link.alt))?;
        let text = self.convert_element(no_id!(&link.base))?;
        Ok(format!("<a class='inline-link' href='{href}'>{text}</a>"))
    }

    fn convert_header(&mut self, text: &Box<BlogBody>, depth: usize, _: &String) -> Result<String> {
        let text  = self.convert_element(no_id!(text))?;
        Ok(format!("<h{depth}>{text}</h{depth}>"))
    }

    fn convert_italics(&mut self, text: &Box<BlogBody>) -> Result<String> {
        let inner_text = self.convert_element(no_id!(&text))?;
        Ok(format!("<i>{inner_text}</i>)"))
    }

    fn convert_bold(&mut self, text: &Box<BlogBody>) -> Result<String> {
        let inner_text = self.convert_element(no_id!(&text))?;
        Ok(format!("<b>{inner_text}</b>"))
    }

    fn convert_inlinecode(&mut self, text: &String) -> Result<String> {
        let text = sanitize_text(text);
        Ok(format!("<code>{text}</code>"))
    }

    fn convert_codeblock(&mut self, text: &String, _: &String) -> Result<String> {
        let first_line = text.lines().nth(0).context("expected at least one line in codeblock")?;
        let mut words = first_line.split(|x: char| x.is_whitespace());
        let _lang = words.nth(0).unwrap_or("plaintext");
        let mut body = text[text.find('\n').context("expected at least one line in codeblock")? + 1..].to_string();

        body = sanitize_text(&body);
        body = body.trim_end().replace("\r\n", "\n");

        Ok(format!("<pre><code>{body}</code></pre>"))
    }

    fn convert_image(&mut self, src: &String, alt: &String, _: &String) -> Result<String> {
        Ok(format!("<img src='{src}' alt='{alt}'></img>"))
    }

    fn convert_quote(&mut self, lines: &Vec<BlogBody>, _: &String) -> Result<String> {
        let mut quote_elements : Vec<String> = vec![];
        for elem in lines {
            let text = self.convert_element(no_id!(elem))?;
            quote_elements.push(format!("{text}<br/>"));
        }
        let text = quote_elements.join("\n");
        Ok(format!("<blockquote class='quote-text'>{text}</blockquote>"))
    }

    fn convert_list(&mut self, list: &Vec<BlogBody>, _: &String) -> Result<String> {
        let mut list_elements: Vec<String> = vec![];
        for elem in list {
            let text = self.convert_element(no_id!(elem))?;
            list_elements.push(format!("<li>{text}</li>"));
        }
        let text = list_elements.join("\n");
        Ok(format!("<ul>{text}</ul>"))
    }

    fn convert_paragraph(&mut self, text: &Box<BlogBody>, _: &String) -> Result<String> {
        let paragraph = self.convert_element(no_id!(text))?;
        Ok(format!("<p>{paragraph}</p>"))
    }

    fn convert_text(&mut self, text: &String) -> Result<String> {
        Ok(sanitize_text(&text).trim_end().to_string())
    }

    fn convert_span(&mut self, span: &Span) -> Result<String> {
        let mut result = String::new();
        for elem in &span.elements {
            result += if let BlogBody::Text(text) = elem {
                sanitize_text(text)
            } else {
                self.convert_element(no_id!(elem))?
            }.as_str()
        }
        Ok(result)
    }
    
    fn convert_citation(&mut self, id: &String) -> Result<String> {
        if let Some(citation) = &self.references.get(id) {
            Ok(to_citation(&citation.def).trim_end().to_string())
        } else {
            Ok("(MISSING CITATION)".into())
        }
    }

    fn convert_note(&mut self, id: &String) -> Result<String> {
        Ok(format!("<sup><a id='{id}-backref' href='#^{id}'>{id}</sup></a>"))
    }
    
    fn convert_factbox_note(&mut self, _: &FactBox, _: Option<&String>, _: &String) -> Result<String> {
        cprintln!("<r>error:</> fact boxes aren't implemented for rss output");
        Ok("".into())
    }

    fn convert_toc(&mut self) -> Result<String> {
        todo!()
    }
    
    fn convert_page_break(&mut self) -> Result<String> {
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

    fn convert(&mut self, md: &PawsMarkdown) -> Result<String> {
        let title = &md.header.title;

        for (key, val) in &md.references {
            self.references.insert(key.clone(), Reference::new(val.clone()));
        }

        let filename = self.filename.clone();
        let date = if md.header.last_update.is_not_none() {
                md.header.last_update.to_date().unwrap_or(chrono::Utc::now())
            } else {
                md.header.date_written.to_date().unwrap_or(chrono::Utc::now())
            }.to_rfc3339();
        let url = &md.header.url;
        // let data_dir = &md.header.data_dir;
        let blog_dir = &md.header.blog_dir;

        self.push_line("<entry>\n");
        self.push_tab();
        self.push_line(format!("<title>{title}</title>"));
        self.push_line(format!("<link href=\"{url}/{blog_dir}/{filename}\"/>"));
        self.push_line(format!("<updated>{date}</updated>"));
        self.push_line(format!("<id>{url}/{blog_dir}/{filename}</id>"));
        self.push_line("<content type=\"xhtml\">");

        self.push_tab();
        for (element, id) in &md.body {
            let result = self.convert_element((element, id))?;
            self.push_line(result.trim_end());
        }

        if md.notes.len() != 0 {
            self.push_line("<hr>");
            for (key, val) in &md.notes {
                let result = self.convert_element(no_id!(val))?;
                self.push_line("<p>");
                self.push_tab();
                        self.push_line(format!("<sup>^{key}:</sup>{result}"));
                self.pop_tab();
                self.push_line("</p>");
            }
        }


        if self.references.len() != 0 {
            self.push_line("<hr>");
            for (_, val) in self.references.clone() {
                if val.times_used == 0 { continue; }
                self.push_line("<p>");
                self.push_tab();
                    self.push_line(to_bibliography(&val.def));
                self.pop_tab();
                self.push_line("</p>");
            }
        }

        self.pop_tab();
        self.push_line("</content>");
        self.pop_tab();
        self.push_line("</entry>");
        
        Ok(self.output.clone())
    }    
}
