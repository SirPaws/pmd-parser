use ordered_map::OrderedMap;
use pmd_html_shared::{ObjectKind, PMDSharedHTMLSerializer, Reference, PMDHTML};
use crate::*;

pub struct PMDHTMLSerializer { 
    pub common: PMDHTML<PMDHTMLSerializer>,
    header: BlogHeader,
    references: OrderedMap<String, Reference<ReferenceDefinition>>,
    notes: OrderedMap<String, Reference<BlogBody>>,
    notes_id: String,
    bibliography_id: String,
/*
    quote_id:   usize,
    list_id :   usize,
    code_id:    usize,
    factbox_id: usize,
    image_id:   usize,
    missing_id: usize,
*/
}

impl PMDSharedHTMLSerializer for PMDHTMLSerializer {
    fn get_header(&self) -> &BlogHeader {
        &self.header
    }

    fn notes_id(&mut self) -> String {
        self.notes_id.clone()
    }

    fn bibliography_id(&mut self) -> String {
        self.bibliography_id.clone()
    }

    fn get_description(&mut self, md: &PawsMarkdown) -> Result<String> {
        let paragraph = md.body.iter().find(|(x, _)| match x { BlogBody::Paragraph(_) => true, _ => false});
        Ok(if let Some((BlogBody::Paragraph(content), id)) = paragraph {
            let text = PMDPureTextSerializer::new().convert_paragraph(content, id)?;
            text.trim_end().to_string()
        } else {
            String::new()
        })
    }

    fn prepare_html_header(&mut self, description: &String) -> String {
        let mut output = String::new();
        let title = &self.header.title;
        let banner = &self.header.banner;
        let url = &self.header.url;
        let data_dir = &self.header.data_dir;
        let blog_dir = &self.header.blog_dir;

        output += "<meta http-equiv=\"content-type\" content=\"text/html; charset=utf-8\">\n";
        output += "\n";
        output += "<!-- highlight.js -->\n";
        output += "<script src=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js\"></script>\n";
        output += "<link rel=\"stylesheet\" href=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/default.min.css\">\n";
        output += "<script>hljs.highlightAll();</script>\n";
        output += "\n";
        output += "<!-- tag needed for media query -->\n";
        output += "<meta name=\"viewport\"    content=\"width=device-width, initial-scale=1, minimum-scale=1\" />\n";
        output += "\n";
        output += "<!-- Primary Meta Tags -->\n";
        output += format!("<title>{}</title>\n", title).as_str();
        output += format!("<meta name=\"title\" content=\"{}\">\n", title).as_str();
        output += format!("<meta name=\"description\" content=\"{}\">\n", description).as_str();
        output += "\n";
        output += "<!-- Open Graph / Facebook -->\n";
        output += "<meta property=\"og:type\" content=\"website\">\n";
        output += format!("<meta property=\"og:url\" content=\"{url}/{blog_dir}/{}.html\">\n", self.common.filename).as_str();
        output += format!("<meta property=\"og:title\" content=\"{}\">\n", title).as_str();
        output += format!("<meta property=\"og:description\" content=\"{}\">\n", description).as_str();
        if banner.len() > 0 {
            output += format!("<meta property=\"og:image\" content=\"{url}/{blog_dir}/{}\">\n", banner).as_str();
        } else {
            output += format!("<meta property=\"og:image\" content=\"{url}/{data_dir}/minibanner.png\">\n").as_str();
        }
        output += "\n";
        output += "<!-- Twitter -->\n";
        if banner.len() > 0 {
            output += "<meta property=\"twitter:card\" content=\"summary_large_image\">\n";
        } else {
            output += "<meta property=\"twitter:card\" content=\"summary\">\n";
        }
        output += format!("<meta property=\"twitter:url\" content=\"{url}/{blog_dir}/{}.html\">\n", self.common.filename).as_str();
        output += format!("<meta property=\"twitter:title\" content=\"{}\">\n", title).as_str();
        output += format!("<meta property=\"twitter:description\" content=\"{}\">\n", description).as_str();
        if banner.len() > 0 {
            output += format!("<meta property=\"twitter:image\" content=\"{url}/{blog_dir}/{}\">\n", banner).as_str();
        } else {
            output += format!("<meta property=\"twitter:image\" content=\"{url}/{blog_dir}/minibanner.png\">\n").as_str();
        }
        output += "\n";
        output += "<!-- stylesheets -->\n";
        output += "<link rel=\"stylesheet\" href=\"../css/base.css\">\n";
        output += "<link rel=\"stylesheet\" href=\"../css/blog.css\">\n";
        output += "\n";
        output += "<!-- for Atkinson Hyperlegible font-->\n";
        output += "<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n";
        output += "<link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n";
        output += "<link href=\"https://fonts.googleapis.com/css2?family=Atkinson+Hyperlegible:ital,wght@0,400;0,700;1,400;1,700&display=swap\" rel=\"stylesheet\">\n";
        output += "\n";
        output += "<script type='text/javascript' src='../js/popup.js'></script>\n";
        output += "<script type='text/javascript' src='../js/popup.js'></script>\n";
        output += "<script type='text/javascript' src='../js/dropdown.js'></script>\n";
        output += "<script type='text/javascript' src='../js/backref.js'></script>\n";
        
        output
    }

