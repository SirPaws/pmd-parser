
use std::num::ParseIntError;

use super::util::ordered_map::OrderedMap;
use super::frontmatter::*;
use super::toplevel::*;
use super::config::*;

pub use super::toplevel::ReferenceDefinition;

#[derive(Debug, PartialEq, Clone)]
pub struct TableOfContent {
    pub title:   String,
    pub index:   usize,
    pub max_depth: usize,
    pub headers: Vec<(Box<Element>, /*depth: */ usize, /*id: */ String)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MetaData {
    pub title: String,
    pub subtitle: String,
    pub banner: String,
    pub url: String,
    pub data_dir: String,
    pub blog_dir: String,
    pub date_written: PmdDate,
    pub last_update: PmdDate,
    pub hide_references: bool,
    pub hide_notes: bool,
    pub hide_contacts: bool,
    pub toc: Option<TableOfContent>,
    pub bibliography_title: String,
    pub notes_title: String,
    pub frontmatter: Option<Frontmatter>,
}

impl MetaData {
    pub fn default() -> Self {
        Self {
            title: "".into(),
            subtitle: "".into(),
            banner: "".into(),
            url: DEFAULT_URL.into(),
            data_dir: DEFAULT_DATA_DIR.into(),
            blog_dir: DEFAULT_BLOG_DIR.into(),
            date_written: PmdDate::None,
            last_update: PmdDate::None,
            toc:      None,
            hide_references: false,
            hide_notes: false,
            hide_contacts: false,
            bibliography_title: DEFAULT_BIBLIOGRAPHY_TITLE.into(),
            notes_title: DEFAULT_NOTES_TITLE.into(),
            frontmatter: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Alternative {
    pub base: Box<Element>,
    pub alt:  Box<Element>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub elements: Vec<Element>
}

#[derive(Debug, PartialEq, Clone)]
pub enum ListKind {
    Unordered, Numbered,
    // we still need to do '1)' lists
}

#[derive(Debug, PartialEq, Clone)]
pub enum ListItem {
    Unordered(Box<Element>),                   // -
    Numbered(usize, Box<Element>),             // 1.
    Alphabetical(String, Box<Element>),        // a.
    NumberedRounded(usize, Box<Element>),      // 1)
    AlphabeticalRounded(String, Box<Element>), // a)
    List(Vec<ListItem>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Cap(isize),
    Ch(isize),
    Em(isize),
    Ex(isize), // this won't ever show up
    Ic(isize),
    Lh(isize),
    Rcap(isize),
    Rch(isize),
    Rem(isize),
    Ric(isize),
    Rlh(isize),
    Vh(isize),
    Vw(isize),
    Vmax(isize),
    Vb(isize),
    Vi(isize),
    Cqw(isize),
    Cqh(isize),
    Cqi(isize),
    Cqb(isize),
    Cqmin(isize),
    Cqmax(isize),
    Px(isize),
    Cm(isize),
    Mm(isize),
    Q(isize),
    In(isize),
    Pc(isize),
    Pt(isize),
    Percentage(isize)
}

#[derive(Debug, Clone)]
pub enum ParseUnitError {
    NotANumber,
    NotAUnit
}

impl std::str::FromStr for Unit {
    type Err = ParseUnitError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let num: String = string.chars().take_while(|c| c.is_digit(10)).collect();
        let text: String = string.chars().skip_while(|c| c.is_digit(10)).collect();
        let value = num.parse::<isize>()?;
        match text.as_str() {
            "cap" => Ok(Self::Cap(value)),
            "ch" => Ok(Self::Ch(value)),
            "em" => Ok(Self::Em(value)),
            "ex" => Ok(Self::Ex(value)),
            "ic" => Ok(Self::Ic(value)),
            "lh" => Ok(Self::Lh(value)),
            "rcap" => Ok(Self::Rcap(value)),
            "rch" => Ok(Self::Rch(value)),
            "rem" => Ok(Self::Rem(value)),
            "ric" => Ok(Self::Ric(value)),
            "rlh" => Ok(Self::Rlh(value)),
            "vh" => Ok(Self::Vh(value)),
            "vw" => Ok(Self::Vw(value)),
            "vmax" => Ok(Self::Vmax(value)),
            "vb" => Ok(Self::Vb(value)),
            "vi" => Ok(Self::Vi(value)),
            "cqw" => Ok(Self::Cqw(value)),
            "cqh" => Ok(Self::Cqh(value)),
            "cqi" => Ok(Self::Cqi(value)),
            "cqb" => Ok(Self::Cqb(value)),
            "cqmin" => Ok(Self::Cqmin(value)),
            "cqmax" => Ok(Self::Cqmax(value)),
            "px" => Ok(Self::Px(value)),
            "cm" => Ok(Self::Cm(value)),
            "mm" => Ok(Self::Mm(value)),
            "q"  => Ok(Self::Q(value)),
            "in" => Ok(Self::In(value)),
            "pc" => Ok(Self::Pc(value)),
            "pt" => Ok(Self::Pt(value)),
            "%" => Ok(Self::Percentage(value)),
            _ => Err(ParseUnitError::NotAUnit)
        }
    }
}

impl std::string::ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Cap(n) => format!("{n}cap"),
            Unit::Ch(n) => format!("{n}ch"),
            Unit::Em(n) => format!("{n}em"),
            Unit::Ex(n) => format!("{n}ex"),
            Unit::Ic(n) => format!("{n}ic"),
            Unit::Lh(n) => format!("{n}lh"),
            Unit::Rcap(n) => format!("{n}rcap"),
            Unit::Rch(n) => format!("{n}rch"),
            Unit::Rem(n) => format!("{n}rem"),
            Unit::Ric(n) => format!("{n}ric"),
            Unit::Rlh(n) => format!("{n}rlh"),
            Unit::Vh(n) => format!("{n}vh"),
            Unit::Vw(n) => format!("{n}vw"),
            Unit::Vmax(n) => format!("{n}vmax"),
            Unit::Vb(n) => format!("{n}vb"),
            Unit::Vi(n) => format!("{n}vi"),
            Unit::Cqw(n) => format!("{n}cqw"),
            Unit::Cqh(n) => format!("{n}cqh"),
            Unit::Cqi(n) => format!("{n}cqi"),
            Unit::Cqb(n) => format!("{n}cqb"),
            Unit::Cqmin(n) => format!("{n}cqmin"),
            Unit::Cqmax(n) => format!("{n}cqmax"),
            Unit::Px(n) => format!("{n}px"),
            Unit::Cm(n) => format!("{n}cm"),
            Unit::Mm(n) => format!("{n}mm"),
            Unit::Q(n) => format!("{n}q"),
            Unit::In(n) => format!("{n}in"),
            Unit::Pc(n) => format!("{n}pc"),
            Unit::Pt(n) => format!("{n}pt"),
            Unit::Percentage(n) => format!("{n}%"),
        }
    }
}

impl std::fmt::Display for ParseUnitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self.to_string();
        write!(f, "{text}")
    }
}

