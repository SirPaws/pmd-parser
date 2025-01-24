
use super::reference::ReferenceDefinition;

#[derive(Debug, PartialEq)]
pub enum ListItem {
    InnerList(Vec<ListItem>),
    Unordered(String),                   // -
    Numbered(usize, String),             // 1.
    Alphabetical(String, String),        // a.
    NumberedRounded(usize, String),      // 1)
    AlphabeticalRounded(String, String), // a)
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImageSize {
    None, 
    Single(String),
    Double(String, String)
}

impl ImageSize {
    pub fn from_width_and_height(width: Option<String>, height: Option<String>) -> Self {
        match (width, height) {
            (Some(x), Some(y)) => Self::Double(x, y),
            (None, Some(y)) => Self::Double("0".to_string(), y),
            (Some(x), None) => Self::Single(x),
            (None, None) => Self::None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Image {
    pub image: String,
    pub alt: String,
    pub size: ImageSize
}

impl Image {
    pub fn new(image: String) -> Self {
        Self { image, alt: "".into(), size: ImageSize::None }
    }
    
    pub fn new_with_alt(image: String, alt: String) -> Self {
        Self { image, alt, size: ImageSize::None }
    }
    
    pub fn new_with_size(image: String, size: ImageSize) -> Self {
        Self { image, alt: "".into(), size }
    }
    
    pub fn new_with_alt_and_size(image: String, alt: String, size: ImageSize) -> Self {
        Self { image, alt, size }
    }
}


#[derive(Debug, PartialEq)]
pub enum TopLevelSyntax {
    FrontMatter(super::super::frontmatter::Frontmatter),
    CodeBlock(String),
    Heading(String, usize),
    Image(Image),
    List(Vec<ListItem>),
    Paragraph(String),
    Quote(Vec<String>),
    ReferenceDefinition(ReferenceDefinition),
    NoteDefinition{id: String, text: String},
    TOC(String),
    PageBreak,
    FactBox{title: String, body: Vec<TopLevelSyntax>},
//  EmbeddedLink(String, String)
}