    fn convert_body(&mut self, md: &PawsMarkdown) -> Result<String> {
        let mut output = String::new();
        for (element, id) in &md.body {
            let result = self.convert_element((element, id))?;
            match element {
                BlogBody::CodeBlock(_) => {
                    output += result.as_str();
                    output += "\n";
                },
                _ => {
                    output += result.as_str();
                }
            }
        }
        Ok(output)
    }

    fn generate_link(&mut self, id: &String, kind: ObjectKind) -> String {
        match kind {
            ObjectKind::Header(depth) => self.element_link(&id, Some(format!("<h{depth}>§</h{depth}>").as_str()), Some("header")),
            ObjectKind::CodeBlock | ObjectKind::Quote | ObjectKind::Image | ObjectKind::FactBox |
            ObjectKind::List  | ObjectKind::Paragraph => self.element_link(id, None, None),
        }
    }
    
    
    fn references(&mut self) -> &OrderedMap<String, Reference<ReferenceDefinition>> {
        &self.references
    }
    
    fn mut_references(&mut self) -> &mut OrderedMap<String, Reference<ReferenceDefinition>> {
        &mut self.references
    }

    fn get_reference<T: AsRef<str>>(&mut self, key: T) -> Option<&Reference<ReferenceDefinition>> {
        self.references.get(key.as_ref())
    }
    
    fn get_mut_reference<T: AsRef<str>>(&mut self, key: T) -> Option<&mut Reference<ReferenceDefinition>> {
        self.references.get_mut(key.as_ref())
    }
}

impl PMDHTMLSerializer {
    pub fn new(filename: &str) -> Box<Self> {
        let mut value = Box::new(Self {
            common: PMDHTML::uninit(),
            header: BlogHeader::default(),
            references: OrderedMap::new(),
            notes: OrderedMap::new(),
            notes_id: String::new(),
            bibliography_id: String::new(),
        });
        value.common = PMDHTML::new(filename, &mut *value);
        value
    }

    fn element_link(&mut self, id: &String, opt_text: Option<&str>, opt_class: Option<&str>) -> String {
        let text  = opt_text.unwrap_or("¶");
        let class = opt_class.unwrap_or("paragraph");

        format!("<a class='{class}' href='#{id}' aria-hidden='true'>{text}</a>")
    }