impl std::error::Error for ParseUnitError {}

impl std::convert::From<ParseIntError> for ParseUnitError {
    fn from(value: ParseIntError) -> Self {
        Self::NotANumber
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImageSize {
    Single(Unit),
    Double(Unit, Unit),
    None
}

#[derive(Debug, PartialEq, Clone)]
pub struct Image {
    pub src: String,
    pub alt: Box<Element>,
    pub size: ImageSize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Hoverable(Alternative),
    Styled(Alternative),
    Link(Alternative),
    Header(Box<Element>, usize),
    Italics(Box<Element>),
    Bold(Box<Element>),
    InlineCode(String),
    CodeBlock(String),
    Image(Image),
    // EmbeddedLink(String, String),
    FactBox(FactBox),
    Quote(Vec<Element>),
    List(Vec<ListItem>),
    Paragraph(Box<Element>),
    Text(String),
    Span(Span),
    Citation(String),
    Note(String),
    PageBreak,
    TOCLocationMarker,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssDownDocument {
    pub meta: MetaData,
    pub bibliography_id: String,
    pub notes_id: String,
    pub references: OrderedMap<String, ReferenceDefinition>,
    pub notes: OrderedMap<String, Element>,
    pub body: Vec<(Element, String)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FactBox {
    pub title: String,
    pub notes: OrderedMap::<String, (Element, String)>,
    pub body: Vec<(Element, String)>
}

#[derive(Debug, PartialEq, Clone)]
pub(super) enum MaybeElement {
    Yes((Element, String)),
    No
}

