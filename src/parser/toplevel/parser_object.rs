
use super::structs::*;
use std::str::pattern::Pattern;

#[derive(Debug, PartialEq)]
pub(in super::super) struct ParseObject {
    is_eating: bool,
    text: String,
    paragraph_buffer: String,
    pub(in super::super) syntax: Vec<TopLevelSyntax>
    //TODO(Paw): add some diagnostics structure in here
}

impl ParseObject {
    pub(in super::super) fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            is_eating: false,
            paragraph_buffer: String::new(),
            syntax: vec![]
        }
    }

    pub(in super::super) fn has_text(&self) -> bool {
        !self.text.is_empty()
    }
    
    pub(in super::super) fn text(&self) -> &str {
        self.text.as_str()
    }

    pub(in super::super) fn current(&self) -> &str {
        self.text.lines().nth(0).unwrap_or("")
    }
    
    pub(in super::super) fn next(&mut self) {
        self.skip(self.current().len() + 1);
    }

    pub(in super::super) fn trimmed_line_find<T: Pattern>(&self, pat: T, offset: usize) -> Option<usize> {
        let full_line = self.current();
        let trimmed_line = full_line.trim_start(); 
        if offset >= trimmed_line.len() { return None }

        let trim_count = full_line.len() - trimmed_line.len();
        let line = &trimmed_line[offset..];
        if let Some(index) = line.find(pat) {
            Some(index + offset + trim_count)
        } else {
            None
        }
    }
    
    pub(in super::super) fn trimmed_find<T: Pattern>(&self, pat: T, offset: usize) -> Option<usize> {
        let full_line = &self.text;
        let trimmed_line = full_line.trim_start(); 
        if offset >= trimmed_line.len() { return None }

        let trim_count = full_line.len() - trimmed_line.len();
        let line = &trimmed_line[offset..];
        if let Some(index) = line.find(pat) {
            Some(index + offset + trim_count)
        } else {
            None
        }
    }
    
    pub(in super::super) fn find<T: Pattern>(&self, pat: T, offset: usize) -> Option<usize> {
        let text = &self.text;
        if offset >= text.len() { return None }

        let line = &text[offset..];
        if let Some(index) = line.find(pat) {
            Some(index + offset)
        } else {
            None
        }
    }

    pub(in super::super) fn starts_with<T: std::str::pattern::Pattern>(&self, pat: T) -> bool {
        self.current().starts_with(pat)
    }

    pub(in super::super) fn skip(&mut self, offset: usize) {
        if offset >= self.text.len() {
            self.text = String::new();
        } else {
            self.text = self.text[offset..].to_string();
        }
    }
    
    pub(in super::super) fn yoink(&mut self, offset: usize, len: usize) -> String {
        let output = self.text[offset..len].to_string();
        self.text.replace_range(offset..len, "");
        output
    }

    pub(in super::super) fn eat(&mut self, text: &str) {
        self.is_eating = true;
        self.paragraph_buffer += text;
        self.paragraph_buffer.push('\n');
    }

    pub(in super::super) fn eat_line(&mut self) {
        // this is stupid this should just be a one line function,
        // but because of the brilliant borrow checker it has to 
        // be inlined manually, the code I wanted to write was
        // self.eat(self.current())
        // which is obviously fine, but the borrow checker can't
        // prove that it is
        let line = self.text.lines().nth(0).unwrap_or("");
        self.is_eating = true;
        self.paragraph_buffer += line;
        self.paragraph_buffer.push('\n');
        if !self.text.is_empty() {
            self.text = self.text[line.len() + 1..].to_string();
        }
    }

    pub(in super::super) fn consume(&mut self) {
        if self.is_eating {
            let text = self.paragraph_buffer.trim();
            if !text.is_empty() {
                self.syntax.push(TopLevelSyntax::Paragraph(format!("{text}\n")));
            }
            self.is_eating = false;
            self.paragraph_buffer.clear();
        }
    }

    pub(in super::super) fn push(&mut self, syntax: TopLevelSyntax) {
        self.consume();
        self.syntax.push(syntax);
    }
}

macro_rules! impl_for_all {
    (impl $trait:ident for ($($type:ty),*) $body:tt ) => {
        $(
            impl $trait for $type $body
        )*
    }
}

pub trait TrimmedStartsWith {
    fn trimmed_starts_with<T: std::str::pattern::Pattern>(&self, text: T) -> bool;
}