    fn prepare_navbar(&mut self) -> Result<String> {
        let mut output = String::new();
        
        output += self.common.tab().as_str();
        output += "<nav class=\"nav-bar\">\n";
        self.common.push_tab();
            output += self.common.tab().as_str();
            output += "<a class=\"paw-holder\" href=\"..\">\n";
            self.common.push_tab();
            output += self.common.tab().as_str();
            output += "<div class=\"paw-beans\">\n";
            self.common.push_tab();
                output += self.common.tab().as_str();
                output += "<div class=\"bean bean-nth-0\"></div>\n";
                output += self.common.tab().as_str();
                output += "<div class=\"bean bean-nth-1\"></div>\n";
                output += self.common.tab().as_str();
                output += "<div class=\"bean bean-nth-2\"></div>\n";
                output += self.common.tab().as_str();
                output += "<div class=\"bean bean-nth-3\"></div>\n";
            self.common.pop_tab();
            output += self.common.tab().as_str();
            output += "</div>\n";

            output += self.common.tab().as_str();
            output += "<div class=\"paw-pad\"></div>\n";

            self.common.pop_tab();
            output += self.common.tab().as_str();
            output += "</a>\n";
            
            output += self.common.tab().as_str();
            output += "<section>\n";
            self.common.push_tab();
                output += self.common.tab().as_str();
                output += "<button class='nav-phone-dropdown-button' onclick='toggle_dropdown()'>☰ </button>\n";
                output += self.common.tab().as_str();
                output += "<ul class='nav-list'>\n";
                self.common.push_tab();
                    output += self.common.tab().as_str();
                    output += "<li><a href='../about.html'>⭐ About</a></li>\n";
                    output += self.common.tab().as_str();
                    output += "<li><a href='../art.html'>🎨 Art</a></li>\n";
                    output += self.common.tab().as_str();
                    output += "<li><a href='../code.html'>🦄 Code</a></li>\n";
                    output += self.common.tab().as_str();
                    output += format!("<li><a href='md/{}.md'>📋 Raw</a></li>\n", self.common.filename).as_str();
                self.common.pop_tab();
                output += self.common.tab().as_str();
                output += "</ul>\n";
            self.common.pop_tab();
            output += self.common.tab().as_str();
            output += "</section>\n";

        self.common.pop_tab();
        output += self.common.tab().as_str();
        output += "</nav>\n";


        output += self.common.tab().as_str();
        output += "<ul id='phone-dropdown' class='nav-phone-dropdown off'>\n";
        self.common.push_tab();
            output += self.common.tab().as_str();
            output += "<li><a href='../about.html'>⭐ About</a></li>\n";
            output += self.common.tab().as_str();
            output += "<li><a href='../art.html'>🎨 Art</a></li>\n";
            output += self.common.tab().as_str();
            output += "<li><a href='../code.html'>🦄 Code</a></li>\n";
            output += self.common.tab().as_str();
            output += format!("<li><a href='md/{}.md'>📋 Raw</a></li>\n", self.common.filename).as_str();
        self.common.pop_tab();
        output += self.common.tab().as_str();
        output += "</ul>\n";
        
        Ok(output)
    } 
}

impl PMDSerializer for PMDHTMLSerializer {
    fn current_factbox(&mut self) -> Option<(FactBox, Option<String>)> {
        self.common.current_factbox()
    }

    fn convert(&mut self, md: &PawsMarkdown) -> Result<String> {
        let navbar = self.prepare_navbar()?;
        self.header = md.header.clone();
        self.notes_id = md.notes_id.clone();
        self.bibliography_id = md.bibliography_id.clone();
        
        for (id, reference) in &md.references {
            self.references.insert(id.clone(), Reference::new(reference.clone()));
        }
        
        for (id, note) in &md.notes {
            self.notes.insert(id.clone(), Reference::new(note.clone()));
        }

        self.common.html(md, Some(&navbar), &md.references, &md.notes)
    }
    
    fn convert_factbox(&mut self, factbox: &FactBox, id: &String) -> Result<String> {
        self.common.convert_factbox(factbox, id)
    }

