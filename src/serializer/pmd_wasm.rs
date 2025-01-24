use crate::{ordered_map::OrderedMap, pmd_html_shared::{ObjectKind, PMDSharedHTMLSerializer, Reference, PMDHTML}, Alternative, BlogBody, BlogHeader, FactBox, Frontmatter, FrontmatterHelper, PMDPureTextSerializer, PMDSerializer, PawsMarkdown, ReferenceDefinition, Span};
use anyhow::{Result, anyhow};
use serde_yaml::Value;

#[cfg(any(feature = "html", feature = "rss", feature = "pdf"))]
compile_error!("wasm only works with default features disable");

pub enum PMDWASMSerializer {
    AsPDF(Box<PMDWASMSerializerPDF>),
    AsHTML(Box<PMDWASMSerializerHTML>)
}

impl PMDWASMSerializer {
    pub fn new(as_pdf: bool) -> Self {
        if as_pdf {
            Self::AsPDF(PMDWASMSerializerPDF::new()) 
        } else {
            Self::AsHTML(PMDWASMSerializerHTML::new())
        }
    }
}

impl PMDSerializer for PMDWASMSerializer {
    fn current_factbox(&mut self) -> Option<(FactBox, Option<String>)> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.current_factbox(),
            PMDWASMSerializer::AsHTML(x) => x.current_factbox(),
        }
    }

    fn convert_hoverable(&mut self, hoverable: &Alternative) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_hoverable(hoverable),
            PMDWASMSerializer::AsHTML(x) => x.convert_hoverable(hoverable),
        }
    }

    fn convert_styled(&mut self, styled: &Alternative) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_styled(styled),
            PMDWASMSerializer::AsHTML(x) => x.convert_styled(styled),
        }
    }

    fn convert_link(&mut self, link: &Alternative) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_link(link),
            PMDWASMSerializer::AsHTML(x) => x.convert_link(link),
        }
    }

    fn convert_header(&mut self, text: &Box<BlogBody>, depth: usize, id: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_header(text, depth, id),
            PMDWASMSerializer::AsHTML(x) => x.convert_header(text, depth, id),
        }
    }

    fn convert_italics(&mut self, text: &Box<BlogBody>) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_italics(text),
            PMDWASMSerializer::AsHTML(x) => x.convert_italics(text),
        }
    }

    fn convert_bold(&mut self, text: &Box<BlogBody>) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_bold(text),
            PMDWASMSerializer::AsHTML(x) => x.convert_bold(text),
        }
    }

    fn convert_inlinecode(&mut self, text: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_inlinecode(text),
            PMDWASMSerializer::AsHTML(x) => x.convert_inlinecode(text),
        }
    }

    fn convert_codeblock(&mut self, text: &String, id: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_codeblock(text, id),
            PMDWASMSerializer::AsHTML(x) => x.convert_codeblock(text, id),
        }
    }

    fn convert_image(&mut self, src: &String, alt: &String, id: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_image(src, alt, id),
            PMDWASMSerializer::AsHTML(x) => x.convert_image(src, alt, id),
        }
    }

    fn convert_quote(&mut self, lines: &Vec<BlogBody>, id: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_quote(lines, id),
            PMDWASMSerializer::AsHTML(x) => x.convert_quote(lines, id),
        }
    }

    fn convert_list(&mut self, list: &Vec<BlogBody>, id: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_list(list, id),
            PMDWASMSerializer::AsHTML(x) => x.convert_list(list, id),
        }
    }

    fn convert_paragraph(&mut self, text: &Box<BlogBody>, id: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_paragraph(text, id),
            PMDWASMSerializer::AsHTML(x) => x.convert_paragraph(text, id),
        }
    }

    fn convert_text(&mut self, text: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_text(text),
            PMDWASMSerializer::AsHTML(x) => x.convert_text(text),
        }
    }

    fn convert_span(&mut self, span: &Span) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_span(span),
            PMDWASMSerializer::AsHTML(x) => x.convert_span(span),
        }
    }

    fn convert_citation(&mut self, citation: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_citation(citation),
            PMDWASMSerializer::AsHTML(x) => x.convert_citation(citation),
        }
    }

    fn convert_note(&mut self, id: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_note(id),
            PMDWASMSerializer::AsHTML(x) => x.convert_note(id),
        }
    }

    fn convert_toc(&mut self) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_toc(),
            PMDWASMSerializer::AsHTML(x) => x.convert_toc(),
        }
    }

    fn convert_page_break(&mut self) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_page_break(),
            PMDWASMSerializer::AsHTML(x) => x.convert_page_break(),
        }
    }

    fn convert_factbox(&mut self, factbox: &FactBox, id: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_factbox(factbox, id),
            PMDWASMSerializer::AsHTML(x) => x.convert_factbox(factbox, id),
        }
    }

    fn convert_factbox_note(&mut self, factbox: &FactBox, factbox_id: Option<&String>, id: &String) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert_factbox_note(factbox, factbox_id, id),
            PMDWASMSerializer::AsHTML(x) => x.convert_factbox_note(factbox, factbox_id, id),
        }
    }

    fn convert(&mut self, md: &PawsMarkdown) -> Result<String> {
        match self {
            PMDWASMSerializer::AsPDF(x)   => x.convert(md),
            PMDWASMSerializer::AsHTML(x) => x.convert(md),
        }
    }
}

struct PMDWASMSerializerHTML { 
    pub common: PMDHTML<PMDWASMSerializerHTML>,
    header: BlogHeader,
    references: OrderedMap<String, Reference<ReferenceDefinition>>,
    notes: OrderedMap<String, Reference<BlogBody>>,
    notes_id: String,
    bibliography_id: String,
}