impl_for_all! {
    impl TrimmedStartsWith for (&str, &String, String) {
        fn trimmed_starts_with<T: std::str::pattern::Pattern>(&self, text: T) -> bool {
            self.trim_start().starts_with(text)
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn creation() {
        let obj = ParseObject::new("");
        assert_eq!(obj, ParseObject{ is_eating: false, text: "".to_string(), paragraph_buffer: String::new(), syntax: vec![] })
    }
    
    #[test]
    fn has_text_none() {
        let empty = ParseObject::new("");
        assert_eq!(empty.has_text(), false);
    }
    
    #[test]
    fn has_text_some() {
        let with_content = ParseObject::new("Scream into the void please, thanks");
        assert_eq!(with_content.has_text(), true);
    }
    
    #[test]
    fn current_empty() {
        let empty = ParseObject::new("");
        assert_eq!(empty.current(), "");
    }
    #[test]
    fn current_single_line() {
        let single_line = ParseObject::new("Scream into the void please, thanks");
        assert_eq!(single_line.current(), "Scream into the void please, thanks");
    }
    #[test]
    fn current_multiple_lines() {
        let multiple_lines = ParseObject::new("Scream into the void please, thanks\nNow stare into the abyss please");
        assert_eq!(multiple_lines.current(), "Scream into the void please, thanks");
    }
    
    #[test]
    fn find_trimmed_line_no_offset_empty() {
        let empty = ParseObject::new("");
        assert_eq!(empty.trimmed_line_find("hi", 0), None);
    }
    #[test]
    fn find_trimmed_line_no_offset_single_line() {
        let single_line = ParseObject::new("hi hello goodbye");
        assert_eq!(single_line.trimmed_line_find("hi", 0), Some(0));
        assert_eq!(single_line.trimmed_line_find("hello", 0), Some(3));
        assert_eq!(single_line.trimmed_line_find("goodbye", 0), Some(9));
    }
    #[test]
    fn find_trimmed_line_no_offset_whitespaced_single_line() {
        let witespaced_single_line = ParseObject::new("   hi hello goodbye   ");
        assert_eq!(witespaced_single_line.trimmed_line_find("hi", 0), Some(3));
        assert_eq!(witespaced_single_line.trimmed_line_find("hello", 0), Some(6));
        assert_eq!(witespaced_single_line.trimmed_line_find("goodbye", 0), Some(12));
    }
    #[test]
    fn find_trimmed_line_no_offset_multiple_lines() {
        let multiple_lines = ParseObject::new("hi hello\ngoodbye");
        assert_eq!(multiple_lines.trimmed_line_find("hi", 0), Some(0));
        assert_eq!(multiple_lines.trimmed_line_find("hello", 0), Some(3));
        assert_eq!(multiple_lines.trimmed_line_find("goodbye", 0), None);
    }
    #[test]
    fn find_trimmed_line_no_offset_whitespaced_multiple_lines() {
        let witespaced_single_line = ParseObject::new("   hi hello\ngoodbye   ");
        assert_eq!(witespaced_single_line.trimmed_line_find("hi", 0), Some(3));
        assert_eq!(witespaced_single_line.trimmed_line_find("hello", 0), Some(6));
        assert_eq!(witespaced_single_line.trimmed_line_find("goodbye", 0), None);
    }
    
    #[test]
    fn find_trimmed_line_valid_offset() {
        let witespaced_single_line = ParseObject::new("   hi hi goodbye   ");
        assert_eq!(witespaced_single_line.trimmed_line_find("hi", 2), Some(6));
    }
    
    #[test]
    fn find_trimmed_line_invalid_offset() {
        let witespaced_single_line = ParseObject::new("   hi hi goodbye   ");
        assert_eq!(witespaced_single_line.trimmed_line_find("hi", 800), None);
    }
    
    #[test]
    fn find_trimmed() {
        assert!(false);
    }
    
    #[test]
    fn find_next() {
        assert!(false);
    }
    
    #[test]
    fn find_skip() {
        assert!(false);
    }
    
    #[test]
    fn eat() {
        let mut obj = ParseObject::new("");
        obj.eat("text");
        assert_eq!(obj, 
            ParseObject{ 
                is_eating: true, 
                text: "".to_string(), 
                paragraph_buffer: "text\n".to_string(), 
                syntax: vec![] 
            }
        );
    }
    
    #[test]
    fn consume_none() {
        let mut obj = ParseObject::new("");
        obj.consume();
        assert_eq!(obj, 
            ParseObject{ 
                is_eating: false, 
                text: "".to_string(), 
                paragraph_buffer: String::new(), 
                syntax: vec![] 
            }
        );
    }
    
    #[test]
    fn consume_some() {
        let mut obj = ParseObject::new("");
        obj.eat("text");
        obj.consume();
        assert_eq!(obj, 
            ParseObject{ 
                is_eating: false, 
                text: "".to_string(), 
                paragraph_buffer: String::new(), 
                syntax: vec![TopLevelSyntax::Paragraph("text\n".to_string())] 
            }
        );
    }
    
    #[test]
    fn consume_without_eating() {
        let mut obj = ParseObject::new("");
        obj.push(TopLevelSyntax::PageBreak);
        assert_eq!(obj, 
            ParseObject{ 
                is_eating: false, 
                text: "".to_string(), 
                paragraph_buffer: String::new(), 
                syntax: vec![TopLevelSyntax::PageBreak] 
            }
        );
    }
    
    #[test]
    fn consume_after_eating() {
        let mut obj = ParseObject::new("");
        obj.eat("text");
        obj.push(TopLevelSyntax::PageBreak);
        assert_eq!(obj, 
            ParseObject{ 
                is_eating: false, 
                text: "".to_string(), 
                paragraph_buffer: String::new(), 
                syntax: vec![
                    TopLevelSyntax::Paragraph("text\n".to_string()),
                    TopLevelSyntax::PageBreak
                ] 
            }
        );
    }

    mod trimmed_start {
        use super::*;

        #[test]
        fn on_empty_str() {
            let text = "";
            assert_eq!(text.trimmed_starts_with("hi"), false)
        }
        #[test]
        fn on_empty_string() {
            let text = "".to_string();
            assert_eq!(text.trimmed_starts_with("hi"), false)
        }
        #[test]
        fn on_empty_string_ref() {
            let text = &"".to_string();
            assert_eq!(text.trimmed_starts_with("hi"), false)
        }
        
        #[test]
        fn on_wrong_str() {
            let text = "this string does not have what we are looking for";
            assert_eq!(text.trimmed_starts_with("hi"), false)
        }
        #[test]
        fn on_wrong_string() {
            let text = "this string does not have what we are looking for"
                        .to_string();
            assert_eq!(text.trimmed_starts_with("hi"), false)
        }
        #[test]
        fn on_wrong_string_ref() {
            let text = &"this string does not have what we are looking for"
                        .to_string();
            assert_eq!(text.trimmed_starts_with("hi"), false)
        }
        
        #[test]
        fn on_basic_str() {
            let text = "hi is the first two characters of this string";
            assert_eq!(text.trimmed_starts_with("hi"), true)
        }
        #[test]
        fn on_basic_string() {
            let text = "hi is the first two characters of this string"
                        .to_string();
            assert_eq!(text.trimmed_starts_with("hi"), true)
        }
        #[test]
        fn on_basic_string_ref() {
            let text = &"hi is the first two characters of this string"
                        .to_string();
            assert_eq!(text.trimmed_starts_with("hi"), true)
        }
        
        #[test]
        fn no_inner_whitespace_str() {
            let text = "hiisthefirsttwocharactersofthisstring";
            assert_eq!(text.trimmed_starts_with("hi"), true)
        }
        #[test]
        fn no_inner_whitespace_string() {
            let text = "hiisthefirsttwocharactersofthisstring"
                        .to_string();
            assert_eq!(text.trimmed_starts_with("hi"), true)
        }
        #[test]
        fn no_inner_whitespace_string_ref() {
            let text = &"hiisthefirsttwocharactersofthisstring"
                        .to_string();
            assert_eq!(text.trimmed_starts_with("hi"), true)
        }
        
        #[test]
        fn with_outer_whitespace_str() {
            let text = "             hi is the first two characters of this string";
            assert_eq!(text.trimmed_starts_with("hi"), true)
        }
        #[test]
        fn with_outer_whitespace_string() {
            let text = "             hi is the first two characters of this string"
                        .to_string();
            assert_eq!(text.trimmed_starts_with("hi"), true)
        }
        #[test]
        fn with_outer_whitespace_string_ref() {
            let text = &"             hi is the first two characters of this string"
                        .to_string();
            assert_eq!(text.trimmed_starts_with("hi"), true);
        }
    }
}