    /*
    fn convert_factbox(&mut self, factbox: &FactBox) -> Result<String> {
        let id = self.generate_missing_factbox_id();
        let link = self.element_link(&id, None, None);
        let title = &factbox.title;

        let mut output = self.common.tab();
        output += format!("<section id='{id}'>\n").as_str();
        self.common.push_tab();
            output += self.common.tab().as_str();
            output += format!("{link}\n").as_str();
            
            output += self.common.tab().as_str();
            output += "<article class='factbox'>\n";
            self.common.push_tab();

                output += self.common.tab().as_str();
                output += "<header class='factbox-header'>\n";
                self.common.push_tab();

                    output += self.common.tab().as_str();
                    output += format!("<h2>{title}</h2>\n").as_str();

                self.common.pop_tab();
                output += self.common.tab().as_str();
                output += "</header>\n";
                
                output += self.common.tab().as_str();
                output += "<section class='factbox-content'>\n";
                self.common.push_tab();
                for element in self.convert_factbox_elements(factbox, Some(&id))? {
                    output += element.as_str();
                }
                    

                if !(factbox.notes.is_empty() || self.header.hide_notes) {
                    let id = PMDHTML::generate_id(&self.header.notes_title, ||"missing".into());
                    let title = self.header.notes_title.clone();
                    let link  = self.element_link(&id, Some(format!("<h1>§</h1>").as_str()), Some("header"));
                
                    
                    output += self.common.tab().as_str();
                    output += format!("<section class='page-break'>\n").as_str();
                    self.common.push_tab();
                        output += self.common.tab().as_str();
                        output += "<hr>\n";
                    self.common.pop_tab();
                    output += self.common.tab().as_str();
                    output += format!("</section>\n").as_str();
                        
                    output += self.common.tab().as_str();
                    output += format!("<section class='notes' id='{id}'>\n").as_str();
                    self.common.push_tab();
                        output += self.common.tab().as_str();
                        output += link.as_str();
                        output.push('\n');
                        
                        output += self.common.tab().as_str();
                        output += format!("<h1>{title}</h1>\n").as_str();
                    self.common.pop_tab();
                    output += self.common.tab().as_str();
                    output += format!("</section>\n").as_str();
                        
                    for (key, val) in &factbox.notes {
                        let result = self.convert_element(val)?;
                        let link = format!("<a href='#^{id}-{key}'>^{key}:</a>");
                    
                        output += self.common.tab().as_str();
                        output += format!("<section class='note' id=\"^{key}\">").as_str();
                        output.push('\n');
                        self.common.push_tab();
                    
                            output += self.common.tab().as_str();
                            output += "<sup>";
                            self.common.push_tab();
                                output += link.as_str();
                                output.push('\n');
                                
                                output += self.common.tab().as_str();
                                output += result.as_str();
                                output.push('\n');
                                
                                output += self.common.tab().as_str();
                                output += format!("<a href=\"#{key}-backref\">").as_str();
                                output += "↩";
                                output += "</a>";
                                output.push('\n');
                            self.common.pop_tab();
                            output += self.common.tab().as_str();
                            output += "</sup>";
                            output.push('\n');
                    
                        self.common.pop_tab();
                        output += self.common.tab().as_str();
                        output += "</section>\n";
                    }
                }

                self.common.pop_tab();
                output += self.common.tab().as_str();
                output += "</section>\n";
            
            self.common.pop_tab();
            output += self.common.tab().as_str();
            output += "</article>\n";
            
        self.common.pop_tab();
        output += self.common.tab().as_str();
        output += "</section>\n";

        Ok(output)
    }
    */

    fn convert_hoverable(&mut self, hoverable: &Alternative) -> Result<String> {
        // let base = self.convert_element(&hoverable.base)?;
        // let alt  = self.convert_element(&hoverable.alt)?;
        // Ok(format!("<span class='hoverable'><span class='hover-base'>{base}</span><span class='hover-alt'>{alt}</span></span>"))
        self.common.convert_hoverable(hoverable)
    }