impl PMDSharedHTMLSerializer for PMDWASMSerializerHTML {
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

impl PMDWASMSerializerHTML {
    pub fn new() -> Box<Self> {
        let mut value = Box::new(Self {
            common: PMDHTML::uninit(),
            header: BlogHeader::default(),
            references: OrderedMap::new(),
            notes: OrderedMap::new(),
            notes_id: String::new(),
            bibliography_id: String::new(),
        });
        value.common = PMDHTML::new("", &mut *value);
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

impl PMDSerializer for PMDWASMSerializerHTML {
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

struct PMDWASMSerializerPDF { 
    pub common: PMDHTML<PMDWASMSerializerPDF>,
    header: BlogHeader,
    references: OrderedMap<String, Reference<ReferenceDefinition>>,
    notes: OrderedMap<String, Reference<BlogBody>>,
    notes_id: String,
    bibliography_id: String,
}

impl PMDSharedHTMLSerializer for PMDWASMSerializerPDF {
    const LINK_ELEMENTS: bool = false;
    const POPUPS: bool = false;
    const SHOW_BACKREFS: bool = false;

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
        Ok(if let Some((BlogBody::Paragraph(content), _)) = paragraph {
            let text = PMDPureTextSerializer::new().convert_paragraph(content, &String::new())?;
            text.trim_end().to_string()
        } else {
            String::new()
        })
    }

    fn prepare_html_header(&mut self, description: &String) -> String {
        let mut output = String::new();

        let mut max_depth = 0;
        if let Some(toc) = self.header.toc.as_ref() {
            max_depth = toc.max_depth;
        }

        let title = &self.header.title;

        let frontmatter = self.header.frontmatter.clone().unwrap_or(Frontmatter::new());

        let top_left   = Self::parse_pdf_string(&frontmatter["pdf-header-left"]  );
        let mut top_center = Self::parse_pdf_string(&frontmatter["pdf-header-center"]);
        let top_right  = Self::parse_pdf_string(&frontmatter["pdf-header-right"]  );

        let bottom_left   = Self::parse_pdf_string(&frontmatter["pdf-footer-left"]  );
        let mut bottom_center = Self::parse_pdf_string(&frontmatter["pdf-footer-center"]);
        let bottom_right  = Self::parse_pdf_string(&frontmatter["pdf-footer-right"]  );
        let font = &frontmatter["pdf-font"].as_str();

        if top_center == "" {
            top_center = Self::parse_pdf_string(&frontmatter["pdf-header"]);
        }
        if bottom_center == "" {
            bottom_center = Self::parse_pdf_string(&frontmatter["pdf-footer"]);
        }

        let text_size   = frontmatter["pdf-text-size"].as_i64();
        let line_height = frontmatter["pdf-line-height"].as_i64();

        output += "<meta http-equiv=\"content-type\" content=\"text/html; charset=utf-8\">\n";
        output += "\n";
        output += "<!-- tag needed for media query -->\n";
        output += "<meta name=\"viewport\"    content=\"width=device-width, initial-scale=1, minimum-scale=1\" />\n";
        output += "\n";
        output += "<!-- Primary Meta Tags -->\n";
        output += format!("<title>{title}</title>\n").as_str();
        output += format!("<meta name=\"title\" content=\"{title}\">\n").as_str();
        output += format!("<meta name=\"description\" content=\"{description}\">\n").as_str();
        output += "\n";
        output += "<!-- stylesheets -->\n";
        output += "<style>\n";
        output += "main {\n";
        if let Some(font) = font {
            output += format!("    font-family: '{font}';\n").as_str()
        } else {
            output += "    font-family: 'Atkinson Hyperlegible', sans-serif;\n";
        }
        if let Some(text_size) = text_size {
            output += format!("    font-size: {text_size}pt;\n").as_str();
        }
        if let Some(line_height) = line_height {
            output += format!("    line-height: {line_height};\n").as_str();
        }
        output += "}\n";
        output += "\n";
        output += "h1 {margin-top: 2em; margin-bottom: 0; font-size: 18pt; }\n";
        output += "h2 {margin-top: 2em; margin-bottom: 0; font-size: 16pt; }\n";
        output += "h3 {margin-top: 2em; margin-bottom: 0; font-size: 14pt; }\n";
        output += "h4 {margin-top: 2em; margin-bottom: 0; font-size: 12pt; }\n";
        output += "h5 {margin-top: 2em; margin-bottom: 0; font-size: 12pt; }\n";
        output += "h6 {margin-top: 2em; margin-bottom: 0; font-size: 12pt; }\n";
        output += ".bibliography h1 { margin-top: auto; margin-bottom: auto; font-size: 18pt; }\n";
        output += ".notes h1 { margin-top: auto; margin-bottom: auto; font-size: 18pt; }\n";
        output += "\n";
        if !frontmatter.has("pdf-no-first-page") {
            output += ".banner { \n";
            output += "    width: 100%;\n";
            output += "}\n";
        } else {
            output += ".banner { \n";
            output += "    max-height: 16cm;\n";
            output += "    align-self: center;\n";
            output += "}\n";
        }
        output += "\n";
        output += ".image { \n";
        output += "    justify-content: center;\n";
        output += "    display: flex;\n";
        output += "}\n";
        output += "\n";
        output += ".quote { \n";
        output += "    justify-content: center;\n";
        output += "    margin-bottom: 1em;\n";
        output += "    display: flex;\n";
        output += "}\n";
        output += "\n";
        output += ".quote-line {\n";
        output += "    width: 1ch;\n";
        output += "    padding-left: 0;\n";
        output += "    padding-right: 1ch;\n";
        output += "    border-left: 0.3ch solid black;\n";
        output += "}\n";
        output += "\n";
        output += ".quote-text {\n";
        output += "    margin-left: 0;\n";
        output += "    margin-top: 0;\n";
        output += "    margin-bottom: 0;\n";
        output += "}\n";
        output += "\n";
        output += "a {\n";
        output += "    text-decoration: none;\n";
        output += "    color: black;\n";
        output += "}\n";
        output += "\n";
        output += ".inline-link:link {color: blue}\n";
        output += ".inline-link:visited {color: purple}\n";
        output += ".inline-link:active {color: red}\n";
        output += "\n";
        output += ".toc ul {\n";
        output += "    padding: 0;\n";
        output += "    list-style: none;\n";
        output += "    font-size: larger;\n";
        output += "}\n";
        output += "\n";
        output += ".toci-header {\n";
        output += "    position: relative;\n";
        output += "    overflow: hidden;\n";
        output += "}\n";
        output += "\n";
        output += ".toci-header::after {\n";
        output += "    position: absolute;\n";
        output += "    padding-left: .25ch;\n";
        output += "    content: \" . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . \"\n";
        output += "    \". . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . \"\n";
        output += "    \". . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . \";\n";
        output += "    text-align: right;\n";
        output += "}\n";
        output += "\n";
        output += ".toci-number {\n";
        output += "    min-width: 1ch;\n";
        output += "}\n";
        output += "\n";
        output += ".toci-number::after {\n";
        output += "    content: target-counter(attr(href), page);\n";
        output += "}\n";
        output += "\n";
        output += ".toci-1 {\n";
        output += "    display: grid;\n";
        output += "    grid-template-columns: auto max-content;\n";
        output += "    grid-template-areas: \"toci-header toci-number\";\n";
        output += "    align-items: end;\n";
        output += "    gap: 0.25rem;\n";
        output += "}\n";
        output += "\n";

        if max_depth > 1 {
            for i in 2..(max_depth + 1) {
                let margin = i - 1;
                output += format!(".toci-{i} {{\n").as_str();
                output += "    display: grid;\n";
                output += "    grid-template-columns: auto max-content;\n";
                output += "    grid-template-areas: \"toci-header toci-number\";\n";
                output += "    align-items: end;\n";
                output += "    gap: 0.25rem;\n";
                output += format!("    margin-left: {margin}em;\n").as_str();
                output += "}\n";
                output += "\n";
            }
        }
        
        output += ".factbox {\n";
        output += "    background-color: #f3f3f3;\n";
        output += "    border-radius: .5vmin;\n";
        output += "    display: flex;\n";
        output += "    flex: auto;\n";
        output += "    flex-direction: column;\n";
        output += "    overflow: hidden;\n";
        output += "    margin-bottom: 1em;\n";
        output += "    padding-bottom: 1em;\n";
        output += "}\n";
        output += "\n";
        output += ".factbox-header {\n";
        output += "    font-size: 14pt;\n";
        output += "    padding: 1pt;\n";
        output += "    padding-left: 1em;\n";
        output += "    margin-bottom: 1em;\n";
        output += "}\n";
        output += "\n";
        output += ".factbox-header > h2 {\n";
        output += "    font-size: 14pt;\n";
        output += "    margin-top: 1em;\n";
        output += "    margin-bottom: 0;\n";
        output += "}\n";
        output += "\n";
        output += ".factbox-content {\n";
        output += "    min-width: fit-content;\n";
        output += "    display: flex;\n";
        output += "    flex: auto;\n";
        output += "    flex-direction: column;\n";
        output += "    align-items: flex-start;\n";
        output += "    padding: 0;\n";
        output += "}\n";
        output += "\n";
        output += ".factbox-content > section {\n";
        output += "    padding-left: 2em;\n";
        output += "    padding-right: 1em;\n";
        output += "}\n";
        output += "\n";
        output += ".factbox-content > .page-break {\n";
        output += "    width: calc(100%);\n";
        output += "    padding-left: 1em;\n";
        output += "}\n";

        output += "\n";
        output += "@media print {\n";
        if !frontmatter.has("pdf-no-first-page") {
            output += "    .title {\n";
            output += "        page-break-after: always;\n";
            output += "    }\n";
        } else {
            output += "    .title {\n";
            output += "        display: flex;\n";
            output += "        flex-direction: column;\n";
            output += "    }\n";
        }
        output += "\n";
        output += "    .toc {\n";
        output += "        page-break-after: always;\n";
        output += "    }\n";
        output += "\n";
        output += "    @page {\n";
        output += format!("       @top-left   {{content: {top_left  }}}\n").as_str();
        output += format!("       @top-center {{content: {top_center}}}\n").as_str();
        output += format!("       @top-right  {{content: {top_right }}}\n").as_str();
        output += "\n";
        output += format!("       @bottom-left   {{content: {bottom_left  }}}\n").as_str();
        output += format!("       @bottom-center {{content: {bottom_center}}}\n").as_str();
        output += format!("       @bottom-right  {{content: {bottom_right }}}\n").as_str();
        output += "    }\n";
        output += "\n";
        if !frontmatter.has("pdf-no-first-page") {
            output += "   @page:first {\n";
            output += "       @top-left   {content: none}\n";
            output += "       @top-center {content: none}\n";
            output += "       @top-right  {content: none}\n";
            output += "\n";
            output += "       @bottom-left   {content: none}\n";
            output += "       @bottom-center {content: none}\n";
            output += "       @bottom-right  {content: none}\n";
            output += "   }\n";
        }
        output += "}\n";
        output += "</style>\n";
        output += "\n";
        output += "<!-- for Atkinson Hyperlegible font-->\n";
        output += "<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n";
        output += "<link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n";
        output += "<link href=\"https://fonts.googleapis.com/css2?family=Atkinson+Hyperlegible:ital,wght@0,400;0,700;1,400;1,700&display=swap\" rel=\"stylesheet\">\n";
        output += "\n";
        output += "<!-- paged.js -->\n";
        output += "<script src='https://unpkg.com/pagedjs/dist/paged.polyfill.js'></script>\n";
        output += "\n";
        output += "<!-- highlight.js -->\n";
        output += "<script src=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js\"></script>\n";
        output += "<link rel=\"stylesheet\" href=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/default.min.css\">\n";
        output += "<script>\n";
        output += "    class HighlightJSHandler extends Paged.Handler {\n";
        output += "        constructor(chunker, polisher, caller) {\n";
        output += "            super(chunker, polisher, caller);\n";
        output += "        }\n";
        output += "\n";
        output += "        afterParsed(parsed) {\n";
        output += "            hljs.highlightAll();\n";
        output += "            parsed.querySelectorAll('code').forEach((el) => {\n";
        output += "                hljs.highlightElement(el);\n";
        output += "            })\n";
        output += "        }\n";
        output += "    }\n";
        output += "\n";
        output += "    Paged.registerHandlers(HighlightJSHandler);\n";
        output += "</script>\n";
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

    fn generate_link(&mut self, _: &String, _: ObjectKind) -> String {
        String::new()
    }

    fn references(&mut self) -> &OrderedMap<String, Reference<ReferenceDefinition>> {
        &self.references
    }

    fn mut_references(&mut self) -> &mut OrderedMap<String, Reference<ReferenceDefinition>> {
        &mut self.references
    }

    fn get_mut_reference<T: AsRef<str>>(&mut self, key: T) -> Option<&mut Reference<ReferenceDefinition>> {
        self.references.get_mut(key.as_ref())
    }

    fn get_reference<T: AsRef<str>>(&mut self, key: T) -> Option<&Reference<ReferenceDefinition>> {
        self.references.get(key.as_ref())
    }
}

impl PMDWASMSerializerPDF {
    pub fn new() -> Box<Self> {
        let mut value = Box::new(Self {
            common: PMDHTML::uninit(),
            header: BlogHeader::default(),
            references: OrderedMap::new(),
            notes: OrderedMap::new(),
            notes_id: String::new(),
            bibliography_id: String::new(),
        });
        value.common = PMDHTML::new("", &mut *value);
        value
    }

    fn parse_pdf_string(text: &Value) -> String {
        if let Some(text) = text.as_string() {
            let mut text = text.clone();
            text.insert(0, '"');
            text.push('"');
            text = text.replace("%pages", "\"counter(pages)\"");
            text = text.replace("%page", "\"counter(page)\"");
            text = text.replace("%np", "\"counter(pages)\"");
            text = text.replace("%p",  "\"counter(page)\"");
            if text.len() == 0 { text = "none".to_string(); }
            text
        } else { "none".to_string() }
    }

    /*
    fn prepare_header(&mut self, frontmatter: &Option<Frontmatter>, max_depth: usize, title: &String, description: &String) -> String {
        let mut output = String::new();

        let frontmatter = frontmatter.clone().unwrap_or(Frontmatter::new());

        let top_left   = Self::parse_pdf_string(&frontmatter["pdf-header-left"]  );
        let mut top_center = Self::parse_pdf_string(&frontmatter["pdf-header-center"]);
        let top_right  = Self::parse_pdf_string(&frontmatter["pdf-header-right"]  );

        let bottom_left   = Self::parse_pdf_string(&frontmatter["pdf-footer-left"]  );
        let mut bottom_center = Self::parse_pdf_string(&frontmatter["pdf-footer-center"]);
        let bottom_right  = Self::parse_pdf_string(&frontmatter["pdf-footer-right"]  );
        let font = &frontmatter["pdf-font"].as_str();

        if top_center == "" {
            top_center = Self::parse_pdf_string(&frontmatter["pdf-header"]);
        }
        if bottom_center == "" {
            bottom_center = Self::parse_pdf_string(&frontmatter["pdf-footer"]);
        }

        let text_size   = frontmatter["pdf-text-size"].as_i64();
        let line_height = frontmatter["pdf-line-height"].as_i64();

        output += "<meta http-equiv=\"content-type\" content=\"text/html; charset=utf-8\">\n";
        output += "\n";
        output += "<!-- tag needed for media query -->\n";
        output += "<meta name=\"viewport\"    content=\"width=device-width, initial-scale=1, minimum-scale=1\" />\n";
        output += "\n";
        output += "<!-- Primary Meta Tags -->\n";
        output += format!("<title>{title}</title>\n").as_str();
        output += format!("<meta name=\"title\" content=\"{title}\">\n").as_str();
        output += format!("<meta name=\"description\" content=\"{description}\">\n").as_str();
        output += "\n";
        output += "<!-- stylesheets -->\n";
        output += "<style>\n";
        output += "main {\n";
        if let Some(font) = font {
            output += format!("    font-family: '{font}';\n").as_str()
        } else {
            output += "    font-family: 'Atkinson Hyperlegible', sans-serif;\n";
        }
        if let Some(text_size) = text_size {
            output += format!("    font-size: {text_size}pt;\n").as_str();
        }
        if let Some(line_height) = line_height {
            output += format!("    line-height: {line_height};\n").as_str();
        }
        output += "}\n";
        output += "\n";
        output += "h1 {margin-top: 2em; margin-bottom: 0; font-size: 18pt; }\n";
        output += "h2 {margin-top: 2em; margin-bottom: 0; font-size: 16pt; }\n";
        output += "h3 {margin-top: 2em; margin-bottom: 0; font-size: 14pt; }\n";
        output += "h4 {margin-top: 2em; margin-bottom: 0; font-size: 12pt; }\n";
        output += "h5 {margin-top: 2em; margin-bottom: 0; font-size: 12pt; }\n";
        output += "h6 {margin-top: 2em; margin-bottom: 0; font-size: 12pt; }\n";
        output += ".bibliography h1 { margin-top: auto; margin-bottom: auto; font-size: 18pt; }\n";
        output += ".notes h1 { margin-top: auto; margin-bottom: auto; font-size: 18pt; }\n";
        output += "\n";
        if !frontmatter.has("pdf-no-first-page") {
            output += ".banner { \n";
            output += "    width: 100%;\n";
            output += "}\n";
        } else {
            output += ".banner { \n";
            output += "    max-height: 16cm;\n";
            output += "    align-self: center;\n";
            output += "}\n";
        }
        output += "\n";
        output += ".image { \n";
        output += "    justify-content: center;\n";
        output += "    display: flex;\n";
        output += "}\n";
        output += "\n";
        output += ".quote { \n";
        output += "    justify-content: center;\n";
        output += "    margin-bottom: 1em;\n";
        output += "    display: flex;\n";
        output += "}\n";
        output += "\n";
        output += ".quote-line {\n";
        output += "    width: 1ch;\n";
        output += "    padding-left: 0;\n";
        output += "    padding-right: 1ch;\n";
        output += "    border-left: 0.3ch solid black;\n";
        output += "}\n";
        output += "\n";
        output += ".quote-text {\n";
        output += "    margin-left: 0;\n";
        output += "    margin-top: 0;\n";
        output += "    margin-bottom: 0;\n";
        output += "}\n";
        output += "\n";
        output += "a {\n";
        output += "    text-decoration: none;\n";
        output += "    color: black;\n";
        output += "}\n";
        output += "\n";
        output += ".inline-link:link {color: blue}\n";
        output += ".inline-link:visited {color: purple}\n";
        output += ".inline-link:active {color: red}\n";
        output += "\n";
        output += ".toc ul {\n";
        output += "    padding: 0;\n";
        output += "    list-style: none;\n";
        output += "    font-size: larger;\n";
        output += "}\n";
        output += "\n";
        output += ".toci-header {\n";
        output += "    position: relative;\n";
        output += "    overflow: hidden;\n";
        output += "}\n";
        output += "\n";
        output += ".toci-header::after {\n";
        output += "    position: absolute;\n";
        output += "    padding-left: .25ch;\n";
        output += "    content: \" . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . \"\n";
        output += "    \". . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . \"\n";
        output += "    \". . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . \";\n";
        output += "    text-align: right;\n";
        output += "}\n";
        output += "\n";
        output += ".toci-number {\n";
        output += "    min-width: 1ch;\n";
        output += "}\n";
        output += "\n";
        output += ".toci-number::after {\n";
        output += "    content: target-counter(attr(href), page);\n";
        output += "}\n";
        output += "\n";
        output += ".toci-1 {\n";
        output += "    display: grid;\n";
        output += "    grid-template-columns: auto max-content;\n";
        output += "    grid-template-areas: \"toci-header toci-number\";\n";
        output += "    align-items: end;\n";
        output += "    gap: 0.25rem;\n";
        output += "}\n";
        output += "\n";

        if max_depth > 1 {
            for i in 2..(max_depth + 1) {
                let margin = i - 1;
                output += format!(".toci-{i} {{\n").as_str();
                output += "    display: grid;\n";
                output += "    grid-template-columns: auto max-content;\n";
                output += "    grid-template-areas: \"toci-header toci-number\";\n";
                output += "    align-items: end;\n";
                output += "    gap: 0.25rem;\n";
                output += format!("    margin-left: {margin}em;\n").as_str();
                output += "}\n";
                output += "\n";
            }
        }
        

        output += "\n";
        output += "@media print {\n";
        if !frontmatter.has("pdf-no-first-page") {
            output += "    .title {\n";
            output += "        page-break-after: always;\n";
            output += "    }\n";
        } else {
            output += "    .title {\n";
            output += "        display: flex;\n";
            output += "        flex-direction: column;\n";
            output += "    }\n";
        }
        output += "\n";
        output += "    .toc {\n";
        output += "        page-break-after: always;\n";
        output += "    }\n";
        output += "\n";
        output += "    @page {\n";
        output += format!("       @top-left   {{content: {top_left  }}}\n").as_str();
        output += format!("       @top-center {{content: {top_center}}}\n").as_str();
        output += format!("       @top-right  {{content: {top_right }}}\n").as_str();
        output += "\n";
        output += format!("       @bottom-left   {{content: {bottom_left  }}}\n").as_str();
        output += format!("       @bottom-center {{content: {bottom_center}}}\n").as_str();
        output += format!("       @bottom-right  {{content: {bottom_right }}}\n").as_str();
        output += "    }\n";
        output += "\n";
        if !frontmatter.has("pdf-no-first-page") {
            output += "   @page:first {\n";
            output += "       @top-left   {content: none}\n";
            output += "       @top-center {content: none}\n";
            output += "       @top-right  {content: none}\n";
            output += "\n";
            output += "       @bottom-left   {content: none}\n";
            output += "       @bottom-center {content: none}\n";
            output += "       @bottom-right  {content: none}\n";
            output += "   }\n";
        }
        output += "}\n";
        output += "</style>\n";
        output += "\n";
        output += "<!-- for Atkinson Hyperlegible font-->\n";
        output += "<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n";
        output += "<link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n";
        output += "<link href=\"https://fonts.googleapis.com/css2?family=Atkinson+Hyperlegible:ital,wght@0,400;0,700;1,400;1,700&display=swap\" rel=\"stylesheet\">\n";
        output += "\n";
        output += "<!-- paged.js -->\n";
        output += "<script src='https://unpkg.com/pagedjs/dist/paged.polyfill.js'></script>\n";
        output += "\n";
        output += "<!-- highlight.js -->\n";
        output += "<script src=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js\"></script>\n";
        output += "<link rel=\"stylesheet\" href=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/default.min.css\">\n";
        output += "<script>\n";
        output += "    class HighlightJSHandler extends Paged.Handler {\n";
        output += "        constructor(chunker, polisher, caller) {\n";
        output += "            super(chunker, polisher, caller);\n";
        output += "        }\n";
        output += "\n";
        output += "        afterParsed(parsed) {\n";
        output += "            hljs.highlightAll();\n";
        output += "            parsed.querySelectorAll('code').forEach((el) => {\n";
        output += "                hljs.highlightElement(el);\n";
        output += "            })\n";
        output += "        }\n";
        output += "    }\n";
        output += "\n";
        output += "    Paged.registerHandlers(HighlightJSHandler);\n";
        output += "</script>\n";
        output
    }

    fn generate_id<F: FnMut()->String>(text: &String, mut default_generator: F) -> String {
        if text.len() == 0 { default_generator() } else {

            let mut len = 0;
            for c in text.chars() {
                if c.is_ascii_punctuation() { 
                    break
                } else { len += c.len_utf8() }
            }
            if len == 0 || text[0..len].trim_start().trim_end().len() == 0 { default_generator() } else {
                let mut result = String::new();
                for c in text[0..len].trim_start().trim_end().chars() {
                    result.push(if c.is_whitespace() { '-' } else { c })
                }
                result.remove_matches(|x: char| x != '-' && x.is_ascii_punctuation());
                result.to_lowercase()
            }
        }
    }*/


}

impl PMDSerializer for PMDWASMSerializerPDF {
    fn current_factbox(&mut self) -> Option<(FactBox, Option<String>)> {
        self.common.current_factbox()
    }

    fn convert(&mut self, md: &PawsMarkdown) -> Result<String> {
        self.header = md.header.clone();
        self.notes_id = md.notes_id.clone();
        self.bibliography_id = md.bibliography_id.clone();

        for (id, reference) in &md.references {
            self.references.insert(id.clone(), Reference::new(reference.clone()));
        }
        
        for (id, reference) in &md.notes {
            self.notes.insert(id.clone(), Reference::new(reference.clone()));
        }

        let output = self.common.html(md, None, &md.references, &md.notes)?;
/*
        let paragraph = md.body.iter().find(|&x| match x { BlogBody::Paragraph(_) => true, _ => false});
        if let Some(BlogBody::Paragraph(content)) = paragraph {
            let text = PMDPureTextSerializer::new().convert_paragraph(content)?;
            description = text.trim_end().to_string();
        } else {
            description = "".into();
        }

        let mut max_depth = 0;
        if let Some(toc) = md.header.toc.as_ref() {
            max_depth = toc.max_depth;
        }

        let header = self.prepare_header(&md.header.frontmatter, max_depth, &md.header.title, &description);

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

        output += self.tab().as_str();
        output += "<main>\n";
        self.push_tab();

        {
            let title    = &md.header.title;
            let subtitle = &md.header.subtitle;
            let banner   = &md.header.banner;
            output += self.tab().as_str();
            output += "<section class='title'>\n";
            self.push_tab();
                output += self.tab().as_str();
                output += format!("<h1>{title}</h1>\n").as_str();
                
                output += self.tab().as_str();
                output += format!("<p class='subtitle'>{subtitle}</p>\n").as_str();
                
                if banner.trim() != "" {
                    output += self.tab().as_str();
                    output += format!("<img src='{banner}'class='banner'></img>\n").as_str();
                }
            self.pop_tab();

            output += self.tab().as_str();
            output += format!("</section>\n").as_str();
        }

        for element in &md.body {
            let result = self.convert_element(element)?;
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
        if md.notes.len() != 0 {
            let id = PMDPDFSerializer::generate_id(&md.header.notes_title, ||"missing".into());
            let title = &md.header.notes_title;
            
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
                output += self.tab().as_str();
                output += format!("<h1>{title}</h1>\n").as_str();
            self.pop_tab();
            output += self.tab().as_str();
            output += format!("</section>\n").as_str();

            for (key, val) in &md.notes {
                let result = self.convert_element(val)?;
                let link = format!("<a href='#^{key}'>^{key}:</a>");

                output += self.tab().as_str();
                output += format!("<section class='note' id=\"^{key}\">").as_str();
                output.push('\n');
                self.push_tab();

                    output += self.tab().as_str();
                    output += "<p>\n";

                    self.push_tab();
                        output += self.tab().as_str();
                        output += "<sup>\n";

                        self.push_tab();
                            output += self.tab().as_str();
                            output += link.as_str();
                            output.push('\n');
                            
                            output += self.tab().as_str();
                            output += result.as_str();
                            output.push('\n');
                            
                            output += self.tab().as_str();
                            output += format!("<a href=\"#{key}-backref\">").as_str();
                            output += "↩";
                            output += "</a>";
                            output.push('\n');
                        self.pop_tab();

                        output += self.tab().as_str();
                        output += "</sup>\n";
                    self.pop_tab();

                    output += self.tab().as_str();
                    output += "</p>";
                    output.push('\n');

                self.pop_tab();
                output += self.tab().as_str();
                output += "</section>\n";
            }
        }
        

        if md.references.len() != 0 {
            let id = PMDPDFSerializer::generate_id(&md.header.bibliography_title, ||"missing".into());
            let title = &md.header.bibliography_title;

            output += self.tab().as_str();
            output += format!("<section class='page-break'>\n").as_str();
            self.push_tab();
                output += self.tab().as_str();
                output += "<hr>\n";
            self.pop_tab();
            output += self.tab().as_str();
            output += format!("</section>\n").as_str();

            output += self.tab().as_str();
            output += format!("<section class='bibliography' id='{id}'>\n").as_str();
            self.push_tab();
                output += self.tab().as_str();
                output += format!("<h1>{title}</h1>\n").as_str();
            self.pop_tab();
            output += self.tab().as_str();
            output += format!("</section>\n").as_str();

            for (key, val) in &self.references {
                if val.times_used == 0 {
                    cprintln!("<y>warning:</> reference '{}' is not used and will not be included", key);
                }
            }
            
            for (key, val) in &md.references {
                let reference = self.references.get(key).unwrap();
                if reference.times_used == 0 { continue; }

                output += self.tab().as_str();
                output += format!("<section id='{key}'>\n").as_str();
                self.push_tab();
                    output += self.tab().as_str();
                    output += format!("<p>\n").as_str();
                    self.push_tab();
                    
                        output += to_html_bibliography(val).as_str();
                        output.push('\n');
                    
                    self.pop_tab();
                    output += self.tab().as_str();
                    output += "</p>\n";

                self.pop_tab();
                output += self.tab().as_str();
                output += "</section>\n";
            }
        }

        self.pop_tab();
        output += self.tab().as_str();
        output +=   "</main>\n";

        self.pop_tab();
        output += self.tab().as_str();
        output +=   "</body>\n";

        self.pop_tab();
        output += self.tab().as_str();
        output +=   "</html>\n";

        */
        // let mut html = Builder::new().suffix(".html").tempfile_in(".")?;
        // 
        // writeln!(html, "{}", output)?;
        // let path = html.path().to_str().expect("could not convert parth to &str");
        // std::fs::write("tmp.html", output)?;
        // 
        // build_pdf(path)
        Ok(output)
    }

    fn convert_factbox(&mut self, factbox: &FactBox, id: &String) -> Result<String> {
        // cprintln!("<r>error:</> fact boxes aren't implemented for pdf output");
        // Ok("".into())
        self.common.convert_factbox(factbox, id)
    }

    fn convert_hoverable(&mut self, hoverable: &Alternative) -> Result<String> {
        Ok(self.convert_element(no_id!(&hoverable.base))?)
    }

    fn convert_styled(&mut self, styled: &Alternative) -> Result<String> {
        /*
        let text  = self.convert_element(&styled.alt)?;
        let style = self.convert_element(&styled.base)?;
        Ok(format!("<span class='embedded-style' style='{style}'>{text}</span>"))
        */
        self.common.convert_styled(styled)
    }

    fn convert_link(&mut self, link: &Alternative) -> Result<String> {
        /*
        let href = self.convert_element(&link.alt)?;
        let text = self.convert_element(&link.base)?;
        Ok(format!("<a class='inline-link' href='{href}'>{text}</a>"))
        */
        self.common.convert_link(link)
    }

    fn convert_header(&mut self, text: &Box<BlogBody>, depth: usize, id: &String) -> Result<String> {
        self.common.convert_header(text, depth, id)
        /*
        let text  = self.convert_element(text)?;
        let id    = Self::generate_id(&text, ||"missing".into());

        let mut result = self.tab();
        result += format!("<section id={id}>\n").as_str();
        self.push_tab();
            result += self.tab().as_str();
            result += format!("<h{depth}>{text}</h{depth}>\n").as_str();
        self.pop_tab();
        result += self.tab().as_str();
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
        // Ok(format!("<code>{text}</code>"))
        self.common.convert_inlinecode(text)
    }

    fn convert_codeblock(&mut self, text: &String, id: &String) -> Result<String> {
        self.common.convert_codeblock(text, id)
        /*
        let first_line = text.lines().nth(0).context("expected at least one line in codeblock")?;
        let mut words = first_line.split(|x: char| x.is_whitespace());
        let lang = words.nth(0).unwrap_or("plaintext");
        let mut body = text[text.find('\n').context("expected at least one line in codeblock")? + 1..].to_string();

        body = body.replace("&", "&amp;")
                   .replace("<", "&lt;")
                   .replace(">", "&gt;");
        body = body.trim_end().replace("\r\n", "\n");
        
        let mut result = self.tab();
        result += format!("<section class='code-block'>\n").as_str();
        self.push_tab();
            result += self.tab().as_str();
            result += format!("<pre><code class='language-{lang}'>{body}</code></pre>\n").as_str();
        self.pop_tab();
        result += self.tab().as_str();
        result += "</section>\n";

        Ok(result)
        */
    }

    fn convert_image(&mut self, src: &String, alt: &String, id: &String) -> Result<String> {
        self.common.convert_image(src, alt, id)
        /*
        let mut result = self.tab();
        result += format!("<section class='image'>\n").as_str();
        self.push_tab();
            result += self.tab().as_str();
            result += format!("<img src='{src}' alt='{alt}'></img>\n").as_str();
        self.pop_tab();
        result += self.tab().as_str();
        result += "</section>\n";

        Ok(result)
        */
    }

    fn convert_quote(&mut self, lines: &Vec<BlogBody>, id: &String) -> Result<String> {
        self.common.convert_quote(lines, id)
        /*
        let mut quote_elements : Vec<String> = vec![];
        for elem in lines {
            let text = self.convert_element(elem)?;
            quote_elements.push(format!("{text}<br/>"));
        }
        let text = quote_elements.join("\n");

        let mut result = self.tab();
        result += format!("<section class='quote'>\n").as_str();
        self.push_tab();
            result += self.tab().as_str();
            result += format!("<div class='quote-line'></div><blockquote class='quote-text'>{text}</blockquote>\n").as_str();
        self.pop_tab();
        result += self.tab().as_str();
        result += "</section>\n";

        Ok(result)
        */
    }

    fn convert_list(&mut self, list: &Vec<BlogBody>, id: &String) -> Result<String> {
        self.common.convert_list(list, id)
        /*
        let mut result = self.tab();
        result += format!("<section class='list'>\n").as_str();
        self.push_tab();
            result += self.tab().as_str();
            result += "<ul>";
            self.push_tab();
            for elem in list {
                let text = self.convert_element(elem)?;
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
        */
    }

    fn convert_paragraph(&mut self, text: &Box<BlogBody>, id: &String) -> Result<String> {
        self.common.convert_paragraph(text, id)
        /*
        let paragraph = self.convert_element(text)?;

        let mut result = self.tab();
        result += format!("<section>\n").as_str();
        self.push_tab();
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
        */
    }

    fn convert_text(&mut self, text: &String) -> Result<String> {
        // Ok(text.clone())
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
            reference.times_used += 1;
            let text = to_citation(&reference.def);
            let mut result = self.tab();
            result += "<cite>";
            result += format!("<a href='#{id}'>").as_str();
            result += text.as_str();
            result += "</a>";
            result += "</cite>";
            Ok(result)
        } else {
            cprintln!("<y>warning:</> {} has no source", id);
            Ok(format!("(MISSING CITATION)").to_string())
        }
        */
    }

    fn convert_factbox_note(&mut self, factbox: &FactBox, factbox_id: Option<&String>, id: &String) -> Result<String> {
        self.common.convert_factbox_note(factbox, factbox_id, id)
    }

    fn convert_note(&mut self, id: &String) -> Result<String> {
        // Ok(format!("<sup><a id='{id}-backref' href='#^{id}'>{id}</a></sup>"))
        self.common.convert_note(id)
    }

    fn convert_toc(&mut self) -> Result<String> {
        // self.common.convert_toc()
       
        if self.header.toc.is_none() { return Err(anyhow!("expected a table of content but it was None")); }
        let toc = self.header.toc.clone().unwrap();
        let title = &toc.title;

        let mut result = self.common.tab();
        
        result += "<section id='table-of-contents'>\n";
        self.common.push_tab();
            result += self.common.tab().as_str();
            result += format!("<h1>{title}</h1>\n").as_str();
        self.common.pop_tab();
        result += self.common.tab().as_str();
        result += "</section>\n";

        result += self.common.tab().as_str();
        result += "<section class='toc'>\n";
        self.common.push_tab();
            result += self.common.tab().as_str();
            result += "<ul>\n";
            self.common.push_tab();
                for (elem, depth, id) in &toc.headers {
                    // let text = self.convert_element(elem)?;
                    // let id    = Self::generate_id(&text, ||"missing".into());
                    let text = if let &box BlogBody::FactBox(fbox) = &elem {
                        fbox.title.clone()
                    } else {
                        self.convert_element(no_id!(elem))?
                    };
                    result += self.common.tab().as_str();
                    result += format!("<li class='toci-{depth}'>\n").as_str();
                    self.common.push_tab();
                        result += self.common.tab().as_str();
                        result += format!("<a class='toci-header' href='#{id}'>{text}</a>\n").as_str();
                        result += self.common.tab().as_str();
                        result += format!("<a class='toci-number' href='#{id}'></a>\n").as_str();
                    self.common.pop_tab();
                    result += self.common.tab().as_str();
                    result += "</li>\n";
                }
                /*
                if !self.notes.is_empty() { 
                    let title = self.header.notes_title.clone();
                    let id = &self.notes_id;
                    result += self.common.tab().as_str();
                    result += format!("<li class='toci-1'>\n").as_str();
                    self.common.push_tab();
                        result += self.common.tab().as_str();
                        result += format!("<a class='toci-header' href='#{id}'>{title}</a>\n").as_str();
                        result += self.common.tab().as_str();
                        result += format!("<a class='toci-number' href='#{id}'></a>\n").as_str();
                    self.common.pop_tab();
                    result += self.common.tab().as_str();
                    result += "</li>\n";
                }
                if !self.references.is_empty() { 
                    let title = self.header.bibliography_title.clone();
                    let id = &self.bibliography_id;
                    result += self.common.tab().as_str();
                    result += format!("<li class='toci-1'>\n").as_str();
                    self.common.push_tab();
                        result += self.common.tab().as_str();
                        result += format!("<a class='toci-header' href='#{id}'>{title}</a>\n").as_str();
                        result += self.common.tab().as_str();
                        result += format!("<a class='toci-number' href='#{id}'></a>\n").as_str();
                    self.common.pop_tab();
                    result += self.common.tab().as_str();
                    result += "</li>\n";
                }
                */
            self.common.pop_tab();
            result += self.common.tab().as_str();
            result += "</ul>\n";

        self.common.pop_tab();
        result += self.common.tab().as_str();
        result += "</section>\n";

        Ok(result)
    }

    fn convert_page_break(&mut self) -> Result<String> {
        self.common.convert_page_break()
        /*
        let mut result = self.tab();

        result += format!("<section class='page-break'>\n").as_str();
        self.push_tab();
            result += self.tab().as_str();
            result += "<hr>\n";
        self.pop_tab();
        result += self.tab().as_str();
        result += format!("</section>\n").as_str();

        Ok(result)
        */
    }
}





