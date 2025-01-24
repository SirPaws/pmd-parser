
use crate::*;


pub trait PMDSerializer {
    // because rust is such a lovely language where I can't actually describe the link between
    // the lifetime of factbox and id here, i'll have to copy them each time, hwo fucking lovely
    fn current_factbox(&mut self) -> Option<(FactBox, Option<String>)>;

    fn convert_hoverable(&mut self, hoverable: &Alternative) -> Result<String>;
    fn convert_styled(&mut self, styled: &Alternative) -> Result<String>;
    fn convert_link(&mut self, link: &Alternative) -> Result<String>;
    fn convert_header(&mut self, text: &Box<BlogBody>, depth: usize, id: &String) -> Result<String>;
    fn convert_italics(&mut self, text: &Box<BlogBody>) -> Result<String>;
    fn convert_bold(&mut self, text: &Box<BlogBody>) -> Result<String>;
    fn convert_inlinecode(&mut self, text: &String) -> Result<String>;
    fn convert_codeblock(&mut self, text: &String, id: &String) -> Result<String>;
    fn convert_image(&mut self, src: &String, alt: &String, id: &String) -> Result<String>;
    fn convert_quote(&mut self, lines: &Vec<BlogBody>, id: &String) -> Result<String>;
    fn convert_list(&mut self, list: &Vec<BlogBody>, id: &String) -> Result<String>;
    fn convert_paragraph(&mut self, text: &Box<BlogBody>, id: &String) -> Result<String>;
    fn convert_text(&mut self, text: &String) -> Result<String>;
    fn convert_span(&mut self, span: &Span) -> Result<String>;
    fn convert_citation(&mut self, citation: &String) -> Result<String>;
    fn convert_note(&mut self, id: &String) -> Result<String>;
    fn convert_toc(&mut self) -> Result<String>;
    fn convert_page_break(&mut self) -> Result<String>;
    // fn convert_embedded_link(&mut self, src: &String, alt: &String) -> Result<String>;

    fn convert_factbox(&mut self, factbox: &FactBox, id: &String) -> Result<String>;
    fn convert_factbox_note(&mut self, factbox: &FactBox, factbox_id: Option<&String>, id: &String) -> Result<String>;

    fn convert_factbox_elements(&mut self, factbox: &FactBox, factbox_id: Option<&String>) -> Result<Vec<String>> {
        let mut result = Vec::new();
        for element in &factbox.body {
            match element {
                (BlogBody::FactBox(factbox), id)                    => result.push(self.convert_factbox(factbox, id)?),
                (BlogBody::Hoverable(hoverable), _)            => result.push(self.convert_hoverable(hoverable)?),
                (BlogBody::Styled(styled), _)                  => result.push(self.convert_styled(styled)?),
                (BlogBody::Link(link), _)                      => result.push(self.convert_link(link)?),
                (BlogBody::Header(text, depth), id)   => result.push(self.convert_header(text, *depth, id)?),
                (BlogBody::Italics(text), _)                 => result.push(self.convert_italics(text)?),
                (BlogBody::Bold(text), _)                    => result.push(self.convert_bold(text)?)   ,
                (BlogBody::InlineCode(text), _)                     => result.push(self.convert_inlinecode(text)?) ,
                (BlogBody::CodeBlock(text), id)                      => result.push(self.convert_codeblock(text, id)?)  ,
                (BlogBody::Image(text, alt), id)            => result.push(self.convert_image(text, alt, id)?),
                // BlogBody::EmbeddedLink(text, alt) => self.convert_embedresult.push(ded_link(text, alt),
                (BlogBody::Quote(lines), id)                  => result.push(self.convert_quote(lines, id)?),
                (BlogBody::List(list), id)                    => result.push(self.convert_list(list, id)?),
                (BlogBody::Paragraph(text), id)               => result.push(self.convert_paragraph(text, id)?),
                (BlogBody::Text(text), _)                           => result.push(self.convert_text(text)?),
                (BlogBody::Span(span), _)                             => result.push(self.convert_span(span)?),
                (BlogBody::Citation(text), _)                       => result.push(self.convert_citation(text)?),
                (BlogBody::Note(text), _)                           => result.push(self.convert_factbox_note(factbox, factbox_id, text)?),
                (BlogBody::TOCLocationMarker, _)                             => result.push(self.convert_toc()?),
                (BlogBody::PageBreak, _)                                     => result.push(self.convert_page_break()?),
            }
        }
        Ok(result)
    }

    fn convert_element(&mut self, element: (&BlogBody, &String)) -> Result<String> {
        match element {
            (BlogBody::FactBox(factbox), id)                    => self.convert_factbox(factbox, id),
            (BlogBody::Hoverable(hoverable), _)    => self.convert_hoverable(hoverable),
            (BlogBody::Styled(styled), _)          => self.convert_styled(styled),
            (BlogBody::Link(link), _)              => self.convert_link(link),
            (BlogBody::Header(text, depth), id)     => self.convert_header(text, *depth, id),
            (BlogBody::Italics(text), _)           => self.convert_italics(text),
            (BlogBody::Bold(text), _)              => self.convert_bold(text)   ,
            (BlogBody::InlineCode(text), _)        => self.convert_inlinecode(text) ,
            (BlogBody::CodeBlock(text), id)         => self.convert_codeblock(text, id)  ,
            (BlogBody::Image(text, alt), id)        => self.convert_image(text, alt, id),
            // BlogBody::EmbeddedLink(text, alt) => self.convert_embedded_link(text, alt),
            (BlogBody::Quote(lines), id)            => self.convert_quote(lines, id),
            (BlogBody::List(list), id)              => self.convert_list(list, id),
            (BlogBody::Paragraph(text), id)         => self.convert_paragraph(text, id),
            (BlogBody::Text(text), _)              => self.convert_text(text),
            (BlogBody::Span(span), _)              => self.convert_span(span),
            (BlogBody::Citation(text), _)          => self.convert_citation(text),
            (BlogBody::Note(text), _)              => {
                if let Some((factbox, id)) = self.current_factbox() {
                    self.convert_factbox_note(&factbox, id.as_ref(), text)
                } else {
                    self.convert_note(text)
                }
            },
            (BlogBody::TOCLocationMarker, _)       => self.convert_toc(),
            (BlogBody::PageBreak, _)               => self.convert_page_break(),
        }
    }

    fn convert(&mut self, md: &PawsMarkdown) -> Result<String>;
}


pub fn to_string<T>(md: &PawsMarkdown, mut serialiser: T) -> Result<String>
    where T: PMDSerializer,
{
    serialiser.convert(md)
}

pub fn to_string_from_boxed<T>(md: &PawsMarkdown, mut serialiser: Box<T>) -> Result<String>
    where T: PMDSerializer,
{
    serialiser.convert(md)
}