    fn convert_styled(&mut self, styled: &Alternative) -> Result<String> {
        // let text  = self.convert_element(&styled.alt)?;
        // let style = self.convert_element(&styled.base)?;
        // Ok(format!("<span class='embedded-style' style='{style}'>{text}</span>"))
        self.common.convert_styled(styled)
    }
    
    // fn convert_embedded_link(&mut self, src: &String, alt: &String) -> Result<String> {
    // }

    fn convert_link(&mut self, link: &Alternative) -> Result<String> {
        self.common.convert_link(link)
        /*
        let href = self.convert_element(&link.alt)?;
        let text = self.convert_element(&link.base)?;
        match &link.alt {
            box BlogBody::Citation(citation) => {
                if let Some(reference) = convert_custom_citation(self.references.get_mut(citation.as_str()) , &citation, &text, self.header.hide_references) {
                    Ok(reference)
                } else {
                    cprintln!("<y>warning:</> {} has no source", citation);
                    Ok(format!("<cite style='color=red; background-color: yellow'>{text}</cite>"))
                }
            }, 
            box BlogBody::Note(note) => {
                if self.header.hide_notes {
                    Ok("".into())
                } else {
                    Ok(format!("<a class='inline-link' href='#^{note}'>{text}</a>"))
                }
            },
            _ => {
                Ok(format!("<a class='inline-link' href='{href}'>{text}</a>"))
            }
        }
        */
    }

    fn convert_header(&mut self, text: &Box<BlogBody>, depth: usize, id: &String) -> Result<String> {
        self.common.convert_header(text, depth, id)
        /*
        let text  = self.convert_element(text)?;
        let id    = Self::generate_id(&text, ||"missing".into());
        let link  = self.element_link(&id, Some(format!("<h{depth}>§</h{depth}>").as_str()), Some("header"));
        
        let mut result = self.common.tab();
        result += format!("<section id='{id}'>\n").as_str();
        self.common.push_tab();
            result += self.common.tab().as_str();
            result += link.as_str();
            result.push('\n');
            
            result += self.common.tab().as_str();
            result += format!("<h{depth}>{text}</h{depth}>\n").as_str();
        self.common.pop_tab();
        result += self.common.tab().as_str();
        result += "</section>\n";

        Ok(result)
            */
    }

    fn convert_italics(&mut self, text: &Box<BlogBody>) -> Result<String> {
        self.common.convert_italics(text)
        /*
        let inner_text = self.convert_element(&text)?;
        Ok(format!("<i>{inner_text}</i>"))
        */
    }

    fn convert_bold(&mut self, text: &Box<BlogBody>) -> Result<String> {
        self.common.convert_bold(text)
        /*
        let inner_text = self.convert_element(&text)?;
        Ok(format!("<b>{inner_text}</b>"))
        */
    }

    fn convert_inlinecode(&mut self, text: &String) -> Result<String> {
        self.common.convert_inlinecode(text)
        // Ok(format!("<code>{text}</code>"))
    }

    fn convert_codeblock(&mut self, text: &String, id: &String) -> Result<String> {
        self.common.convert_codeblock(text, id)
        /*
        let first_line = text.lines().nth(0).context("expected at least one line in codeblock")?;
        let mut words = first_line.split(|x: char| x.is_whitespace());
        let lang = words.nth(0).unwrap_or("plaintext");
        let mut body = text[text.find('\n').context("expected at least one line in codeblock")? + 1..].to_string();
        let id = Self::generate_id(&"".into(), ||self.generate_missing_code_id());
        let link = self.element_link(&id, None, None);

        body = body.replace("&", "&amp;")
                   .replace("<", "&lt;")
                   .replace(">", "&gt;");
        body = body.trim_end().replace("\r\n", "\n");
        
        let mut result = self.common.tab();
        result += format!("<section class='code-block' id='{id}'>\n").as_str();
        self.common.push_tab();
            result += self.common.tab().as_str();
            result += format!("{link}\n").as_str();
            
            result += self.common.tab().as_str();
            result += format!("<pre><code class='language-{lang}'>{body}</code></pre>\n").as_str();
        self.common.pop_tab();
        result += self.common.tab().as_str();
        result += "</section>\n";

        Ok(result)
        */
    }

    fn convert_image(&mut self, src: &String, alt: &String, id: &String) -> Result<String> {
        self.common.convert_image(src, alt, id)
        /*
        let id = Self::generate_id(alt, || self.generate_missing_image_id());
        let link = self.element_link(&id, None, None);

        let mut result = self.common.tab();
        result += format!("<section class='image' id='{id}'>\n").as_str();
        self.common.push_tab();
            result += self.common.tab().as_str();
            result += format!("{link}\n").as_str();
            
            result += self.common.tab().as_str();
            result += format!("<img onclick='makePopup(this)' src='{src}' alt='{alt}'></img>\n").as_str();
        self.common.pop_tab();
        result += self.common.tab().as_str();
        result += "</section>\n";

        Ok(result)
        */
    }

    fn convert_quote(&mut self, lines: &Vec<BlogBody>, id: &String) -> Result<String> {
        self.common.convert_quote(lines, id)
        /*
        let id = Self::generate_id(&"".to_string(), || self.generate_missing_quote_id());
        let link = self.element_link(&id, None, None);
        
        let mut quote_elements : Vec<String> = vec![];
        for elem in lines {
            let text = self.convert_element(elem)?;
            quote_elements.push(format!("{text}<br/>"));
        }
        let text = quote_elements.join("\n");

        let mut result = self.common.tab();
        result += format!("<section class='quote' id='{id}'>\n").as_str();
        self.common.push_tab();
            result += self.common.tab().as_str();
            result += format!("{link}\n").as_str();
            
            result += self.common.tab().as_str();
            result += format!("<div class='quote-line'></div><blockquote class='quote-text'>{text}</blockquote>\n").as_str();
        self.common.pop_tab();
        result += self.common.tab().as_str();
        result += "</section>\n";

        Ok(result)
        */
    }

    fn convert_list(&mut self, list: &Vec<BlogBody>, id: &String) -> Result<String> {
        self.common.convert_list(list, id)
        /*
        let id = Self::generate_id(&"".to_string(), ||self.generate_missing_list_id());
        let link = self.element_link(&id, None, None);

        let mut result = self.common.tab();
        result += format!("<section class='list' id='{id}'>\n").as_str();
        self.common.push_tab();
            result += self.common.tab().as_str();
            result += format!("{link}\n").as_str();
            
            result += self.common.tab().as_str();
            result += "<ul>";
            self.common.push_tab();
            for elem in list {
                let text = self.convert_element(elem)?;
                result += self.common.tab().as_str();
                result += format!("<li>{text}</li>\n").as_str();
            }
            self.common.pop_tab();
            result += self.common.tab().as_str();
            result += "</ul>";
        self.common.pop_tab();
        result += self.common.tab().as_str();
        result += "</section>\n";

        Ok(result)
        */
    }

    fn convert_paragraph(&mut self, text: &Box<BlogBody>, id: &String) -> Result<String> {
        self.common.convert_paragraph(text, id)
        /*
        let paragraph = self.convert_element(text)?;
        let id = Self::generate_id(&paragraph, || self.generate_missing_id());
        let link = self.element_link(&id, None, None);

        let mut result = self.common.tab();
        result += format!("<section id='{id}'>\n").as_str();
        self.common.push_tab();
            result += self.common.tab().as_str();
            result += format!("{link}\n").as_str();
            
            result += self.common.tab().as_str();
            result += "<p>\n";
            self.common.push_tab();
            
            for line in paragraph.trim_end().lines() {
                result += self.common.tab().as_str();
                result += line;
                result.push('\n');
            }

            self.common.pop_tab();
            result += self.common.tab().as_str();
            result += "</p>\n";
        self.common.pop_tab();
        result += self.common.tab().as_str();
        result += "</section>\n";

        Ok(result)
        */
    }

    fn convert_text(&mut self, text: &String) -> Result<String> {
        self.common.convert_text(text)
    }

    fn convert_span(&mut self, span: &Span) -> Result<String> {
        self.common.convert_span(span)
        /*
        let mut result = String::new();
        for elem in &span.elements {
            result += if let BlogBody::Text(text) = elem {
                text.to_string()
            } else {
                self.convert_element(elem)?
            }.as_str()
        }
        Ok(result)
        */
    }
    
    fn convert_citation(&mut self, id: &String) -> Result<String> {
        self.common.convert_citation(id)
        /*
        if let Some(reference) = self.references.get_mut(id) {
            let num = reference.times_used;
            reference.times_used += 1;
            if self.header.hide_references {
                Ok("".into())
            } else {
                let text = to_citation(&reference.def);
                let mut result = String::new();
                result += "<cite>";
                result += format!("<a id='{id}-{num}' href='#{id}' onclick='backref(\"{id}\", \"{id}-{num}\")'>").as_str();
                result += text.as_str();
                result += "</a>";
                result += "</cite>";
                Ok(result)
            }
        } else {
            cprintln!("<y>warning:</> {} has no source", id);
            Ok(format!("(MISSING CITATION)").to_string())
        }
        */
    }
    

    fn convert_note(&mut self, id: &String) -> Result<String> {
        self.common.convert_note(id)
        /*
        if self.header.hide_notes {
            Ok("".into())
        } else {
            Ok(format!("<sup><a id='{id}-backref' href='#^{id}'>{id}</a></sup>"))
        }
        */
    }
    
    fn convert_factbox_note(&mut self, factbox: &FactBox, factbox_id: Option<&String>, id: &String) -> Result<String> {
        self.common.convert_factbox_note(factbox, factbox_id, id)
    }

    fn convert_toc(&mut self) -> Result<String> {
        self.common.convert_toc()
        /*
        if self.header.toc.is_none() { return Err(anyhow!("")); }
        let link  = self.element_link(&String::from("table-of-contents"), Some("<h1>§</h1>"), Some("header"));
        let toc = self.header.toc.clone().unwrap();
        let title = &toc.title;

        let mut result = self.common.tab();
        
        result += "<section id='table-of-contents'>\n";
        self.common.push_tab();
            result += self.common.tab().as_str();
            result += link.as_str();
            result.push('\n');
            
            result += self.common.tab().as_str();
            result += format!("<h1>{title}</h1>\n").as_str();
        self.common.pop_tab();
        result += self.common.tab().as_str();
        result += "</section>\n";

        result += self.common.tab().as_str();
        result += "<section>\n";
        self.common.push_tab();
            result += self.common.tab().as_str();
            result += "<ul>\n";
            self.common.push_tab();
                for (elem, depth) in &toc.headers {
                    let text = self.convert_element(elem)?;
                    let id    = Self::generate_id(&text, ||"missing".into());
                    result += self.common.tab().as_str();
                    result += format!("<li class='toci-{depth}'><a href='#{id}'>{text}</a></li>\n").as_str();
                }
            self.common.pop_tab();
            result += self.common.tab().as_str();
            result += "</ul>\n";

        self.common.pop_tab();
        result += self.common.tab().as_str();
        result += "</section>\n";

        Ok(result)
        */
    }

    fn convert_page_break(&mut self) -> Result<String> {
        self.common.convert_page_break()
        /*
        let mut result = self.common.tab();

        result += format!("<section class='page-break'>\n").as_str();
        self.common.push_tab();
            result += self.common.tab().as_str();
            result += "<hr>\n";
        self.common.pop_tab();
        result += self.common.tab().as_str();
        result += format!("</section>\n").as_str();

        Ok(result)
        */
    }
}
